# Documentation

This directory contains the GitHub Pages documentation for @tego/bot.

## Structure

- `/` - Main documentation site (Jekyll)
  - `index.md` - Homepage
  - `api.md` - API documentation index
  - `_config.yml` - Jekyll configuration
  - `_layouts/` - Jekyll layouts
  - `Gemfile` - Ruby dependencies

- `/api/` - TypeDoc generated API documentation (HTML)
  - Generated from `packages/botjs/src/index.ts`

- `/developments/` - Development notes and research documents
  - `index.md` - Development notes index
  - Various research markdown files

## Local Development

### Prerequisites

1. Install Ruby and Bundler
2. Install Jekyll dependencies:
   ```bash
   cd docs
   bundle install
   ```

### Generate API Documentation

```bash
# From project root
pnpm docs:api
```

### Serve Documentation Locally

```bash
# From project root
pnpm docs:serve

# Or manually
cd docs
bundle exec jekyll serve
```

Visit http://localhost:4000

### Build Documentation

```bash
# From project root
pnpm docs:build

# Or manually
cd docs
bundle exec jekyll build
```

## Deployment

GitHub Pages automatically builds and deploys from the `/docs` directory on the `main` branch.

Configure in GitHub repository settings:
- Settings > Pages
- Source: Deploy from a branch
- Branch: `main` / `/docs`

## Notes

- TypeDoc output (`/api/`) is excluded from Jekyll processing via `exclude` in `_config.yml`
- The `.nojekyll` file is NOT used because we want Jekyll to process the markdown files
- API documentation links point directly to TypeDoc HTML files
