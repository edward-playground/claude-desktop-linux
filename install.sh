#!/bin/bash

# Claude for Linux - One-click installer
# Usage: curl -fsSL https://raw.githubusercontent.com/edward-playground/claude-desktop-linux/main/install.sh | bash

set -e

echo "============================================"
echo "  Claude for Linux - One-click Installer"
echo "============================================"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    echo "Error: Please do not run this script as root."
    echo "It will ask for sudo when needed."
    exit 1
fi

# Detect distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        # Check ID_LIKE for derivative distros
        case "$ID" in
            ubuntu|debian|pop|linuxmint|elementary|zorin) echo "debian" ;;
            fedora|rhel|centos|rocky|alma) echo "fedora" ;;
            arch|manjaro|endeavouros|garuda) echo "arch" ;;
            opensuse*|suse|sles) echo "opensuse" ;;
            *)
                # Fallback to ID_LIKE
                case "$ID_LIKE" in
                    *debian*|*ubuntu*) echo "debian" ;;
                    *fedora*|*rhel*) echo "fedora" ;;
                    *arch*) echo "arch" ;;
                    *suse*) echo "opensuse" ;;
                    *) echo "$ID" ;;
                esac
                ;;
        esac
    else
        echo "unknown"
    fi
}

DISTRO=$(detect_distro)
echo "Detected: $DISTRO"

# Install system dependencies
echo ""
echo "[1/6] Installing system dependencies..."
case $DISTRO in
    debian)
        sudo apt-get update -qq
        sudo apt-get install -y -qq \
            build-essential curl wget git file \
            libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev \
            libayatana-appindicator3-dev librsvg2-dev libsecret-1-dev patchelf
        ;;
    fedora)
        sudo dnf install -y -q \
            gcc gcc-c++ curl wget git file \
            webkit2gtk4.1-devel openssl-devel gtk3-devel \
            libappindicator-gtk3-devel librsvg2-devel libsecret-devel patchelf
        ;;
    arch)
        sudo pacman -S --needed --noconfirm \
            base-devel curl wget git file webkit2gtk openssl gtk3 \
            libappindicator-gtk3 librsvg libsecret patchelf
        ;;
    opensuse)
        sudo zypper install -y \
            gcc gcc-c++ curl wget git file \
            webkit2gtk3-devel libopenssl-devel gtk3-devel \
            libappindicator3-devel librsvg-devel libsecret-devel patchelf
        ;;
    *)
        echo "Error: Unsupported distribution: $DISTRO"
        echo "Please install dependencies manually. See README.md"
        exit 1
        ;;
esac

# Install Rust
echo ""
echo "[2/6] Installing Rust..."
export CARGO_HOME="$HOME/.cargo"
export RUSTUP_HOME="$HOME/.rustup"
if [ -f "$CARGO_HOME/env" ]; then
    source "$CARGO_HOME/env"
fi
if command -v rustc &> /dev/null; then
    echo "Rust already installed: $(rustc --version)"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path
    source "$CARGO_HOME/env"
fi
export PATH="$CARGO_HOME/bin:$PATH"

# Install Node.js
echo ""
echo "[3/6] Installing Node.js..."
export NVM_DIR="$HOME/.nvm"
if [ -s "$NVM_DIR/nvm.sh" ]; then
    \. "$NVM_DIR/nvm.sh"
fi
if command -v node &> /dev/null && [[ $(node -v | cut -d'.' -f1 | tr -d 'v') -ge 20 ]]; then
    echo "Node.js already installed: $(node --version)"
else
    if [ ! -d "$NVM_DIR" ]; then
        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | PROFILE=/dev/null bash
    fi
    \. "$NVM_DIR/nvm.sh"
    nvm install 20
    nvm use 20
fi

# Install pnpm
echo ""
echo "[4/6] Installing pnpm..."
if command -v pnpm &> /dev/null; then
    echo "pnpm already installed: $(pnpm --version)"
else
    npm install -g pnpm@9
fi

# Clone and build
echo ""
echo "[5/6] Cloning and building..."
INSTALL_DIR="$HOME/claude-desktop-linux"
if [ -d "$INSTALL_DIR" ]; then
    echo "Updating existing installation..."
    cd "$INSTALL_DIR"
    git pull
else
    git clone https://github.com/edward-playground/claude-desktop-linux.git "$INSTALL_DIR"
    cd "$INSTALL_DIR"
fi

pnpm install
pnpm tauri build

# Install package
echo ""
echo "[6/6] Installing application..."
case $DISTRO in
    debian)
        sudo dpkg -i src-tauri/target/release/bundle/deb/*.deb
        ;;
    fedora)
        sudo rpm -U --force src-tauri/target/release/bundle/rpm/*.rpm 2>/dev/null || \
        sudo rpm -i src-tauri/target/release/bundle/rpm/*.rpm
        ;;
    arch|opensuse|*)
        # Use AppImage for Arch, openSUSE, and unknown distros
        echo "Installing AppImage to ~/.local/bin/"
        mkdir -p ~/.local/bin
        APPIMAGE=$(find src-tauri/target/release/bundle/appimage -name "*.AppImage" | head -1)
        if [ -n "$APPIMAGE" ]; then
            cp "$APPIMAGE" ~/.local/bin/claude-for-linux
            chmod +x ~/.local/bin/claude-for-linux

            # Create desktop entry
            mkdir -p ~/.local/share/applications
            cat > ~/.local/share/applications/claude-for-linux.desktop << 'DESKTOP'
[Desktop Entry]
Name=Claude for Linux
Comment=Unofficial Community Desktop Client for Claude
Exec=$HOME/.local/bin/claude-for-linux
Icon=claude-for-linux
Type=Application
Categories=Utility;Network;
StartupWMClass=claude-for-linux
DESKTOP
            # Fix the Exec path
            sed -i "s|\$HOME|$HOME|g" ~/.local/share/applications/claude-for-linux.desktop

            # Add to PATH hint
            if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
                echo ""
                echo "Note: Add ~/.local/bin to your PATH:"
                echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
            fi
        else
            echo "Error: AppImage not found"
            exit 1
        fi
        ;;
esac

echo ""
echo "============================================"
echo "  Installation complete!"
echo "============================================"
echo ""
echo "Run 'claude-for-linux' or find it in your application menu."
echo ""
echo "Note: You'll need an Anthropic API key to use the app."
echo "Get one at: https://console.anthropic.com/"
echo ""
