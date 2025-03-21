# ThroughTheTerminal

*A simple yet customizable script to open files in the terminal.*

This project allows you to 'natively' open files, either through File Explorer or Finder, inside the terminal using your preferred terminal-based text editor.

## Table of Contents

1. [Features](#features)
2. [Configuration](#configuration)
3. [Building Instructions](#building-instructions)

## Features

- Open files in a new terminal window.
- Open files in a new tab inside an existing terminal window.
- Supports Windows and macOS.

### Supported Terminals

| Terminal                                                               | Windows | macOS | Linux |
|------------------------------------------------------------------------|:-------:|:-----:|:-----:|
| [Ghostty](https://ghostty.org)                                         | ❌       | ✅    |  ❌   |
| [Terminal](https://support.apple.com/en-gb/guide/terminal/welcome/mac) | ✅       | ❌    |  ❌   |
| [Windows Terminal](https://github.com/microsoft/terminal)              | ✅       | ❌    |  ❌   |
| [WezTerm](https://wezterm.org)                                         | ❌       | ❌    |  ❌   |

## Configuration

The configuration file located at `~/.throughtheterminal` uses the `toml` file format:

| Key        | Description                                        | Options                                         |
|------------|----------------------------------------------------|-------------------------------------------------|
| `terminal` | Which terminal you want files to open in.          | `"Terminal"`, `"Windows Terminal"`, `"Ghostty"` |
| `editor`   | Which editor you want files to open in.            | `"vim"`, `"nvim"`, `"emacs"`, etc.              |
| `method`   | Whether you want files to open in a tab or window. | `"tab"`, `"window"`                             |

An example `.throughtheterminal` configuration file might look something like this:

```toml
terminal = "WezTerm"
editor = "nvim"
method = "tab"
```

## Building Instructions

### Windows

```
git clone https://github.com/ostromia/throughtheterminal.git
cd .\throughtheterminal\
cargo build --release
```

### macOS

```
git clone https://github.com/ostromia/throughtheterminal.git
cd throughtheterminal/
cargo bundle --release
```
