use crate::Instruction;
use alloc::boxed::Box;
use core::fmt::{Display, Formatter};

/// Represents an `ONBUILD` instruction.
/// ```rust
/// use nanite_docker::{OnBuild, Instruction, Run};
///
/// let onbuild = OnBuild {
///     instruction: Box::new(Instruction::Run(Run{
///         argv: vec!["echo".into(), "hello".into()],
///         mounts: vec![],
///         network: None,
///         security: None,
///     })),
/// };
/// let onbuild_built = format!("{onbuild}");
/// assert_eq!(onbuild_built, r#"ONBUILD RUN ["echo", "hello"]"#);
/// ```
#[derive(Clone, Debug)]
pub struct OnBuild {
    pub instruction: Box<Instruction>,
}
impl Display for OnBuild {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "ONBUILD {}", self.instruction)
    }
}
