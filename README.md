# GIT SPY

[![Language](https://img.shields.io/badge/language-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE) [![Build Status](https://img.shields.io/github/actions/workflow/status/lukasmetzler/git-spy/release.yml)](https://github.com/lukasmetzler/git-spy/actions)

**Git Spy** is a fast CLI tool for GitHub. Built in Rust, it fetches user profiles and repository stats, displaying them in a stylish Cyberpunk design directly in your terminal.

## Interface Preview

![Git Spy Terminal Preview](asset/preview.png)

## System Capabilities

**Profile Scan:** Fetches and displays key user details like bio, location, followers, and repository stats in a clean, aligned layout.

**Language Stats:** Calculates the most used programming languages across all repositories and highlights them with colors for a quick overview.

**Interactive Menu:** Select a repository from the list to open it immediately in your browser—no need to copy-paste URLs.

## Usage

Initiate a scan by providing a target username.

```bash
git-spy --user <USERNAME>
```

### Command Line Arguments

| Argument    | Flag              | Description                                       |
| :---------- | :---------------- | :------------------------------------------------ |
| **User**    | `-u`, `--user`    | The target GitHub username to analyze (Required). |
| **Help**    | `-h`, `--help`    | Displays system manual and options.               |
| **Version** | `-V`, `--version` | Displays current build version.                   |

### Example

```bash
# Analyze the creator of Linux
git-spy -u torvalds
```

## Installation Protocols

### Method A: Pre-compiled Binary (Recommended)

Navigate to the [Releases Page](https://github.com/lukasmetzler/git-spy/releases). Download the appropriate executable for your architecture (Windows, Linux, or macOS) and execute it directly from your terminal. No dependencies required.

### Method B: Compile from Source

Ensure a valid Rust toolchain is installed on your local machine.

1.  **Clone the repository:**

    ```bash
    git clone [https://github.com/lukasmetzler/git-spy.git](https://github.com/lukasmetzler/git-spy.git)
    cd git-spy
    ```

2.  **Compile release build:**

    ```bash
    cargo build --release
    ```

3.  **Execute:**

    ```bash
    ./target/release/git-spy --user lukasmetzler
    ```

## Technical Stack

This system is engineered using the following crates:

- **Tokio:** Asynchronous runtime for non-blocking I/O.
- **Reqwest:** HTTP client for API communication.
- **Clap:** Command Line Argument Parser.
- **Tabled:** ANSI-compatible table formatting.
- **Indicatif:** Progress reporting and spinners.
- **Dialoguer:** Interactive terminal menus.
- **Figlet-rs:** ASCII art generation.

---

[License: MIT](https://www.google.com/search?q=LICENSE) | [Report Issue](https://github.com/lukasmetzler/git-spy/issues)
