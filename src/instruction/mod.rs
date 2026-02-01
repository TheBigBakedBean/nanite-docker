// A few notes about editing the instructions: (this is a code comment as it is only designed for contributors)
// 1. Every instruction gets its own file - This makes maintaining all the instructions easier and avoids accidental modifications where they're not intended
// 2. No generic representations, each instruction should own its own processes - This means that a single modification to ONE instruction doesn't change many
// 3. Make each instruction well documentated where there is ambiguity - These instructions make up the majority of the public API, so people need to be 100% sure of what they do

mod add;
mod arg;
mod cmd;
mod copy;
mod entrypoint;
mod env;
mod expose;
mod healthcheck;
mod label;
mod maintainer;
mod onbuild;
mod run;
mod shell;
mod stopsignal;
mod user;
mod volume;
mod workdir;

// NOTE: This directly defines the public API
// Editing this list of exports REQUIRES a major semver update
pub use add::{Add, AddOpt};
pub use arg::Arg;
pub use cmd::Cmd;
pub use copy::{Copy, CopyOpt};
pub use entrypoint::Entrypoint;
pub use env::Env;
pub use expose::{Expose, ExposeProtocol};
pub use healthcheck::{HealthCheck, HealthCheckOpt};
pub use label::Label;
pub use maintainer::Maintainer;
pub use onbuild::OnBuild;
pub use run::{
    Run, RunMount, RunMountBind, RunMountBindOpts, RunMountCache, RunMountCacheOpts, RunMountSSH,
    RunMountSSHOpts, RunMountSecret, RunMountSecretOpts, RunMountTmpfs, RunMountTmpfsOpts,
    RunNetwork, RunSecurity, RunSharing,
};
pub use shell::Shell;
pub use stopsignal::StopSignal;
pub use user::User;
pub use volume::Volume;
pub use workdir::WorkDir;

use core::fmt::{Display, Formatter};

/// The Instruction is one of the 3 core units of the Dockerfile Rust Intermediate Representation
/// It directly represents a typed version of Dockerfile [Instruction](https://docs.docker.com/reference/dockerfile/).
/// Note that the `From` Instruction is not available, this is becuase each stage can only have one `From` instruction so it is defined in the stage
#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Add),
    Arg(Arg),
    Cmd(Cmd),
    Copy(Copy),
    Entrypoint(Entrypoint),
    Env(Env),
    Expose(Expose),
    HealthCheck(HealthCheck),
    Label(Label),
    Maintainer(Maintainer),
    OnBuild(OnBuild),
    Run(Run),
    Shell(Shell),
    StopSignal(StopSignal),
    User(User),
    Volume(Volume),
    Workdir(WorkDir),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Add(e) => write!(f, "{e}"),
            Self::Arg(e) => write!(f, "{e}"),
            Self::Cmd(e) => write!(f, "{e}"),
            Self::Copy(e) => write!(f, "{e}"),
            Self::Entrypoint(e) => write!(f, "{e}"),
            Self::Env(e) => write!(f, "{e}"),
            Self::Expose(e) => write!(f, "{e}"),
            Self::HealthCheck(e) => write!(f, "{e}"),
            Self::Label(e) => write!(f, "{e}"),
            Self::Maintainer(e) => write!(f, "{e}"),
            Self::OnBuild(e) => write!(f, "{e}"),
            Self::Run(e) => write!(f, "{e}"),
            Self::Shell(e) => write!(f, "{e}"),
            Self::StopSignal(e) => write!(f, "{e}"),
            Self::User(e) => write!(f, "{e}"),
            Self::Volume(e) => write!(f, "{e}"),
            Self::Workdir(e) => write!(f, "{e}"),
        }
    }
}

/// A selection of Instructions that can be placed prior to a FROM Statement in a Stage.
#[derive(Debug, Clone)]
pub enum PreFromInstruction {
    Arg(Arg),
    Label(Label),
}
impl Display for PreFromInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Arg(e) => write!(f, "{e}"),
            Self::Label(e) => write!(f, "{e}"),
        }
    }
}
