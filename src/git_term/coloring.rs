use super::git::{Directory, PathAndInfo, Statuses};
use clap::ArgEnum;
use derive_more::Display;
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    fn name(&self) -> String {
        match self {
            Color::Black => "black",
            Color::Red => "red",
            Color::Green => "green",
            Color::Yellow => "yellow",
            Color::Blue => "blue",
            Color::Magenta => "magenta",
            Color::Cyan => "cyan",
            Color::White => "white",
        }
        .to_string()
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%F{{{}}}", self.name())
    }
}

#[derive(Debug)]
pub struct OutputColoring {
    pub work_dir: Color,
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
                write!(f, "%B{}{}%b ", self.1.work_dir, dir)?;
                write!(f, "{}{}", self.1.git_status, git_status)
            }
            PathAndInfo::Fallback => write!(f, r"%B{}%~%b ", self.1.work_dir),
        }
    }
}

impl PathAndInfo {
    pub fn into_colored(self, colors: OutputColoring) -> ColoredPathAndInfo {
        ColoredPathAndInfo(self, colors)
    }
}
