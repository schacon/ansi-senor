# Ansi Señor

This is a simple Rust binary that will run whatever command you specify next with `CLICOLOR_FORCE=1` automatically exported, then both show the output and also capture it in a buffer file, then run an `ansi2html` conversion that then writes an html file with the output with proper ansi coloring to an output file specified.

## Usage

```
$ ansi-senor git status

---
❯ git status                                                                                                            took 9h4m23s
On branch gitbutler/workspace
Changes not staged for commit:
  (use "git add <file>..." to update what will be committed)
  (use "git restore <file>..." to discard changes in working directory)
        modified:   content/2025/2025-11-05-gitbutler-cli.mdx

no changes added to commit (use "git add" and/or "git commit -a")
---

Output saved to /tmp/ansi-senor/git-status-fedd38ae.html
```

### Options

- `-o, --output <file-path.html>` - Specify a custom output file path
- `-t, --theme <light|dark>` - Choose the color theme for HTML output (default: dark)

### Examples

```bash
# Use light theme
ansi-senor --theme light git status

# Specify output file and theme
ansi-senor -o output.html -t light ls -la

# Short form
ansi-senor -t light git diff
```

## Prerequisites

- [Rust](https://www.rust-lang.org/) (1.70 or later recommended)
- Cargo (comes with Rust)

## Building

To build the project in debug mode:

```bash
cargo build
```

To build an optimized release version:

```bash
cargo build --release
```

The compiled binary will be located at:

- Debug: `target/debug/ansi-senor`
- Release: `target/release/ansi-senor`

## Installation

### Install from crates.io

The easiest way to install `ansi-senor` is from [crates.io](https://crates.io/crates/ansi-senor):

```bash
cargo install ansi-senor
```

This will download, compile, and install the binary to your Cargo bin directory (`~/.cargo/bin`), making the `ansi-senor` command available system-wide (assuming `~/.cargo/bin` is in your PATH).

### Install from source

Alternatively, you can install from source by cloning this repository:

```bash
cargo install --path .
```

## Running

### Run directly with Cargo

```bash
cargo run -- <command>
```

For example:

```bash
cargo run -- git status
cargo run -- ls -la
```

### Run the compiled binary

After building:

```bash
./target/debug/ansi-senor <command>
# or
./target/release/ansi-senor <command>
```

After installing:

```bash
ansi-senor <command>
```
