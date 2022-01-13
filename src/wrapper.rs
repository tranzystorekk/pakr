#[cfg(not(target_family = "unix"))]
compile_error!("Works only on Unix");

use crate::config::{Config, Wrapper};

use std::borrow::Cow;
use std::ffi::OsString;
use std::io::{Error, ErrorKind, Result as IoResult};
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus};

use bstr::ByteSlice;
use itertools::Itertools;
use log::{debug, info};

const SUDO_CMD: &str = "/usr/bin/sudo";

const AS_DEPS_FLAG: &str = "--asdeps";

const CLEAN_FLAG: &str = "-Sc";
const INFO_FLAG: &str = "-Qi";
const INSTALL_FLAG: &str = "-S";
const LIST_ORPHANS_FLAG: &str = "-Qtd";
const LIST_ORPHANS_SILENT_FLAG: &str = "-Qtdq";
const REMOVE_FLAG: &str = "-Rs";
const REMOVE_NO_SAVE_FLAG: &str = "-Rns";
const UPGRADE_FLAG: &str = "-Syu";

enum CommandMaker {
    Regular(String),
    Root(String),
}

pub struct PacmanWrapper {
    cmd_maker: CommandMaker,
}

impl CommandMaker {
    pub fn from_wrapper_config(config: Wrapper) -> Self {
        if config.requires_root {
            Self::Root(config.command)
        } else {
            Self::Regular(config.command)
        }
    }

    pub fn construct(&self) -> Command {
        match self {
            CommandMaker::Regular(cmd) => Command::new(cmd),
            CommandMaker::Root(cmd) => {
                let mut rooted = Command::new(SUDO_CMD);
                rooted.arg(cmd);

                rooted
            }
        }
    }

    pub fn as_string(&self) -> String {
        match self {
            CommandMaker::Regular(cmd) => cmd.clone(),
            CommandMaker::Root(cmd) => format!("{SUDO_CMD} {cmd}"),
        }
    }

    pub fn check_command(&self) -> IoResult<ExitStatus> {
        let mut cmd = Command::new("sh");

        cmd.arg("-c");

        match self {
            Self::Root(command) | Self::Regular(command) => {
                let verify_command = format!("command -v {command}");

                debug!("Executing command: `sh -c {verify_command}");

                cmd.arg(verify_command);
            }
        };

        cmd.stdout(std::process::Stdio::null()).status()
    }
}

impl PacmanWrapper {
    pub fn from_config() -> IoResult<Self> {
        let config: Config = confy::load(crate::cli::PKG_NAME)
            .map_err(|_| Error::new(ErrorKind::Other, "Failed to load configuration"))?;

        let result = Self {
            cmd_maker: CommandMaker::from_wrapper_config(config.wrapper),
        };

        Ok(result)
    }

    pub fn install(&self, packages: &[String], as_deps: bool) -> IoResult<ExitStatus> {
        let mut cmd = self.cmd_maker.construct();
        let mut deps_flag_format = String::from("");

        cmd.arg(INSTALL_FLAG);

        if as_deps {
            deps_flag_format = format!(" {AS_DEPS_FLAG}");
            cmd.arg(AS_DEPS_FLAG);
        }

        debug!(
            "Executing command: `{} {}{} {}`",
            self.cmd_maker.as_string(),
            INSTALL_FLAG,
            deps_flag_format,
            packages.iter().format(" ")
        );

        cmd.args(packages).status()
    }

    pub fn uninstall(&self, packages: &[String]) -> IoResult<ExitStatus> {
        let mut cmd = self.cmd_maker.construct();

        debug!(
            "Executing command: `{} {} {}`",
            self.cmd_maker.as_string(),
            REMOVE_FLAG,
            packages.iter().format(" ")
        );

        cmd.arg(REMOVE_FLAG).args(packages).status()
    }

    pub fn info(&self, packages: &[String]) -> IoResult<ExitStatus> {
        let mut cmd = self.cmd_maker.construct();

        debug!(
            "Executing command `{} {} {}`",
            self.cmd_maker.as_string(),
            INFO_FLAG,
            packages.iter().format(" ")
        );

        cmd.arg(INFO_FLAG).args(packages).status()
    }

    pub fn upgrade(&self) -> IoResult<ExitStatus> {
        let mut cmd = self.cmd_maker.construct();

        debug!(
            "Executing command: `{} {}`",
            self.cmd_maker.as_string(),
            UPGRADE_FLAG
        );

        cmd.arg(UPGRADE_FLAG).status()
    }

    pub fn clean(&self) -> IoResult<ExitStatus> {
        let mut cmd = self.cmd_maker.construct();

        debug!(
            "Executing command: `{} {}`",
            self.cmd_maker.as_string(),
            CLEAN_FLAG
        );

        cmd.arg(CLEAN_FLAG).status()
    }

    pub fn list_orphans(&self) -> IoResult<ExitStatus> {
        let mut cmd = self.cmd_maker.construct();

        debug!(
            "Executing command: `{} {}`",
            self.cmd_maker.as_string(),
            LIST_ORPHANS_FLAG
        );

        cmd.arg(LIST_ORPHANS_FLAG).status()
    }

    pub fn remove_orphans(&self) -> IoResult<ExitStatus> {
        let packages = self.get_orphans()?;

        debug!("Orphans: {packages:?}");

        if packages.is_empty() {
            info!("No orphans to remove");
            return Ok(ExitStatus::from_raw(0));
        }

        self.delete_orphans(&packages)
    }

    pub fn verify_command(&self) -> IoResult<ExitStatus> {
        self.cmd_maker.check_command()
    }

    fn get_orphans(&self) -> IoResult<Vec<OsString>> {
        let mut cmd = self.cmd_maker.construct();

        debug!(
            "Executing command: `{} {}`",
            self.cmd_maker.as_string(),
            LIST_ORPHANS_SILENT_FLAG
        );

        let output = cmd.arg(LIST_ORPHANS_SILENT_FLAG).output()?;
        let packages = output
            .stdout
            .fields()
            .map(ByteSlice::to_os_str_lossy)
            .map(Cow::into_owned)
            .collect();

        Ok(packages)
    }

    fn delete_orphans(&self, packages: &[OsString]) -> IoResult<ExitStatus> {
        let mut cmd = self.cmd_maker.construct();

        debug!(
            "Executing command: `{} {} {:?}`",
            self.cmd_maker.as_string(),
            REMOVE_NO_SAVE_FLAG,
            packages.iter().format(" ")
        );

        cmd.arg(REMOVE_NO_SAVE_FLAG).args(packages).status()
    }
}
