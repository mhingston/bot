import { describe, expect, it } from "vitest";
import {
  extractCodeFromResponse,
  validateExtractedCode,
} from "../src/ai/parser";

describe("AI Parser", () => {
  describe("extractCodeFromResponse", () => {
    it("should extract code from typescript code block", () => {
      const response = `Here's the code:

\`\`\`typescript
import { moveMouse } from '@tego/botjs';

moveMouse(100, 200);
\`\`\`

This will move the mouse to position (100, 200).`;

      const code = extractCodeFromResponse(response);

      expect(code).toBe(
        `import { moveMouse } from '@tego/botjs';\n\nmoveMouse(100, 200);`,
      );
    });

    it("should extract code from ts code block", () => {
      const response = `\`\`\`ts
import { click } from '@tego/botjs';
click();
\`\`\``;

      const code = extractCodeFromResponse(response);

      expect(code).toBe(`import { click } from '@tego/botjs';\nclick();`);
    });

    it("should extract code from generic code block", () => {
      const response = `\`\`\`
import { typeString } from '@tego/botjs';
typeString('Hello');
\`\`\``;

      const code = extractCodeFromResponse(response);

      expect(code).toBe(
        `import { typeString } from '@tego/botjs';\ntypeString('Hello');`,
      );
    });

    it("should handle multiple code blocks by extracting the first one", () => {
      const response = `\`\`\`typescript
import { moveMouse } from '@tego/botjs';
moveMouse(10, 10);
\`\`\`

And here's another example:

\`\`\`typescript
import { click } from '@tego/botjs';
click();
\`\`\``;

      const code = extractCodeFromResponse(response);

      expect(code).toBe(
        `import { moveMouse } from '@tego/botjs';\nmoveMouse(10, 10);`,
      );
    });

    it("should return trimmed response if no code block found", () => {
      const response = `  import { moveMouse } from '@tego/botjs';
moveMouse(100, 200);  `;

      const code = extractCodeFromResponse(response);

      expect(code).toBe(
        `import { moveMouse } from '@tego/botjs';\nmoveMouse(100, 200);`,
      );
    });

    it("should handle empty response", () => {
      const response = "";

      const code = extractCodeFromResponse(response);

      expect(code).toBe("");
    });

    it("should handle code block with extra whitespace", () => {
      const response = `\`\`\`typescript

import { moveMouse } from '@tego/botjs';

moveMouse(100, 200);

\`\`\``;

      const code = extractCodeFromResponse(response);

      expect(code).toBe(
        `import { moveMouse } from '@tego/botjs';\n\nmoveMouse(100, 200);`,
      );
    });
  });

  describe("validateExtractedCode", () => {
    it("should validate code with single-quote import", () => {
      const code = `import { moveMouse } from '@tego/botjs';
moveMouse(100, 200);`;

      const result = validateExtractedCode(code);

      expect(result.valid).toBe(true);
      expect(result.error).toBeUndefined();
    });

    it("should validate code with double-quote import", () => {
      const code = `import { click } from "@tego/botjs";
click();`;

      const result = validateExtractedCode(code);

      expect(result.valid).toBe(true);
      expect(result.error).toBeUndefined();
    });

    it("should reject empty code", () => {
      const code = "";

      const result = validateExtractedCode(code);

      expect(result.valid).toBe(false);
      expect(result.error).toBe("No code extracted from response");
    });

    it("should reject code without @tego/botjs import", () => {
      const code = `import { something } from 'other-package';
something();`;

      const result = validateExtractedCode(code);

      expect(result.valid).toBe(false);
      expect(result.error).toBe("Code does not import from '@tego/botjs'");
    });

    it("should reject whitespace-only code", () => {
      const code = "   \n\n  ";

      const result = validateExtractedCode(code);

      expect(result.valid).toBe(false);
      // Whitespace-only code will fail the import check, not the empty check
      expect(result.error).toBe("Code does not import from '@tego/botjs'");
    });

    it("should validate code with multiple imports including @tego/botjs", () => {
      const code = `import { moveMouse, click } from '@tego/botjs';
import fs from 'fs';

moveMouse(100, 200);
click();`;

      const result = validateExtractedCode(code);

      expect(result.valid).toBe(true);
      expect(result.error).toBeUndefined();
    });

    it("should validate code with complex structure", () => {
      const code = `import { moveMouse, click, typeString } from '@tego/botjs';

async function automate() {
  moveMouse(100, 200);
  click();
  typeString('Hello World');
}

automate();`;

      const result = validateExtractedCode(code);

      expect(result.valid).toBe(true);
      expect(result.error).toBeUndefined();
    });
  });
});
