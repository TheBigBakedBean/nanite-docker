use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Represents a `WORKDIR` instruction.
/// ```rust
/// use nanite_docker::WorkDir;
///
/// let workdir = WorkDir {
///     path: "/var/log".into(),
/// };
/// let workdir_built = format!("{workdir}");
/// assert_eq!(workdir_built, r#"WORKDIR /var/log"#);
/// ```
#[derive(Clone, Debug)]
pub struct WorkDir {
    pub path: String,
}
impl Display for WorkDir {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "WORKDIR {}", self.path)
    }
}
