/**
 * Script execution
 */

import { spawn } from "child_process";
import path from "path";

export interface ExecutionResult {
  success: boolean;
  stdout: string;
  stderr: string;
  error?: Error;
}

/**
 * Execute a TypeScript script using tsx
 */
export async function executeScript(
  scriptPath: string,
): Promise<ExecutionResult> {
  return new Promise((resolve) => {
    const absolutePath = path.resolve(scriptPath);

    // Use tsx to execute TypeScript directly
    const child = spawn("npx", ["tsx", absolutePath], {
      stdio: ["inherit", "pipe", "pipe"],
      shell: true,
    });

    let stdout = "";
    let stderr = "";

    if (child.stdout) {
      child.stdout.on("data", (data) => {
        const text = data.toString();
        stdout += text;
        process.stdout.write(text);
      });
    }

    if (child.stderr) {
      child.stderr.on("data", (data) => {
        const text = data.toString();
        stderr += text;
        process.stderr.write(text);
      });
    }

    child.on("error", (error) => {
      resolve({
        success: false,
        stdout,
        stderr,
        error,
      });
    });

    child.on("close", (code) => {
      resolve({
        success: code === 0,
        stdout,
        stderr,
      });
    });
  });
}

/**
 * Execute code string (saves to temp file first)
 */
export async function executeCodeString(
  code: string,
  tempFileName: string = "temp",
): Promise<ExecutionResult> {
  const fs = await import("fs/promises");
  const os = await import("os");

  const tempDir = os.tmpdir();
  const tempFile = path.join(tempDir, `${tempFileName}-${Date.now()}.ts`);

  try {
    await fs.writeFile(tempFile, code, "utf-8");
    return await executeScript(tempFile);
  } finally {
    // Clean up temp file
    await fs.unlink(tempFile).catch(() => {});
  }
}
