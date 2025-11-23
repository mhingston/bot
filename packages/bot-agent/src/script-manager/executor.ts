/**
 * Script execution
 */

import { pathToFileURL } from "node:url";
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
  const absolutePath = path.resolve(scriptPath);

  let stdout = "";
  let stderr = "";

  // Capture stdout
  const originalStdoutWrite = process.stdout.write.bind(process.stdout);
  process.stdout.write = ((chunk: any, ...args: any[]) => {
    const text = chunk.toString();
    stdout += text;
    return originalStdoutWrite(chunk, ...args);
  }) as any;

  // Capture stderr
  const originalStderrWrite = process.stderr.write.bind(process.stderr);
  process.stderr.write = ((chunk: any, ...args: any[]) => {
    const text = chunk.toString();
    stderr += text;
    return originalStderrWrite(chunk, ...args);
  }) as any;

  try {
    // Register tsx to enable TypeScript loading
    await import("tsx");

    // Add timestamp to force fresh import
    const fileUrl = `${pathToFileURL(absolutePath).href}?t=${Date.now()}`;
    await import(fileUrl);

    // Restore stdout/stderr
    process.stdout.write = originalStdoutWrite;
    process.stderr.write = originalStderrWrite;

    return {
      success: true,
      stdout,
      stderr,
    };
  } catch (error) {
    // Restore stdout/stderr
    process.stdout.write = originalStdoutWrite;
    process.stderr.write = originalStderrWrite;

    return {
      success: false,
      stdout,
      stderr,
      error: error instanceof Error ? error : new Error(String(error)),
    };
  }
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
