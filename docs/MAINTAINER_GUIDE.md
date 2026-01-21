# Maintainer Quick Start Guide

This guide is for DevSweep project maintainers to help manage releases, contributions, and project health.

## Table of Contents

- [Daily Tasks](#daily-tasks)
- [Release Process](#release-process)
- [Managing Issues](#managing-issues)
- [Managing Pull Requests](#managing-pull-requests)
- [GitHub Actions](#github-actions)
- [Security](#security)
- [Community Management](#community-management)

## Daily Tasks

### Morning Routine

1. **Check GitHub Notifications**
   - New issues
   - New pull requests
   - Comments on existing items

2. **Review CI Status**
   - Check GitHub Actions for failures
   - Address any broken builds

3. **Triage New Issues**
   - Add appropriate labels
   - Ask for clarification if needed
   - Close duplicates

4. **Review New PRs**
   - Initial review within 48 hours
   - Request changes or approve

## Release Process

### Planning a Release

1. **Review Unreleased Changes**
   ```bash
   git log v0.1.0..HEAD --oneline
   ```

2. **Decide Version Number** (see `VERSIONING.md`)
   - Breaking change? → MAJOR
   - New feature? → MINOR
   - Bug fix only? → PATCH

3. **Check for Security Issues**
   ```bash
   cargo audit
   ```

### Creating a Release

#### Step 1: Update Version

Edit `Cargo.toml`:
```toml
[package]
version = "0.2.0"  # Update this
```

#### Step 2: Update CHANGELOG

Edit `CHANGELOG.md`:
```markdown
## [0.2.0] - 2024-01-15

### Added
- New feature X
- Support for Y

### Fixed
- Bug in Z

### Changed
- Improved performance of A
```

Also update the links at the bottom:
```markdown
[Unreleased]: https://github.com/canggihpw/devsweep/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/canggihpw/devsweep/compare/v0.1.0...v0.2.0
```

#### Step 3: Commit and Tag

```bash
# Commit version bump
git add Cargo.toml CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git push origin main

# Create and push tag
git tag -a v0.2.0 -m "Release version 0.2.0"
git push origin v0.2.0
```

#### Step 4: Verify Release

1. GitHub Actions will automatically:
   - Build the project
   - Create app bundle
   - Generate DMG
   - Create GitHub Release
   - Upload artifacts

2. Monitor the workflow at:
   - Actions tab → Release workflow

3. Verify the release:
   - Check GitHub Releases page
   - Download and test the DMG
   - Verify checksums

#### Step 5: Announce

- Update README if needed
- Post in Discussions
- Share on social media (if applicable)

### Hotfix Process

For urgent security or critical bug fixes:

1. **Create hotfix branch**
   ```bash
   git checkout -b hotfix/0.1.1 v0.1.0
   ```

2. **Fix the issue**
   ```bash
   # Make changes
   git commit -m "fix: critical bug in scanner"
   ```

3. **Update version**
   - Bump PATCH version
   - Update CHANGELOG

4. **Merge and release**
   ```bash
   git checkout main
   git merge --no-ff hotfix/0.1.1
   git tag -a v0.1.1 -m "Hotfix 0.1.1"
   git push origin main v0.1.1
   ```

## Managing Issues

### Issue Labels

Use these labels consistently:

- `bug` - Something isn't working
- `enhancement` - New feature or request
- `documentation` - Documentation improvements
- `good first issue` - Good for newcomers
- `help wanted` - Extra attention needed
- `question` - Further information requested
- `duplicate` - Already reported
- `wontfix` - Will not be worked on
- `security` - Security vulnerability

### Issue Triage Checklist

- [ ] Is it a duplicate? → Close and link to original
- [ ] Is it clear? → Ask for more details
- [ ] Is it valid? → Add appropriate labels
- [ ] Is it high priority? → Add to milestone
- [ ] Can someone help? → Add `help wanted` or `good first issue`

### Closing Issues

**Good reasons to close:**
- Fixed in a release
- Working as intended
- Duplicate
- Stale (no response after 30 days)
- Out of scope

**Always:**
- Explain why you're closing
- Be polite and thankful
- Link to relevant issues/PRs

## Managing Pull Requests

### PR Review Checklist

#### Code Quality
- [ ] Follows coding standards (`cargo fmt`, `cargo clippy`)
- [ ] Tests included/updated
- [ ] Documentation updated
- [ ] No unnecessary changes
- [ ] Commit messages follow convention

#### Functionality
- [ ] Builds successfully
- [ ] Tests pass
- [ ] Feature works as described
- [ ] No regressions

#### Documentation
- [ ] README updated (if needed)
- [ ] CHANGELOG updated (significant changes)
- [ ] Code comments added (complex logic)

### Responding to PRs

**Within 48 hours:**
- Acknowledge the PR
- Initial review or request changes
- Add labels

**Be constructive:**
```markdown
Thanks for the PR! A few suggestions:
- Could you add tests for the new function?
- Let's update the CHANGELOG to mention this fix
- Consider extracting this into a separate function

Overall looks great! 👍
```

### Merging PRs

**Before merging:**
1. All CI checks pass
2. Code reviewed and approved
3. No merge conflicts
4. CHANGELOG updated (for significant changes)

**Merge options:**
- **Squash and merge**: For feature branches with messy commits
- **Rebase and merge**: For clean commit history
- **Create merge commit**: For release branches

**After merging:**
- Delete the branch (if from fork, contributor handles this)
- Close related issues
- Thank the contributor

## GitHub Actions

### Monitoring CI

Check the Actions tab daily for:
- Failed builds
- Failed tests
- Security alerts

### Common CI Issues

**Clippy warnings:**
```bash
# Run locally to fix
cargo clippy --fix
```

**Format issues:**
```bash
# Run locally to fix
cargo fmt
```

**Test failures:**
```bash
# Run tests locally
cargo test
cargo test -- --nocapture  # with output
```

### Troubleshooting Releases

**Release workflow fails:**

1. Check the logs in Actions tab
2. Common issues:
   - DMG creation failed → Check `create-dmg` installation
   - Code signing failed → Check entitlements.plist
   - Upload failed → Check GITHUB_TOKEN permissions

3. Fix and re-run:
   ```bash
   # Delete the tag
   git tag -d v0.2.0
   git push origin :refs/tags/v0.2.0
   
   # Fix issues, commit
   
   # Re-create tag
   git tag -a v0.2.0 -m "Release version 0.2.0"
   git push origin v0.2.0
   ```

## Security

### Handling Security Reports

**When a vulnerability is reported:**

1. **Acknowledge within 48 hours**
   - Thank the reporter
   - Confirm you're investigating

2. **Assess severity**
   - Critical: Fix immediately
   - High: Fix within 7 days
   - Medium: Fix within 30 days
   - Low: Schedule for next release

3. **Develop fix privately**
   - Don't push to public branch
   - Test thoroughly

4. **Coordinate disclosure**
   - Agree on disclosure date with reporter
   - Prepare security advisory
   - Prepare patch release

5. **Release and disclose**
   - Release patch version
   - Publish security advisory
   - Credit reporter (unless anonymous)
   - Update SECURITY.md if needed

### Regular Security Checks

**Weekly:**
```bash
cargo audit
```

**Monthly:**
- Review dependencies for updates
- Check for deprecated dependencies
- Review GitHub security alerts

## Community Management

### Code of Conduct Enforcement

**Minor violations:**
- Private warning
- Explain what was inappropriate
- Give chance to correct

**Serious violations:**
- Public warning
- Temporary ban if repeated
- Document incidents

**Severe violations:**
- Immediate ban
- Report to GitHub if necessary

### Encouraging Contributors

**For first-time contributors:**
- Be extra welcoming
- Provide detailed feedback
- Thank them publicly
- Consider adding to Contributors list

**For regular contributors:**
- Give them credit in releases
- Consider granting triage permissions
- Invite to discussions about direction

### Managing Expectations

**Be clear about:**
- Project scope (what's in/out)
- Timeline (no promises, but estimates)
- Priorities (security > bugs > features)
- Resources (volunteer project, limited time)

**Set boundaries:**
- You're not obligated to implement every feature
- You can decline contributions that don't fit
- It's okay to close old, stale issues

## Project Health Metrics

### Monitor These

**Code:**
- Build success rate
- Test coverage
- Open critical bugs
- Dependency security issues

**Community:**
- Open issues (should be < 20)
- PR response time (< 48h goal)
- Contributor growth
- Stars/forks trend

**Releases:**
- Release cadence (monthly goal)
- Time from report to fix
- Download counts

### Tools

```bash
# Dependency check
cargo audit

# Test coverage (with tarpaulin)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Outdated dependencies
cargo install cargo-outdated
cargo outdated
```

## Communication

### Announcement Channels

- GitHub Releases (automatic)
- GitHub Discussions (manual)
- README badges (update for major releases)
- Social media (optional)

### Regular Updates

**Monthly:**
- Project status update in Discussions
- Thank contributors
- Highlight progress

**Per Release:**
- Detailed changelog
- Migration guide (if breaking changes)
- Known issues

## Emergency Procedures

### Critical Bug in Release

1. Immediately create issue and pin it
2. Start hotfix branch
3. Fix, test, release ASAP
4. Communicate on all channels

### Compromised Dependencies

1. Check impact with `cargo tree`
2. Update or replace dependency
3. Release patch version
4. Publish security advisory

### Maintainer Unavailable

**Before absence:**
- Notify co-maintainers
- Set GitHub status
- Pin notice in Discussions

**Emergency contacts:**
- List co-maintainers in README
- Emergency contact info in SECURITY.md

## Onboarding New Maintainers

### Checklist

- [ ] Add to GitHub repository collaborators
- [ ] Grant appropriate permissions (Write/Maintain)
- [ ] Share this guide
- [ ] Review recent issues/PRs together
- [ ] Add to CODEOWNERS file (if exists)
- [ ] Add to security contact list
- [ ] Introduce in Discussions

### Recommended Permissions

**Triage:**
- Label issues/PRs
- Close/reopen issues
- Request PR reviews

**Write:**
- All triage permissions
- Push to branches
- Merge PRs
- Edit wiki

**Maintain:**
- All write permissions
- Manage issues/PRs
- Edit repository settings
- Manage releases

## Resources

- [Semantic Versioning](https://semver.org/)
- [Keep a Changelog](https://keepachangelog.com/)
- [GitHub Docs - Maintainers](https://docs.github.com/en/communities)
- [Open Source Guides](https://opensource.guide/)

## Questions?

- Review `VERSIONING.md` for release questions
- Check `CONTRIBUTING.md` for contributor guidelines
- Open a discussion in GitHub Discussions

---

**Project**: DevSweep
**Remember:** You're doing great! Maintaining an open source project is hard work. Thank you for your service to the community! 💚