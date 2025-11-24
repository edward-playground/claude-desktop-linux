# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of Claude for Linux
- Core chat functionality with streaming responses
- API key secure storage via system keyring (GNOME Keyring/KWallet)
- Multi-conversation management
- First-run setup wizard
- Dark/Light/System theme support
- English and Traditional Chinese (繁體中文) translations
- Markdown rendering with syntax highlighting
- Privacy mode (no local storage)
- Model selection (Claude Sonnet 4, Opus 4.5, Haiku 4.5)
- Keyboard shortcuts (Ctrl+N, Ctrl+Enter, Ctrl+,, Ctrl+B)
- Settings dialog with API, theme, and privacy options
- AppImage, deb, and rpm packaging

### Security
- API keys stored in system keyring, never in plain text
- Capability-based IPC security model
- Strict Content Security Policy
- HTTPS only for API communication

## [0.1.0] - 2024-XX-XX

Initial MVP release.

---

## Roadmap

### v0.2.0 (Planned)
- [ ] SQLCipher database encryption
- [ ] File upload support (images, PDFs)
- [ ] Conversation export (Markdown, JSON)
- [ ] Conversation search
- [ ] System tray integration

### v0.3.0 (Planned)
- [ ] Plugin system architecture
- [ ] MCP client as plugin
- [ ] Command palette (Ctrl+K)
- [ ] Conversation tags and pinning
- [ ] Flatpak packaging

### v1.0.0 (Future)
- [ ] Full plugin SDK
- [ ] Formal security audit
- [ ] Performance optimization
- [ ] Additional language support
