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

### 2. Make Changes and Commit Locally

```bash
# Make your changes
# Edit files...

# Stage changes
git add .

# Commit with conventional commit message
git commit -m "feat: add Flutter cache checker"

# ⚠️ WAIT! Don't push yet!
# Continue working and making more commits...
```

**Important**: Commit frequently but push deliberately!

### 3. Make Multiple Commits (Optional)

```bash
# Continue working on your feature
# Make more changes...

git add .
git commit -m "test: add tests for Flutter checker"

# More changes...
git commit -m "docs: update readme with Flutter support"

# Still not pushing - accumulate commits locally
```

### 4. Review Your Work Before Pushing

```bash
# Review what you've committed
git log --oneline -5

# Check diff with development
git diff development

# Verify all tests pass locally
cargo test

# Check code quality
cargo fmt --check
cargo clippy
```

### 5. Push When Ready (User Decision)

```bash
# Only push when you're ready to share your work
# This is a DELIBERATE action, not automatic

# First time pushing this branch
git push -u origin feature/add-new-checker

# Or if branch already exists
git push
```

**Why wait to push?**
- ✅ Accumulate related commits together
- ✅ Verify tests pass before sharing
- ✅ Fix mistakes locally without polluting remote
- ✅ Group logical chunks of work
- ✅ Reduce noise in remote repository

### 6. Create Pull Request

**On GitHub:**
1. Go to repository
2. Click "Pull requests" → "New pull request"
3. Base: `development` ← Compare: `feature/add-new-checker`
4. Fill in PR description
5. Click "Create pull request"
6. Wait for CI checks to pass
7. Request review if needed

### 7. After PR is Merged

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

## Why Commit Locally Before Pushing?

### Benefits of Local Commits

1. **Fix Mistakes Privately** ✅
   ```bash
   # Made a typo in commit message?
   git commit --amend -m "fix: correct typo in message"
   
   # Need to add forgotten files?
   git add forgotten_file.rs
   git commit --amend --no-edit
   
   # Want to combine commits?
   git rebase -i HEAD~3
   ```

2. **Work Offline** ✅
   - Commit without internet connection
   - Full version control locally
   - Push when ready and online

3. **Organize Your Work** ✅
   - Group related commits
   - Create logical checkpoints
   - Clean up before sharing

4. **Reduce Remote Noise** ✅
   - Don't push "WIP" commits
   - Don't push broken code
   - Only share polished work

### Workflow Pattern

```bash
# Day 1: Work locally
git commit -m "feat: initial structure"
git commit -m "feat: add basic logic"
git commit -m "fix: typo"
# Don't push yet - still working

# Day 2: Continue working
git commit -m "test: add unit tests"
git commit -m "refactor: improve naming"
# Still not pushing

# Day 3: Ready to share
# Run final checks
cargo test && cargo clippy
# Everything looks good? NOW push
git push -u origin feature/my-feature
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

### Recommended Workflow

```bash
# 1. Start work
git checkout development && git pull
git checkout -b feature/my-feature

# 2. Work and commit LOCALLY (repeat as needed)
# Edit files...
git add .
git commit -m "feat: description"

# More work...
git commit -m "test: add tests"

# More work...
git commit -m "docs: update docs"

# 3. Verify before pushing
cargo test
cargo clippy

# 4. Push when ready (DELIBERATE action)
git push -u origin feature/my-feature

# 5. Create PR on GitHub (manual)

# 6. After merge, cleanup
git checkout development && git pull
git branch -d feature/my-feature
```

### Anti-Pattern to Avoid

```bash
# ❌ DON'T: Commit and immediately push
git commit -m "WIP"
git push  # Too soon!

# ❌ DON'T: Push broken code
git commit -m "doesn't work yet"
git push  # Tests failing!

# ❌ DON'T: Push every tiny change
git commit -m "fix typo"
git push
git commit -m "fix another typo"
git push
git commit -m "oops one more"
git push  # Noisy!
```

### Best Practice Pattern

```bash
# ✅ DO: Accumulate commits, push once ready
git commit -m "feat: initial implementation"
git commit -m "test: add comprehensive tests"
git commit -m "docs: document new feature"
# All tests pass? Good to share!
git push -u origin feature/my-feature
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
| Commit locally first | Push immediately after every commit |
| Accumulate related commits | Push "WIP" or broken commits |
| Test before pushing | Push untested code |
| Push when ready to share | Push work-in-progress constantly |
| Create PRs for review | Merge without CI passing |
| Follow naming conventions | Use vague branch names |
| Write clear commit messages | Commit "WIP" or "fixes" |
| Keep branches focused | Mix multiple features in one branch |
| Delete merged branches | Leave stale branches around |
| Sync with development regularly | Let branches diverge too much |

---

## Key Takeaways

1. **Commit frequently** - Save your work locally often
2. **Push deliberately** - Only push when ready to share
3. **Test before pushing** - Verify quality locally first
4. **Use feature branches** - Never push directly to main/development
5. **Wait for confirmation** - Don't automate the push step

**Remember**: 
- **Commit** = Save your work (local, frequent, safe)
- **Push** = Share your work (remote, deliberate, reviewed)

Feature branches + local commits keep your workflow clean, CI efficient, and enable better collaboration! 🚀