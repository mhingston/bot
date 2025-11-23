// ============================================================================
// Explicit exports from @tego/bot for better TypeDoc documentation
// ============================================================================

/**
 * Keyboard automation class for simulating keyboard input
 * @example
 * ```typescript
 * const keyboard = new Keyboard();
 * keyboard.keyTap('a');
 * keyboard.typeString('Hello World');
 * ```
 */
export { Keyboard } from "@tego/bot";

/**
 * Mouse automation class for controlling mouse movements and clicks
 * @example
 * ```typescript
 * const mouse = new Mouse();
 * mouse.moveMouse(100, 200);
 * mouse.mouseClick('left');
 * ```
 */
export { Mouse } from "@tego/bot";

/**
 * Screen capture class for taking screenshots
 * @example
 * ```typescript
 * const screen = new Screen();
 * const bitmap = await screen.capture(0, 0, 800, 600);
 * ```
 */
export { Screen } from "@tego/bot";

// Types/Interfaces
export type {
  Bitmap,
  MousePosition,
  PixelColor,
  ScreenCapture,
  ScreenSize,
  WindowInfo,
} from "@tego/bot";

// ============================================================================
// Mouse Functions
// ============================================================================

/**
 * Move the mouse cursor to the specified coordinates instantly
 * @param x - X coordinate
 * @param y - Y coordinate
 * @example
 * ```typescript
 * moveMouse(100, 200);
 * ```
 */
export { moveMouse } from "@tego/bot";

/**
 * Move the mouse cursor smoothly to the specified coordinates with easing
 * @param x - X coordinate
 * @param y - Y coordinate
 * @param speed - Optional movement speed (default: 3.0)
 * @example
 * ```typescript
 * moveMouseSmooth(500, 500);
 * moveMouseSmooth(500, 500, 5.0); // faster movement
 * ```
 */
export { moveMouseSmooth } from "@tego/bot";

/**
 * Click the mouse button at the current position
 * @param button - Mouse button: "left", "right", or "middle" (default: "left")
 * @param double - Whether to perform a double click (default: false)
 * @example
 * ```typescript
 * mouseClick('left');
 * mouseClick('right', true); // right double-click
 * ```
 */
export { mouseClick } from "@tego/bot";

/**
 * Toggle mouse button state (press or release)
 * @param down - "down" to press, "up" to release
 * @param button - Mouse button: "left", "right", or "middle" (default: "left")
 * @example
 * ```typescript
 * mouseToggle('down', 'left');
 * // ... do something ...
 * mouseToggle('up', 'left');
 * ```
 */
export { mouseToggle } from "@tego/bot";

/**
 * Drag the mouse from current position to target coordinates
 * @param x - Target X coordinate
 * @param y - Target Y coordinate
 * @example
 * ```typescript
 * moveMouse(100, 100);
 * dragMouse(500, 500); // drag from (100,100) to (500,500)
 * ```
 */
export { dragMouse } from "@tego/bot";

/**
 * Scroll the mouse wheel
 * @param x - Horizontal scroll amount (positive = right, negative = left)
 * @param y - Vertical scroll amount (positive = down, negative = up)
 * @example
 * ```typescript
 * scrollMouse(0, 3);  // scroll down
 * scrollMouse(0, -3); // scroll up
 * scrollMouse(2, 0);  // scroll right
 * ```
 */
export { scrollMouse } from "@tego/bot";

/**
 * Get the current mouse cursor position
 * @returns Object with x and y coordinates
 * @example
 * ```typescript
 * const pos = getMousePos();
 * console.log(`Mouse at: ${pos.x}, ${pos.y}`);
 * ```
 */
export { getMousePos } from "@tego/bot";

/**
 * Set the delay between mouse operations in milliseconds
 * @param delay - Delay in milliseconds
 * @example
 * ```typescript
 * setMouseDelay(50); // 50ms delay between operations
 * ```
 */
export { setMouseDelay } from "@tego/bot";

// ============================================================================
// Keyboard Functions
// ============================================================================

/**
 * Tap a key (press and release)
 * @param key - Key to tap (e.g., 'a', 'enter', 'f1')
 * @param modifier - Optional modifier keys array (e.g., ['control', 'shift'])
 * @example
 * ```typescript
 * keyTap('a');
 * keyTap('c', ['control']); // Ctrl+C
 * keyTap('v', ['control', 'shift']); // Ctrl+Shift+V
 * ```
 */
export { keyTap } from "@tego/bot";

/**
 * Toggle a key state (press or release)
 * @param key - Key to toggle
 * @param down - "down" to press, "up" to release
 * @param modifier - Optional modifier keys array
 * @example
 * ```typescript
 * keyToggle('shift', 'down');
 * keyTap('a'); // types 'A'
 * keyToggle('shift', 'up');
 * ```
 */
