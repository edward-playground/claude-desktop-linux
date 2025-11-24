#!/bin/bash

# Development environment setup script for Claude for Linux
# Supports Ubuntu/Debian, Fedora, and Arch Linux

set -e

echo "=========================================="
echo "Claude for Linux - Development Setup"
echo "=========================================="
echo ""

# Detect Linux distribution
detect_distro() {
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        echo "$ID"
    elif [ -f /etc/debian_version ]; then
        echo "debian"
    elif [ -f /etc/fedora-release ]; then
        echo "fedora"
    elif [ -f /etc/arch-release ]; then
        echo "arch"
    else
        echo "unknown"
    fi
}

DISTRO=$(detect_distro)
echo "Detected distribution: $DISTRO"
echo ""

# Install system dependencies
install_dependencies() {
    case $DISTRO in
        ubuntu|debian|linuxmint|pop)
            echo "Installing dependencies for Debian/Ubuntu..."
            sudo apt-get update
            sudo apt-get install -y \
                build-essential \
                curl \
                wget \
                file \
                libwebkit2gtk-4.1-dev \
                libssl-dev \
                libgtk-3-dev \
                libayatana-appindicator3-dev \
                librsvg2-dev \
                libsecret-1-dev \
                patchelf
            ;;
        fedora|rhel|centos)
            echo "Installing dependencies for Fedora/RHEL..."
            sudo dnf install -y \
                gcc \
                gcc-c++ \
                curl \
                wget \
                file \
                webkit2gtk4.1-devel \
                openssl-devel \
                gtk3-devel \
                libappindicator-gtk3-devel \
                librsvg2-devel \
                libsecret-devel \
                patchelf
            ;;
        arch|manjaro|endeavouros)
            echo "Installing dependencies for Arch Linux..."
            sudo pacman -S --needed --noconfirm \
                base-devel \
                curl \
                wget \
                file \
                webkit2gtk \
                openssl \
                gtk3 \
                libappindicator-gtk3 \
                librsvg \
                libsecret \
                patchelf
            ;;
        opensuse*)
            echo "Installing dependencies for openSUSE..."
            sudo zypper install -y \
                gcc \
                gcc-c++ \
                curl \
                wget \
                file \
                webkit2gtk3-devel \
                libopenssl-devel \
                gtk3-devel \
                libappindicator3-devel \
                librsvg-devel \
                libsecret-devel \
                patchelf
            ;;
        *)
            echo "Warning: Unknown distribution. Please install dependencies manually."
            echo "Required packages:"
            echo "  - WebKitGTK 4.1"
            echo "  - GTK3"
            echo "  - OpenSSL"
            echo "  - libsecret"
            echo "  - librsvg"
            echo "  - patchelf"
            ;;
    esac
}

# Install Rust
install_rust() {
    if command -v rustc &> /dev/null; then
        echo "Rust is already installed: $(rustc --version)"
    else
        echo "Installing Rust..."
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
        source "$HOME/.cargo/env"
    fi
}

# Install Node.js (via nvm)
install_node() {
    if command -v node &> /dev/null; then
        NODE_VERSION=$(node --version)
        echo "Node.js is already installed: $NODE_VERSION"
    else
        echo "Installing Node.js via nvm..."
        curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.0/install.sh | bash
        export NVM_DIR="$HOME/.nvm"
        [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"
        nvm install 20
        nvm use 20
    fi
}

# Install pnpm
install_pnpm() {
    if command -v pnpm &> /dev/null; then
        echo "pnpm is already installed: $(pnpm --version)"
    else
        echo "Installing pnpm..."
        npm install -g pnpm@9
    fi
}

# Main setup
echo "Step 1: Installing system dependencies..."
install_dependencies
echo ""

echo "Step 2: Installing Rust..."
install_rust
echo ""

echo "Step 3: Installing Node.js..."
install_node
echo ""

echo "Step 4: Installing pnpm..."
install_pnpm
echo ""

echo "Step 5: Installing project dependencies..."
pnpm install
echo ""

echo "=========================================="
echo "Setup complete!"
echo "=========================================="
echo ""
echo "To start development:"
echo "  pnpm tauri dev"
echo ""
echo "To build for production:"
echo "  pnpm tauri build"
echo ""
