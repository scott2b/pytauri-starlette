# Build and Distribution Guide

This document outlines the steps needed to build and distribute the Tauri + Python sidecar application.

## Prerequisites

- Node.js and pnpm
- Rust and Cargo
- Python 3.12+ with pip
- Platform-specific Tauri dependencies (see [Tauri prerequisites](https://tauri.app/v2/guides/getting-started/prerequisites))

## 1. Python Sidecar Build

First, build the Python sidecar executable:

```bash
# Ensure you're in the virtual environment
source .venv/bin/activate  # or equivalent for your shell

# Install Python dependencies if not already done
pip install -r requirements.txt

# Build the Python executable with PyInstaller
cd src-tauri/python
pyinstaller server.spec

# Move the built executable to Tauri's binaries directory with platform-specific suffix
# For macOS ARM64:
mkdir -p ../binaries
mv dist/server ../binaries/server-aarch64-apple-darwin

# For macOS Intel:
# mv dist/server ../binaries/server-x86_64-apple-darwin

# For Linux:
# mv dist/server ../binaries/server-x86_64-unknown-linux-gnu

# For Windows:
# mv dist/server.exe ../binaries/server-x86_64-pc-windows-msvc.exe
```

## 2. Frontend Build

Build the SvelteKit frontend:

```bash
# Install Node.js dependencies
pnpm install

# Build the frontend
pnpm build
```

## 3. Tauri Application Build

Build the complete Tauri application:

```bash
# Development build
pnpm tauri dev

# Production build
pnpm tauri build
```

The production build will create platform-specific installers in `src-tauri/target/release/bundle/`.

## Platform-Specific Notes

### macOS
- The built app will be in `src-tauri/target/release/bundle/dmg/`
- Code signing and notarization required for distribution (see [Tauri macOS signing guide](https://tauri.app/v2/guides/distribution/sign-macos))

### Windows
- The built app will be in `src-tauri/target/release/bundle/msi/`
- Code signing recommended for distribution (see [Tauri Windows signing guide](https://tauri.app/v2/guides/distribution/sign-windows))

### Linux
- Various bundle formats available (AppImage, deb, rpm)
- Built bundles will be in `src-tauri/target/release/bundle/`

## Development Workflow

For development, you can:

1. Start the Python server directly:
```bash
cd src-tauri/python
python server.py
```

2. Run the Tauri development build:
```bash
pnpm tauri dev
```

## Troubleshooting

### Python Sidecar
- Ensure the Python executable is built with the correct target triple suffix
- Check the PyInstaller build output for any missing dependencies
- Verify the executable path in `tauri.conf.json` matches your platform
- If the Python server appears to be running after closing the app, you can check for and kill lingering processes:
  ```bash
  # On macOS/Linux:
  ps aux | grep server
  kill <PID>

  # On Windows:
  tasklist | findstr server
  taskkill /PID <PID> /F
  ```

### Tauri Build
- Check `src-tauri/target/debug/build/` for build logs
- Ensure all Tauri system dependencies are installed
- Verify capabilities are correctly configured in `src-tauri/capabilities/default.json` 