# Contributing to Claude for Linux

First off, thank you for considering contributing to Claude for Linux! 🎉

## Code of Conduct

This project adheres to a [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates.

**When reporting a bug, include:**

- Your Linux distribution and version
- Desktop environment (GNOME, KDE, etc.)
- Display server (X11 or Wayland)
- App version
- Steps to reproduce
- Expected vs actual behavior
- Screenshots if applicable

### Suggesting Features

Feature requests are welcome! Please:

1. Check if the feature is already planned in the [Roadmap](docs/ROADMAP.md)
2. Search existing issues for similar suggestions
3. Provide a clear use case
4. Consider the scope (is it useful for most users?)

### Pull Requests

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests and linting
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to your branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## Development Setup

### Prerequisites

- Node.js 20+
- pnpm 9+
- Rust 1.75+
- Linux development dependencies (see README.md)

### Getting Started

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/claude-desktop-linux.git
cd claude-desktop-linux

# Install dependencies
pnpm install

# Start development server
pnpm tauri dev
```

### Project Structure

```
├── src/                  # React frontend
│   ├── components/       # UI components
│   ├── stores/          # Zustand state management
│   ├── services/        # API and service layers
│   └── i18n/            # Translations
├── src-tauri/           # Rust backend
│   ├── src/
│   │   ├── commands/    # Tauri IPC commands
│   │   ├── services/    # Business logic
│   │   └── models/      # Data models
│   └── capabilities/    # Security permissions
└── tests/               # Test suites
```

## Coding Standards

### TypeScript/React

- Use functional components with hooks
- Follow the existing code style (Prettier + ESLint)
- Add TypeScript types for all props and state
- Use meaningful variable and function names
- Keep components small and focused

```typescript
// Good
export function MessageItem({ message, onDelete }: MessageItemProps) {
  // ...
}

// Avoid
export function MI({ m, od }: any) {
  // ...
}
```

### Rust

- Follow Rust conventions (rustfmt)
- Use meaningful error messages
- Add documentation comments for public APIs
- Handle errors properly (no unwrap in production code)

```rust
// Good
/// Retrieves the API key from the system keyring.
///
/// # Errors
/// Returns `AppError::NotFound` if no API key is stored.
pub fn get_api_key(&self) -> Result<String, AppError> {
    // ...
}

// Avoid
pub fn get_key() -> String {
    entry.get_password().unwrap()
}
```

### Commits

- Use conventional commits format:
  - `feat:` new features
  - `fix:` bug fixes
  - `docs:` documentation changes
  - `style:` formatting changes
  - `refactor:` code refactoring
  - `test:` adding tests
  - `chore:` maintenance tasks

```bash
# Good
git commit -m "feat: add conversation search"
git commit -m "fix: prevent crash when keyring unavailable"

# Avoid
git commit -m "fixed stuff"
git commit -m "WIP"
```

### Tests

- Add tests for new features
- Maintain existing test coverage
- Run the full test suite before submitting PR

```bash
# Run all tests
pnpm test

# Run Rust tests
cargo test --manifest-path src-tauri/Cargo.toml

# Run linting
pnpm lint
cargo clippy --manifest-path src-tauri/Cargo.toml
```

## Pull Request Process

1. **Update documentation** if you changed APIs or added features
2. **Add tests** for new functionality
3. **Update CHANGELOG.md** with your changes
4. **Ensure CI passes** all checks
5. **Request review** from maintainers

### PR Checklist

- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] CHANGELOG.md updated
- [ ] No console.log or debug statements
- [ ] No secrets or credentials in code

## Translations

We welcome translations! To add a new language:

1. Copy `src/i18n/en.json` to `src/i18n/{language-code}.json`
2. Translate all strings
3. Add the language to `src/i18n/index.ts`
4. Test the translation in the app
5. Submit a PR

## Getting Help

- Join discussions in GitHub Issues
- Ask questions in Pull Requests
- Check existing documentation

## Recognition

Contributors will be recognized in:
- README.md contributors section
- Release notes
- GitHub contributors page

Thank you for contributing! 🙏
