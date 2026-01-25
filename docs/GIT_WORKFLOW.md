# Git Workflow & Branching Strategy

## Branch Structure (GitHub Flow)

- **`main`** - Single default branch, always deployable
- **Feature branches** - All work happens here, merged via PR

## Branch Naming

| Prefix | Use |
|--------|-----|
| `feature/` | New features |
| `fix/` | Bug fixes |
| `release/` | Release preparation |
| `docs/` | Documentation |
| `refactor/` | Code refactoring |
| `chore/` | Maintenance |

## Basic Workflow

```bash
# 1. Start from main
git checkout main && git pull

# 2. Create feature branch
git checkout -b feature/my-feature

# 3. Work and commit locally (repeat)
git add . && git commit -m "feat: description"

# 4. Verify before pushing
cargo test && cargo clippy

# 5. Push when ready
git push -u origin feature/my-feature

# 6. Create PR on GitHub → main

# 7. After merge, cleanup
git checkout main && git pull
git branch -d feature/my-feature
```

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>
```

| Type | Description |
|------|-------------|
| `feat` | New feature |
| `fix` | Bug fix |
| `docs` | Documentation |
| `test` | Tests |
| `refactor` | Refactoring |
| `chore` | Maintenance |
| `perf` | Performance |

## Release Process

**Never push directly to main** - use release branches.

```bash
# 1. Create release branch
git checkout main && git pull
git checkout -b release/v0.3.0

# 2. Update versions
# - Cargo.toml version
# - CHANGELOG.md (move Unreleased to new version)
# - docs/README.md version table

# 3. Verify
cargo check && cargo test && cargo clippy

# 4. Commit and push
git add -A
git commit -m "chore: prepare release v0.3.0"
git push -u origin release/v0.3.0

# 5. Create PR on GitHub → main

# 6. After merge, tag from main
git checkout main && git pull
git tag v0.3.0
git push origin v0.3.0

# 7. Manually trigger release workflow on GitHub Actions
# (Go to Actions → Release → Run workflow → Enter tag)

# 8. Cleanup
git branch -d release/v0.3.0
```

### Release Checklist

**Before PR:**
- [ ] Version in `Cargo.toml`
- [ ] `CHANGELOG.md` with date
- [ ] `docs/README.md` version
- [ ] Tests passing
- [ ] No clippy warnings

**After merge:**
- [ ] Tag created and pushed
- [ ] Release workflow triggered
- [ ] Release branch deleted

## Automated Releases (CI)

The release workflow is **manually triggered** (not on tag push).

**To create a GitHub Release:**
1. Push tag: `git push origin v0.3.0`
2. GitHub → Actions → Release → Run workflow
3. Enter tag name (e.g., `v0.3.0`)

This allows tags without releases (for milestones, dev markers).

## AI Assistant Commit & Tag Workflow

When you ask to commit, the AI will:

1. **Check changes since last tag** (CHANGELOG.md [Unreleased])
2. **Recommend whether to tag**:

| Changes | Recommendation |
|---------|----------------|
| 1-2 small fixes | No tag yet |
| Multiple bug fixes | PATCH (v0.3.1) |
| New feature(s) | MINOR (v0.4.0) |
| Breaking changes | MAJOR (v1.0.0) |

3. **You decide** - just commit, or commit + prepare release

See [VERSIONING.md](VERSIONING.md) for version decision criteria.

## Quick Reference

### Do ✅

- Use feature branches
- Commit locally, push deliberately
- Test before pushing
- Create PRs to main
- Delete merged branches

### Don't ❌

- Push directly to main
- Push WIP/broken code
- Push every tiny change
- Merge without CI passing
