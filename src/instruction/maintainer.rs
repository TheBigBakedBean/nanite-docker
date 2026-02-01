use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Represents a `MAINTAINER` instruction.
/// > WARNING: THE MAINTAINER INSTRUCTION IS DEPRECATED. FOR PARITY TO DOCKER, IT IS LEFT IN BUT PLEASE USE THE LABEL INSTRUCTION INSTEAD.
/// > SEE <https://docs.docker.com/reference/dockerfile/#maintainer-deprecated> FOR MORE DETAILS.
/// ```rust
/// use nanite_docker::Maintainer;
/// let maintainer = Maintainer {
///     name: "John Doe".to_string(),
/// };
/// let maintainer_built = format!("{maintainer}");
/// assert_eq!(maintainer_built, "MAINTAINER John Doe");
/// ```
#[derive(Clone, Debug)]
pub struct Maintainer {
    pub name: String,
}

impl Display for Maintainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MAINTAINER {}", self.name)
    }
}
