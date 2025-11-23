import { describe, expect, it } from "vitest";
import { validateTypeScriptCode } from "../src/script-manager/validator";

describe("TypeScript Validator", () => {
  describe("validateTypeScriptCode", () => {
    it("should validate correct TypeScript code", () => {
      const code = `import { moveMouse, click } from '@tego/botjs';

async function automate() {
  moveMouse(100, 200);
  click();
}

automate();`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it("should detect syntax errors", () => {
      const code = `import { moveMouse } from '@tego/botjs';

function broken() {
  moveMouse(100, 200;  // Missing closing parenthesis
}`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it("should detect unclosed braces", () => {
      const code = `import { moveMouse } from '@tego/botjs';

function broken() {
  moveMouse(100, 200);
  // Missing closing brace`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it("should warn when @tego/botjs import is missing", () => {
      const code = `import fs from 'fs';

function doSomething() {
  console.log('Hello');
}`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true); // No syntax errors
      expect(result.warnings).toContain(
        "Code does not import from '@tego/botjs'",
      );
    });

    it("should accept @tego/bot import as valid", () => {
      const code = `import { moveMouse } from '@tego/bot';

moveMouse(100, 200);`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true);
      expect(result.warnings).not.toContain(
        "Code does not import from '@tego/botjs'",
      );
    });

    it("should validate code with async/await", () => {
      const code = `import { captureScreen } from '@tego/botjs';

async function capture() {
  const screen = await captureScreen();
  console.log(screen);
}`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it("should validate code with type annotations", () => {
      const code = `import { moveMouse } from '@tego/botjs';

function moveToPosition(x: number, y: number): void {
  moveMouse(x, y);
}

moveToPosition(100, 200);`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it("should validate code with interfaces", () => {
      const code = `import { moveMouse, click } from '@tego/botjs';

interface Position {
  x: number;
  y: number;
}

function moveAndClick(pos: Position): void {
  moveMouse(pos.x, pos.y);
  click();
}`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it("should detect invalid string literal", () => {
      const code = `import { typeString } from '@tego/botjs';

typeString('unclosed string`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });

    it("should validate empty code", () => {
      const code = "";

      const result = validateTypeScriptCode(code);

      // Empty code is syntactically valid
      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
      expect(result.warnings).toContain(
        "Code does not import from '@tego/botjs'",
      );
    });

    it("should validate code with comments", () => {
      const code = `import { moveMouse } from '@tego/botjs';

// Move mouse to center of screen
moveMouse(960, 540);

/*
 * Multi-line comment
 * explaining the automation
 */`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it("should detect missing semicolons as warnings or valid (depending on TS config)", () => {
      const code = `import { moveMouse } from '@tego/botjs'

moveMouse(100, 200)`;

      const result = validateTypeScriptCode(code);

      // TypeScript parser should still parse this as valid (ASI - Automatic Semicolon Insertion)
      expect(result.valid).toBe(true);
    });

    it("should validate complex automation script", () => {
      const code = `import { moveMouse, click, typeString, setMouseDelay } from '@tego/botjs';

async function complexAutomation() {
  setMouseDelay(100);

  // Navigate to search box
  moveMouse(500, 100);
  click();

  // Type search query
  typeString('automation test');

  // Submit
  moveMouse(600, 150);
  click();
}

complexAutomation().catch(console.error);`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    it("should detect invalid token", () => {
      const code = `import { moveMouse } from '@tego/botjs';

@@@invalid@@@`;

      const result = validateTypeScriptCode(code);

      expect(result.valid).toBe(false);
      expect(result.errors.length).toBeGreaterThan(0);
    });
  });
});
