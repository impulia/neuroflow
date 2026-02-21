# Setup Guide

This guide will help you get Neflo up and running on your macOS system.

## System Requirements

- **Operating System**: macOS (Neflo uses macOS-specific APIs for idle detection).
- **Architecture**: Intel or Apple Silicon (M1/M2/M3).
- **Rust**: Version 1.85 or later is recommended.

## Installation

### Download Universal Binary (Recommended)

Neflo provides a universal binary for macOS that runs natively on both Intel and Apple Silicon (M1/M2/M3) Macs.

1.  Download the latest `neflo-macos.tar.gz` from the [GitHub Releases](https://github.com/impulia/neuroflow/releases) page.
2.  Extract the archive:
    ```bash
    tar -xzf neflo-macos.tar.gz
    ```
3.  Move the binary to your path:
    ```bash
    mv neflo /usr/local/bin/
    ```

### From Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/yourusername/neflo.git
   cd neflo
   ```

2. **Build the project**:
   ```bash
   cargo build --release
   ```

3. **Install the binary**:
   You can move the binary to a directory in your `PATH`, such as `/usr/local/bin`:
   ```bash
   cp target/release/neflo /usr/local/bin/
   ```

### Verification

After installation, verify that Neflo is working by running:
```bash
neflo --help
```

---

[Home](index.md) | [Previous: Introduction](introduction.md) | [Next: Usage](usage.md)
