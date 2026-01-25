# Git Workflow & Branching Strategy

## Branch Structure

We follow **GitHub Flow** - a simple, streamlined workflow:

- **`main`** - The single default branch, always deployable
  - All work branches off from `main`
  - All finished work merges back to `main`
  - CI/CD runs on `main` and PRs to `main`

## Branching Guidelines

### ✅ DO: Use Feature Branches

**Always create a feature branch for any changes**. Never commit directly to `main`.

```bash
# Start from main
git checkout main
git pull origin main

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

### ❌ DON'T: Push Directly to Main

```bash
# ❌ WRONG - Don't do this
git checkout main
git commit -m "some changes"
git push origin main

# ✅ CORRECT - Use feature branch
git checkout -b feature/my-changes
git commit -m "some changes"
git push origin feature/my-changes
```

## Workflow Steps

### 1. Start New Work

```bash
# Update your local main branch
git checkout main
git pull origin main

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
3. Base: `main` ← Compare: `feature/add-new-checker`
4. Fill in PR description
5. Click "Create pull request"
6. Wait for CI checks to pass
7. Merge when ready (squash and merge recommended)

### 7. After PR is Merged

```bash
# Switch back to main
git checkout main

# Pull the merged changes
git pull origin main

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
- CI workflows only run on `main` and PRs to `main`
- Feature branches (direct pushes) don't trigger CI runs
- Saves GitHub Actions minutes
- Faster iteration

### 2. **Code Review** ✅
- Pull requests enable code review
- Discuss changes before merging
- Catch issues early
- Team collaboration

### 3. **Clean History** ✅
- Main branch stays stable and deployable
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
# Periodically sync with main
git checkout feature/my-feature
git fetch origin
git merge origin/main

# Or use rebase for cleaner history
git rebase origin/main
```

### Scenario 2: Need to Fix Critical Bug

```bash
# Create hotfix branch from main
git checkout main
git pull
git checkout -b fix/urgent-bug

# Make fix
git commit -m "fix: critical bug in scanner"

# Push and create PR immediately
git push origin fix/urgent-bug

# Create PR and merge ASAP (can skip review if critical)
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
- `main` branch (after merge)
- Pull requests to `main`

### Workflows DON'T Run On:
- Feature branches (direct pushes)
- Experiment branches
- Random branch pushes

### When CI Runs:
```yaml
on:
  push:
    branches: [main]  # Only main
  pull_request:
    branches: [main]  # And PRs to main
```

This means:
- ✅ Push to `feature/xyz` → No CI (fast, iterate freely)
- ✅ Open PR to `main` → CI runs (checks your code)
- ✅ Merge PR → CI runs on main (validates integration)

### Badge Configuration:
- Build badge points to `main` branch
- Coverage badge points to `main` branch
- All status badges reflect `main` state

## Release Process

Releases follow the same branch workflow - never push directly to `main`.

### Release Branch Naming

Use the `release/` prefix:
- `release/v0.3.0` - For version 0.3.0 release
- `release/v1.0.0` - For version 1.0.0 release

### Release Workflow

```bash
# 1. Start from updated main
git checkout main
git pull origin main

# 2. Create release branch
git checkout -b release/v0.3.0

# 3. Prepare release (update version numbers)
# - Update version in Cargo.toml
# - Move CHANGELOG.md [Unreleased] to new version section
# - Update version references in docs

# 4. Verify everything works
cargo check
cargo test
cargo clippy

# 5. Commit release changes
git add -A
git commit -m "chore: prepare release v0.3.0"

# 6. Push release branch
git push -u origin release/v0.3.0

# 7. Create Pull Request on GitHub
# - Base: main ← Compare: release/v0.3.0
# - Title: "Release v0.3.0"
# - Description: Copy changelog entries for this release

# 8. Wait for CI to pass, then merge PR

