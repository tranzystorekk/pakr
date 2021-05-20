use std::os::unix::process::ExitStatusExt;

use log::{debug, error};
use structopt::StructOpt;

use pakr::cli::{Cli, Command, Orphans};
use pakr::wrapper::PacmanWrapper;

fn verbosity(verbose: bool) -> usize {
    match verbose {
        false => 2,
        true => 3,
    }
}

fn run_app() -> std::io::Result<i32> {
    let cli = Cli::from_args();
    let pac = PacmanWrapper::from_config()?;

    stderrlog::new()
        .module(module_path!())
        .verbosity(verbosity(cli.verbose))
        .init()
        .unwrap();

    debug!("Verifying that wrapper command exists");

    let command_status = pac.verify_command()?;

    if !command_status.success() {
        error!("Failed to verify wrapper command");

        return Ok(command_status
            .code()
            .unwrap_or_else(|| command_status.signal().unwrap()));
    }

    let status = match cli.sub {
        Command::Install { packages, as_deps } => pac.install(&packages, as_deps),
        Command::Remove { packages } => pac.remove(&packages),
        Command::Info { packages } => pac.info(&packages),
        Command::Upgrade => pac.upgrade(),
        Command::Clean => pac.clean(),
        Command::Orphans { cmd } => match cmd.unwrap_or_default() {
            Orphans::List => pac.list_orphans(),
            Orphans::Remove => pac.remove_orphans(),
        },
        Command::Completion { shell } => pakr::cli::generate_completion(shell),
    };

    status.map(|st| st.code().unwrap_or_else(|| st.signal().unwrap()))
}

fn main() -> std::io::Result<()> {
    let code = run_app()?;

    std::process::exit(code);
}
