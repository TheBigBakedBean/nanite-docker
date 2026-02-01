use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Represents a `USER` instruction.
/// ```rust
/// use nanite_docker::{User, Instruction};
///
/// let user = User::ByName {
///     name: "user".into(),
///     group: Some("group".into()),
/// };
/// let user_built = format!("{user}");
/// assert_eq!(user_built, r#"USER user:group"#);
///
/// let user = User::ById {
///     uid: 1000,
///     gid: Some(1000),
/// };
/// let user_built = format!("{user}");
/// assert_eq!(user_built, r#"USER 1000:1000"#);
/// ```
#[derive(Clone, Debug)]
pub enum User {
    ByName { name: String, group: Option<String> },
    ById { uid: usize, gid: Option<usize> },
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            User::ByName { name, group } => {
                if let Some(group) = group {
                    write!(f, "USER {}:{}", name, group)
                } else {
                    write!(f, "USER {}", name)
                }
            }
            User::ById { uid, gid } => {
                if let Some(gid) = gid {
                    write!(f, "USER {}:{}", uid, gid)
                } else {
                    write!(f, "USER {}", uid)
                }
            }
        }
    }
}
