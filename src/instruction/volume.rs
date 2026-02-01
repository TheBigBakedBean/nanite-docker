use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Display, Formatter};

/// Represents a `VOLUME` instruction.
/// ```rust
/// use nanite_docker::Volume;
///
/// let volume = Volume {
///     volumes: vec!["/var/log".into()],
/// };
/// let volume_built = format!("{volume}");
/// assert_eq!(volume_built, r#"VOLUME ["/var/log"]"#);
/// ```
#[derive(Clone, Debug)]
pub struct Volume {
    pub volumes: Vec<String>,
}
impl Display for Volume {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "VOLUME [")?;

        for (i, arg) in self.volumes.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, r#""{arg}""#)?;
        }

        write!(f, "]")?;

        Ok(())
    }
}
