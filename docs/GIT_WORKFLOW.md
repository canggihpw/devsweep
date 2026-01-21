# Git Workflow & Branching Strategy

## Branch Structure

We follow a simplified Git Flow strategy with these permanent branches:

- **`main`** - Production-ready code, stable releases only
- **`development`** - Integration branch, latest development work

## Branching Guidelines

### ✅ DO: Use Feature Branches

**Always create a feature branch for any changes**. Never commit directly to `main` or `development`.

```bash
# Start from development
git checkout development
git pull origin development

# Create feature branch
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
# or
git checkout -b docs/documentation-update
```

### Branch Naming Convention

Use these prefixes:

- `feature/` - New features
  - Example: `feature/add-flutter-checker`
  - Example: `feature/export-results-json`

- `fix/` - Bug fixes
  - Example: `fix/scan-cache-crash`
  - Example: `fix/quarantine-permission-error`

- `docs/` - Documentation only
  - Example: `docs/add-contributing-guide`
  - Example: `docs/update-readme`

- `test/` - Test improvements
  - Example: `test/improve-coverage`
  - Example: `test/add-docker-checker-tests`

- `refactor/` - Code refactoring (no feature changes)
  - Example: `refactor/extract-cache-logic`

- `chore/` - Build, dependencies, tooling
  - Example: `chore/update-dependencies`
  - Example: `chore/setup-ci`

### ❌ DON'T: Push Directly to Protected Branches

```bash
# ❌ WRONG - Don't do this
git checkout development
git commit -m "some changes"
git push origin development

# ✅ CORRECT - Use feature branch
git checkout -b feature/my-changes
git commit -m "some changes"
git push origin feature/my-changes
```

## Workflow Steps

### 1. Start New Work

```bash
# Update your local development branch
git checkout development
git pull origin development

# Create feature branch
git checkout -b feature/add-new-checker

# Verify you're on the right branch
git branch
```

### 2. Make Changes

```bash
# Make your changes
# Edit files...

# Stage changes
git add .

# Commit with conventional commit message
git commit -m "feat: add Flutter cache checker"
```

### 3. Push Feature Branch

```bash
# Push to remote (creates new branch on GitHub)
git push origin feature/add-new-checker

# Or if branch already exists
git push
```

### 4. Create Pull Request

**On GitHub:**
1. Go to repository
2. Click "Pull requests" → "New pull request"
3. Base: `development` ← Compare: `feature/add-new-checker`
4. Fill in PR description
5. Click "Create pull request"
6. Wait for CI checks to pass
7. Request review if needed

### 5. After PR is Merged

```bash
# Switch back to development
git checkout development

# Pull the merged changes
git pull origin development

# Delete local feature branch
git branch -d feature/add-new-checker

# Delete remote feature branch (optional, GitHub can do this)
git push origin --delete feature/add-new-checker
```

## Why Use Feature Branches?

### 1. **CI Efficiency** ✅
- CI workflows only run on `main` and `development`
- Feature branches don't trigger expensive CI runs
- Saves GitHub Actions minutes
- Faster iteration

### 2. **Code Review** ✅
- Pull requests enable code review
- Discuss changes before merging
- Catch issues early
- Team collaboration

### 3. **Clean History** ✅
- Development branch stays stable
- Easy to revert problematic changes
- Clear feature separation
- Better git log

### 4. **Parallel Work** ✅
- Multiple features in progress
- No conflicts between developers
- Independent testing
- Flexible merging order

## Commit Message Convention

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation only
- `test:` - Adding or updating tests
- `refactor:` - Code refactoring
- `chore:` - Maintenance tasks
- `style:` - Code formatting (no logic change)
- `perf:` - Performance improvements

### Examples

```bash
# Feature
git commit -m "feat(checkers): add Flutter cache detection"

# Bug fix
git commit -m "fix(backend): prevent crash on invalid path"

# Documentation
git commit -m "docs: add git workflow guide"

# Test
git commit -m "test: improve backend coverage to 70%"

# Multiple paragraphs
git commit -m "feat: add export to JSON

Implements JSON export functionality for scan results.
Includes CSV export as well.

Closes #42"
```

## Pull Request Template

When creating a PR, include:

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] All tests pass locally
- [ ] Added new tests for this feature
- [ ] Manually tested the feature

## Screenshots (if applicable)
Add screenshots for UI changes

## Checklist
- [ ] Code follows project style
- [ ] Self-reviewed my code
- [ ] Commented complex logic
- [ ] Updated documentation
- [ ] No new warnings
```

## Common Scenarios

### Scenario 1: Long-Running Feature

```bash
# Periodically sync with development
git checkout feature/my-feature
git fetch origin
git merge origin/development

# Or use rebase for cleaner history
git rebase origin/development
```

### Scenario 2: Need to Fix Development Branch Urgently

```bash
# Create hotfix branch from development
git checkout development
git pull
git checkout -b fix/urgent-bug

# Make fix
git commit -m "fix: critical bug in scanner"

# Push and create PR immediately
git push origin fix/urgent-bug

# Create PR with "urgent" label
```

### Scenario 3: Experiment/POC

```bash
# Use descriptive branch name
git checkout -b experiment/try-new-algorithm

# Work freely
# If successful, clean up and create proper PR
# If failed, just delete branch
git branch -D experiment/try-new-algorithm
```

### Scenario 4: Multiple Commits in Feature Branch

```bash
# Make several commits
git commit -m "feat: add basic structure"
git commit -m "feat: add tests"
git commit -m "fix: typo"
git commit -m "refactor: improve logic"

# Option 1: Squash during PR merge (GitHub does this)
# Option 2: Interactive rebase to clean up
git rebase -i HEAD~4  # Squash last 4 commits
```

## CI/CD Integration

### Workflows Run On:
- `main` branch
- `development` branch
- Pull requests to `main` or `development`

### Workflows DON'T Run On:
- Feature branches (direct pushes)
- Experiment branches
- Random branch pushes

### When CI Runs:
```yaml
on:
  push:
    branches: [main, development]  # Only these
  pull_request:
    branches: [main, development]  # And PRs to these
```

This means:
- ✅ Push to `feature/xyz` → No CI (fast)
- ✅ Open PR to `development` → CI runs (checks your code)
- ✅ Merge PR → CI runs (validates integration)

## Quick Reference

```bash
# Start work
git checkout development && git pull
git checkout -b feature/my-feature

# Work and commit
git add . && git commit -m "feat: description"

# Push (first time)
git push -u origin feature/my-feature

# Push (subsequent times)
git push

# Create PR on GitHub
# After merge, cleanup
git checkout development && git pull
git branch -d feature/my-feature
```

## Branch Protection Rules (Recommended)

**For `main` branch:**
- Require pull request reviews (1-2 reviewers)
- Require status checks to pass
- Require branches to be up to date
- No direct pushes

**For `development` branch:**
- Require status checks to pass
- Require branches to be up to date
- Allow maintainers to bypass (for urgent fixes)

## Summary

| ✅ DO | ❌ DON'T |
|-------|----------|
| Use feature branches | Push directly to `main` or `development` |
| Create PRs for review | Merge without CI passing |
| Follow naming conventions | Use vague branch names |
| Write clear commit messages | Commit "WIP" or "fixes" |
| Keep branches focused | Mix multiple features in one branch |
| Delete merged branches | Leave stale branches around |
| Sync with development regularly | Let branches diverge too much |

---

**Remember**: Feature branches keep your workflow clean, CI efficient, and enable better collaboration! 🚀