export { keyToggle } from "@tego/bot";

/**
 * Type a string of text
 * @param string - Text to type
 * @example
 * ```typescript
 * typeString('Hello, World!');
 * typeString('user@example.com');
 * ```
 */
export { typeString } from "@tego/bot";

/**
 * Type a string with a specified delay between characters
 * @param string - Text to type
 * @param cpm - Characters per minute (typing speed)
 * @example
 * ```typescript
 * typeStringDelayed('Hello', 300); // 300 CPM (slow typing)
 * typeStringDelayed('Fast typing', 1000); // 1000 CPM
 * ```
 */
export { typeStringDelayed } from "@tego/bot";

/**
 * Tap a Unicode character by its code point
 * @param value - Unicode code point (e.g., 0x1F600 for 😀)
 * @example
 * ```typescript
 * unicodeTap(0x1F600); // types 😀
 * unicodeTap(0x2764);  // types ❤
 * ```
 */
export { unicodeTap } from "@tego/bot";

/**
 * Set the delay between keyboard operations in milliseconds
 * @param ms - Delay in milliseconds
 * @example
 * ```typescript
 * setKeyboardDelay(10); // 10ms delay between key presses
 * ```
 */
export { setKeyboardDelay } from "@tego/bot";

// ============================================================================
// Screen Functions
// ============================================================================

/**
 * Get color at specific coordinates in a bitmap
 * @param bitmap - Bitmap object
 * @param x - X coordinate
 * @param y - Y coordinate
 * @returns Hex color string (e.g., "#FF0000")
 * @example
 * ```typescript
 * const bitmap = await captureScreen();
 * const color = bitmapColorAt(bitmap, 100, 200);
 * console.log(color); // "#FF0000"
 * ```
 */
export { bitmapColorAt } from "@tego/bot";

/**
 * Capture the entire screen
 * @returns Promise resolving to screen capture data with PNG buffer
 * @example
 * ```typescript
 * const screenshot = await captureScreen();
 * fs.writeFileSync('screenshot.png', screenshot.image);
 * ```
 */
export { captureScreen } from "@tego/bot";

/**
 * Capture a specific region of the screen
 * @param x - X coordinate of top-left corner
 * @param y - Y coordinate of top-left corner
 * @param width - Width of the region
 * @param height - Height of the region
 * @returns Promise resolving to screen capture data with PNG buffer
 * @example
 * ```typescript
 * const region = await captureScreenRegion(100, 100, 800, 600);
 * fs.writeFileSync('region.png', region.image);
 * ```
 */
export { captureScreenRegion } from "@tego/bot";

/**
 * Get the color of a pixel at specific coordinates
 * @param x - X coordinate
 * @param y - Y coordinate
 * @returns Promise resolving to hex color string (e.g., "#FF0000")
 * @example
 * ```typescript
 * const color = await getPixelColor(100, 200);
 * console.log(color); // "#FF0000"
 * ```
 */
export { getPixelColor } from "@tego/bot";

/**
 * Get the global screen instance
 * @returns Screen object for capture operations
 * @example
 * ```typescript
 * const screen = getScreen();
 * const bitmap = await screen.capture();
 * ```
 */
export { getScreen } from "@tego/bot";

/**
 * Get the screen dimensions
 * @returns Object with width and height in pixels
 * @example
 * ```typescript
 * const size = getScreenSize();
 * console.log(`Screen: ${size.width}x${size.height}`);
 * ```
 */
export { getScreenSize } from "@tego/bot";

/**
 * Update screen metrics (refresh monitor information)
 * @example
 * ```typescript
 * updateScreenMetrics();
 * const size = getScreenSize(); // get updated size
 * ```
 */
export { updateScreenMetrics } from "@tego/bot";

// ============================================================================
// Clipboard Functions
// ============================================================================

/**
 * Get text content from the clipboard
 * @returns Current clipboard text
 * @example
 * ```typescript
 * const text = getClipboard();
 * console.log('Clipboard:', text);
 * ```
 */
export { getClipboard } from "@tego/bot";

/**
 * Set text content to the clipboard
 * @param text - Text to copy to clipboard
 * @example
 * ```typescript
 * setClipboard('Hello from @tego/bot!');
 * ```
 */
export { setClipboard } from "@tego/bot";

/**
 * Clear the clipboard contents
 * @example
 * ```typescript
 * clearClipboard();
 * ```
 */
export { clearClipboard } from "@tego/bot";

/**
 * Get image from clipboard as PNG buffer
 * @returns PNG-encoded image buffer
 * @example
 * ```typescript
 * const image = getClipboardImage();
 * fs.writeFileSync('clipboard.png', image);
 * ```
 */
export { getClipboardImage } from "@tego/bot";

