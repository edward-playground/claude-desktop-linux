# Security Policy

## Disclaimer

Claude for Linux is an **unofficial community client**. It is NOT developed, maintained, or endorsed by Anthropic. Use at your own discretion.

## Threat Model

### Assets Protected

1. **API Key**: Your Anthropic API key grants access to paid API services
2. **Conversation Data**: May contain sensitive personal or business information
3. **System Integrity**: The application should not compromise your system

### Attack Vectors Considered

| Vector | Risk Level | Mitigation |
|--------|------------|------------|
| API key theft from disk | High | System keyring storage (encrypted) |
| API key theft from memory | Medium | Rust memory safety, no logging of keys |
| Man-in-the-middle | High | HTTPS only, certificate validation |
| Malicious plugin (future) | High | Sandboxed execution, permission system |
| Supply chain attack | Medium | Lockfiles, dependency auditing |
| Local data theft | Medium | Optional SQLCipher encryption |
| XSS in WebView | Medium | Strict CSP, input sanitization |
| IPC command injection | Medium | Command whitelist, input validation |

## Security Measures

### API Key Storage

- **Primary**: System keyring via D-Bus Secret Service API
  - GNOME Keyring (GNOME, MATE, Cinnamon)
  - KWallet (KDE Plasma)
- **Fallback**: Encrypted file with user passphrase (if keyring unavailable)
- **Never**: Plain text files, environment variables on disk, logs

```
API Key Flow:
User Input → Validation → Keyring Storage → Memory (when needed) → API Call
                                 ↓
                         Encrypted at rest
```

### Local Data Storage

- **Location**: `~/.local/share/claude-for-linux/`
- **Permissions**: Directory created with mode 0700 (owner only)
- **Database**: SQLite with optional SQLCipher encryption
- **Future**: Full database encryption with key derived from:
  - Machine ID
  - User salt (stored in keyring)
  - PBKDF2-HMAC-SHA512 (256,000 iterations)

### Network Security

- HTTPS only for API communication
- No HTTP fallback
- Certificate validation enabled
- Proxy support with authentication
- No third-party analytics or tracking

### IPC Security (Tauri)

The application uses Tauri's capability-based security model:

```json
{
  "permissions": [
    "core:default",
    "app:allow-get-api-key",
    "app:allow-set-api-key",
    "app:deny-execute-shell"
  ]
}
```

- Commands require explicit permission grants
- No shell execution commands exposed
- Input validation in Rust backend

### Content Security Policy

```
default-src 'self';
img-src 'self' data: https:;
script-src 'self';
style-src 'self' 'unsafe-inline';
connect-src 'self' https://api.anthropic.com
```

### Supply Chain Security

- `pnpm-lock.yaml` and `Cargo.lock` committed to repository
- Dependabot enabled for automated security updates
- `cargo audit` and `pnpm audit` run in CI
- Signed releases with checksums

## Privacy Features

### Privacy Mode

When enabled:
- Conversations are not saved to disk
- Only kept in memory during session
- Automatically cleared on app close

### Data Clearing

- One-click option to clear all local data
- Removes: conversations, messages, settings, API key
- Database file securely deleted

### No Telemetry

- No analytics or telemetry by default
- Optional opt-in for anonymous usage statistics (future)
- Never collects: conversation content, API keys, personal data

## Vulnerability Reporting

If you discover a security vulnerability, please report it responsibly:

1. **DO NOT** create a public GitHub issue
2. Email details to: [security@example.com] (replace with actual contact)
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We aim to respond within 48 hours and provide a fix within 7 days for critical issues.

## Security Updates

- Subscribe to GitHub releases for security updates
- Critical vulnerabilities will be announced via GitHub Security Advisories
- Update to the latest version promptly

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅ Current |
| < 0.1   | ❌ Not released |

## Security Best Practices for Users

1. **Protect your API key**: Treat it like a password
2. **Keep the app updated**: Install security updates promptly
3. **Use Privacy Mode**: For sensitive conversations
4. **Review permissions**: Check what the app can access
5. **Verify downloads**: Check checksums of downloaded packages

## Third-Party Dependencies

Major dependencies and their security considerations:

| Dependency | Purpose | Security Notes |
|------------|---------|----------------|
| Tauri 2.0 | App framework | Audited by Radically Open Security |
| rusqlite | Database | Well-maintained, bundled SQLite |
| keyring-rs | Secret storage | Uses OS native APIs |
| reqwest | HTTP client | rustls TLS, no OpenSSL |
| React | UI framework | No known critical vulnerabilities |

## Audit Status

- ❌ No formal security audit performed
- ✅ Open source for community review
- ✅ Static analysis tools (clippy, eslint) in CI
- 🔄 Planning formal audit for v1.0 release

---

Last updated: 2024-11-24
