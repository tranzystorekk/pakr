use std::os::unix::process::ExitStatusExt;
use std::process::ExitStatus;

use structopt::clap::Shell;
use structopt::StructOpt;

pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");

pub fn generate_completion(shell: Shell) -> std::io::Result<ExitStatus> {
    Cli::clap().gen_completions_to(PKG_NAME, shell, &mut std::io::stdout());

    Ok(ExitStatus::from_raw(0))
}

#[derive(Copy, Clone, Debug, StructOpt)]
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

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Install packages
    Install {
        /// Packages to be installed
        #[structopt(value_name = "PKG", required = true, parse(from_str))]
        packages: Vec<String>,

        /// Mark installed packages as dependencies
        #[structopt(long, short = "d")]
        as_deps: bool,
    },

    /// Remove packages
    Uninstall {
        /// Packages to be removed
        #[structopt(value_name = "PKG", required = true, parse(from_str))]
        packages: Vec<String>,
    },

    /// Display package info
    Info {
        /// Packages to be inspected
        #[structopt(value_name = "PKG", required = true, parse(from_str))]
        packages: Vec<String>,
    },

    /// Upgrade all installed packages
    Upgrade,

    /// Clean package cache
    ///
    /// This removes all packages that are not installed from cache.
    /// If you want to keep the most recent versions, consider using other tools.
    Clean,

    /// Manage orphaned packages
    Orphans {
        #[structopt(subcommand)]
        cmd: Option<Orphans>,
    },

    /// Generate a completion script
    Completion {
        /// Shell type
        #[structopt(value_name = "SHELL", possible_values = &["bash", "zsh", "fish", "powershell", "elvish"])]
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
#[derive(Debug, StructOpt)]
#[structopt(verbatim_doc_comment)]
pub struct Cli {
    #[structopt(subcommand)]
    pub sub: Command,

    /// Display verbose logs (debug etc.)
    #[structopt(short, long, global = true)]
    pub verbose: bool,
}
