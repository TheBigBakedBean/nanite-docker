use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Represents a `LABEL` instruction
/// ```rust
/// use nanite_docker::{Label};
///
/// let label = Label {
///     key: "key".to_string(),
///     value: "value".to_string(),
/// };
/// let label_built = format!("{label}");
/// assert_eq!(label_built, "LABEL key=value");
/// ```
#[derive(Clone, Debug)]
pub struct Label {
    pub key: String,
    pub value: String,
}

impl Display for Label {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "LABEL {}={}", self.key, self.value)
    }
}
