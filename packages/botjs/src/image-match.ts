/**
 * Image Template Matching Module
 *
 * Provides image template matching functionality for finding UI elements on screen
 * by searching for template images with configurable confidence and scale settings.
 *
 * @example
 * ```typescript
 * import { imageResource, findOnScreen, getMatchCenter, moveMouse, mouseClick } from "@tego/botjs";
 *
 * const button = await imageResource("./button.png");
 * const match = await findOnScreen(button, { confidence: 0.8 });
 *
 * if (match) {
 *   const center = getMatchCenter(match);
 *   moveMouse(center.x, center.y);
 *   mouseClick();
 * }
 * ```
 *
 * @module image-match
 */

import * as fs from "node:fs";
import * as bot from "@tego/bot";

// ============================================================================
// Types
// ============================================================================

/**
 * Configuration options for image template matching
 */
export interface MatchConfig {
  /**
   * Search at multiple scales to find scaled versions of the template.
   * Useful when UI elements may be displayed at different sizes.
   * @default true
   */
  searchMultipleScales?: boolean;

  /**
   * Convert images to grayscale before matching.
   * Can improve matching for color-independent patterns.
   * @default false
   */
  useGrayscale?: boolean;

  /**
   * Scale factors to search at when searchMultipleScales is true.
   * Values less than 1.0 search for smaller versions of the template.
   * @default [1.0, 0.9, 0.8, 0.7, 0.6, 0.5]
   */
  scaleSteps?: number[];

  /**
   * Minimum confidence threshold (0.0 to 1.0).
   * Higher values require closer matches but may miss valid results.
   * Lower values find more matches but may include false positives.
   * @default 0.8
   */
  confidence?: number;

  /**
   * Maximum number of results to return.
   * Results are sorted by confidence descending.
   * @default 100
   */
  limit?: number;
}

/**
 * Result from a successful image template match
 */
export interface MatchResult {
  /** X coordinate of the match (top-left corner) */
  x: number;
  /** Y coordinate of the match (top-left corner) */
  y: number;
  /** Width of the matched region */
  width: number;
  /** Height of the matched region */
  height: number;
  /** Confidence score from 0.0 to 1.0 (higher = better match) */
  confidence: number;
  /** Scale at which the template was matched (1.0 = original size) */
  scale: number;
}

/**
 * An image resource that can be used for template matching.
 * Contains the PNG-encoded image buffer.
 */
export interface ImageResource {
  /** PNG-encoded image buffer */
  buffer: Buffer;
  /** Original file path (if loaded from file) */
  path?: string;
}

// ============================================================================
// Image Loading Functions
// ============================================================================

/**
 * Load an image from file for use as a template
 *
 * @param path - Path to the image file (PNG, JPG, BMP, etc.)
 * @returns Promise resolving to ImageResource
 *
 * @example
 * ```typescript
 * import { imageResource, findOnScreen } from "@tego/botjs";
 *
 * const button = await imageResource("./assets/button.png");
 * const match = await findOnScreen(button);
 * ```
 */
export async function imageResource(path: string): Promise<ImageResource> {
  const buffer = await fs.promises.readFile(path);
  return {
    buffer,
    path,
  };
}

/**
 * Load an image synchronously from file for use as a template
 *
 * @param path - Path to the image file (PNG, JPG, BMP, etc.)
 * @returns ImageResource
 *
 * @example
 * ```typescript
 * import { imageResourceSync, findOnScreen } from "@tego/botjs";
 *
 * const icon = imageResourceSync("./assets/icon.png");
 * const match = await findOnScreen(icon);
 * ```
 */
export function imageResourceSync(path: string): ImageResource {
  const buffer = fs.readFileSync(path);
  return {
    buffer,
    path,
  };
}

/**
 * Create an image resource from a Buffer
 *
 * @param buffer - PNG, JPG, or other image format buffer
 * @returns ImageResource
 *
 * @example
 * ```typescript
 * import { imageResourceFromBuffer, findOnScreen, captureScreenRegion } from "@tego/botjs";
 *
 * // Capture a region and use it as template
 * const capture = await captureScreenRegion(100, 100, 50, 50);
 * const template = imageResourceFromBuffer(capture.image);
 * const matches = await findAllOnScreen(template);
 * ```
 */
export function imageResourceFromBuffer(buffer: Buffer): ImageResource {
  return { buffer };
}

// ============================================================================
// Match Configuration Conversion
// ============================================================================

function toMatchConfigJs(config?: MatchConfig): bot.MatchConfigJs | undefined {
  if (!config) return undefined;
  return {
    searchMultipleScales: config.searchMultipleScales,
    useGrayscale: config.useGrayscale,
    scaleSteps: config.scaleSteps,
    confidence: config.confidence,
    limit: config.limit,
  };
}

