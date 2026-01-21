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

### 1. Version Decision

Before creating a release, determine the version number:

1. Review all changes since the last release
2. Identify the highest level of change (MAJOR, MINOR, or PATCH)
3. Determine if a pre-release version is needed
4. Update version in `Cargo.toml`

### 2. Update CHANGELOG

Update `CHANGELOG.md` following [Keep a Changelog](https://keepachangelog.com/) format:

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

### 3. Create Git Tag

Create an annotated tag for the release:

```bash
git tag -a v1.2.0 -m "Release version 1.2.0"
git push origin v1.2.0
```

### 4. GitHub Release

Create a GitHub release:

1. Go to repository → Releases → Draft a new release
2. Choose the tag (e.g., `v1.2.0`)
3. Release title: `Version 1.2.0` or `v1.2.0 - Feature Name`
4. Description: Copy relevant section from CHANGELOG.md
5. Attach compiled binaries (`.app`, `.dmg`)
6. Mark as pre-release if applicable

### 5. Post-Release

After creating a release:

1. Increment version in `Cargo.toml` to next development version
2. Add new section in CHANGELOG.md for unreleased changes
3. Update README.md if installation instructions changed

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
name = "dev-cleaner"
version = "1.2.0"
```

This version is automatically available in the code and used for:
- About dialog display
- Build metadata
- User agent strings (if applicable)

## Branching Strategy

- `main` - Stable release branch, always production-ready
- `develop` - Development branch for next release
- `feature/*` - Feature branches
- `release/*` - Release preparation branches
- `hotfix/*` - Urgent fixes for production

### Release Branch Workflow

1. Create release branch from `develop`:
   ```bash
   git checkout -b release/1.2.0 develop
   ```

2. Update version numbers and CHANGELOG

3. Test and fix bugs (only bug fixes, no new features)

4. Merge to `main` and tag:
   ```bash
   git checkout main
   git merge --no-ff release/1.2.0
   git tag -a v1.2.0 -m "Release 1.2.0"
   ```

5. Merge back to `develop`:
   ```bash
   git checkout develop
   git merge --no-ff release/1.2.0
   ```

6. Delete release branch:
   ```bash
   git branch -d release/1.2.0
   ```

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