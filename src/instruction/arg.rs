use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Represents an `ARG` instruction
///
/// ```rust
/// use nanite_docker::{Arg};
///
/// let arg = Arg {
///     name: "name".to_string(),
///     default: Some("default".to_string()),
/// };
/// let arg_built = format!("{arg}");
/// assert_eq!(arg_built, "ARG name=default");
/// ```

#[derive(Clone, Debug)]
pub struct Arg {
    pub name: String,
    pub default: Option<String>,
}
impl Display for Arg {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.default {
            Some(default) => write!(f, "ARG {}={default}", self.name),
            None => write!(f, "ARG {}", self.name),
        }
    }
}
