# Git Best Practices

This document outlines Git best practices and conventions for maintaining a clean, professional repository history.

## Commit Message Guidelines

### Commit Message Format
Use the conventional commit format for consistency and automated tooling support:

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools and libraries

### Examples
```bash
feat(dice): implement Keep Highest (K) and Keep Lowest (k) syntax

Add advanced dice rolling mechanics:
- Keep Highest (K): Roll multiple dice and keep only the highest
- Keep Lowest (k): Roll multiple dice and keep only the lowest
- Works with arithmetic operations
- Comprehensive test coverage with 7 new test cases

Closes #123

fix(parser): handle edge case in dice notation parsing

The parser was failing when parsing implicit single die notation
like "d20" without a leading count. Fixed by defaulting count to 1.

test(integration): add comprehensive dice-to-dice operation tests

Added 6 new tests covering:
- Basic dice + dice operations
- Daggerheart Hope/Fear mechanics
- Complex combinations with keep/drop operations
```

### Subject Line Rules
- Use imperative mood ("Add feature" not "Added feature")
- Keep under 50 characters
- Don't end with a period
- Capitalize the first letter
- Be specific and descriptive

### Body Guidelines
- Wrap at 72 characters
- Explain what and why, not how
- Use bullet points for multiple changes
- Reference issues and pull requests when relevant

## Branching Strategy

### Branch Naming Convention
Use descriptive branch names with prefixes:

```bash
feature/dice-to-dice-operations
fix/parser-edge-case
docs/update-readme
refactor/simplify-cli
test/add-integration-tests
chore/update-dependencies
```

