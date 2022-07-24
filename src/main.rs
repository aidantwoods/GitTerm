#![feature(try_trait_v2)]

pub mod git_term;

use clap::Parser;

use crate::git_term::{
    bash::PromptCommand,
    coloring::{Color, OutputColoring},
    git::path_and_info,
};

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
    fn colors(&self) -> OutputColoring {
        OutputColoring {
            path: self.path_color,
            git_status: self.git_status_color,
        }
    }
}

fn main() {
    let args = Args::parse();

    let path_and_info = path_and_info().into_colored(args.colors());

    if args.path_and_info_only {
        print!("{}", path_and_info)
    } else {
        print!("{}", PromptCommand(path_and_info))
    }
}
