use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

/// Represents a `CMD` instruction
///
/// ```rust
/// use nanite_docker::{Cmd};
///
/// let cmd = Cmd {
///     argv: vec![
///         "arg1".to_string(),
///         "arg2".to_string(),
///     ],
/// };
/// let cmd_built = format!("{cmd}");
/// assert_eq!(cmd_built, r#"CMD ["arg1", "arg2"]"#);
/// ```
#[derive(Clone, Debug)]
pub struct Cmd {
    pub argv: Vec<String>,
}
impl Display for Cmd {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "CMD [")?;

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
