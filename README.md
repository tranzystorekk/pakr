# Paker

[![Latest version of 'pakr' @ Cloudsmith](https://api-prd.cloudsmith.io/v1/badges/version/tranzystorek-crates/default/cargo/pakr/latest/x/?render=true&show_latest=true)](https://cloudsmith.io/~tranzystorek-crates/repos/default/packages/detail/cargo/pakr/latest/)

## About

Paker (typed `pakr` for convenience) is a Rust wrapper for any utilities
compatible with `pacman`'s CLI API (flags like `-Syu` etc.).

In short, it gives you a nicer, more descriptive interface for common
operations on Arch Linux packages, including:

- Installing packages
- Removing packages
- Displaying detailed package info
- Performing a system upgrade
- Listing and automatically removing orphaned packages
- Cleaning pacman's package cache

Currently used flags:

| Subcommand                    | Flags                        |
| ----------------------------- | ---------------------------- |
| `install`                     | `-S`                         |
| `install -d`                  | `-S --asdeps`                |
| `remove` (alias `uninstall`)  | `-Rs`                        |
| `info`                        | `-Qi`                        |
| `upgrade`                     | `-Syu`                       |
| `clean`                       | `-Sc`                        |
| `orphans list`                | `-Qtd`                       |
| `orphans remove`              | `-Qtdq` :arrow_right: `-Rns` |

## Installation

### Via Cargo

Paker is hosted on a custom Cloudsmith repository, so first set up a
[Cargo registry](https://cloudsmith.io/~tranzystorek-crates/repos/default/setup/#formats-cargo).

Then, install `pakr` using the selected registry name:

```sh
cargo install pakr --registry tranzystorek-crates-default
```

## Configuration

All configuration resides under `$XDG_CONFIG_HOME/pakr/pakr.toml` (usually `$HOME/.config/pakr/pakr.toml`):

```toml
[wrapper]
command = "pacman"      # name of the wrapper command
requires_root = true    # whether this wrapper needs root permissions (granted via sudo)
```

If this file is missing, a default configuration is created that runs `sudo pacman`.

## Examples

Installing `kakoune` with the `trizen` wrapper:

```console
$ pakr install kakoune
:: Pacman command: /usr/bin/sudo /usr/bin/pacman -S kakoune
[sudo] password for devuser:
resolving dependencies...
looking for conflicting packages...

Packages (1) kakoune-2020.09.01-1

Total Download Size:   1.03 MiB
Total Installed Size:  3.50 MiB

:: Proceed with installation? [Y/n]
:: Retrieving packages...
 kakoune-2020.09.01-1-x86_64                                     1057.4 KiB  1792 KiB/s 00:01 [#######################################################] 100%
(1/1) checking keys in keyring                                                                [#######################################################] 100%
(1/1) checking package integrity                                                              [#######################################################] 100%
(1/1) loading package files                                                                   [#######################################################] 100%
(1/1) checking for file conflicts                                                             [#######################################################] 100%
(1/1) checking available disk space                                                           [#######################################################] 100%
:: Processing package changes...
(1/1) installing kakoune                                                                      [#######################################################] 100%
Optional dependencies for kakoune
    aspell: spell checking support
    clang: C/C++ completion and diagnostics support
    kak-lsp: LSP client
    ranger: filesystem explorer
    tmux: splitting and creating windows [installed]
    xdotool: X11 utility to focus arbitrary kakoune clients
    xorg-xmessage: display debug messages in a new window
:: Running post-transaction hooks...
(1/1) Arming ConditionNeedsUpdate...
```

## FAQ

### Can/will `pakr` replace `insert package manager name here`?

No, it's a non-goal.

`pakr` was made to target a narrow set of tasks that I do often.
I still use the underlying package manager to do specific tasks
like listing manually installed packages or installing packages from manual sources.

Some common things that pacman doesn't do are included
in the [pacman-contrib](https://git.archlinux.org/pacman-contrib.git/about/) package,
you might want to check it out.
