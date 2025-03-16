#!/bin/bash
set -e

echo "Setting up git hooks for automated code quality checks..."

# Configure git to use the hooks from .githooks directory
git config core.hooksPath .githooks

# Ensure the pre-commit hook is executable
chmod +x .githooks/pre-commit

echo "Git hooks have been set up successfully!"
echo "The pre-commit hook will now check for compilation errors and warnings before each commit."
