# Semantic Versioning Guide

This document outlines the versioning strategy for the DevSweep project.

## Overview

DevSweep follows [Semantic Versioning 2.0.0](https://semver.org/) (SemVer). This means our version numbers follow the format:

```
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]
```

For example: `1.2.3`, `2.0.0-beta.1`, `1.5.0-rc.2+20240115`

## Version Number Components

### MAJOR version (X.0.0)

Increment when you make **incompatible API changes** or **major breaking changes**:

- Removing or renaming public configuration options
- Changing the quarantine file format in an incompatible way
- Removing support for previously supported file types
- Major UI/UX redesigns that fundamentally change user workflows
- Removing or significantly changing core features

**Example:** `1.5.3` → `2.0.0`

### MINOR version (0.X.0)

Increment when you add **new features** in a **backward-compatible** manner:

- Adding support for new development tool categories (new checkers)
- Adding new settings or configuration options
- Adding new tabs or major UI sections
- Implementing new scanning algorithms (while maintaining compatibility)
- Adding export/import functionality
- Performance improvements that are user-visible

**Example:** `1.5.3` → `1.6.0`

### PATCH version (0.0.X)

Increment when you make **backward-compatible bug fixes**:

- Fixing crashes or errors
- Correcting calculation errors in size reporting
- UI/UX bug fixes
- Performance optimizations (non-breaking)
- Documentation updates
- Security patches
- Dependency updates (without new features)

**Example:** `1.5.3` → `1.5.4`

## Pre-release Versions

Pre-release versions are denoted by appending a hyphen and a series of dot-separated identifiers:

### Alpha (α)

Early development versions, may be unstable:

```
1.0.0-alpha.1
1.0.0-alpha.2
```

**Use when:**
- Feature is implemented but not fully tested
- Known bugs may exist
- API might still change
- Only for early testing by developers

### Beta (β)

Feature-complete but may contain bugs:

```
1.0.0-beta.1
1.0.0-beta.2
```

**Use when:**
- All planned features for the version are implemented
- Ready for wider testing
- No major known bugs
- API is frozen (no more breaking changes)

### Release Candidate (rc)

Potentially final version, barring any critical bugs:

```
1.0.0-rc.1
1.0.0-rc.2
```

**Use when:**
- All features are complete and tested
- No known bugs (only potential undiscovered issues)
- Ready for final validation before release
- If no issues found, will become the release version

## Version Precedence

Version precedence determines which version is newer:

```
1.0.0-alpha.1 < 1.0.0-alpha.2 < 1.0.0-beta.1 < 1.0.0-beta.2 < 1.0.0-rc.1 < 1.0.0 < 1.0.1 < 1.1.0 < 2.0.0
```

## Initial Development

The current version is `0.1.0`, indicating initial development phase:

- Version `0.x.x` is for initial development
- Anything may change at any time
- The public API should not be considered stable
- Version `1.0.0` defines the first stable public API

### During 0.x.x Phase:

- **0.X.0**: Add new features or make breaking changes
- **0.0.X**: Bug fixes and minor improvements

## Release Process

DevSweep uses **GitHub Actions workflows** to automate the release process. The workflows are located in `.github/workflows/`:

- **`ci.yml`** - Runs on pull requests (formatting, linting, tests with coverage)
- **`release.yml`** - Automated release build and distribution

### Automated Release Workflow

The release workflow (`.github/workflows/release.yml`) automatically:

1. **Triggers** when you push a tag matching `v*.*.*` (e.g., `v1.2.0`)
2. **Extracts changelog** from `CHANGELOG.md` for that version
3. **Creates GitHub Release** with changelog as description
4. **Builds release binary** for macOS
5. **Creates app bundle** using `scripts/create-app-bundle.sh`
6. **Generates DMG installer** (e.g., `DevSweep-1.2.0.dmg`)
7. **Uploads assets** to the release:
   - DMG installer
   - Raw binary
   - Checksums file
8. **Marks as pre-release** if tag contains `alpha`, `beta`, or `rc`

### Step-by-Step Release Process

#### 1. Version Decision

Before creating a release, determine the version number:

1. Review all changes since the last release
2. Identify the highest level of change (MAJOR, MINOR, or PATCH)
3. Determine if a pre-release version is needed

#### 2. Create Release Branch

Since `main` branch is protected, create a release branch:

```bash
# Create release branch from main (or current development branch)
git checkout -b release/v1.2.0

# Make your version updates
```

#### 3. Update Version Files

Update the following files on the release branch:

**`Cargo.toml`:**
```toml
[package]
version = "1.2.0"
```

**`CHANGELOG.md`** following [Keep a Changelog](https://keepachangelog.com/) format:

```markdown
## [1.2.0] - 2024-01-15

### Added
- Support for Flutter cache scanning
- Export scan results to JSON

### Changed
- Improved scan performance by 40%

### Fixed
- Fixed crash when scanning network drives
- Corrected size calculation for symlinks
```

**Important**: The release workflow extracts changelog content between `## [VERSION]` headers, so proper formatting is critical.

**`README.md`:**
- Update version references (e.g., DMG filename examples)
- Update "About Tab" version mention

**`docs/PROJECT_SUMMARY.md`:**
- Update "Current Version"
- Update "Completed Features" if needed

#### 4. Commit and Push Release Branch

```bash
git add Cargo.toml CHANGELOG.md README.md docs/PROJECT_SUMMARY.md
git commit -m "chore: release version 1.2.0"
git push -u origin release/v1.2.0
```

#### 5. Create Pull Request

1. Go to GitHub and create a PR from `release/v1.2.0` to `main`
2. Use the PR template to document changes
3. Review and get approval
4. Merge the PR

#### 6. Create and Push Git Tag

After the PR is merged to `main`:

```bash
# Switch to main and pull latest
git checkout main
git pull origin main

# Create annotated tag
git tag -a v1.2.0 -m "Release version 1.2.0"

# Push the tag (this triggers the release workflow)
git push origin v1.2.0
```

#### 7. Monitor Automated Release

1. Go to **Actions** tab on GitHub
2. Watch the **Release** workflow run
3. The workflow will:
   - Build the macOS app
   - Create the DMG installer
   - Create a GitHub Release
   - Upload all assets

#### 8. Verify Release

Once the workflow completes:

1. Go to **Releases** tab
2. Verify the new release appears
3. Check that all assets are attached:
   - `DevSweep-1.2.0.dmg`
   - `devsweep-macos-1.2.0` (binary)
   - `checksums.txt`
4. Verify the changelog content is correct
5. Test download and installation

#### 9. Post-Release (Optional)

If continuing development immediately:

1. Create new `[Unreleased]` section in `CHANGELOG.md`
2. Consider bumping to next development version (e.g., `1.3.0-dev`)

### Manual Release (If Workflow Fails)

If the automated workflow fails, you can create a release manually:

1. Build locally:
   ```bash
   cargo build --release
   ./scripts/create-app-bundle.sh
   ```

2. Go to repository → Releases → Draft a new release
3. Choose the tag (e.g., `v1.2.0`)
4. Release title: `Release 1.2.0`
5. Description: Copy relevant section from CHANGELOG.md
6. Attach the DMG and checksums
7. Mark as pre-release if applicable
8. Publish release

## Version Numbering Examples

### Scenario 1: Bug Fix

Current version: `1.2.3`
Change: Fixed a crash when clicking scan button
New version: `1.2.4`

### Scenario 2: New Feature

Current version: `1.2.4`
Change: Added support for Ruby gems cache
New version: `1.3.0`

### Scenario 3: Breaking Change

Current version: `1.3.0`
Change: Completely redesigned settings format (incompatible with old settings)
New version: `2.0.0`

### Scenario 4: Multiple Changes

Current version: `1.3.0`
Changes:
- Added Swift package manager support (MINOR)
- Fixed UI glitch in quarantine tab (PATCH)
- Updated documentation (PATCH)

New version: `1.4.0` (highest level wins)

### Scenario 5: Pre-release

Current version: `1.4.0`
Change: Major refactoring, needs testing
New versions:
1. `1.5.0-alpha.1` (initial alpha)
2. `1.5.0-alpha.2` (after fixes)
3. `1.5.0-beta.1` (feature complete)
4. `1.5.0-rc.1` (ready for release)
5. `1.5.0` (final release)

## Version in Code

The version is primarily defined in `Cargo.toml`:

```toml
[package]
name = "devsweep"
version = "1.2.0"
```

This version is automatically available in the code and used for:
- About dialog display
- Build metadata
- User agent strings (if applicable)

## Branching Strategy

DevSweep uses **GitHub Flow** - a simplified branching model with a single main branch:

- `main` - Stable production branch, protected from direct pushes
- `feature/*` - Feature branches for new functionality
- `release/*` - Release preparation branches (version updates, changelog)
- `hotfix/*` - Urgent fixes for production issues

### Why GitHub Flow?

- Simpler than Git Flow (no separate `develop` branch)
- Works well with automated CI/CD and GitHub Actions
- Main branch always reflects production state
- All changes go through Pull Request review

### Release Branch Workflow

Since `main` is protected and requires PR review:

1. Create release branch from `main`:
   ```bash
   git checkout main
   git pull origin main
   git checkout -b release/v1.2.0
   ```

2. Update version numbers, CHANGELOG, and documentation

3. Commit and push:
   ```bash
   git add Cargo.toml CHANGELOG.md README.md docs/PROJECT_SUMMARY.md
   git commit -m "chore: release version 1.2.0"
   git push -u origin release/v1.2.0
   ```

4. Create Pull Request to `main` and get review approval

5. Merge PR to `main`

6. Create and push tag (triggers automated release):
   ```bash
   git checkout main
   git pull origin main
   git tag -a v1.2.0 -m "Release 1.2.0"
   git push origin v1.2.0
   ```

7. GitHub Actions workflow automatically builds and publishes release

8. Delete release branch:
   ```bash
   git branch -d release/v1.2.0
   git push origin --delete release/v1.2.0
   ```

### Feature Branch Workflow

1. Create feature branch from `main`:
   ```bash
   git checkout -b feature/add-new-checker
   ```

2. Develop and commit changes

3. Push and create Pull Request to `main`

4. After PR review and CI passes, merge to `main`

For more details, see [GIT_WORKFLOW.md](GIT_WORKFLOW.md).

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types:
- `feat`: New feature (MINOR version bump)
- `fix`: Bug fix (PATCH version bump)
- `docs`: Documentation only
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `perf`: Performance improvements
- `test`: Adding tests
- `chore`: Maintenance tasks
- `breaking`: Breaking change (MAJOR version bump) or use `!` after type

Examples:

```
feat(checker): add Flutter cache support

Implements scanning for Flutter build caches and pub cache.

Closes #42
```

```
fix(ui): prevent crash when scan path is invalid

Added validation before starting scan operation.

Fixes #38
```

```
feat!: redesign settings storage format

BREAKING CHANGE: Settings format changed from JSON to TOML.
Users will need to reconfigure settings after upgrade.
```

## FAQ

### When do we release 1.0.0?

Version 1.0.0 will be released when:
- All core features are implemented and stable
- API/configuration format is stable
- Sufficient testing has been performed
- Documentation is complete
- Ready for production use by general public

### What if I'm not sure which version to bump?

When in doubt:
1. Review the changes in detail
2. Consider user impact
3. Ask: "Will existing users need to change anything?"
   - Yes, significantly → MAJOR
   - No, but they get new features → MINOR
   - No, just fixes → PATCH

### Should internal refactoring bump the version?

If the refactoring:
- Changes user-facing behavior → Appropriate bump (MINOR/MAJOR)
- Improves performance noticeably → MINOR
- Only improves code quality → PATCH or wait for next release

### How to handle security fixes?

Security fixes should be released immediately:
- As PATCH if the fix doesn't change functionality
- Create hotfix branch from latest release
- Release and backport to all supported versions if needed

## Resources

- [Semantic Versioning Specification](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [GitHub Releases Documentation](https://docs.github.com/en/repositories/releasing-projects-on-github)

## Questions?

If you're unsure about versioning decisions, open an issue for discussion or ask in pull requests.