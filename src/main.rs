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

extern crate derive_more;

fn main() {
    print!("{}", ps1());
}

#[derive(Display, Debug)]
enum Ps1 {
    #[display(fmt = r"\[\033[m\]\u:{}{} {}{}\[\033[m\]\$ ", Color::Blue, _0, Color::Yellow, _1)]
    Git(String, Statuses),
    #[display(fmt = r"\[\033[m\]\u:{}\w \[\033[m\]\$ ", Color::Blue)]
    Fallback,
}

impl FromResidual<Option<Infallible>> for Ps1 {
    fn from_residual(_: Option<Infallible>) -> Self {
        Ps1::Fallback
    }
}

#[derive(Display, Debug)]
enum Color {
    #[display(fmt = r"\[\033[36;1m\]")]
    Blue,
    #[display(fmt = r"\[\033[33;1m\]")]
    Yellow,
}

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

fn ps1() -> Ps1 {
    Ps1::Git(relative_git_dir()?,  git_statuses()?)
}

fn relative_git_dir() -> Option<String> {
    let current_dir = env::current_dir().ok()?;
    let current_path = Path::new(&current_dir);
    
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--show-toplevel")
        .output().ok()?;

    let git_parent_path = Path::new(std::str::from_utf8(&output.stdout).ok()?).parent()?;

    let relative_git_path = current_path.strip_prefix(git_parent_path.to_str()?).ok()?;

    Some(relative_git_path.to_str()?.to_string())
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
