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

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Install packages
    Install {
        #[structopt(name = "PKG", required = true, parse(from_str))]
        /// Packages to be installed
        packages: Vec<String>,

        #[structopt(long, short = "d")]
        /// Mark installed packages as dependencies
        as_deps: bool,
    },

    /// Remove packages
    Remove {
        #[structopt(name = "PKG", required = true, parse(from_str))]
        /// Packages to be removed
        packages: Vec<String>,
    },

    /// Display package info
    Info {
        #[structopt(name = "PKG", required = true, parse(from_str))]
        /// Packages to be inspected
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
        #[structopt(value_name = "SHELL", possible_values = &["bash", "zsh", "fish", "powershell", "elvish"])]
        /// Shell type
        shell: Shell,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(verbatim_doc_comment)]
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
pub struct Cli {
    #[structopt(subcommand)]
    pub sub: Command,

    #[structopt(short, long, global = true)]
    /// Display verbose logs (debug etc.)
    pub verbose: bool,
}

impl Default for Orphans {
    fn default() -> Self {
        Self::Remove
    }
}
