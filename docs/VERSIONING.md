# Semantic Versioning Guide

DevSweep follows [Semantic Versioning 2.0.0](https://semver.org/).

## Version Format

```
MAJOR.MINOR.PATCH[-PRERELEASE]
```

Examples: `1.2.3`, `2.0.0-beta.1`, `1.5.0-rc.1`

## When to Bump

| Change Type | Version | Examples |
|-------------|---------|----------|
| **MAJOR** (X.0.0) | Breaking changes | Config format change, removed features, major UI redesign |
| **MINOR** (0.X.0) | New features (backward-compatible) | New checker, new settings, new tab, export feature |
| **PATCH** (0.0.X) | Bug fixes (backward-compatible) | Crash fix, UI fix, perf optimization, docs update |

**Rule**: When multiple change types exist, use the highest level.

## Pre-release Versions

| Stage | Format | Use When |
|-------|--------|----------|
| Alpha | `1.0.0-alpha.1` | Early testing, may be unstable |
| Beta | `1.0.0-beta.1` | Feature-complete, needs wider testing |
| RC | `1.0.0-rc.1` | Ready for release, final validation |

**Precedence**: `alpha < beta < rc < release`

## Initial Development (0.x.x)

During `0.x.x` phase:
- **0.X.0**: New features or breaking changes
- **0.0.X**: Bug fixes only
- Version `1.0.0` marks first stable release

## Version Files to Update

When releasing, update:
1. `Cargo.toml` - `version = "X.Y.Z"`
2. `CHANGELOG.md` - Move `[Unreleased]` to `[X.Y.Z] - YYYY-MM-DD`
3. `docs/README.md` - Version table

## AI Assistant Tagging Recommendations

When you ask to commit, the AI will:

1. **Check changes since last tag** via `CHANGELOG.md` [Unreleased] section
2. **Recommend tag or not** based on:

| Changes Since Last Tag | Recommendation |
|------------------------|----------------|
| 1-2 small fixes/docs | No tag yet |
| Multiple bug fixes | PATCH (v0.3.1) |
| New feature(s) | MINOR (v0.4.0) |
| Breaking changes | MAJOR (v1.0.0) |
| WIP / tests failing | No tag |

3. **You decide** - Recommendation is advisory only

## Quick Decision Guide

**"Will existing users need to change anything?"**
- Yes, significantly → **MAJOR**
- No, but they get new features → **MINOR**  
- No, just fixes → **PATCH**

## Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GIT_WORKFLOW.md](GIT_WORKFLOW.md) - For release process and branching
