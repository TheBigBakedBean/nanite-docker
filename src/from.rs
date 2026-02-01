use alloc::string::String;
use core::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
pub struct From {
    pub platform: Option<String>,
    pub image: String,
    pub alias: Option<String>,
}
impl Display for From {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "FROM ")?;
        match &self.platform {
            Some(platform) => write!(f, "--platform={platform}")?,
            None => {}
        }
        write!(f, "{}", self.image)?;
        match &self.alias {
            Some(alias) => write!(f, " AS {alias}")?,
            None => {}
        }

        Ok(())
    }
}
