# Security Policy

## Supported Versions

We release patches for security vulnerabilities for DevSweep in the following versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

**Note**: As this project is in early development (0.x versions), we recommend always using the latest release.

## Reporting a Vulnerability

We take the security of DevSweep seriously. If you discover a security vulnerability, please follow these steps:

### 1. **DO NOT** Open a Public Issue

Security vulnerabilities should not be reported through public GitHub issues, as this could put users at risk.

### 2. Report Privately

Please report security vulnerabilities by:

- **Email**: Send details to the project maintainers (create a security advisory on GitHub)
- **GitHub Security Advisory**: Use the "Security" tab → "Report a vulnerability" feature

### 3. What to Include

When reporting a vulnerability, please include:

- **Description**: A clear description of the vulnerability
- **Impact**: What could an attacker accomplish?
- **Steps to Reproduce**: Detailed steps to reproduce the issue
- **Affected Versions**: Which versions are affected?
- **Proposed Fix**: If you have suggestions (optional)
- **Proof of Concept**: Code or screenshots demonstrating the issue (optional)

### 4. Response Timeline

We aim to:

- **Acknowledge** your report within **48 hours**
- **Provide an initial assessment** within **7 days**
- **Release a fix** within **30 days** for critical issues
- **Credit you** in the security advisory (unless you prefer to remain anonymous)

## Security Considerations

### File System Access

DevSweep requires access to your file system to scan and delete files. Please be aware:

- **Full Disk Access**: The app may request Full Disk Access permissions on macOS
- **Quarantine System**: Files are moved to quarantine before permanent deletion
- **No Network Access**: The app does not transmit any data over the network
- **Local Storage Only**: All data (cache, settings) is stored locally on your machine

### What We Protect Against

- **Accidental Deletion**: Two-stage deletion with quarantine system
- **Data Loss**: Undo functionality for quarantined items
- **Path Traversal**: Validation of all file paths
- **Race Conditions**: File locking mechanisms
- **Malicious Input**: Sanitization of user inputs

### What You Should Know

1. **Review Before Deleting**: Always review what will be deleted before proceeding
2. **Backup Important Data**: While we have safety mechanisms, maintain regular backups
3. **Test Quarantine**: Test the quarantine/restore functionality before deleting large amounts
4. **Grant Minimal Permissions**: Only grant the permissions necessary for the features you use

### Code Signing & Distribution

- **Development Builds**: Ad-hoc signed for local use
- **Official Releases**: Will be properly code-signed (when available)
- **Notarization**: Official releases will be notarized by Apple (planned for 1.0)

⚠️ **Warning**: Currently in alpha stage, builds are ad-hoc signed. Verify you're downloading from official sources.

## Best Practices for Users

### Installation

1. Download only from official sources:
   - GitHub Releases page
   - Official repository
2. Verify checksums of downloaded files
3. Review permissions requested by the app

### Usage

1. **Start Small**: Test with small directories first
2. **Use Quarantine**: Don't bypass the quarantine system
3. **Review Scans**: Check what was found before taking action
4. **Keep Updated**: Update to the latest version for security fixes

### For Developers/Contributors

1. **Sanitize Inputs**: Always validate and sanitize file paths and user inputs
2. **Test Permissions**: Test with minimal permissions
3. **Review Dependencies**: Regularly audit dependencies for vulnerabilities
4. **Follow Guidelines**: Adhere to secure coding practices
5. **Report Issues**: Report any security concerns immediately

## Known Security Considerations

### Current Limitations (v0.1.x)

- Ad-hoc code signing (not suitable for distribution)
- Limited sandboxing
- Requires Full Disk Access for complete functionality

### Planned Improvements

- [ ] Proper code signing with Developer ID
- [ ] App notarization for macOS
- [ ] Enhanced sandboxing where possible
- [ ] Automated security scanning in CI/CD
- [ ] Regular dependency audits

## Security Updates

Security updates will be:

- Released as soon as possible after validation
- Clearly marked in release notes
- Announced in the CHANGELOG.md
- Potentially backported to previous versions for critical issues

## Dependency Security

We use `cargo audit` to check for known vulnerabilities in dependencies:

```bash
cargo install cargo-audit
cargo audit
```

Contributors should run this before submitting pull requests.

## Disclosure Policy

When we receive a security vulnerability report:

1. We work with the reporter to understand and validate the issue
2. We develop and test a fix
3. We prepare a security advisory
4. We release the fix and publish the advisory
5. We credit the reporter (unless they prefer anonymity)

We follow **coordinated disclosure**:
- 90-day disclosure deadline from initial report
- Earlier disclosure if fix is released
- May request extended timeline for complex issues

## Bug Bounty Program

Currently, we do not offer a bug bounty program. However, we:

- Greatly appreciate security research
- Will credit researchers in security advisories
- Consider security contributions for project recognition

## Questions?

If you have questions about security but don't have a vulnerability to report:

- Open a GitHub Discussion in the Security category
- Check existing security discussions
- Review this document for common questions

## Acknowledgments

We thank the security research community for helping keep DevSweep and our users safe.

### Security Researchers

(We will acknowledge security researchers here who responsibly disclose vulnerabilities)

---

**Last Updated**: 2024  
**Policy Version**: 1.0