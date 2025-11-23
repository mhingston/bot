/**
 * List command - Show all saved automation scripts
 */

import { ScriptStorage } from "../script-manager/storage.js";
import {
  displayError,
  displayScriptList,
  displaySectionHeader,
} from "../ui/display.js";

export async function listCommand(): Promise<void> {
  try {
    displaySectionHeader("Saved Automation Scripts");

    const storage = new ScriptStorage();
    const scripts = await storage.listScripts();

    displayScriptList(
      scripts.map((s) => ({
        name: s.name,
        description: s.description,
        updatedAt: s.updatedAt,
      })),
    );
  } catch (error) {
    if (error instanceof Error) {
      displayError(error.message);
    } else {
      displayError("An unexpected error occurred");
    }
    process.exit(1);
  }
}
