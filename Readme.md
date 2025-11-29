# Rust CLI Application

A command-line application built with Rust.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70.0 or later)
- Cargo (comes with Rust)

## Development

### Running the CLI in Development

```bash
# Run with cargo run
cargo run -- [arguments]

# Example: if your CLI has a --help flag
cargo run -- --help

# Example: with subcommands
cargo run -- init --path /tmp/project
```

The `--` separates cargo arguments from your CLI arguments.

### Building the Project

```bash
# Debug build (faster compilation, slower runtime)
cargo build

# Release build (optimized, slower compilation, faster runtime)
cargo build --release
```

Debug builds go to `target/debug/`, release builds go to `target/release/`.

## Installation

### Install Locally

Install the binary to your system (usually `~/.cargo/bin/`):

```bash
cargo install --path .
```

After installation, you can run the CLI directly by name:

```bash
myapp --help
```

### Install from GitHub (for users)

```bash
cargo install --git https://github.com/yourusername/yourrepo
```

### Uninstall

```bash
cargo uninstall myapp
```

## Usage

```bash
# Show help
myapp --help

# Run a command
myapp init --path /path/to/project

# Use verbose mode
myapp run --verbose
```

## Project Structure

```
src/
├── main.rs              # CLI parsing and routing
├── commands/
│   ├── mod.rs          # Command module exports
│   ├── init.rs         # Init command implementation
│   └── run.rs          # Run command implementation
└── lib.rs              # Shared business logic
```

## Development Tips

### Running with Arguments

```bash
# Instead of typing cargo run -- every time, create an alias
alias myapp-dev='cargo run --'

# Then use it like:
myapp-dev --help
myapp-dev init --path /tmp
```

### Quick Testing

```bash
# Run tests
cargo test

# Run with backtrace on errors
RUST_BACKTRACE=1 cargo run -- your-command

# Check your code without building
cargo check
```

### Making the Binary Available System-Wide

**Option 1: Install with cargo**
```bash
cargo install --path .
```

**Option 2: Add to PATH manually**
```bash
# Build release version
cargo build --release

# Copy to a directory in your PATH
cp target/release/myapp ~/.local/bin/

# Or create a symlink
ln -s $(pwd)/target/release/myapp ~/.local/bin/myapp
```

**Option 3: Add target/release to PATH**
```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$PATH:/path/to/your/project/target/release"
```

## Cargo.toml Configuration

Make sure your `Cargo.toml` has the binary configured:

```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "myapp"
path = "src/main.rs"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
anyhow = "1.0"
```

## Publishing (Optional)

To publish to crates.io:

```bash
# Login to crates.io
cargo login

# Publish
cargo publish
```

Then others can install with:
```bash
cargo install myapp
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

[Your License Here]
