// Integration tests for Tego Bot
//
// This is a placeholder test file. Since this crate is a Node.js addon (cdylib),
// it cannot be tested directly with `cargo test` because N-API symbols are only
// available at runtime when loaded by Node.js.
//
// Actual tests are located in:
// - packages/botjs/tests/ - TypeScript/Vitest tests for the Node.js bindings
//
// To run tests:
// 1. Build the library: pnpm rs:build
// 2. Run Node.js tests: pnpm --filter @tego/botjs test

fn main() {
    println!("This crate contains N-API bindings for Node.js.");
    println!("Tests should be run via the @tego/botjs package:");
    println!("  pnpm --filter @tego/botjs test");
}
