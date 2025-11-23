# @tego/bot-agent

AI-powered CLI for generating and managing @tego/bot automation scripts.

## Installation

```bash
# Install in the workspace
pnpm install

# Build the package
pnpm --filter @tego/bot-agent build
```

## Configuration

Set the following environment variables:

```bash
export OPENAI_API_KEY="your-api-key"
export OPENAI_BASE_URL="https://api.openai.com/v1"  # Optional, defaults to OpenAI
export OPENAI_MODEL="gpt-4"  # Optional, defaults to gpt-4
```

## Usage

### Generate a new automation script

```bash
bot-agent generate

# Or with a description
bot-agent generate "Click at 100,200 then type hello and press enter"
```

### Edit an existing script

```bash
bot-agent edit

# Or specify the script name
bot-agent edit my-script
```

### Execute a saved script

```bash
bot-agent execute

# Or specify the script name
bot-agent execute my-script

# Alias: run
bot-agent run my-script
```

### List all saved scripts

```bash
bot-agent list

# Alias: ls
bot-agent ls
```

## Features

- **AI-Powered Generation**: Uses OpenAI-compatible APIs to generate automation code
- **Conversational Editing**: Iteratively refine scripts through natural language
- **Script Management**: All scripts saved to `~/.tego/bot-scripts/`
- **Conversation History**: Each script maintains its conversation context
- **Code Validation**: TypeScript validation before execution
- **Interactive UI**: Beautiful CLI interface with syntax highlighting

## Script Storage

All generated scripts are stored in:
```
~/.tego/bot-scripts/
  ├── script-name.ts          # The TypeScript code
  └── script-name.meta.json   # Metadata and conversation history
```

## Development

```bash
# Build
pnpm build

# Watch mode
pnpm dev

# Run tests
pnpm test
```

## License

MIT
