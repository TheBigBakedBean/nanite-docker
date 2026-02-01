use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

/// Represents a `SHELL` instruction.
/// ```rust
/// use nanite_docker::{Shell, Instruction, Run};
///
/// let shell = Shell {
///     argv: vec!["sh".into(), "-c".into()],
/// };
/// let shell_built = format!("{shell}");
/// assert_eq!(shell_built, r#"SHELL ["sh", "-c"]"#);
/// ```
#[derive(Clone, Debug)]
pub struct Shell {
    pub argv: Vec<String>,
}
impl Display for Shell {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "SHELL [")?;

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
