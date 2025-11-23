// Re-export all functions from @tego/bot
export * from "@tego/bot";

// Import functions needed for helper methods
import { mouseClick, mouseToggle, moveMouse } from "@tego/bot";

// ============================================================================
// Mouse Shortcut Methods
// ============================================================================

/**
 * Perform a double click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 */
export function doubleClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    moveMouse(x, y);
  }
  mouseClick(undefined, true);
}

/**
 * Perform a right click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 */
export function rightClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    moveMouse(x, y);
  }
  mouseClick("right", false);
}

/**
 * Perform a middle click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 */
export function middleClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    moveMouse(x, y);
  }
  mouseClick("middle", false);
}

/**
 * Perform a left click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 */
export function leftClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    moveMouse(x, y);
  }
  mouseClick("left", false);
}

/**
 * Click and hold the mouse button down
 * @param button - Mouse button to hold ("left", "right", or "middle"), defaults to "left"
 */
export function mouseDown(button: "left" | "right" | "middle" = "left"): void {
  mouseToggle("down", button);
}

/**
 * Release the mouse button
 * @param button - Mouse button to release ("left", "right", or "middle"), defaults to "left"
 */
export function mouseUp(button: "left" | "right" | "middle" = "left"): void {
  mouseToggle("up", button);
}
