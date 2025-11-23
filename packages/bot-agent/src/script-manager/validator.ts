/**
 * TypeScript code validation
 */

import ts from "typescript";

export interface ValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

/**
 * Validate TypeScript code syntax and basic structure
 */
export function validateTypeScriptCode(code: string): ValidationResult {
  const errors: string[] = [];
  const warnings: string[] = [];

  // Create source file for syntax checking
  const sourceFile = ts.createSourceFile(
    "script.ts",
    code,
    ts.ScriptTarget.Latest,
    true,
  );

  // Check for syntax errors
  const diagnostics = [...sourceFile.parseDiagnostics];

  for (const diagnostic of diagnostics) {
    if (diagnostic.category === ts.DiagnosticCategory.Error) {
      const message = ts.flattenDiagnosticMessageText(
        diagnostic.messageText,
        "\n",
      );
      errors.push(message);
    } else if (diagnostic.category === ts.DiagnosticCategory.Warning) {
      const message = ts.flattenDiagnosticMessageText(
        diagnostic.messageText,
        "\n",
      );
      warnings.push(message);
    }
  }

  // Check for @tego/botjs imports
  let hasTegoImport = false;
  ts.forEachChild(sourceFile, (node) => {
    if (ts.isImportDeclaration(node)) {
      const moduleSpecifier = node.moduleSpecifier;
      if (
        ts.isStringLiteral(moduleSpecifier) &&
        (moduleSpecifier.text === "@tego/botjs" ||
          moduleSpecifier.text === "@tego/bot")
      ) {
        hasTegoImport = true;
      }
    }
  });

  if (!hasTegoImport) {
    warnings.push("Code does not import from '@tego/botjs'");
  }

  return {
    valid: errors.length === 0,
    errors,
    warnings,
  };
}