function fromMatchResultJs(result: bot.MatchResultJs): MatchResult {
  return {
    x: result.x,
    y: result.y,
    width: result.width,
    height: result.height,
    confidence: result.confidence,
    scale: result.scale,
  };
}

// ============================================================================
// Core Matching Functions
// ============================================================================

/**
 * Find first match of template image on screen
 *
 * @param template - ImageResource to search for
 * @param config - Optional matching configuration
 * @returns Promise resolving to MatchResult or null if not found
 *
 * @example
 * ```typescript
 * import { imageResource, findOnScreen, moveMouse, mouseClick } from "@tego/botjs";
 *
 * const button = await imageResource("./button.png");
 * const match = await findOnScreen(button, { confidence: 0.85 });
 *
 * if (match) {
 *   console.log(`Found at (${match.x}, ${match.y}) with ${match.confidence * 100}% confidence`);
 *   const center = getMatchCenter(match);
 *   moveMouse(center.x, center.y);
 *   mouseClick();
 * } else {
 *   console.log("Button not found on screen");
 * }
 * ```
 */
export async function findOnScreen(
  template: ImageResource,
  config?: MatchConfig,
): Promise<MatchResult | null> {
  const result = await bot.findOnScreen(
    template.buffer,
    toMatchConfigJs(config),
  );
  return result ? fromMatchResultJs(result) : null;
}

/**
 * Find all matches of template image on screen
 *
 * @param template - ImageResource to search for
 * @param config - Optional matching configuration
 * @returns Promise resolving to array of MatchResults, sorted by confidence descending
 *
 * @example
 * ```typescript
 * import { imageResource, findAllOnScreen, moveMouse, mouseClick } from "@tego/botjs";
 *
 * const icon = await imageResource("./checkbox.png");
 * const matches = await findAllOnScreen(icon, { confidence: 0.7, limit: 10 });
 *
 * console.log(`Found ${matches.length} checkboxes`);
 *
 * // Click each checkbox
 * for (const match of matches) {
 *   const center = getMatchCenter(match);
 *   moveMouse(center.x, center.y);
 *   mouseClick();
 *   await sleep(100);
 * }
 * ```
 */
export async function findAllOnScreen(
  template: ImageResource,
  config?: MatchConfig,
): Promise<MatchResult[]> {
  const results = await bot.findAllOnScreen(
    template.buffer,
    toMatchConfigJs(config),
  );
  return results.map(fromMatchResultJs);
}

/**
 * Find first match of template image in a specific screen region
 *
 * @param template - ImageResource to search for
 * @param x - X coordinate of search region
 * @param y - Y coordinate of search region
 * @param width - Width of search region
 * @param height - Height of search region
 * @param config - Optional matching configuration
 * @returns Promise resolving to MatchResult or null (coordinates are absolute screen coordinates)
 *
 * @example
 * ```typescript
 * import { imageResource, findInRegion } from "@tego/botjs";
 *
 * const button = await imageResource("./button.png");
 * // Search only in the left half of a 1920x1080 screen
 * const match = await findInRegion(button, 0, 0, 960, 1080, { confidence: 0.8 });
 *
 * if (match) {
 *   console.log(`Found at (${match.x}, ${match.y})`);
 * }
 * ```
 */
export async function findInRegion(
  template: ImageResource,
  x: number,
  y: number,
  width: number,
  height: number,
  config?: MatchConfig,
): Promise<MatchResult | null> {
  const result = await bot.findInRegion(
    template.buffer,
    x,
    y,
    width,
    height,
    toMatchConfigJs(config),
  );
  return result ? fromMatchResultJs(result) : null;
}

/**
 * Find all matches of template image in a specific screen region
 *
 * @param template - ImageResource to search for
 * @param x - X coordinate of search region
 * @param y - Y coordinate of search region
 * @param width - Width of search region
 * @param height - Height of search region
 * @param config - Optional matching configuration
 * @returns Promise resolving to array of MatchResults (coordinates are absolute screen coordinates)
 *
 * @example
 * ```typescript
 * import { imageResource, findAllInRegion, getActiveWindow } from "@tego/botjs";
 *
 * const icon = await imageResource("./icon.png");
 * const win = getActiveWindow();
 *
 * // Search only within the active window
 * const matches = await findAllInRegion(
 *   icon,
 *   win.x, win.y, win.width, win.height,
 *   { confidence: 0.75 }
 * );
 * ```
 */
