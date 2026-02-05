#!/bin/bash
# Setup script to install git pre-push hook
# This installs a pre-push hook that runs cargo fmt, test, and clippy checks

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
HOOK_SOURCE="$SCRIPT_DIR/hooks/pre-push"
HOOK_DEST="$REPO_ROOT/.git/hooks/pre-push"

echo "Installing pre-push git hook..."

if [ ! -f "$HOOK_SOURCE" ]; then
  echo "❌ Error: Hook source file not found at $HOOK_SOURCE"
  exit 1
fi

# Copy the hook
cp "$HOOK_SOURCE" "$HOOK_DEST"
chmod +x "$HOOK_DEST"

echo "✅ Pre-push git hook installed successfully at $HOOK_DEST"
echo ""
echo "The hook will run the following checks before each push:"
echo "  - cargo fmt --check (code formatting)"
echo "  - cargo test (run tests)"
echo "  - cargo clippy -- -D warnings (linting)"
echo ""
echo "To bypass the hook in emergency situations, use: git push --no-verify"
