use alloc::{string::String, vec::Vec};

use crate::Cmd;
use core::fmt::{Display, Formatter};

/// Represents a `HEALTHCHECK` instruction.
/// ```rust
/// use nanite_docker::{HealthCheck, HealthCheckOpt, Cmd};
///
/// let healthcheck = HealthCheck::Some {
///     opts: vec![
///         HealthCheckOpt::Interval("30s".into()),
///         HealthCheckOpt::Retries(5),
///     ],
///     cmd: Cmd{ argv: vec!["echo".into(), "hello".into()] },
/// };
///
/// let healthcheck_built = format!("{healthcheck}");
/// assert_eq!(healthcheck_built, r#"HEALTHCHECK --interval=30s --retries=5 CMD ["echo", "hello"]"#)
/// ```
#[derive(Clone, Debug)]
pub enum HealthCheck {
    Some { opts: Vec<HealthCheckOpt>, cmd: Cmd },
    None,
}

impl Display for HealthCheck {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Some { opts, cmd } => {
                write!(f, "HEALTHCHECK ")?;

                for opt in opts {
                    write!(f, "{opt} ")?;
                }

                write!(f, "{}", cmd)
            }
            Self::None => write!(f, "HEALTHCHECK NONE"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum HealthCheckOpt {
    Interval(String),
    Timeout(String),
    StartPeriod(String),
    StartInterval(String),
    Retries(i32),
}

impl Display for HealthCheckOpt {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Interval(d) => write!(f, "--interval={d}"),
            Self::Timeout(d) => write!(f, "--timeout={d}"),
            Self::StartPeriod(d) => write!(f, "--start-period={d}"),
            Self::StartInterval(d) => write!(f, "--start-interval={d}"),
            Self::Retries(n) => write!(f, "--retries={n}"),
        }
    }
}
