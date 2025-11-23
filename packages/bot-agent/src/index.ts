/**
 * Main CLI module
 */

import { Command } from "commander";
import { editCommand } from "./commands/edit.js";
import { executeCommand } from "./commands/execute.js";
import { generateCommand } from "./commands/generate.js";
import { listCommand } from "./commands/list.js";
import { displayWelcomeBanner } from "./ui/display.js";

const program = new Command();

program
  .name("bot-agent")
  .description(
    "AI-powered CLI for generating and managing @tego/bot automation scripts",
  )
  .version("0.0.1");

program
  .command("generate")
  .description("Generate a new automation script")
  .argument("[description]", "Description of what to automate")
  .action(async (description?: string) => {
    displayWelcomeBanner();
    await generateCommand(description);
  });

program
  .command("edit")
  .description("Edit an existing automation script")
  .argument("[script-name]", "Name of the script to edit")
  .action(async (scriptName?: string) => {
    displayWelcomeBanner();
    await editCommand(scriptName);
  });

program
  .command("execute")
  .alias("run")
  .description("Execute a saved automation script")
  .argument("[script-name]", "Name of the script to execute")
  .action(async (scriptName?: string) => {
    await executeCommand(scriptName);
  });

program
  .command("list")
  .alias("ls")
  .description("List all saved automation scripts")
  .action(async () => {
    await listCommand();
  });

export { program };
