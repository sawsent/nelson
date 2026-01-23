use crate::context::Context;
use std::path::PathBuf;

pub fn init(fp: &PathBuf, ctx: &Context) {
    ctx.vprint(format_args!("{:?}", fp));
}
