# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

@tego/bot is a high-performance desktop automation library for Node.js, powered by a Rust core using N-API bindings. It provides robotjs-compatible APIs for mouse control, keyboard input, and screen capture with superior performance and memory safety.

## Architecture

The project uses a **monorepo workspace** structure with three main packages:

### 1. `packages/bot` - Rust Core (N-API bindings)
- **Language**: Rust 2024 edition (requires Rust 1.85+)
- **Build system**: Cargo with napi-build
- **Key dependencies**:
  - `napi` and `napi-derive` for Node.js bindings
  - `enigo` for cross-platform input simulation
  - `xcap` for screen capture
  - `rdev` for input event monitoring
- **Module structure**:
  - `api.rs` - Global API exports matching robotjs interface (uses `#[napi]` macros)
  - `mouse.rs` - Mouse control (Enigo-based, with smooth movement using easing functions)
  - `keyboard.rs` - Keyboard input (supports modifiers, Unicode, and delayed typing)
  - `screen.rs` - Screen capture (xcap-based, returns PNG-encoded buffers)
- **Binary output**: Compiled to `cdylib` (native Node.js addon)

### 2. `packages/botjs` - TypeScript Wrapper
- **Language**: TypeScript (ESM)
- **Purpose**: Re-exports the Rust bindings from `@tego/bot` with TypeScript types
- **Build**: `tsdown` for TypeScript compilation
- **Testing**: Vitest with optional integration tests (set `ENABLE_INTEGRATION_TESTS=true`)
- **Documentation**: TypeDoc for API docs generation

### 3. `packages/bot-agent` - AI-Powered CLI
- **Language**: TypeScript (ESM)
- **Purpose**: CLI tool for generating automation scripts using AI
- **Binary**: `bot-agent` command-line tool
- **Key features**:
  - Generate automation scripts from natural language descriptions
  - Edit existing scripts through conversational AI
  - Execute and manage saved scripts
  - Store scripts in `~/.tego/bot-scripts/` with conversation history
- **Dependencies**: OpenAI API client, Commander.js, Inquirer, Chalk, Ora
- **Configuration**: Uses environment variables (OPENAI_API_KEY, OPENAI_BASE_URL, OPENAI_MODEL)

### Key Design Patterns
- **Thread-safe state**: Rust modules use `Arc<Mutex<>>` for shared state (Enigo instances, delay settings)
- **Async operations**: Screen capture operations are async (use `tokio` runtime)
- **Global delay settings**: Mouse and keyboard operations respect global delay values set via `set_mouse_delay()` and `set_keyboard_delay()`
- **Rust 2024 edition**: Code uses modern iterator patterns and edition-specific optimizations

## Development Commands

### Build
```bash
# Full build (Rust + TypeScript)
pnpm build

# Build Rust bindings only
pnpm rs:build

# Build TypeScript wrapper only
pnpm ts:build

# Build Rust for specific platform
pnpm rs:build --platform

# Build AI agent CLI only
pnpm agent:build
```

### Testing
```bash
# Run all tests (Rust + TypeScript)
pnpm test

# Run TypeScript tests only
pnpm --filter @tego/botjs test

# Run TypeScript tests with coverage
pnpm --filter @tego/botjs test:coverage

# Run integration tests (requires ENABLE_INTEGRATION_TESTS=true)
pnpm --filter @tego/botjs test:integration

# Run Rust tests only
cargo test --all-features

# Run Rust tests in a specific package
cargo test -p bot

# Run AI agent tests
pnpm agent:test
```

### Linting & Formatting
```bash
# Check TypeScript/JavaScript formatting
pnpm fmt

# Fix TypeScript/JavaScript formatting
pnpm fmt:fix

# Lint TypeScript/JavaScript
pnpm lint

# Fix TypeScript/JavaScript linting issues
pnpm lint:fix

# Check and fix both formatting and linting
pnpm check:fix

# Rust formatting
cargo fmt --all

# Rust linting
cargo clippy --all-targets --all-features -- -D warnings

# Rust compilation check
cargo check --all-targets --all-features
```

