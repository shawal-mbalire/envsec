# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Initial release
- AES-256-GCM encryption with Argon2id key derivation
- Local-first secret vault in `~/.envsec/`
- Git-shareable encrypted vault
- Clipboard auto-clear after 2 minutes
- Session management with configurable expiry (default 2 hours)
- Project binding via `.envsec` files
- `.env` file import and export
- `envsec run` for injecting secrets into commands
- Self-update via `envsec update`
- Cross-platform support: Linux, macOS, Windows (x86_64 and ARM64)
- One-liner install script
- Automatic update notifications
