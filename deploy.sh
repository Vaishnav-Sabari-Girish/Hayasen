#!/usr/bin/env bash

git worktree remove /tmp/hayasen-docs

acp() {
  # Check if gum is installed
  if ! command -v gum >/dev/null 2>&1; then
    echo 'Error: gum is not installed. Please install it from https://github.com/charmbracelet/gum'
    return 1
  fi

  # Stage all changes
  git add .

  # Prompt for commit message using gum
  commit_msg=$(gum input --placeholder 'commit message')
  if [ -z "$commit_msg" ]; then
    echo 'Error: Commit message cannot be empty'
    return 1
  fi

  # Commit changes
  git commit -m "$commit_msg"

  # Prompt for branch name using gum
  branch=$(git branch | gum choose | sed 's/^* //')
  if [ -z "$branch" ]; then
    echo 'Error: Branch name cannot be empty'
    return 1
  fi

  # Verify branch exists
  if ! git rev-parse --verify "$branch" >/dev/null 2>&1; then
    echo "Error: Branch $branch does not exist"
    return 1
  fi

  # Checkout the specified branch
  git checkout "$branch"

  # Get all remote names
  remotes=$(git remote)

  # Get all remote names into an array
  remotes=($(git remote))

  # Push to all remotes
  for remote in "${remotes[@]}"; do
    echo "Debug: Pushing to remote - $remote"
    git push "$remote" "$branch"
  done

  echo 'Changes added, committed, and pushed to all remotes'
}

# Exit on any error
set -e

mdbook build docs/

echo "Starting deployment process..."

# Create/update worktree for gh-pages branch
echo "Setting up worktree..."
git worktree add /tmp/hayasen-docs gh-pages 2>/dev/null || echo "Worktree already exists, continuing..."

# Clear existing content
echo "Clearing existing content..."
rm -rf /tmp/hayasen-docs/*

# Copy new content
echo "Copying new content..."
cp -r docs/book/* /tmp/hayasen-docs/

# Change to the worktree directory
cd /tmp/hayasen-docs

# Use the custom acp function
echo "Running acp to commit and push to gh-pages..."
acp

echo "Pushed to gh-pages"

cd -

echo "Pushing to main...."
acp

echo "Pushed to main"
