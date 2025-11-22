/**
 * Integration tests for @tego/bot
 *
 * These tests require actual system interaction (mouse, keyboard, screen access)
 * and will be skipped in CI environments by default.
 *
 * To run these tests locally, set ENABLE_INTEGRATION_TESTS=true
 */
import type { Bitmap, MousePosition, ScreenSize } from "@tego/bot";
import {
  bitmapColorAt,
  getMousePos,
  getPixelColor,
  getScreen,
  getScreenSize,
  Mouse,
  moveMouse,
  moveMouseSmooth,
  Screen,
} from "@tego/bot";
import { describe, expect, it } from "vitest";

const ENABLE_INTEGRATION_TESTS =
  process.env.ENABLE_INTEGRATION_TESTS === "true";

describe.skipIf(!ENABLE_INTEGRATION_TESTS)(
  "@tego/bot Integration Tests",
  () => {
    describe("Mouse operations", () => {
      it("should get mouse position", () => {
        const pos = getMousePos();
        expect(pos).toBeDefined();
        expect(typeof pos.x).toBe("number");
        expect(typeof pos.y).toBe("number");
        expect(pos.x).toBeGreaterThanOrEqual(0);
        expect(pos.y).toBeGreaterThanOrEqual(0);
      });

      it("should get mouse position from Mouse class", () => {
        const mouse = new Mouse();
        const pos = mouse.getMousePos();
        expect(pos).toBeDefined();
        expect(typeof pos.x).toBe("number");
        expect(typeof pos.y).toBe("number");
        expect(pos.x).toBeGreaterThanOrEqual(0);
        expect(pos.y).toBeGreaterThanOrEqual(0);
      });

      it("should move mouse", () => {
        expect(() => {
          moveMouse(100, 200);
        }).not.toThrow();
      });

      it("should move mouse smoothly", () => {
        expect(() => {
          moveMouseSmooth(300, 400);
        }).not.toThrow();
      });

      it("should move mouse smoothly with speed", () => {
        expect(() => {
          moveMouseSmooth(500, 600, 5.0);
        }).not.toThrow();
      });
    });

    describe("Screen operations", () => {
      it("should get screen size", () => {
        const size = getScreenSize();
        expect(size).toBeDefined();
        expect(typeof size.width).toBe("number");
        expect(typeof size.height).toBe("number");
        expect(size.width).toBeGreaterThan(0);
        expect(size.height).toBeGreaterThan(0);
      });

      it("should capture screen region", async () => {
        const screen = new Screen();
        const bitmap = await screen.capture(0, 0, 100, 100);
        expect(bitmap).toBeDefined();
        expect(bitmap.width).toBeGreaterThan(0);
        expect(bitmap.height).toBeGreaterThan(0);
        expect(bitmap.image).toBeInstanceOf(Buffer);
        expect(bitmap.byteWidth).toBeGreaterThan(0);
        expect(bitmap.bitsPerPixel).toBeGreaterThan(0);
        expect(bitmap.bytesPerPixel).toBeGreaterThan(0);
      });

      it("should get pixel color", async () => {
        const color = await getPixelColor(100, 200);
        expect(color).toBeDefined();
        expect(typeof color).toBe("string");
        expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
      });

      it("should get color from bitmap", async () => {
        const screen = getScreen();
        const bitmap = await screen.capture(0, 0, 100, 100);

        const color = bitmapColorAt(bitmap, 50, 50);
        expect(color).toBeDefined();
        expect(typeof color).toBe("string");
        expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
      });

      it("should handle out of bounds bitmap color", async () => {
        const screen = getScreen();
        const bitmap = await screen.capture(0, 0, 100, 100);

        // This should throw an error for out of bounds coordinates
        expect(() => {
          bitmapColorAt(bitmap, 999, 999);
        }).toThrow("Coordinates out of bounds");
      });

      it("should have correct Bitmap type", async () => {
        const screen = getScreen();
        const bitmap: Bitmap = await screen.capture(0, 0, 100, 100);
        expect(bitmap).toHaveProperty("width");
        expect(bitmap).toHaveProperty("height");
        expect(bitmap).toHaveProperty("image");
        expect(bitmap).toHaveProperty("byteWidth");
        expect(bitmap).toHaveProperty("bitsPerPixel");
        expect(bitmap).toHaveProperty("bytesPerPixel");
        expect(bitmap.image).toBeInstanceOf(Buffer);
      });

      it("should have correct ScreenSize type", () => {
        const size: ScreenSize = getScreenSize();
        expect(size).toHaveProperty("width");
        expect(size).toHaveProperty("height");
        expect(typeof size.width).toBe("number");
        expect(typeof size.height).toBe("number");
      });

      it("should have correct MousePosition type", () => {
        const pos: MousePosition = getMousePos();
        expect(pos).toHaveProperty("x");
        expect(pos).toHaveProperty("y");
        expect(typeof pos.x).toBe("number");
        expect(typeof pos.y).toBe("number");
      });
    });
  },
);
