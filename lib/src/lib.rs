use airlang::{
    initial_ctx,
    interpret_mutable,
    parse,
    Ctx,
    FuncVal,
    MutableCtx,
    Val,
};

pub fn solver() -> FuncVal {
    let solver = import("../air/agi.air");
    let Val::Func(solver) = solver else {
        unreachable!()
    };
    solver
}

pub(crate) fn ext_ctx() -> Ctx {
    let mut ctx = initial_ctx();
    let mut mut_ctx = MutableCtx::new(&mut ctx);
    airlang_ext::init_ctx(mut_ctx.reborrow());
    ctx
}

// PWD is lib/
pub(crate) fn import(path: &str) -> Val {
    let src = format!("build.import \"{path}\"");
    let Ok(val) = parse(&src) else { unreachable!() };
    let mut ctx = ext_ctx();
    interpret_mutable(MutableCtx::new(&mut ctx), val)
}

#[cfg(test)]
mod test;
