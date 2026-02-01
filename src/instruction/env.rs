use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Represents an `ENV` instruction.
/// ```rust
/// use nanite_docker::Env;
///
/// let env = Env {
///     key: "key".to_string(),
///     value: "value".to_string(),
/// };
/// let env_built = format!("{env}");
/// assert_eq!(env_built, "ENV key=value");
/// ```
#[derive(Clone, Debug)]
pub struct Env {
    pub key: String,
    pub value: String,
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "ENV {}={}", self.key, self.value)
    }
}
