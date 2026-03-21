# Setup Guide

This guide will help you get Neflo up and running on your macOS system.

## System Requirements

- **Operating System**: macOS (Neflo uses macOS CoreGraphics APIs for idle detection).
- **Architecture**: Intel or Apple Silicon (M1/M2/M3/M4).
- **Rust**: Version 1.85 or later.
- **Node.js**: Version 18 or later (for the Svelte frontend).

## Installation

### Download Pre-Built Binary (Recommended)

Neflo provides pre-built macOS binaries for both Intel and Apple Silicon.

1. Download the latest release from the [GitHub Releases](https://github.com/impulia/neuroflow/releases) page.
2. Move the `.app` bundle to your `/Applications` folder.
3. Launch Neflo from your Applications or Spotlight.

### From Source

1. **Clone the repository**:
   ```bash
   git clone https://github.com/impulia/neuroflow.git
   cd neuroflow
   ```

2. **Install frontend dependencies**:
   ```bash
   cd ui && npm install && cd ..
   ```

3. **Build the Tauri app**:
   ```bash
   cargo tauri build
   ```

   The built `.app` bundle will be in `src-tauri/target/release/bundle/macos/`.

### Verification

After installation, launch Neflo. You should see a small indicator (●) appear in your macOS menu bar. Click it to open the dashboard popover.

---

[Home](index.md) | [Previous: Introduction](introduction.md) | [Next: Usage](usage.md)
