use std::error::Error;

use airlang::{
    interpret_mutable,
    parse,
    Ctx,
    Invariant,
    MutableCtx,
    Symbol,
    Val,
};

use crate::{
    ext_ctx,
    solver,
};

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

fn test_interpret_with_ctx(ctx: Ctx, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
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

fn test_with_solver(input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = ext_ctx();
    let mut mut_ctx = MutableCtx::new(&mut ctx);
    let mut meta = Ctx::default();
    let mut mut_meta = MutableCtx::new(&mut meta);
    let key_solver = unsafe { Symbol::from_str_unchecked("solver") };
    let _ = mut_meta.put(key_solver, Invariant::Const, Val::Func(solver()));
    mut_ctx.set_meta(Some(meta));
    test_interpret_with_ctx(ctx, input, file_name)
}

#[test]
fn test_solver_no_panic() {
    solver();
}

#[ignore]
#[test]
fn test_trivial() -> Result<(), Box<dyn Error>> {
    test_with_solver(include_str!("test/trivial.air"), "trivial.air")
}
