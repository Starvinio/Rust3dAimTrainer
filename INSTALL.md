# Installation Instructions

## Prerequisites

Before installing Rust3dAimTrainer, ensure you have the following installed on your system:

### Required Software

1. **Rust Toolchain** (version 1.70.0 or later)
    - Install from [rustup.rs](https://rustup.rs/)
    - Verify installation: `rustc --version`

2. **Git** (for cloning the repository)
    - Download from [git-scm.com](https://git-scm.com/)

### System Requirements

- **Operating System**: Windows 10/11, Linux (Ubuntu 20.04+), or macOS 10.15+
- **RAM**: 4GB minimum, 8GB recommended
- **Disk Space**: 500MB free space

## Installation Methods

### Method 1: Build from Source (Recommended)

#### Step 1: Clone the Repository

```bash
git clone https://github.com/Starvinio/Rust3dAimTrainer.git
cd Rust3dAimTrainer
```

#### Step 2: Build the Project

```bash
cargo build --release
```

This will compile the project with optimizations enabled. The process may take 2-5 minutes depending on your system.

#### Step 3: Run the Application

```bash
cargo run --release
```

Alternatively, run the compiled binary directly:

**Windows:**
```bash
.\target\release\Rust3dAimTrainer.exe
```

**Linux/macOS:**
```bash
./target/release/Rust3dAimTrainer
```

### Method 2: Download Pre-built Binary

*Note: Pre-built binaries will be available in the [Releases](https://github.com/Starvinio/Rust3dAimTrainer/releases) section once published.*

1. Go to the [Releases page](https://github.com/Starvinio/Rust3dAimTrainer/releases)
2. Download the appropriate binary for your operating system:
    - `rust3daimtrainer-windows-x64.zip` (Windows)
    - `rust3daimtrainer-linux-x64.tar.gz` (Linux)
    - `rust3daimtrainer-macos-x64.tar.gz` (macOS)
3. Extract the archive
4. Run the executable

## Platform-Specific Instructions

### Windows

**Additional Dependencies:**
- Visual Studio C++ Build Tools (if building from source)
    - Download from [Visual Studio](https://visualstudio.microsoft.com/downloads/)
    - Select "Desktop development with C++" workload

**Running the Application:**
```cmd
cd Rust3dAimTrainer
cargo run --release
```

### Linux (Ubuntu/Debian)

**Additional Dependencies:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libx11-dev libxcursor-dev libxrandr-dev libxi-dev
```

**For other distributions:**
- Fedora: `sudo dnf install gcc libX11-devel libXcursor-devel libXrandr-devel libXi-devel`
- Arch: `sudo pacman -S base-devel libx11 libxcursor libxrandr libxi`

**Running the Application:**
```bash
cd Rust3dAimTrainer
cargo run --release
```

### macOS

**Additional Dependencies:**
- Xcode Command Line Tools
  ```bash
  xcode-select --install
  ```

**Running the Application:**
```bash
cd Rust3dAimTrainer
cargo run --release
```

## Configuration

Configuration files are located in the project root or `src/` directory. You can modify settings before running:

1. Copy `config.toml` (if available) to customize settings
2. Edit configuration values as needed
3. Save and run the application

