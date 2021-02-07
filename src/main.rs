use std::os::unix::process::ExitStatusExt;

use structopt::StructOpt;

use pakr::cli::{Cli, Command, Orphans};
use pakr::wrapper::PacmanWrapper;

fn run_app() -> std::io::Result<i32> {
    let cli = Cli::from_args();
    let pac = PacmanWrapper::from_config();

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
