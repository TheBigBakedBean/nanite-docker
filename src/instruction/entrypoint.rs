use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

/// Represents an `ENTRYPOINT` instruction.
/// ```rust
/// use nanite_docker::Entrypoint;
///
/// let entrypoint = Entrypoint {
///     argv: vec![
///         "arg1".to_string(),
///         "arg2".to_string(),
///     ],
/// };
/// let entrypoint_built = format!("{entrypoint}");
/// assert_eq!(entrypoint_built, r#"ENTRYPOINT ["arg1", "arg2"]"#);
/// ```
#[derive(Clone, Debug)]
pub struct Entrypoint {
    pub argv: Vec<String>,
}
impl Display for Entrypoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "ENTRYPOINT [")?;

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