### Examples
```bash
# Run example scripts
pnpm ex:run
```

### Documentation
```bash
# Generate TypeScript API docs
pnpm --filter @tego/botjs docs

# Watch mode for docs
pnpm --filter @tego/botjs docs:watch

# Serve generated docs
pnpm --filter @tego/botjs docs:serve
```

### AI Agent CLI
```bash
# Build the CLI
pnpm agent:build

# Development mode with watch
pnpm agent:dev

# Generate new automation script
npx bot-agent generate

# Edit existing script
npx bot-agent edit [script-name]

# Execute saved script
npx bot-agent execute [script-name]

# List all saved scripts
npx bot-agent list
```

## Git Hooks (Lefthook)

The repository uses **Lefthook** for git hooks, NOT simple-git-hooks. Hook configuration is in `lefthook.yml`:

### Pre-commit Hooks (parallel)
- **Rust**: `cargo fmt --check`, `cargo clippy`, `cargo check`
- **TypeScript**: `biome check --write` on staged files

### Pre-push Hooks (sequential)
- **Tests**: `cargo test --all-features`
- **Security**: `cargo audit` and `pnpm audit --audit-level moderate`

### Commit Message Validation
- Enforces **Conventional Commits** format: `<type>[optional scope]: <description>`
- Valid types: `build`, `chore`, `ci`, `docs`, `feat`, `fix`, `perf`, `refactor`, `revert`, `style`, `test`
- Max length: 72 characters
- Requires imperative mood (e.g., "add" not "added")

## Cross-Platform Considerations

### macOS
- Requires macOS 10.13+
- Screen recording permission required (System Preferences > Security & Privacy > Screen Recording)

### Linux
- May require system dependencies:
  ```bash
  # Ubuntu/Debian
  sudo apt-get install libxcb1-dev libxrandr-dev libdbus-1-dev

  # Fedora
  sudo dnf install libxcb-devel libXrandr-devel dbus-devel
  ```

### Windows
- Windows 10+ supported
- No additional configuration needed

## Performance Optimization

The Rust release profile in `Cargo.toml` is optimized for maximum performance:
- LTO enabled (`lto = "fat"`)
- Single codegen unit (`codegen-units = 1`)
- Full optimization (`opt-level = 3`)
- Binary stripping enabled (`strip = true`)

## AI Agent (@tego/bot-agent)

The AI agent CLI generates automation scripts using OpenAI-compatible APIs. It provides a conversational interface for creating, editing, and executing automation scripts.

### Configuration
Set environment variables before using the agent:
```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_BASE_URL="https://api.openai.com/v1"  # Optional
export OPENAI_MODEL="gpt-4"  # Optional, defaults to gpt-4
```

### Script Storage
- All scripts are saved to `~/.tego/bot-scripts/`
- Each script has two files:
  - `script-name.ts` - The TypeScript code
  - `script-name.meta.json` - Metadata including conversation history
- Conversation history is preserved for iterative editing

### Workflow
1. **Generate**: User describes automation task → AI generates TypeScript code
2. **Review**: Code is displayed with syntax highlighting and validation
3. **Action**: User can execute, save, edit, or regenerate
4. **Edit**: Iterative refinement through natural language feedback
5. **Execute**: Scripts run using `tsx` for direct TypeScript execution

### Code Generation
The AI is instructed to:
- Import only from `@tego/botjs`
- Use proper async/await for screen operations
- Include error handling
- Add descriptive comments
- Follow TypeScript best practices

## Important Notes

- **N-API bindings**: The Rust code compiles to a native Node.js addon. Changes to Rust code require rebuilding with `pnpm rs:build`.
- **Workspace structure**: Use `pnpm --filter` to run commands in specific packages, or `pnpm -r` for recursive execution across all packages.
- **Async screen operations**: Screen capture functions (`captureScreen`, `getPixelColor`, etc.) are async and return Promises.
- **Binary distribution**: The `@tego/bot` package includes pre-built binaries in the `dist` directory after building.
- **AI Agent**: Requires OpenAI API key set in environment variables. Scripts are stored locally in `~/.tego/bot-scripts/`.
