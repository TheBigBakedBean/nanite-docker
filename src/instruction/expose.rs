use core::fmt::{Display, Formatter};

/// Represents an `EXPOSE` instruction.
/// ```rust
/// use nanite_docker::{Expose, ExposeProtocol};
///
/// let expose = Expose {
///     port: 8080,
///     protocol: Some(ExposeProtocol::tcp),
/// };
/// let expose_built = format!("{expose}");
/// assert_eq!(expose_built, "EXPOSE 8080/tcp");
/// ```
#[derive(Clone, Debug)]
pub struct Expose {
    pub port: u16,
    pub protocol: Option<ExposeProtocol>,
}

impl Display for Expose {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match &self.protocol {
            Some(p) => match p {
                ExposeProtocol::tcp => write!(f, "EXPOSE {}/tcp", self.port),
                ExposeProtocol::udp => write!(f, "EXPOSE {}/udp", self.port),
            },
            None => write!(f, "EXPOSE {}", self.port),
        }
    }
}

/// Defines different port types for the EXPOSE Instruction
#[allow(
    nonstandard_style,
    reason = "tcp and udp are standards that should be lowercase for legibility"
)]
#[derive(Clone, Debug)]
pub enum ExposeProtocol {
    tcp,
    udp,
}
