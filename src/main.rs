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

fn main() {
    let args = Args::parse();

    let path_and_info = path_and_info(&args);

    if args.path_and_info_only {
        print!("{}", path_and_info)
    } else {
        print!("{}", PromptCommand(path_and_info))
    }
}

#[derive(Display, Debug)]
#[display(fmt = r"\[\033[m\]\u:{}\[\033[m\]\$ ", _0)]
struct PromptCommand(PathAndInfo);

#[derive(Display, Debug)]
enum PathAndInfo {
    #[display(fmt = r"{} {}", _0, _1)]
    Git(Directory, Statuses),
    #[display(fmt = r"\w ")]
    Fallback,
}

impl FromResidual<Option<Infallible>> for PathAndInfo {
    fn from_residual(_: Option<Infallible>) -> Self {
        PathAndInfo::Fallback
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
struct Directory(String, Color);

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
struct Statuses(HashSet<Status>, Color);

impl Display for Statuses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut statuses = self.0.iter().collect::<Vec<_>>();

        statuses.sort();

        write!(f, "{}", self.1)?;

        for status in statuses {
            write!(f, "{}", status)?;
        }

        Ok(())
    }
}

fn path_and_info(args: &Args) -> PathAndInfo {
    PathAndInfo::Git(relative_git_dir(args.path_color)?,  git_statuses(args.git_status_color)?)
}

fn relative_git_dir(color: Color) -> Option<Directory> {
    let current_dir = env::current_dir().ok()?;
    let current_path = Path::new(&current_dir);
    
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output().ok()?;

    let git_parent_path = Path::new(std::str::from_utf8(&output.stdout).ok()?).parent()?;

    let relative_git_path = current_path.strip_prefix(git_parent_path.to_str()?).ok()?;

    Some(Directory(relative_git_path.to_str()?.to_string(), color))
}

fn git_statuses(color: Color) -> Option<Statuses> {
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

    Some(Statuses(HashSet::from_iter(statuses), color))
}
