# Claude for Linux

**Unofficial Community Desktop Client for Claude on Linux**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/edward-playground/claude-desktop-linux/actions/workflows/ci.yml/badge.svg)](https://github.com/edward-playground/claude-desktop-linux/actions/workflows/ci.yml)

> ⚠️ **DISCLAIMER**: This is NOT an official Anthropic product. This is an open-source community project. Claude and Anthropic are trademarks of Anthropic, PBC.

A native, secure, and feature-rich desktop client for Claude AI on Linux, built with Tauri 2.0.

## Features

- 🚀 **Native Performance**: Built with Tauri 2.0 + Rust backend (~100MB RAM)
- 🔒 **Secure API Key Storage**: Uses system keyring (GNOME Keyring/KWallet)
- 💬 **Streaming Responses**: Real-time message streaming with SSE
- 🌙 **Dark/Light/System Themes**: Automatic theme detection
- 🌐 **Multi-language**: English and Traditional Chinese (繁體中文)
- 📝 **Markdown Rendering**: Code syntax highlighting with copy button
- 🔐 **Privacy Mode**: Optional mode that doesn't save conversations
- ⌨️ **Keyboard Shortcuts**: Efficient workflow with hotkeys

## Quick Start

```bash
curl -fsSL https://raw.githubusercontent.com/edward-playground/claude-desktop-linux/main/install.sh | bash
```

Done! The script automatically:
1. Detects your distro (Ubuntu/Fedora/Arch/openSUSE)
2. Installs system dependencies
3. Installs Rust, Node.js 20, pnpm
4. Clones, builds, and installs the app

After installation, run `claude-for-linux` or find it in your application menu.

---

## Manual Installation

<details>
<summary>Ubuntu/Debian</summary>

```bash
# 1. Install dependencies
sudo apt update && sudo apt install -y \
  build-essential curl wget file libwebkit2gtk-4.1-dev libssl-dev \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libsecret-1-dev patchelf

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"

# 3. Install Node.js 20+
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
source ~/.nvm/nvm.sh && nvm install 20

# 4. Install pnpm
npm install -g pnpm@9

# 5. Clone and build
git clone https://github.com/edward-playground/claude-desktop-linux.git
cd claude-desktop-linux
pnpm install
pnpm tauri build

# 6. Install the .deb package
sudo dpkg -i src-tauri/target/release/bundle/deb/*.deb
```

</details>

<details>
<summary>Fedora/RHEL</summary>

```bash
# 1. Install dependencies
sudo dnf install -y gcc gcc-c++ curl wget file webkit2gtk4.1-devel \
  openssl-devel gtk3-devel libappindicator-gtk3-devel librsvg2-devel libsecret-devel patchelf

# 2. Install Rust, Node.js, pnpm (same as above)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source "$HOME/.cargo/env"
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
source ~/.nvm/nvm.sh && nvm install 20 && npm install -g pnpm@9

# 3. Clone and build
git clone https://github.com/edward-playground/claude-desktop-linux.git
cd claude-desktop-linux && pnpm install && pnpm tauri build

# 4. Install the .rpm package
sudo rpm -i src-tauri/target/release/bundle/rpm/*.rpm
```

</details>

<details>
<summary>Arch Linux</summary>

```bash
# 1. Install dependencies
sudo pacman -S --needed --noconfirm base-devel curl wget file webkit2gtk openssl gtk3 \
  libappindicator-gtk3 librsvg libsecret patchelf

# 2-4. Same as above (Rust, Node.js, clone, build)
```

</details>

<details>
<summary>Uninstall</summary>

```bash
# Debian/Ubuntu
sudo apt remove claude-for-linux

# Fedora/RHEL
sudo rpm -e claude-for-linux

# Arch (if installed via pacman)
sudo pacman -R claude-for-linux
```

</details>

## Requirements

- Anthropic API key ([Get one here](https://console.anthropic.com/))

## Compatibility Matrix

### Tested Distributions

| Distribution | Version | Status |
|-------------|---------|--------|
| Ubuntu | 22.04+ | ✅ Supported |
| Fedora | 38+ | ✅ Supported |
| Arch Linux | Rolling | ✅ Supported |
| Debian | 12+ | ✅ Supported |
| openSUSE | Tumbleweed | ✅ Supported |

### Desktop Environments

| Environment | X11 | Wayland |
|-------------|-----|---------|
| GNOME | ✅ | ✅ |
| KDE Plasma | ✅ | ✅ |
| XFCE | ✅ | N/A |

> **Note**: Nvidia users on Wayland may need to set `GDK_BACKEND=x11` for best compatibility.

### Input Methods

- ✅ IBus
- ✅ Fcitx/Fcitx5
- ⚠️ XIM (basic support)

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+N` | New conversation |
| `Ctrl+Enter` | Send message |
| `Ctrl+,` | Open settings |
| `Ctrl+B` | Toggle sidebar |

## Development

```bash
# After installation, run in dev mode with hot reload
cd claude-desktop-linux
pnpm tauri dev

# Commands
pnpm lint          # Run linting
pnpm test          # Run tests
pnpm format        # Format code
pnpm type-check    # Type check
pnpm tauri build   # Production build
```

## Troubleshooting

### WebKitGTK not found

```bash
# Ubuntu/Debian - ensure you have the correct version
sudo apt install libwebkit2gtk-4.1-dev

# If using older distro, you may need WebKitGTK 4.0 instead
```

### Keyring issues

If you get keyring errors, ensure your system keyring is unlocked:

```bash
# GNOME
seahorse  # Open and unlock the keyring

# KDE
kwalletmanager
```

### Wayland display issues (Nvidia)

```bash
# Run with X11 backend
GDK_BACKEND=x11 ./claude-for-linux_x86_64.AppImage
```

### Build fails with Rust errors

```bash
# Update Rust to the latest stable version
rustup update stable
```

## Configuration

Configuration files are stored following the XDG Base Directory specification:

- **Config**: `~/.config/claude-for-linux/`
- **Data**: `~/.local/share/claude-for-linux/`
- **Cache/Logs**: `~/.cache/claude-for-linux/`

## Security

- API keys are stored in the system keyring (GNOME Keyring or KWallet)
- Local database uses SQLite (optional SQLCipher encryption in future releases)
- All network traffic uses HTTPS
- No telemetry or analytics (opt-in only)

See [SECURITY.md](SECURITY.md) for the full security model.

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Anthropic](https://anthropic.com) for creating Claude
- [Tauri](https://tauri.app) for the amazing framework
- [shadcn/ui](https://ui.shadcn.com) for the beautiful UI components

---

**Note**: This is an unofficial community project. For the official Claude experience, please visit [claude.ai](https://claude.ai).
