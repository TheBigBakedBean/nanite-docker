use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter, Write};

/// Represents an `ADD` instruction.
///
/// ```rust
/// use nanite_docker::{Add, AddOpt};
///
/// let add = Add {
///     opts: vec![AddOpt::KeepGitDir],
///     src: vec!["src".to_string()],
///     dest: "dest".to_string(),
/// };
/// let add_built = format!("{add}");
/// assert_eq!(add_built, r#"ADD --keep-git-dir=true "src" "dest""#);
///
/// ```
#[derive(Clone, Debug)]
pub struct Add {
    pub opts: Vec<AddOpt>,
    pub src: Vec<String>,
    pub dest: String,
}
impl Display for Add {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        let mut opt_string = String::new();

        for opt in &self.opts {
            // Add a space to the end (THIS IS NOT A BUG)
            write!(opt_string, "{opt} ")?;
        }

        let src_string = self
            .src
            .iter()
            .map(|i| format!(r#""{i}""#))
            .collect::<Vec<String>>()
            .join(" ");

        write!(f, r#"ADD {opt_string}{src_string} "{}""#, self.dest)
    }
}

#[derive(Clone, Debug)]
pub enum AddOpt {
    KeepGitDir,
    Checksum { hash: String },
    Chown { user: String, group: Option<String> },
    Chmod { perms: String },
    Link,
    Exclude { path: String },
}

impl Display for AddOpt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            AddOpt::KeepGitDir => write!(f, "--keep-git-dir=true"),

            AddOpt::Checksum { hash: e } => write!(f, "--checksum={e}"),

            AddOpt::Chown { user: u, group: g } => match g {
                Some(group) => write!(f, "--chown={u}:{group}"),
                None => write!(f, "--chown={u}"),
            },

            AddOpt::Chmod { perms: p } => write!(f, "--chmod={p}"),

            AddOpt::Link => write!(f, "--link"),

            AddOpt::Exclude { path: p } => write!(f, r#"--exclude="{p}""#),
        }
    }
}
