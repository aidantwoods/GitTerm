#![feature(try_trait_v2)]

pub mod cmd;
pub mod coloring;
pub mod git;

use clap::Parser;

use crate::{
    cmd::Prompt,
    coloring::{Color, OutputColoring},
    git::path_and_info,
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Only generate the path and git info section
    #[clap(short, long, value_parser)]
    minimal: bool,

    /// Color of working directory
    #[clap(short, long, value_parser, default_value = "blue")]
    work_dir_color: Color,

    /// Color of git status
    #[clap(short, long, value_parser, default_value = "yellow")]
    git_status_color: Color,
}

impl Args {
    fn colors(&self) -> OutputColoring {
        OutputColoring {
            work_dir: self.work_dir_color,
            git_status: self.git_status_color,
        }
    }
}

fn main() {
    let args = Args::parse();

    let path_and_info = path_and_info().into_colored(args.colors());

    if args.minimal {
        print!("{}", path_and_info)
    } else {
        print!("{}", Prompt(path_and_info))
    }
}
