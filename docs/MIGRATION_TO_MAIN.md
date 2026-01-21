# Migration Guide: Development Branch → Main Branch

## Overview

We are switching from **Git Flow** (main + development) to **GitHub Flow** (main only) for a simpler, more efficient workflow.

## What's Changing?

### Before (Git Flow)
```
main (production releases)
  ↑
development (integration branch)
  ↑
feature branches
```

### After (GitHub Flow)
```
main (always deployable)
  ↑
feature branches
```

## Why This Change?

### Benefits
- ✅ **Simpler workflow** - One branch to rule them all
- ✅ **Easier badges** - CI/Coverage badges all point to `main`
- ✅ **Fewer conflicts** - Shorter-lived feature branches
- ✅ **Faster iteration** - Merge to `main` immediately
- ✅ **Always deployable** - `main` is always in good state
- ✅ **Clearer CI status** - One branch to monitor

### What We Lose
- ❌ Separate "integration" branch (not needed for this project)
- ❌ Ability to have "unreleased" work sitting in development

**Verdict**: The benefits far outweigh the costs for this project.

---

## Migration Steps

### For Maintainers

#### Step 1: Merge Development to Main

```bash
# Make sure everything is committed
git status

# Switch to main
git checkout main
git pull origin main

# Merge development into main
git merge development

# Resolve any conflicts if they exist
# Then commit and push
git push origin main
```

#### Step 2: Update Default Branch on GitHub

1. Go to: `https://github.com/canggihpw/devsweep/settings`
2. Click "Branches" in the left sidebar
3. Under "Default branch", click the switch icon
4. Select `main`
5. Click "Update"
6. Confirm the change

#### Step 3: Update Branch Protection Rules

1. Go to: `https://github.com/canggihpw/devsweep/settings/branches`
2. Remove protection from `development` branch (if any)
3. Add/update protection for `main`:
   - ✅ Require pull request reviews
   - ✅ Require status checks to pass
   - ✅ Require branches to be up to date
   - ✅ Automatically delete head branches

#### Step 4: Deprecate Development Branch

**Option A: Archive it (recommended)**
```bash
# Create a tag for historical reference
git tag archive/development development
git push origin archive/development

# Delete remote development branch
git push origin --delete development

# Delete local development branch
git branch -d development
```

**Option B: Keep it temporarily**
- Leave `development` branch as-is
- Add note in README: "⚠️ Use `main` branch, `development` is deprecated"
- Delete after 30 days

#### Step 5: Update Documentation

Already done in this PR! But verify:
- [x] `docs/GIT_WORKFLOW.md` - Updated to GitHub Flow
- [x] `.github/workflows/ci.yml` - Points to `main`
- [x] `.github/workflows/coverage.yml` - Points to `main`
- [x] `README.md` - Badges point to `main`

---

### For Contributors

#### If You Have Local `development` Branch

```bash
# Update main to latest
git checkout main
git pull origin main

# Delete your local development branch
git branch -d development

# Delete remote tracking branch
git fetch --prune
```

#### If You Have Feature Branches Based on `development`

**Option 1: Rebase onto main (recommended)**
```bash
# Switch to your feature branch
git checkout feature/my-feature

# Rebase onto main
git fetch origin
git rebase origin/main

# Force push (your branch, so it's safe)
git push --force-with-lease
```

**Option 2: Recreate from main**
```bash
# Save your work
git checkout feature/my-feature
git diff main > my-changes.patch

# Create new branch from main
git checkout main
git pull origin main
git checkout -b feature/my-feature-new

# Apply your changes
git apply my-changes.patch
git add .
git commit -m "your message"
```

#### Update Your Workflow

**Old workflow:**
```bash
git checkout development
git pull origin development
git checkout -b feature/xyz
```

**New workflow:**
```bash
git checkout main
git pull origin main
git checkout -b feature/xyz
```

---

## Timeline

- **Day 0 (Today)**: Merge this PR with GitHub Flow documentation
- **Day 1**: Merge `development` to `main`
- **Day 2**: Update default branch on GitHub to `main`
- **Day 3**: Add deprecation notice to `development` branch
- **Day 7**: Delete `development` branch

---

## FAQ

### Q: What happens to my open PRs targeting `development`?

**A:** Update them to target `main` instead:
1. Go to your PR on GitHub
2. Click "Edit" next to the base branch
3. Change base from `development` to `main`
4. PR will be updated automatically

### Q: What if I already pushed to `development`?

**A:** Your changes are safe! They'll be merged to `main` in Step 1.

### Q: Will this break CI/CD?

**A:** No! The CI/CD workflows have been updated to work with `main`.

### Q: Can I still use feature branches?

**A:** Absolutely! Feature branches are still the way to work. They just branch from `main` now instead of `development`.

### Q: What about releases?

**A:** Releases still come from `main`. We'll use Git tags for versioning:
```bash
git tag v1.0.0
git push origin v1.0.0
```

### Q: Is `main` stable?

**A:** Yes! `main` should always be in a deployable state. All PRs must pass CI before merging.

### Q: What if we need to batch features before release?

**A:** Use release branches:
```bash
# Create release branch from main
git checkout -b release/v1.2.0 main

# Cherry-pick or merge features
git merge feature/a
git merge feature/b

# Test and finalize
# Then merge to main and tag
git checkout main
git merge release/v1.2.0
git tag v1.2.0
```

---

## Verification Checklist

After migration, verify:

- [ ] Default branch on GitHub is `main`
- [ ] CI badge points to `main` branch
- [ ] Codecov badge points to `main` branch
- [ ] CI workflow runs on pushes to `main`
- [ ] Coverage workflow runs on pushes to `main`
- [ ] New PRs default to `main` as base
- [ ] Branch protection rules set on `main`
- [ ] Old `development` branch deleted (or deprecated)

---

## Rollback Plan

If issues arise, we can rollback:

```bash
# Restore development branch (if deleted)
git checkout -b development archive/development
git push origin development

# Update workflows
git checkout -b fix/restore-development
# Edit .github/workflows/*.yml to include development
git push -u origin fix/restore-development
# Create PR
```

But we don't expect to need this! GitHub Flow is simpler and better for this project.

---

## Support

Questions about the migration? Open an issue or discussion on GitHub.

**Reference**: [GitHub Flow Guide](https://docs.github.com/en/get-started/quickstart/github-flow)

---

**Migration Status**: 🟡 In Progress

Last Updated: 2024