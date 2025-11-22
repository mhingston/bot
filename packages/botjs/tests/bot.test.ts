import {
  // Global functions
  dragMouse,
  getScreen,
  // Classes
  Keyboard,
  keyTap,
  keyToggle,
  Mouse,
  mouseClick,
  mouseToggle,
  moveMouse,
  moveMouseSmooth,
  Screen,
  scrollMouse,
  setKeyboardDelay,
  setMouseDelay,
  typeString,
  typeStringDelayed,
  unicodeTap,
  updateScreenMetrics,
} from "@tego/bot";
import { describe, expect, it } from "vitest";

describe("@tego/bot", () => {
  describe("Exports", () => {
    it("should export Keyboard class", () => {
      expect(Keyboard).toBeDefined();
      expect(typeof Keyboard).toBe("function");
    });

    it("should export Mouse class", () => {
      expect(Mouse).toBeDefined();
      expect(typeof Mouse).toBe("function");
    });

    it("should export Screen class", () => {
      expect(Screen).toBeDefined();
      expect(typeof Screen).toBe("function");
    });

    it("should export all global functions", () => {
      expect(typeof getScreen).toBe("function");
      expect(typeof moveMouse).toBe("function");
      expect(typeof moveMouseSmooth).toBe("function");
      expect(typeof mouseClick).toBe("function");
      expect(typeof mouseToggle).toBe("function");
      expect(typeof dragMouse).toBe("function");
      expect(typeof scrollMouse).toBe("function");
      expect(typeof keyTap).toBe("function");
      expect(typeof keyToggle).toBe("function");
      expect(typeof typeString).toBe("function");
      expect(typeof typeStringDelayed).toBe("function");
      expect(typeof unicodeTap).toBe("function");
      expect(typeof setKeyboardDelay).toBe("function");
      expect(typeof setMouseDelay).toBe("function");
      expect(typeof updateScreenMetrics).toBe("function");
    });
  });

  describe("Keyboard class", () => {
    it("should create Keyboard instance", () => {
      const keyboard = new Keyboard();
      expect(keyboard).toBeDefined();
      expect(keyboard).toBeInstanceOf(Keyboard);
    });

    it("should have all required methods", () => {
      const keyboard = new Keyboard();
      expect(typeof keyboard.keyTap).toBe("function");
      expect(typeof keyboard.keyToggle).toBe("function");
      expect(typeof keyboard.typeString).toBe("function");
      expect(typeof keyboard.typeStringDelayed).toBe("function");
      expect(typeof keyboard.setKeyboardDelay).toBe("function");
    });

    it("should set keyboard delay", () => {
      const keyboard = new Keyboard();
      expect(() => {
        keyboard.setKeyboardDelay(50);
      }).not.toThrow();
    });
  });

  describe("Mouse class", () => {
    it("should create Mouse instance", () => {
      const mouse = new Mouse();
      expect(mouse).toBeDefined();
      expect(mouse).toBeInstanceOf(Mouse);
    });

    it("should have all required methods", () => {
      const mouse = new Mouse();
      expect(typeof mouse.moveMouse).toBe("function");
      expect(typeof mouse.moveMouseSmooth).toBe("function");
      expect(typeof mouse.moveMouseSmoothWithSpeed).toBe("function");
      expect(typeof mouse.getMousePos).toBe("function");
      expect(typeof mouse.mouseClick).toBe("function");
      expect(typeof mouse.mouseToggle).toBe("function");
      expect(typeof mouse.dragMouse).toBe("function");
      expect(typeof mouse.scrollMouse).toBe("function");
      expect(typeof mouse.setMouseDelay).toBe("function");
    });

    it("should set mouse delay", () => {
      const mouse = new Mouse();
      expect(() => {
        mouse.setMouseDelay(50);
      }).not.toThrow();
    });
  });

  describe("Screen class", () => {
    it("should create Screen instance", () => {
      const screen = new Screen();
      expect(screen).toBeDefined();
      expect(screen).toBeInstanceOf(Screen);
    });

    it("should have capture method", () => {
      const screen = new Screen();
      expect(typeof screen.capture).toBe("function");
    });
  });

  describe("Global functions - Mouse", () => {
    it("should set mouse delay", () => {
      expect(() => {
        setMouseDelay(50);
      }).not.toThrow();
    });
  });

  describe("Global functions - Keyboard", () => {
    it("should set keyboard delay", () => {
      expect(() => {
        setKeyboardDelay(10);
      }).not.toThrow();
    });
  });

  describe("Global functions - Screen", () => {
    it("should get screen instance", () => {
      const screen = getScreen();
      expect(screen).toBeDefined();
      expect(screen).toBeInstanceOf(Screen);
    });

    it("should update screen metrics", () => {
      expect(() => {
        updateScreenMetrics();
      }).not.toThrow();
    });
  });
});
