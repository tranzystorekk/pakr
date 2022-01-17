use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

use clap::{IntoApp, Parser, Subcommand};
use clap_complete::Shell;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn generate_completion(shell: Shell) -> std::io::Result<ExitStatus> {
    clap_complete::generate(
        shell,
        &mut Cli::into_app(),
        PKG_NAME,
        &mut std::io::stdout(),
    );

    Ok(ExitStatus::from_raw(0))
}

#[derive(Copy, Clone, Debug, Subcommand)]
pub enum Orphans {
    /// List orphans
    List,

    /// Remove orphans (default if not specified)
    Remove,
}

impl Default for Orphans {
    fn default() -> Self {
        Self::Remove
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install packages
    Install {
        /// Packages to be installed
        #[clap(value_name = "PKG", required = true)]
        packages: Vec<String>,

        /// Mark installed packages as dependencies
        #[clap(long, short = 'd')]
        as_deps: bool,
    },

    /// Remove packages
    Uninstall {
        /// Packages to be removed
        #[clap(value_name = "PKG", required = true)]
        packages: Vec<String>,
    },

    /// Display package info
    Info {
        /// Packages to be inspected
        #[clap(value_name = "PKG", required = true)]
        packages: Vec<String>,
    },

    /// Upgrade all installed packages
    #[clap(visible_alias = "up")]
    Upgrade,

    /// Clean package cache
    ///
    /// This removes all packages that are not installed from cache.
    /// If you want to keep the most recent versions, consider using other tools.
    Clean,

    /// Manage orphaned packages
    Orphans {
        #[clap(subcommand)]
        cmd: Option<Orphans>,
    },

    /// Generate a completion script
    Completion {
        /// Shell type
        #[clap(value_name = "SHELL", arg_enum)]
        shell: Shell,
    },
}

/// Manage Arch Linux packages via any pacman-compatible wrapper
///
/// CONFIGURATION:
/// Config files are read from $XDG_CONFIG_HOME/pakr/pakr.toml:
///
/// [wrapper]
/// command = "pacman"      # name of the wrapper command
/// requires_root = true    # whether this wrapper needs root permissions (granted via sudo)
///
/// If this file is missing, a default configuration is created that runs `sudo pacman`.
#[derive(Debug, Parser)]
#[clap(verbatim_doc_comment, version)]
pub struct Cli {
    #[clap(subcommand)]
    pub sub: Command,

    /// Display verbose logs (debug etc.)
    #[clap(short, long, global = true)]
    pub verbose: bool,
}
