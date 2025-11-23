/**
 * UI utilities for prompts and user interaction
 */

import { confirm, input, select } from "@inquirer/prompts";

/**
 * Prompt for script description
 */
export async function promptForDescription(): Promise<string> {
  return await input({
    message: "Describe what you want to automate:",
    validate: (value) => {
      if (!value || value.trim().length === 0) {
        return "Description cannot be empty";
      }
      return true;
    },
  });
}

/**
 * Prompt for script name
 */
export async function promptForScriptName(
  defaultName?: string,
): Promise<string> {
  return await input({
    message: "Enter a name for this script:",
    default: defaultName,
    validate: (value) => {
      if (!value || value.trim().length === 0) {
        return "Name cannot be empty";
      }
      // Validate filename-safe characters
      if (!/^[a-zA-Z0-9_-]+$/.test(value)) {
        return "Name can only contain letters, numbers, hyphens, and underscores";
      }
      return true;
    },
  });
}

/**
 * Prompt for action after code generation
 */
export async function promptForAction(): Promise<
  "execute" | "save" | "edit" | "regenerate" | "cancel"
> {
  return await select({
    message: "What would you like to do?",
    choices: [
      { name: "Execute the script now", value: "execute" },
      { name: "Save without executing", value: "save" },
      { name: "Make changes to the script", value: "edit" },
      { name: "Regenerate the code", value: "regenerate" },
      { name: "Cancel", value: "cancel" },
    ],
  });
}

/**
 * Prompt for edit feedback
 */
export async function promptForEditFeedback(): Promise<string> {
  return await input({
    message: "What changes would you like to make?",
    validate: (value) => {
      if (!value || value.trim().length === 0) {
        return "Please describe the changes you want";
      }
      return true;
    },
  });
}

/**
 * Prompt for confirmation
 */
export async function promptForConfirmation(message: string): Promise<boolean> {
  return await confirm({
    message,
    default: true,
  });
}

/**
 * Prompt to select from saved scripts
 */
export async function promptForScriptSelection(
  scripts: Array<{ name: string; description: string }>,
): Promise<string> {
  if (scripts.length === 0) {
    throw new Error("No scripts available");
  }

  return await select({
    message: "Select a script:",
    choices: scripts.map((script) => ({
      name: `${script.name} - ${script.description}`,
      value: script.name,
    })),
  });
}

/**
 * Prompt for continuation
 */
export async function promptForContinuation(): Promise<boolean> {
  return await confirm({
    message: "Would you like to make more changes?",
    default: false,
  });
}
