# Jet CLI

A lightning-fast CLI tool for navigating between projects and repositories.

## Overview

[![asciicast](https://asciinema.org/a/YLe0RhK0rwzAqaE0DUydBeEZI.svg)](https://asciinema.org/a/YLe0RhK0rwzAqaE0DUydBeEZI)

Jet CLI is a command-line utility designed to simplify project navigation. It helps you:

- Quickly find and jump between repositories
- Navigate to specific projects within repositories
- Use context-aware searching based on your current location
- Preview directory contents before selection

Built in Rust for speed and efficiency, Jet CLI uses fuzzy finding to help you navigate your codebase faster.

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/filipesilva-l/jet-cli.git
cd jet-cli

# Build and install
cargo install --path .
```

### Shell Integration

To enable directory changing functionality, you need to set up a shell function:

#### Fish Shell

Add the following to your Fish configuration:

```fish
function jet
    set -x JET_SHELL true

    set new_dir (jet-cli $argv)

    if test $status -eq 0
        cd $new_dir
    end
end
```

You can also copy the provided script:

```bash
cp scripts/jet.fish ~/.config/fish/functions/
```

## Usage

Jet CLI provides several commands to navigate your codebase:

```
jet                # Smart mode (default) - context-aware search
jet repos          # Search for repositories only
jet projects       # Search for projects only
jet up             # Jump to repository root
jet-cli edit       # Edit configuration
```

### Options

```
-n, --no-selection     # Outputs paths without interactive selection
-h, --help             # Show help information
-V, --version          # Show version information
```

### Examples

```bash
# Find and navigate to a project or repository with fuzzy search
jet

# List all repositories without interactive selection
jet repos -n

# Jump to the root of the current repository
jet up

# Edit your configuration
jet-cli edit
```

## Configuration

Jet CLI is configured with a TOML file located at:

- `~/.config/jet-cli/config.toml` (Linux)
- `~/Library/Application Support/jet-cli/config.toml` (macOS)
- `%APPDATA%\jet-cli\config.toml` (Windows)

### Configuration Format

```toml
# List of root directories to search for repositories and projects
roots = [
  "/home/user/code",
  "/home/user/work"
]
```

### Creating Configuration

1. Run `jet-cli edit` to create and open the configuration file
2. Add your root directories to search
3. Save the file

## Project Detection

Jet CLI identifies projects by looking for:

- `Cargo.toml` files (Rust)
- `go.mod` files (Go)
- `.csproj` files (C#/.NET)
- `.git` directories (Git repositories)

## License

MIT
