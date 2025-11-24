#!/bin/bash

# Claude for Linux - One-click installer
# Usage: curl -fsSL https://raw.githubusercontent.com/edward-playground/claude-desktop-linux/main/install.sh | bash

set -e

echo "============================================"
echo "  Claude for Linux - One-click Installer"
echo "============================================"
echo ""

# Detect distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
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
    ubuntu|debian|linuxmint|pop)
        sudo apt-get update -qq
        sudo apt-get install -y -qq \
            build-essential curl wget git file \
            libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev \
            libayatana-appindicator3-dev librsvg2-dev libsecret-1-dev patchelf
        ;;
    fedora|rhel|centos)
        sudo dnf install -y -q \
            gcc gcc-c++ curl wget git file \
            webkit2gtk4.1-devel openssl-devel gtk3-devel \
            libappindicator-gtk3-devel librsvg2-devel libsecret-devel patchelf
        ;;
    arch|manjaro|endeavouros)
        sudo pacman -S --needed --noconfirm \
            base-devel curl wget git file webkit2gtk openssl gtk3 \
            libappindicator-gtk3 librsvg libsecret patchelf
        ;;
    opensuse*)
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
    ubuntu|debian|linuxmint|pop)
        sudo dpkg -i src-tauri/target/release/bundle/deb/*.deb
        ;;
    fedora|rhel|centos)
        sudo rpm -i src-tauri/target/release/bundle/rpm/*.rpm
        ;;
    arch|manjaro|endeavouros)
        # For Arch, use the AppImage or install manually
        echo "Installing AppImage to ~/.local/bin/"
        mkdir -p ~/.local/bin
        cp src-tauri/target/release/bundle/appimage/*.AppImage ~/.local/bin/claude-for-linux
        chmod +x ~/.local/bin/claude-for-linux
        ;;
    *)
        echo "Package installed in: $INSTALL_DIR/src-tauri/target/release/bundle/"
        ;;
esac

echo ""
echo "============================================"
echo "  Installation complete!"
echo "============================================"
echo ""
echo "Run 'claude-for-linux' or find it in your application menu."
echo ""