# 9. After merge, create and push tag from main
git checkout main
git pull origin main
git tag v0.3.0
git push origin v0.3.0

# 10. Cleanup
git branch -d release/v0.3.0
git push origin --delete release/v0.3.0
```

### Release Checklist

Before creating release PR:
- [ ] Version updated in `Cargo.toml`
- [ ] `CHANGELOG.md` updated with release date
- [ ] Version links updated at bottom of `CHANGELOG.md`
- [ ] `docs/README.md` version table updated
- [ ] All tests passing
- [ ] No clippy warnings

After PR is merged:
- [ ] Tag created on main branch
- [ ] Tag pushed to origin
- [ ] GitHub Release created (optional, CI may automate)
- [ ] Release branch deleted

### Why Use Release Branches?

1. **Code Review** - Release preparation gets reviewed like any other change
2. **CI Validation** - All checks run before release hits main
3. **Clean History** - Release commits go through normal PR flow
4. **Rollback Safety** - Easy to reject a problematic release PR
5. **Consistency** - Same workflow for features and releases

### Tagging Strategy

Tags are created **after** the release PR is merged to main:

```bash
# ✅ CORRECT - Tag after merge
git checkout main
git pull origin main
git tag v0.3.0
git push origin v0.3.0

# ❌ WRONG - Don't tag on release branch
git checkout release/v0.3.0
git tag v0.3.0  # Wrong branch!
```

This ensures the tag points to the actual commit on `main`, not the branch.

### Automated Releases (CI)

The release workflow (`.github/workflows/release.yml`) triggers on version tags:

```yaml
on:
  push:
    tags:
      - 'v*.*.*'
```

When you push a tag like `v0.3.0`, CI automatically:
1. Builds release binary
2. Creates app bundle and DMG
3. Creates GitHub Release with artifacts

## Quick Reference

### Recommended Workflow

```bash
# 1. Start work
git checkout main && git pull
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

# 5. Create PR on GitHub (manual) to main

# 6. After merge, cleanup
git checkout main && git pull
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
- Require pull request reviews (optional: 1 reviewer for team projects)
- Require status checks to pass (CI, tests, coverage)
- Require branches to be up to date before merge
- No direct pushes (enforce via GitHub settings)
- Automatically delete head branches after merge
- Use "Squash and merge" as default merge strategy

## Summary

| ✅ DO | ❌ DON'T |
|-------|----------|
| Use feature branches | Push directly to `main` |
| Commit locally first | Push immediately after every commit |
| Accumulate related commits | Push "WIP" or broken commits |
| Test before pushing | Push untested code |
| Push when ready to share | Push work-in-progress constantly |
| Create PRs to `main` | Merge without CI passing |
| Follow naming conventions | Use vague branch names |
| Write clear commit messages | Commit "WIP" or "fixes" |
| Keep branches focused | Mix multiple features in one branch |
| Delete merged branches | Leave stale branches around |
| Sync with main regularly | Let branches diverge too much |

---

## Key Takeaways

1. **One branch to rule them all** - `main` is the only permanent branch
2. **Commit frequently** - Save your work locally often
3. **Push deliberately** - Only push when ready to share
4. **Test before pushing** - Verify quality locally first
5. **Use feature branches** - Never push directly to `main`
6. **Wait for confirmation** - Don't automate the push step
7. **Merge back quickly** - Keep features small and merge often

**GitHub Flow Benefits**:
- ✅ Simpler than Git Flow (no development branch complexity)
- ✅ Easier badge management (all point to `main`)
- ✅ Fewer merge conflicts (shorter-lived branches)
- ✅ Always deployable `main` branch
- ✅ Perfect for continuous deployment

**Remember**: 
- **Commit** = Save your work (local, frequent)
- **Push** = Share your work (remote, deliberate)
- **Main** = Always stable, always deployable

Feature branches + local commits + GitHub Flow = clean workflow, efficient CI, better collaboration! 🚀