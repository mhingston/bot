# Running Integration Tests in CI

## Overview

Integration tests for `Tego Bot` require actual system interaction (mouse, keyboard, screen access). These tests are **disabled by default** in CI environments because:

1. **GitHub Actions runners are typically headless** - No graphical interface available
2. **Limited system access** - Virtualized environments may not support hardware interaction
3. **Permission requirements** - Screen recording and accessibility permissions needed
4. **Flakiness** - Tests may be unreliable in virtualized environments

## Running Integration Tests in CI

### Linux (Ubuntu)

It's **possible but challenging** to run integration tests on Linux runners:

1. **Setup Xvfb (Virtual Display Server)**
   ```yaml
   - name: Setup virtual display
     run: |
       sudo apt-get install -y xvfb
       export DISPLAY=:99
       Xvfb :99 -screen 0 1024x768x24 > /dev/null 2>&1 &
   ```

2. **Enable Integration Tests**
   ```yaml
   env:
     ENABLE_INTEGRATION_TESTS: "true"
     DISPLAY: ":99"
   ```

**Limitations:**
- Mouse/keyboard simulation may not work properly
- Screen capture might work but limited
- May require additional configuration for Wayland/X11

### macOS

**Partially possible** but with limitations:

- macOS runners have some GUI capabilities
- Screen capture may work with proper permissions
- Mouse/keyboard simulation may be limited
- Requires proper permission setup

**Setup:**
```yaml
runs-on: macos-latest
env:
  ENABLE_INTEGRATION_TESTS: "true"
```

### Windows

**Possible** but requires configuration:

- Windows runners may support GUI automation
- Requires proper setup for screen capture
- Mouse/keyboard simulation should work

**Setup:**
```yaml
runs-on: windows-latest
env:
  ENABLE_INTEGRATION_TESTS: "true"
```

## Recommended Approach

### Option 1: Self-Hosted Runners (Best for Integration Tests)

Use self-hosted runners for reliable integration testing:

```yaml
jobs:
  integration-tests:
    runs-on: self-hosted
    steps:
      - uses: actions/checkout@v4
      - name: Run integration tests
        run: pnpm test:integration
        env:
          ENABLE_INTEGRATION_TESTS: "true"
```

**Advantages:**
- Full system access
- Real hardware interaction
- More reliable results
- Can run on physical machines

**Disadvantages:**
- Requires maintaining your own infrastructure
- Security considerations
- Additional setup and maintenance

### Option 2: Conditional CI Job

Create a separate job that only runs on specific conditions:

```yaml
jobs:
  integration-tests:
    if: github.event_name == 'workflow_dispatch' || contains(github.event.head_commit.message, '[test-integration]')
    runs-on: ubuntu-latest
    steps:
      - name: Setup and run integration tests
        run: |
          # Setup virtual display
          # Run tests with ENABLE_INTEGRATION_TESTS=true
```

### Option 3: Manual Testing Only

Keep integration tests for local development only:

- Run `pnpm test:integration` locally
- CI only runs unit tests
- Integration tests verify API correctness without actual system interaction

## Current Configuration

By default, integration tests are **disabled** in CI. The test file uses:

```typescript
const ENABLE_INTEGRATION_TESTS = process.env.ENABLE_INTEGRATION_TESTS === "true";
describe.skipIf(!ENABLE_INTEGRATION_TESTS)("@tego/bot Integration Tests", () => {
  // Tests here
});
```

And in `vitest.config.ts`:

```typescript
exclude: [
  ...(process.env.ENABLE_INTEGRATION_TESTS !== "true"
    ? ["tests/**/*.integration.test.ts"]
    : []),
]
```

## Enabling in CI

To enable integration tests in CI, uncomment the environment variable in `.github/workflows/ci.yml`:

```yaml
env:
  ENABLE_INTEGRATION_TESTS: "true"
```

**Note:** This may cause tests to fail if the CI environment doesn't support the required system interactions.

## Best Practices

1. **Keep integration tests separate** - Don't block CI on integration test failures
2. **Use conditional execution** - Only run on specific branches or manual triggers
3. **Consider self-hosted runners** - For projects requiring reliable integration testing
4. **Mock when possible** - Use mocks for unit tests, real interaction for integration tests
5. **Document limitations** - Clearly document what works and what doesn't in CI