export async function findAllInRegion(
  template: ImageResource,
  x: number,
  y: number,
  width: number,
  height: number,
  config?: MatchConfig,
): Promise<MatchResult[]> {
  const results = await bot.findAllInRegion(
    template.buffer,
    x,
    y,
    width,
    height,
    toMatchConfigJs(config),
  );
  return results.map(fromMatchResultJs);
}

// ============================================================================
// Utility Functions
// ============================================================================

/**
 * Get the center point of a match result
 *
 * @param match - MatchResult to calculate center for
 * @returns Object with x and y coordinates of the center
 *
 * @example
 * ```typescript
 * import { imageResource, findOnScreen, getMatchCenter, moveMouse, mouseClick } from "@tego/botjs";
 *
 * const button = await imageResource("./button.png");
 * const match = await findOnScreen(button);
 *
 * if (match) {
 *   const center = getMatchCenter(match);
 *   moveMouse(center.x, center.y);
 *   mouseClick();
 * }
 * ```
 */
export function getMatchCenter(match: MatchResult): { x: number; y: number } {
  return {
    x: Math.round(match.x + match.width / 2),
    y: Math.round(match.y + match.height / 2),
  };
}

/**
 * Get the bounding rectangle of a match result
 *
 * @param match - MatchResult to get bounds for
 * @returns Object with left, top, right, bottom coordinates
 *
 * @example
 * ```typescript
 * import { imageResource, findOnScreen, getMatchBounds } from "@tego/botjs";
 *
 * const match = await findOnScreen(template);
 * if (match) {
 *   const bounds = getMatchBounds(match);
 *   console.log(`Match bounds: (${bounds.left}, ${bounds.top}) to (${bounds.right}, ${bounds.bottom})`);
 * }
 * ```
 */
export function getMatchBounds(match: MatchResult): {
  left: number;
  top: number;
  right: number;
  bottom: number;
} {
  return {
    left: match.x,
    top: match.y,
    right: match.x + match.width,
    bottom: match.y + match.height,
  };
}

/**
 * Wait for a template image to appear on screen
 *
 * @param template - ImageResource to wait for
 * @param timeout - Maximum time to wait in milliseconds (default: 10000)
 * @param interval - Time between checks in milliseconds (default: 500)
 * @param config - Optional matching configuration
 * @returns Promise resolving to MatchResult when found, or null if timeout
 *
 * @example
 * ```typescript
 * import { imageResource, waitFor, getMatchCenter, moveMouse, mouseClick } from "@tego/botjs";
 *
 * const dialog = await imageResource("./dialog.png");
 *
 * // Wait up to 5 seconds for dialog to appear
 * const match = await waitFor(dialog, 5000, 250, { confidence: 0.9 });
 *
 * if (match) {
 *   console.log("Dialog appeared!");
 *   const center = getMatchCenter(match);
 *   moveMouse(center.x, center.y);
 *   mouseClick();
 * } else {
 *   console.log("Dialog did not appear within timeout");
 * }
 * ```
 */
export async function waitFor(
  template: ImageResource,
  timeout: number = 10000,
  interval: number = 500,
  config?: MatchConfig,
): Promise<MatchResult | null> {
  const startTime = Date.now();

  while (Date.now() - startTime < timeout) {
    const match = await findOnScreen(template, config);
    if (match) {
      return match;
    }
    await sleep(interval);
  }

  return null;
}

/**
 * Wait for a template image to disappear from screen
 *
 * @param template - ImageResource to wait for disappearance
 * @param timeout - Maximum time to wait in milliseconds (default: 10000)
 * @param interval - Time between checks in milliseconds (default: 500)
 * @param config - Optional matching configuration
 * @returns Promise resolving to true if disappeared, false if timeout
 *
 * @example
 * ```typescript
 * import { imageResource, waitForGone, mouseClick } from "@tego/botjs";
 *
 * const loadingSpinner = await imageResource("./loading.png");
 *
 * // Wait up to 30 seconds for loading spinner to disappear
 * const isGone = await waitForGone(loadingSpinner, 30000, 1000);
 *
 * if (isGone) {
 *   console.log("Loading complete!");
 * } else {
 *   console.log("Loading took too long");
 * }
 * ```
 */
export async function waitForGone(
  template: ImageResource,
  timeout: number = 10000,
  interval: number = 500,
  config?: MatchConfig,
): Promise<boolean> {
  const startTime = Date.now();

  while (Date.now() - startTime < timeout) {
    const match = await findOnScreen(template, config);
    if (!match) {
      return true;
    }
    await sleep(interval);
  }

  return false;
}

// ============================================================================
// Helper Functions
// ============================================================================

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms));
}
