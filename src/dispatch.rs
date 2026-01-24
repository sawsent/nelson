use crate::context::Context;
use crate::domain::Init;
use std::path::PathBuf;

pub fn init(_cmd: &Init, fp: &PathBuf, ctx: &Context) {
    ctx.vprint(format_args!("{:?}", fp));
}