/**
 * Set image to clipboard from PNG buffer
 * @param imageBuffer - PNG-encoded image buffer
 * @example
 * ```typescript
 * const imageData = fs.readFileSync('image.png');
 * setClipboardImage(imageData);
 * ```
 */
export { setClipboardImage } from "@tego/bot";

// ============================================================================
// Window Management Functions
// ============================================================================

/**
 * Get information about the currently active (focused) window
 * @returns WindowInfo object with title, process, position, and size
 * @example
 * ```typescript
 * const activeWindow = getActiveWindow();
 * console.log(`Active: ${activeWindow.title}`);
 * console.log(`Process: ${activeWindow.processPath}`);
 * console.log(`Position: ${activeWindow.x}, ${activeWindow.y}`);
 * console.log(`Size: ${activeWindow.width}x${activeWindow.height}`);
 * ```
 */
export { getActiveWindow } from "@tego/bot";

/**
 * Get a list of all visible windows
 * Note: Currently returns only the active window due to API limitations
 * @returns Array of WindowInfo objects
 * @example
 * ```typescript
 * const windows = getAllWindows();
 * console.log(`Found ${windows.length} windows`);
 * windows.forEach(win => console.log(win.title));
 * ```
 */
export { getAllWindows } from "@tego/bot";

/**
 * Find windows by title using case-insensitive partial match
 * Note: Currently searches only the active window due to API limitations
 * @param title - Title text to search for
 * @returns Array of matching WindowInfo objects
 * @example
 * ```typescript
 * const chromeWindows = findWindowsByTitle('chrome');
 * chromeWindows.forEach(win => console.log(win.title));
 * ```
 */
export { findWindowsByTitle } from "@tego/bot";

/**
 * Find windows by process name using case-insensitive partial match
 * Note: Currently searches only the active window due to API limitations
 * @param processName - Process name to search for
 * @returns Array of matching WindowInfo objects
 * @example
 * ```typescript
 * const vscodeWindows = findWindowsByProcess('code');
 * vscodeWindows.forEach(win => console.log(win.title));
 * ```
 */
export { findWindowsByProcess } from "@tego/bot";

// ============================================================================
// Mouse Shortcut Methods (botjs-specific helpers)
// ============================================================================

// Import required functions for internal use
import {
  mouseClick as _mouseClick,
  mouseToggle as _mouseToggle,
  moveMouse as _moveMouse,
} from "@tego/bot";

/**
 * Perform a double click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 * @example
 * ```typescript
 * // Double click at current position
 * doubleClick();
 *
 * // Double click at specific coordinates
 * doubleClick(100, 200);
 * ```
 */
export function doubleClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    _moveMouse(x, y);
  }
  _mouseClick(undefined, true);
}

/**
 * Perform a right click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 * @example
 * ```typescript
 * // Right click at current position
 * rightClick();
 *
 * // Right click at specific coordinates
 * rightClick(300, 400);
 * ```
 */
export function rightClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    _moveMouse(x, y);
  }
  _mouseClick("right", false);
}

/**
 * Perform a middle click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 * @example
 * ```typescript
 * // Middle click at current position
 * middleClick();
 *
 * // Middle click at specific coordinates
 * middleClick(500, 600);
 * ```
 */
export function middleClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    _moveMouse(x, y);
  }
  _mouseClick("middle", false);
}

/**
 * Perform a left click at the current mouse position or specified coordinates
 * @param x - Optional X coordinate
 * @param y - Optional Y coordinate
 * @example
 * ```typescript
 * // Left click at current position
 * leftClick();
 *
 * // Left click at specific coordinates
 * leftClick(150, 250);
 * ```
 */
export function leftClick(x?: number, y?: number): void {
  if (x !== undefined && y !== undefined) {
    _moveMouse(x, y);
  }
  _mouseClick("left", false);
}

/**
 * Click and hold the mouse button down
 * @param button - Mouse button to hold ("left", "right", or "middle"), defaults to "left"
 * @example
 * ```typescript
 * // Hold left button
 * mouseDown("left");
 * // ... perform drag operation ...
 * mouseUp("left");
 *
 * // Hold right button
 * mouseDown("right");
 * ```
 */
export function mouseDown(button: "left" | "right" | "middle" = "left"): void {
  _mouseToggle("down", button);
}

/**
 * Release the mouse button
 * @param button - Mouse button to release ("left", "right", or "middle"), defaults to "left"
 * @example
 * ```typescript
 * mouseDown("left");
 * // ... perform drag operation ...
 * mouseUp("left");
 *
 * // Release right button
 * mouseUp("right");
 * ```
 */
export function mouseUp(button: "left" | "right" | "middle" = "left"): void {
  _mouseToggle("up", button);
}
