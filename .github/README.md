# Development Hooks Setup

This directory contains scripts to set up Git hooks for development.

## Pre-Push Hook

The pre-push hook automatically runs the following checks before allowing a push:

1. **Code Formatting** - `cargo fmt --check`
2. **Tests** - `cargo test`
3. **Linting** - `cargo clippy -- -D warnings`

## Setup Instructions

### Option 1: Using the Setup Script

Run the setup script to install the pre-push hook:

```bash
bash .github/setup-hooks.sh
```

### Option 2: Using GitHub Copilot Tasks (if available)

If you're using GitHub Copilot with workspace tasks, you can use the `setup-dev-hooks` task:

1. Open the Command Palette in your editor
2. Select "Run Task" 
3. Choose "setup-dev-hooks"

### Option 3: Manual Installation

You can manually copy the hook:

```bash
cp .github/hooks/pre-push .git/hooks/pre-push
chmod +x .git/hooks/pre-push
```

## Bypassing the Hook

In emergency situations, you can bypass the pre-push hook using:

```bash
git push --no-verify
```

**Note:** This should be used sparingly and only when necessary.

## What the Hook Does

Before each push, the hook will:

1. ✅ Check that your code is properly formatted
2. ✅ Run all tests to ensure they pass (skips if build dependencies are unavailable)
3. ✅ Run clippy linter to catch common issues (skips if build dependencies are unavailable)

If any check fails, the push will be blocked and you'll see an error message explaining what needs to be fixed.

**Note:** The hook is smart about build/dependency issues. If tests or linting can't run due to missing dependencies or network issues (common in CI environments), the hook will warn but allow the push to continue. However, actual code formatting errors, test failures, or linting issues will block the push.
