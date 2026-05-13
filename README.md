# Highjump

[🇰🇷 한국어로 보기](README.ko.md)

Highjump is a fast and intuitive CLI tool for bookmarking directories and navigating between them seamlessly. It provides an interactive fuzzy search interface to jump to your frequently used paths.

## Features

- **Bookmark current directory:** Easily save your current working directory.
- **Interactive fuzzy search:** Find and jump to saved directories using keyboard arrows, typing path names, or selecting index numbers.
- **Persistent storage:** Paths are securely saved in `~/.highjump_paths.json`.

## Installation

### 1. Build the Rust CLI

First, clone the repository and build the project using Cargo:

```bash
git clone <repository-url>
cd highjump
cargo build --release
```

Move the compiled binary to a directory included in your system's `PATH`:

```bash
cp target/release/highjump ~/.cargo/bin/highjump
```

*(Make sure `~/.cargo/bin` or your chosen directory is in your `$PATH`.)*

### 2. Configure Shell Wrapper

Due to the fundamental design of UNIX-like operating systems, a child process (the Rust CLI) cannot change the current working directory of its parent process (the Shell).

To enable the `cd` functionality, you **must** add a shell function. Add the following code to your `~/.zshrc` or `~/.bashrc`:

```bash
# Highjump shell wrapper
function hj() {
    if [ $# -eq 0 ]; then
        # Navigation mode
        local TARGET_DIR=$(highjump)
        if [ -n "$TARGET_DIR" ] && [ -d "$TARGET_DIR" ]; then
            cd "$TARGET_DIR" || return
        fi
    else
        # Save mode or Help (--save, --help)
        highjump "$@"
    fi
}
```

Reload your shell configuration:

```bash
source ~/.zshrc  # or source ~/.bashrc
```

## Usage

Use the `hj` command for all operations.

**Save the current directory:**

```bash
hj --save
# or
hj -s
```

**Jump to a saved directory:**

```bash
hj
```

This will open an interactive prompt. You can:

* Use **Up/Down arrow keys** to navigate.
* Type the **index number** to filter by number.
* Type **part of the directory path** to fuzzy search.
* Press **Enter** to jump.

**View Help:**

```bash
hj --help
```