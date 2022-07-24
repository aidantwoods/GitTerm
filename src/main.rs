#![feature(try_trait_v2)]

use std::ops::FromResidual;
use std::convert::Infallible;

use std::env;
use std::fmt::Display;
use std::hash::Hash;
use std::process::Command;
use std::path::Path;
use derive_more::Display;
use std::collections::HashSet;
use clap::{Parser, ArgEnum};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Only generate the path and git info section
    #[clap(long, value_parser)]
    path_and_info_only: bool,

    /// Color of path section
    #[clap(long, value_parser, default_value = "blue")]
    path_color: Color,

    /// Color of git status section
    #[clap(long, value_parser, default_value = "yellow")]
    git_status_color: Color,
}

impl Args {
    fn path_and_info_colors(&self) -> PathAndInfoColors {
        PathAndInfoColors(self.path_color, self.git_status_color)
    }
}

#[derive(Debug)]
struct PathAndInfoColors(Color, Color);

fn main() {
    let args = Args::parse();

    let path_and_info = path_and_info().into_colored(&args);

    if args.path_and_info_only {
        print!("{}", path_and_info)
    } else {
        print!("{}", PromptCommand(path_and_info))
    }
}

#[derive(Display, Debug)]
#[display(fmt = r"\[\033[m\]\u:{}\[\033[m\]\$ ", _0)]
struct PromptCommand(ColoredPathAndInfo);

#[derive(Debug)]
struct ColoredPathAndInfo(PathAndInfo, PathAndInfoColors);

impl Display for ColoredPathAndInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            PathAndInfo::Git(dir, git_status) =>  {
                write!(f, "{}{} ", self.1.0, dir)?;
                write!(f, "{}{}", self.1.1, git_status)
            },
            PathAndInfo::Fallback => write!(f, r"{}\w ", self.1.0),
        }
    }
}

#[derive(Debug)]
enum PathAndInfo {
    Git(Directory, Statuses),
    Fallback,
}

impl FromResidual<Option<Infallible>> for PathAndInfo {
    fn from_residual(_: Option<Infallible>) -> Self {
        PathAndInfo::Fallback
    }
}

impl PathAndInfo {
    fn into_colored(self, args: &Args) -> ColoredPathAndInfo {
        ColoredPathAndInfo(self, args.path_and_info_colors())
    }
}

#[derive(Display, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum Color {
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

#[derive(Display, Debug)]
#[display(fmt = r"{}{}", _1, _0)]
struct ColoredDirectory(Directory, Color);

#[derive(Display, Debug)]
#[display(fmt = r"{}", _0)]
struct Directory(String);

#[derive(Display, Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
enum Status {
    #[display(fmt = "*")]
    Modified,
    #[display(fmt = "+")]
    Added,
    #[display(fmt = "-")]
    Deleted,
    #[display(fmt = "?")]
    Untracked
}

#[derive(Debug)]
struct ColoredStatuses(Statuses, Color);

#[derive(Debug)]
struct Statuses(HashSet<Status>);

impl Display for Statuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut statuses = self.0.iter().collect::<Vec<_>>();

        statuses.sort();

        for status in statuses {
            write!(f, "{}", status)?;
        }

        Ok(())
    }
}

fn path_and_info() -> PathAndInfo {
    PathAndInfo::Git(relative_git_dir()?,  git_statuses()?)
}

fn relative_git_dir() -> Option<Directory> {
    let current_dir = env::current_dir().ok()?;
    let current_path = Path::new(&current_dir);
    
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output().ok()?;

    let git_parent_path = Path::new(std::str::from_utf8(&output.stdout).ok()?).parent()?;

    let relative_git_path = current_path.strip_prefix(git_parent_path.to_str()?).ok()?;

    Some(Directory(relative_git_path.to_str()?.to_string()))
}

fn git_statuses() -> Option<Statuses> {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output().ok()?;

    let lines_iter = std::str::from_utf8(&output.stdout).ok()?.split("\n");

    let statuses_chars = lines_iter.map(|l| l.get(0..=1).unwrap_or(""))
        .collect::<Vec<_>>()
        .join("");

    let pairs = [
        (Status::Modified, statuses_chars.contains(['M', 'C', 'R', 'U'])),
        (Status::Added, statuses_chars.contains(['A'])),
        (Status::Deleted, statuses_chars.contains(['D'])),
        (Status::Untracked, statuses_chars.contains(['?']))
    ];

    let statuses = pairs.iter().filter(|p| p.1).map(|p|p.0);

    Some(Statuses(HashSet::from_iter(statuses)))
}
