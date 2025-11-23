import { defineConfig } from "tsdown";

export default defineConfig({
  entry: ["src/index.ts", "bin/bot-agent.ts"],
  format: ["esm"],
  dts: true,
  clean: true,
  shims: true,
  platform: "node",
  target: "node18",
  external: [
    "@tego/botjs",
    "commander",
    "@inquirer/prompts",
    "openai",
    "chalk",
    "ora",
    "boxen",
    "cli-highlight",
    "tsx",
  ],
});
