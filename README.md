# Solace - Zig Package Manager

A modern package manager for the Zig programming language, written in Rust.

## Quick Start

```bash
# Install solace
cargo install solace_rust

# Create a new Zig project
solace init my_project
cd my_project

# Build and run
zig build run
```

## Installation

### Quick Install
```bash
cargo install solace_rust
```

### From Source
```bash
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust
cargo install --path .
```

## Features

- **Project Initialization**: Create new Zig projects with sensible defaults
- **Dependency Management**: Add, remove, and update Zig packages
- **Lock Files**: Ensure reproducible builds with `solace.lock`
- **Fast & Lightweight**: Built in Rust for maximum performance
- **Build Integration**: Seamless integration with Zig's build system

## Usage

### Basic Commands

After installation, use the following commands:

```bash
# Initialize a new Zig project
solace init my_project

# Navigate to project directory
cd my_project

# Build and run the project
zig build run
```

### Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `solace init [name]` | Initialize a new Zig project | `solace init hello_world` |
| `solace add <package>` | Add a dependency | `solace add zig-clap` |
| `solace remove <package>` | Remove a dependency | `solace remove zig-clap` |
| `solace list` | List all dependencies | `solace list` |
| `solace update` | Update dependencies | `solace update` |

### Getting Started Example

```bash
# Install solace
$ cargo install solace_rust

# Create a new project
$ solace init hello_world
Project 'hello_world' initialized successfully!

# Navigate to project
$ cd hello_world

# Build and run
$ zig build run
Hello, hello_world!
```

## Project Structure

```
my_project/
├── src/
│   ├── main.zig    # Main application entry
│   └── root.zig    # Library root
├── build.zig       # Build configuration
├── build.zig.zon   # Package configuration
└── solace.lock     # Dependency lock file
```

## Development

```bash
# Clone the repository
git clone https://github.com/KayanoLiam/solace.git
cd solace_rust

# Build for development
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Issues**: [GitHub Issues](https://github.com/KayanoLiam/solace/issues)
- **Discussions**: [GitHub Discussions](https://github.com/KayanoLiam/solace/discussions)
- **Email**: Kayano04@outlook.jp