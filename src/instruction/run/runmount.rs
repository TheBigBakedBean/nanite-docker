// WARNING: This is even worse than mod.rs. Most of the RUN instruction is implemented here
// If you get easily overwhelmed, please look at mod.rs's `pub use` first just to see how much is here
// As said in the comment at the top of mod.rs, this isn't a good example of how the system works.
// ENTER AT YOUR OWN RISK

// If your first reaction is to complain about the repetition, look at the note in instruction/mod.rs. The repetition is intentional.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

/// Describes a --mount option for the RUN command
#[derive(Debug, Clone)]
pub enum RunMount {
    Bind(RunMountBind),
    Cache(RunMountCache),
    Tmpfs(RunMountTmpfs),
    Secret(RunMountSecret),
    Ssh(RunMountSSH),
}

#[derive(Debug, Clone)]
pub struct RunMountBind {
    pub target: String,
    pub opts: Vec<RunMountBindOpts>,
}
impl Display for RunMountBind {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "--mount=type=bind")?;
        write!(f, ",target={}", self.target)?;

        for i in &self.opts {
            write!(f, ",{i}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RunMountBindOpts {
    From(String),
    ReadWrite,
}

impl Display for RunMountBindOpts {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            RunMountBindOpts::From(s) => write!(f, "from={}", s),
            RunMountBindOpts::ReadWrite => write!(f, "readwrite=true"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunMountCache {
    pub target: String,
    pub opts: Vec<RunMountCacheOpts>,
}
impl Display for RunMountCache {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "--mount=type=cache")?;
        write!(f, ",target={}", self.target)?;

        for i in &self.opts {
            write!(f, ",{i}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum RunMountCacheOpts {
    Id(String),
    ReadOnly,
    Sharing(RunSharing),
    From(String),
    Source(String),
    Mode(String), // This is in octal, may be changed later if a better octal representation is found
    Uid(usize),
    Gid(usize),
}

impl Display for RunMountCacheOpts {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Id(i) => write!(f, "id={i}"),
            Self::ReadOnly => write!(f, "readonly=true"),
            Self::Sharing(s) => write!(f, "sharing={s}"),
            Self::From(r) => write!(f, "from={r}"),
            Self::Source(s) => write!(f, "source={s}"),
            Self::Mode(m) => write!(f, "mode={m}"),
            Self::Uid(u) => write!(f, "uid={u}"),
            Self::Gid(g) => write!(f, "gid={g}"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum RunSharing {
    Shared,
    Private,
    Locked,
}

impl Display for RunSharing {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Shared => write!(f, "shared"),
            Self::Private => write!(f, "private"),
            Self::Locked => write!(f, "locked"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RunMountTmpfs {
    pub target: String,
    pub opts: Vec<RunMountTmpfsOpts>, // Ehehe, a vec of only one possible option
}

impl Display for RunMountTmpfs {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "--mount=type=tmpfs")?;
        write!(f, ",target={}", self.target)?;

        for i in &self.opts {
            write!(f, ",{i}")?;
        }

        Ok(())
    }
}

// Yes I know this has only one varient, but it means if docker adds more options in the future, we can easily add them here
#[derive(Clone, Debug)]
pub enum RunMountTmpfsOpts {
    Size(String), // I'm not sure what docker uses for size, a new system may be released to better type sizes
}

impl Display for RunMountTmpfsOpts {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Size(s) => write!(f, "size={s}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RunMountSecret {
    pub target: Option<String>,
    pub opts: Vec<RunMountSecretOpts>,
}
impl Display for RunMountSecret {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "--mount=type=secret")?;
        if let Some(t) = &self.target {
            write!(f, ",source={t}")?;
        }

        for i in &self.opts {
            write!(f, ",{i}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum RunMountSecretOpts {
    Id(String),
    Env(String),
    Required,
    Mode(String), // See the mention of octal in RunMountCacheOpts::Mode
    Uid(usize),
    Gid(usize),
}
impl Display for RunMountSecretOpts {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Id(i) => write!(f, "id={i}"),
            Self::Env(e) => write!(f, "env={e}"),
            Self::Required => write!(f, "required=true"),
            Self::Mode(m) => write!(f, "mode={m}"),
            Self::Uid(u) => write!(f, "uid={u}"),
            Self::Gid(g) => write!(f, "gid={g}"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RunMountSSH {
    pub target: Option<String>,
    pub opts: Vec<RunMountSSHOpts>,
}
impl Display for RunMountSSH {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "--mount=type=ssh")?;
        if let Some(target) = &self.target {
            write!(f, ",target={}", target)?;
        }

        for i in &self.opts {
            write!(f, ",{i}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum RunMountSSHOpts {
    Id(String),
    Required,
    Mode(String), // See the mention of octal in RunMountCacheOpts::Mode
    Uid(usize),
    Gid(usize),
}
impl Display for RunMountSSHOpts {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Id(i) => write!(f, "id={i}"),
            Self::Required => write!(f, "required=true"),
            Self::Mode(m) => write!(f, "mode={m}"),
            Self::Uid(u) => write!(f, "uid={u}"),
            Self::Gid(g) => write!(f, "gid={g}"),
        }
    }
}
