use super::git::{Directory, PathAndInfo, Statuses};
use clap::ArgEnum;
use derive_more::Display;
use std::fmt::Display;

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Color {
    #[display(fmt = r"\[\033[30;1m\]")]
    Black,
    #[display(fmt = r"\[\033[31;1m\]")]
    Red,
    #[display(fmt = r"\[\033[32;1m\]")]
    Green,
    #[display(fmt = r"\[\033[33;1m\]")]
    Yellow,
    #[display(fmt = r"\[\033[34;1m\]")]
    Blue,
    #[display(fmt = r"\[\033[35;1m\]")]
    Magenta,
    #[display(fmt = r"\[\033[36;1m\]")]
    Cyan,
    #[display(fmt = r"\[\033[37;1m\]")]
    White,
}

#[derive(Debug)]
pub struct OutputColoring {
    pub path: Color,
    pub git_status: Color,
}

#[derive(Display, Debug)]
#[display(fmt = r"{}{}", _1, _0)]
pub struct ColoredDirectory(pub Directory, pub Color);

#[derive(Debug)]
pub struct ColoredStatuses(pub Statuses, pub Color);

#[derive(Debug)]
pub struct ColoredPathAndInfo(pub PathAndInfo, pub OutputColoring);

impl Display for ColoredPathAndInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            PathAndInfo::Git(dir, git_status) => {
                write!(f, "{}{} ", self.1.path, dir)?;
                write!(f, "{}{}", self.1.git_status, git_status)
            }
            PathAndInfo::Fallback => write!(f, r"{}\w ", self.1.path),
        }
    }
}

impl PathAndInfo {
    pub fn into_colored(self, colors: OutputColoring) -> ColoredPathAndInfo {
        ColoredPathAndInfo(self, colors)
    }
}
