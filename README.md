# Arkaive Quick Attendance Commandline Utility

## Overview

`arkaive` is a command-line tool designed to streamline and automate attendance check-ins for classes on the Arkaive platform. With this tool, you can quickly log in, list available classes, and perform a check-in with minimal effort.

**Note:** Credentials set via `set-config` are stored in plain text within the config file. Ensure you understand this security trade-off before storing sensitive information.

## Features

- **Automated Login:** Store credentials to avoid repeated logins.
- **List Classes:** Quickly retrieve and display available classes.
- **Check-in:** Perform a one-command check-in for a selected class.
- **Configuration Management:** Save and manage credentials in a local config file.

## Installation

### Prerequisites

- **Rust & Cargo:** Ensure you have the latest stable [Rust](https://www.rust-lang.org/tools/install) toolchain installed.
- **Network Access:** Requires an internet connection to communicate with `https://arkaive.com`.

### Install via Cargo

Since there are currently no precompiled binaries available, you can install directly from the repository using Cargo:

```bash
cargo install --git https://github.com/WERDXZ/arkaive.git
```

This will fetch, build, and install the `arkaive` binary into your Cargo bin directory (typically `~/.cargo/bin`).

## Configuration

By default, `arkaive` looks for a configuration file in `$CONFIG_HOME/arkaive.toml`. If `$CONFIG_HOME` is not set, it will typically default to a platform-specific config directory (e.g., `~/.config/arkaive.toml` on Linux).

**Important:** Credentials are stored in plain text within `arkaive.toml`.

To set your credentials permanently:
```bash
arkaive set-config -u <USERNAME> -p <PASSWORD>
```

This stores your credentials in `arkaive.toml`. If you prefer not to store credentials, provide them directly as command arguments whenever needed.

## Usage

Once installed, you can use the `arkaive` command followed by subcommands:

### 1. Set Credentials

```bash
arkaive set-config -u <USERNAME> -p <PASSWORD>
```

- **Description:** Stores your credentials for future sessions.
- **Security Note:** Credentials are saved in plain text.
- **Example:**
  ```bash
  arkaive set-config -u john_doe -p supersecretpassword
  ```

### 2. Test Login

```bash
arkaive test-login
```

- **Description:** Verifies that your stored (or provided) credentials are correct by attempting to log in.
- **Example:**
  ```bash
  arkaive test-login
  ```

### 3. List Classes

```bash
arkaive list-classes
```

- **Description:** Shows available classes.  
- **Options:**
  - `-u, --username`: Provide username if not set in config.
  - `-p, --password`: Provide password if not set in config.
  - `-i, --id-only`: Show only class IDs for scripting.
- **Example:**
  ```bash
  arkaive list-classes
  arkaive list-classes --id-only
  ```

### 4. Check-In to a Class

```bash
arkaive checkin <CLASS_ID>
```

- **Description:** Attempts to check in to the given class ID.
- **Options:**
  - `-u, --username`: Provide username if not set in config.
  - `-p, --password`: Provide password if not set in config.
- **Example:**
  ```bash
  arkaive checkin 12345
  ```

### 5. Generate Shell Completions

```bash
arkaive generate-completions --shell <SHELL>
```

- **Description:** Generate shell completions for your preferred shell.
- **Supported Shells:** bash, zsh, fish, powershell, elvish
- **Example:**
  ```bash
  arkaive generate-completions --shell bash > /etc/bash_completion.d/arkaive
  ```

## Troubleshooting

- **Invalid Credentials:** If you receive an authentication error, re-run `arkaive set-config` or provide `-u` and `-p` flags.
- **No Classes Listed:** Ensure you are properly logged in; use `arkaive test-login` to confirm.
- **Network Issues:** Check your internet connection and proxy settings if requests fail.
- **Config File Location:** If uncertain, run `echo $XDG_CONFIG_HOME` or `echo $HOME` to locate `arkaive.toml`.

## License

This project is licensed under the *Do What The Fuck You Want To Public License* (WTFPL). See [LICENSE](LICENSE) for details.