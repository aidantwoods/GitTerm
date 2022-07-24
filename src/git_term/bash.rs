use derive_more::Display;

use super::coloring::ColoredPathAndInfo;

#[derive(Display, Debug)]
#[display(fmt = r"\[\033[m\]\u:{}\[\033[m\]\$ ", _0)]
pub struct PromptCommand(pub ColoredPathAndInfo);