### Branch Types
- **feature/**: New features or enhancements
- **fix/**: Bug fixes
- **docs/**: Documentation updates
- **refactor/**: Code refactoring without functional changes
- **test/**: Test additions or improvements
- **chore/**: Maintenance tasks, dependency updates

### Main Branch Protection
- Keep `main` branch stable and deployable
- All changes should go through feature branches
- Use pull requests for code review (when working with teams)
- Never force push to main branch

## Commit Best Practices

### Atomic Commits
- Make each commit a single logical change
- If you can't describe the commit without using "and", it's probably too big
- Each commit should be able to stand alone

### Commit Frequency
- Commit early and often during development
- Don't wait until the end of the day to commit
- Use `git add -p` to stage partial changes when needed

### Before Committing
Always run these checks before committing:
```bash
# Run tests
cargo test

# Check code quality
cargo clippy

# Format code
cargo fmt

# Check for compilation errors
cargo check
```

### Staging Best Practices
```bash
# Review changes before staging
git diff

# Stage specific files
git add src/lib.rs src/main.rs

# Stage interactively for partial commits
git add -p

# Review staged changes
git diff --cached
```

## Repository Hygiene

### .gitignore Best Practices
Keep the `.gitignore` file comprehensive and up-to-date:

```gitignore
# Rust
/target/
Cargo.lock  # Only for applications, not libraries

# IDE
.vscode/
.idea/
*.swp
*.swo

# OS
.DS_Store
Thumbs.db

# Logs
*.log

# Environment
.env
.env.local
```

### File Organization
- Keep repository root clean
- Use directories to organize related files
- Document directory structure in README
- Remove unused files and dependencies

## History Management

### Rewriting History (Use with Caution)
Only rewrite history on feature branches, never on shared branches:

```bash
# Interactive rebase to clean up commits
git rebase -i HEAD~3

# Amend the last commit
git commit --amend

# Squash commits during merge
git merge --squash feature-branch
```

### Merge vs Rebase
- **Merge**: Preserves branch history, creates merge commits
- **Rebase**: Creates linear history, cleaner but rewrites commits

For this project, prefer merge for feature integration to preserve development history.

## Collaboration Guidelines

### Pull Request Best Practices
When working with teams:

1. **Create descriptive PR titles and descriptions**
2. **Reference related issues**
3. **Keep PRs focused and reasonably sized**
4. **Ensure all tests pass**
5. **Request appropriate reviewers**
6. **Respond to feedback promptly**

### Code Review Guidelines
- Review for functionality, not just style
- Be constructive and specific in feedback
- Test the changes locally when possible
- Approve only when confident in the changes

## Release Management

### Tagging Releases
Use semantic versioning for tags:

```bash
# Create annotated tag
git tag -a v1.0.0 -m "Release version 1.0.0"

# Push tags
git push origin --tags
```

### Version Numbering
Follow semantic versioning (semver):
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Changelog Maintenance
Keep a CHANGELOG.md file updated with:
- New features
- Bug fixes
- Breaking changes
- Deprecations

## Git Configuration

### Essential Git Config
```bash
# Set your identity
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"

# Set default branch name
git config --global init.defaultBranch main

# Enable helpful colors
git config --global color.ui auto

# Set default editor
git config --global core.editor "code --wait"  # VS Code
# or
git config --global core.editor "vim"  # Vim

# Set up aliases for common commands
git config --global alias.st status
git config --global alias.co checkout
git config --global alias.br branch
git config --global alias.ci commit
git config --global alias.unstage 'reset HEAD --'
git config --global alias.last 'log -1 HEAD'
git config --global alias.visual '!gitk'
```

### Useful Git Aliases
```bash
# Pretty log format
git config --global alias.lg "log --color --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr) %C(bold blue)<%an>%Creset' --abbrev-commit"

# Show files in last commit
git config --global alias.dl "diff --name-only HEAD~1 HEAD"

# Undo last commit (keep changes)
git config --global alias.undo "reset --soft HEAD~1"
```

## Security Best Practices

### Sensitive Information
- Never commit passwords, API keys, or secrets
- Use environment variables for sensitive configuration
- Add sensitive files to `.gitignore`
- Use tools like `git-secrets` to prevent accidental commits

### Commit Signing
Consider signing commits for additional security:
```bash
# Set up GPG signing
git config --global user.signingkey YOUR_GPG_KEY_ID
git config --global commit.gpgsign true
```

## Troubleshooting Common Issues

### Undoing Changes
```bash
# Undo unstaged changes
git checkout -- filename

# Undo staged changes
git reset HEAD filename

# Undo last commit (keep changes)
git reset --soft HEAD~1

# Undo last commit (discard changes)
git reset --hard HEAD~1
```

### Fixing Mistakes
```bash
# Fix last commit message
git commit --amend -m "New commit message"

# Add forgotten files to last commit
git add forgotten-file.rs
git commit --amend --no-edit

# Remove file from staging
git reset HEAD filename
```

### Branch Management
```bash
# Delete local branch
git branch -d feature-branch

# Delete remote branch
git push origin --delete feature-branch

# Rename current branch
git branch -m new-branch-name
```

## Integration with Development Workflow

### Pre-commit Hooks
Consider setting up pre-commit hooks to automatically run checks:

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Run tests
cargo test || exit 1

# Run clippy
cargo clippy -- -D warnings || exit 1

# Format code
cargo fmt -- --check || exit 1
```

### Continuous Integration
Ensure your Git workflow integrates well with CI/CD:
- All commits should pass automated tests
- Use meaningful commit messages for better CI logs
- Tag releases for automated deployments

## Summary

Following these Git best practices will help maintain:
- **Clean commit history** that tells the story of your project
- **Professional development workflow** suitable for collaboration
- **Reliable codebase** with proper testing and quality checks
- **Easy maintenance** and debugging through clear history

Remember: Good Git practices are habits that improve over time. Start with the basics and gradually incorporate more advanced techniques as you become comfortable with the workflow.

## AI Tool Guidelines

### Commit Authorization
**IMPORTANT**: AI tools (including Amazon Q, GitHub Copilot, etc.) should **NEVER** commit changes without explicit user permission.

**Required workflow for AI tools:**
1. **Always ask before committing** - Present the proposed commit message and changes
2. **Wait for explicit approval** - Do not proceed without clear user consent
3. **Respect user decisions** - If the user declines, do not commit

**Example interaction:**
```
AI: "All tests are passing. Would you like me to commit this dice-to-dice operations implementation?"
User: "Yes, go ahead" ← Required approval
AI: [Proceeds with commit]
```

This ensures:
- **User maintains control** over repository history
- **Intentional commits** rather than automated ones
- **Proper review** of changes before they become permanent
- **Compliance with team workflows** and approval processes

**Never assume permission** - always ask explicitly before any `git commit` operation.
