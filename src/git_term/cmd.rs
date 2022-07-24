use derive_more::Display;

use super::coloring::ColoredPathAndInfo;

#[derive(Display, Debug)]
#[display(fmt = r"PROMPT='%f%n:{}%f%(?.. %F{{red}}%?%f)$ '", _0)]
pub struct Prompt(pub ColoredPathAndInfo);
