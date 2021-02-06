use structopt::clap::Shell;
use structopt::StructOpt;

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
    Clean,

    /// Manage orphaned packages
    Orphans {
        #[structopt(subcommand)]
        cmd: Option<Orphans>,
    },

    /// Generate a completion script
    Completion {
        #[structopt(name = "SHELL")]
        /// Shell type: {bash|zsh|fish|powershell|elvish}
        shell: Shell,
    },
}

#[derive(Debug, StructOpt)]
#[structopt(verbatim_doc_comment)]
/// Manage Arch Linux packages via any pacman-compatible wrapper
///
/// CONFIGURATION:
/// Config files are read from $XDG_CONFIG_HOME/pakr.toml:
///
/// [wrapper]
/// command = "pacman"      # name of the wrapper command
/// requires_root = true    # whether this wrapper needs root permissions (granted via sudo)
pub struct Cli {
    #[structopt(subcommand)]
    pub sub: Command,
}

impl Default for Orphans {
    fn default() -> Self {
        Self::Remove
    }
}
