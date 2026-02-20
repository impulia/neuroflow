# Setup Guide

This guide will help you get Neflo up and running on your macOS system.

## System Requirements

- **Operating System**: macOS (Neflo uses macOS-specific APIs for idle detection).
- **Architecture**: Intel or Apple Silicon (M1/M2/M3).
- **Rust**: Version 1.85 or later is recommended.

## Installation

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
