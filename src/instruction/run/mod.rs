// WARNING: This is by far the longest and most complex system in the whole project. Most Instructions have only maybe 30 odd lines of code and were written in an hour
// This one took me a few weekends to finally figure out how to make it ergonomic
// ENTER AT YOUR OWN RISK

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

mod runmount;
pub use runmount::{
    RunMount, RunMountBind, RunMountBindOpts, RunMountCache, RunMountCacheOpts, RunMountSSH,
    RunMountSSHOpts, RunMountSecret, RunMountSecretOpts, RunMountTmpfs, RunMountTmpfsOpts,
    RunSharing,
};
/// Represents a `RUN` instruction.
/// The RUN instruction is a bit more complex than other instructions, due to how mounting works.
/// ```rust
/// use nanite_docker::*;
///
/// let run = Run {
///     argv: vec![
///         "bash".into(),
///         "-c".into(),
///         "echo building && make all".into(),
///     ],
///     mounts: vec![
///         RunMount::Bind(RunMountBind {
///             target: "/mnt/bind".into(),
///             opts: vec![RunMountBindOpts::ReadWrite],
///         }),
///         RunMount::Cache(RunMountCache {
///             target: "/mnt/cache".into(),
///             opts: vec![
///                 RunMountCacheOpts::Id("build-cache".into()),
///                 RunMountCacheOpts::Sharing(RunSharing::Shared),
///                 RunMountCacheOpts::Uid(1000),
///                 RunMountCacheOpts::Gid(1000),
///                 RunMountCacheOpts::Mode("0o755".into()),
///             ],
///         }),
///         RunMount::Ssh(RunMountSSH {
///             target: None,
///             opts: vec![
///                 RunMountSSHOpts::Id("default".into()),
///                 RunMountSSHOpts::Required,
///             ],
///         }),
///         RunMount::Secret(RunMountSecret {
///             target: Some("/run/secrets/mysecret".into()),
///             opts: vec![
///                 RunMountSecretOpts::Id("mysecret".into()),
///                 RunMountSecretOpts::Required,
///             ],
///         }),
///         RunMount::Tmpfs(RunMountTmpfs {
///             target: "/mnt/tmpfs".into(),
///             opts: vec![RunMountTmpfsOpts::Size("65536".into())],
///         }),
///     ],
///     network: Some(RunNetwork::Host),
///     security: Some(RunSecurity::Sandbox),
/// };
///
/// let run_built = format!("{run}");
/// assert_eq!(run_built, r#"RUN --mount=type=bind,target=/mnt/bind,readwrite=true --mount=type=cache,target=/mnt/cache,id=build-cache,sharing=shared,uid=1000,gid=1000,mode=0o755 --mount=type=ssh,id=default,required=true --mount=type=secret,source=/run/secrets/mysecret,id=mysecret,required=true --mount=type=tmpfs,target=/mnt/tmpfs,size=65536 --network=host --security=sandbox ["bash", "-c", "echo building && make all"]"#)
/// ```
#[derive(Debug, Clone)]
pub struct Run {
    pub argv: Vec<String>,
    pub mounts: Vec<runmount::RunMount>,
    pub network: Option<RunNetwork>,
    pub security: Option<RunSecurity>,
}

impl Display for Run {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "RUN ")?;
        for i in &self.mounts {
            match i {
                RunMount::Bind(b) => write!(f, "{b} "),
                RunMount::Cache(c) => write!(f, "{c} "),
                RunMount::Tmpfs(t) => write!(f, "{t} "),
                RunMount::Secret(s) => write!(f, "{s} "),
                RunMount::Ssh(s) => write!(f, "{s} "),
            }?;
        }

        match &self.network {
            Some(n) => write!(f, "{n} ")?,
            None => {}
        };

        match &self.security {
            Some(s) => write!(f, "{s} ")?,
            None => {}
        }

        write!(f, "[")?;

        for (i, arg) in self.argv.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, r#""{arg}""#)?;
        }

        write!(f, "]")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RunSecurity {
    Sandbox,
    Insecure,
}

impl Display for RunSecurity {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            RunSecurity::Sandbox => write!(f, "--security=sandbox"),
            RunSecurity::Insecure => write!(f, "--security=insecure"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunNetwork {
    Default,
    None,
    Host,
}

impl Display for RunNetwork {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            RunNetwork::Default => write!(f, "--network=default"),
            RunNetwork::None => write!(f, "--network=none"),
            RunNetwork::Host => write!(f, "--network=host"),
        }
    }
}
