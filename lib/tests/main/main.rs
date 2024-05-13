use std::error::Error;

use airlang::{
    initial_ctx,
    interpret_mutable,
    parse,
    Ctx,
    MutableCtx,
    Val,
};

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

pub(crate) fn testing_ctx() -> Ctx {
    let ctx = import("../tests/testing_ctx.air");
    let Val::Ctx(ctx) = ctx else { unreachable!() };
    ctx.into()
}

#[test]
fn test_ctx_no_panic() {
    testing_ctx();
}

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

pub(crate) fn test(ctx: Ctx, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    if input.is_empty() {
        return Ok(());
    }
    let backup = ctx;

    let tests = input.split(MAIN_DELIMITER);
    for test in tests {
        let split_err = format!("file {file_name}, case ({test}): invalid test case format");
        let (i, o) = test.split_once(SUB_DELIMITER).expect(&split_err);
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): input ({i}) parse failed\n{e}");
            e
        })?;
        let mut ctx = backup.clone();
        let ret = interpret_mutable(MutableCtx::new(&mut ctx), src);

        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): output ({o}) parse failed\n{e}");
            e
        })?;
        let mut ret_ctx = backup.clone();
        let ret_expected = interpret_mutable(MutableCtx::new(&mut ret_ctx), ret_expected);
        assert_eq!(
            ret, ret_expected,
            "file {file_name}, case({test}): interpreting output is not as expected! real output: {ret:#?}, \
            current context: {ctx:#?}",
        );
    }
    Ok(())
}

mod trivial;
