use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Represents a `STOPSIGNAL` instruction.
/// ```rust
/// use nanite_docker::{StopSignal, Instruction};
///
/// let stop_signal = StopSignal::ByName("SIGTERM".into());
/// let stop_signal_built = format!("{stop_signal}");
/// assert_eq!(stop_signal_built, r#"STOPSIGNAL SIGTERM"#);
///
/// let stop_signal = StopSignal::ById(15);
/// let stop_signal_built = format!("{stop_signal}");
/// assert_eq!(stop_signal_built, r#"STOPSIGNAL 15"#);
/// ```
#[derive(Clone, Debug)]
pub enum StopSignal {
    ByName(String),
    ById(usize),
}
impl Display for StopSignal {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ByName(n) => write!(f, "STOPSIGNAL {n}"),
            Self::ById(i) => write!(f, "STOPSIGNAL {i}"),
        }
    }
}
