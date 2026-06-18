#!/usr/bin/env bash
set -euo pipefail

REPO="shawal-mbalire/envsec"
BINARY_NAME="envsec"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
GITHUB="https://github.com"

detect_os() {
    local os
    os="$(uname -s)"
    case "$os" in
        Linux*)   echo "linux" ;;
        Darwin*)  echo "darwin" ;;
        CYGWIN*|MINGW*|MSYS*) echo "windows" ;;
        *)        echo "unsupported" ;;
    esac
}

detect_arch() {
    local arch
    arch="$(uname -m)"
    case "$arch" in
        x86_64|amd64)   echo "x86_64" ;;
        aarch64|arm64)   echo "aarch64" ;;
        *)               echo "unsupported" ;;
    esac
}

determine_target() {
    local os="$1"
    local arch="$2"

    case "$os-$arch" in
        linux-x86_64)    echo "x86_64-unknown-linux-gnu" ;;
        linux-aarch64)   echo "aarch64-unknown-linux-gnu" ;;
        darwin-x86_64)   echo "x86_64-apple-darwin" ;;
        darwin-aarch64)  echo "aarch64-apple-darwin" ;;
        windows-x86_64)  echo "x86_64-pc-windows-msvc" ;;
        *)               echo "" ;;
    esac
}

determine_archive_ext() {
    local os="$1"
    case "$os" in
        windows) echo "zip" ;;
        *)       echo "tar.gz" ;;
    esac
}

get_latest_version() {
    local url="https://api.github.com/repos/${REPO}/releases/latest"
    if command -v curl &>/dev/null; then
        curl -sL "$url" | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/'
    elif command -v wget &>/dev/null; then
        wget -qO- "$url" | grep '"tag_name"' | head -1 | sed -E 's/.*"tag_name":\s*"([^"]+)".*/\1/'
    else
        echo ""
    fi
}

download_file() {
    local url="$1"
    local output="$2"

    if command -v curl &>/dev/null; then
        curl -sL -o "$output" "$url"
    elif command -v wget &>/dev/null; then
        wget -qO "$output" "$url"
    else
        echo "Error: curl or wget is required" >&2
        return 1
    fi
}

main() {
    local os arch target ext version url tmpdir archive

    os="$(detect_os)"
    if [ "$os" = "unsupported" ]; then
        echo "Error: unsupported operating system $(uname -s)" >&2
        exit 1
    fi

    arch="$(detect_arch)"
    if [ "$arch" = "unsupported" ]; then
        echo "Error: unsupported architecture $(uname -m)" >&2
        exit 1
    fi

    target="$(determine_target "$os" "$arch")"
    if [ -z "$target" ]; then
        echo "Error: no binary available for ${os}/${arch}" >&2
        exit 1
    fi

    ext="$(determine_archive_ext "$os")"
    version="$(get_latest_version)"

    if [ -z "$version" ]; then
        echo "Error: could not determine latest version" >&2
        exit 1
    fi

    url="${GITHUB}/${REPO}/releases/download/${version}/${BINARY_NAME}-${version}-${target}.${ext}"

    echo "Installing ${BINARY_NAME} ${version} for ${target}..."

    tmpdir="$(mktemp -d)"
    trap 'rm -rf "$tmpdir"' EXIT

    archive="${tmpdir}/${BINARY_NAME}.${ext}"
    download_file "$url" "$archive"

    if [ "$os" = "windows" ]; then
        if command -v unzip &>/dev/null; then
            unzip -o "$archive" -d "$tmpdir"
        elif command -v 7z &>/dev/null; then
            7z x "$archive" -o"$tmpdir" -y
        else
            echo "Error: unzip or 7z is required on Windows" >&2
            exit 1
        fi
        local dest="${INSTALL_DIR}/${BINARY_NAME}.exe"
    else
        tar xzf "$archive" -C "$tmpdir"
        local dest="${INSTALL_DIR}/${BINARY_NAME}"
    fi

    # Try install dir, fall back to ~/.local/bin
    if [ ! -d "$INSTALL_DIR" ] || [ ! -w "$INSTALL_DIR" ]; then
        if [ "$os" = "windows" ]; then
            INSTALL_DIR="$HOME/.local/bin"
        else
            INSTALL_DIR="$HOME/.local/bin"
        fi
        mkdir -p "$INSTALL_DIR"
        dest="${INSTALL_DIR}/${BINARY_NAME}"
        if [ "$os" = "windows" ]; then
            dest="${dest}.exe"
        fi
    fi

    if [ "$os" = "windows" ]; then
        cp "${tmpdir}/${BINARY_NAME}.exe" "$dest"
    else
        install -m 755 "${tmpdir}/${BINARY_NAME}" "$dest"
    fi

    echo ""
    echo "${BINARY_NAME} ${version} installed to ${dest}"

    # Check if install dir is in PATH
    case ":${PATH}:" in
        *":${INSTALL_DIR}:"*) ;;
        *)
            echo ""
            echo "WARNING: ${INSTALL_DIR} is not in your PATH."
            echo "Add it by running:"
            echo ""
            if [ -f "$HOME/.bashrc" ]; then
                echo "  echo 'export PATH=\"${INSTALL_DIR}:\$PATH\"' >> ~/.bashrc"
            fi
            if [ -f "$HOME/.zshrc" ]; then
                echo "  echo 'export PATH=\"${INSTALL_DIR}:\$PATH\"' >> ~/.zshrc"
            fi
            echo ""
            ;;
    esac
}

main "$@"
