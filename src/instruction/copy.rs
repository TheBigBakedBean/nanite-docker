use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter, Write};

/// Represents a `COPY` instruction
///
/// ```rust
/// use nanite_docker::{Copy, CopyOpt};
///
/// let copy = Copy {
///     opts: vec![
///         CopyOpt::From { stage: "stage".to_string() },
///         CopyOpt::Link,
///     ],
///     src: vec![
///         "src1".to_string(),
///         "src2".to_string(),
///     ],
///     dest: "dest".to_string(),
/// };
/// let copy_built = format!("{copy}");
/// assert_eq!(copy_built, r#"COPY --from=stage --link "src1" "src2" "dest""#);
/// ```
#[derive(Clone, Debug)]
pub struct Copy {
    pub opts: Vec<CopyOpt>,
    pub src: Vec<String>,
    pub dest: String,
}

// I know this is the same as it is for Add, I haven't found a cleaner way yet
impl Display for Copy {
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

        write!(f, r#"COPY {opt_string}{src_string} "{}""#, self.dest)
    }
}

#[derive(Clone, Debug)]
pub enum CopyOpt {
    From { stage: String },
    Chown { user: String, group: Option<String> },
    Chmod { perms: String },
    Link,
    Parents,
    Exclude { path: String },
}

impl Display for CopyOpt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            CopyOpt::From { stage: s } => write!(f, "--from={s}"),

            CopyOpt::Chown { user: u, group: g } => match g {
                Some(group) => write!(f, "--chown={u}:{group}"),
                None => write!(f, "--chown={u}"),
            },

            CopyOpt::Chmod { perms: p } => write!(f, "--chmod={p}"),

            CopyOpt::Link => write!(f, "--link"),

            CopyOpt::Parents => write!(f, "--parents"),

            CopyOpt::Exclude { path: p } => write!(f, r#"--exclude="{p}""#),
        }
    }
}
