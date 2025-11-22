module.exports = {
  // For Rust files, run cargo commands on the entire workspace
  // This ignores the file arguments passed by lint-staged
  "*.rs": () => {
    return [
      "bash -c 'cd packages/bot && cargo fmt -- --check'",
      "bash -c 'cd packages/bot && cargo clippy --all-targets -- -D warnings'",
    ];
  },
  // For JS/TS files, use biome
  "*.{js,ts,tsx,json,jsonc}": ["pnpm biome check --write"],
};
