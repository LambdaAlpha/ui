use std::error::Error;

use airlang::{
    AirCell,
    Mode,
    Text,
    Val,
    parse,
};

pub(crate) fn ext_air_cell() -> AirCell {
    let mut cell = AirCell::default();
    airlang_ext::init_ctx(cell.ctx_mut());
    cell
}

pub(crate) fn import(path: &str) -> Val {
    let src = generate_import(path);
    let val = parse(&src).expect("parse should never fail");
    let mut cell = ext_air_cell();
    cell.interpret(val)
}

fn generate_import(path: &str) -> String {
    let mut src = Text::from("build.import \"");
    src.push_str_escaped(env!("CARGO_MANIFEST_DIR"));
    src.push_str(path);
    src.push('"');
    src.into()
}

pub(crate) fn testing_air_cell() -> AirCell {
    let ctx = import("/../tests/testing_ctx.air");
    let Val::Ctx(ctx) = ctx else { unreachable!() };
    AirCell::new(Mode::default(), ctx.into())
}

#[test]
fn test_ctx_no_panic() {
    testing_air_cell();
}

const MAIN_DELIMITER: &str = "=====";
const SUB_DELIMITER: &str = "-----";

pub(crate) fn test(air: AirCell, input: &str, file_name: &str) -> Result<(), Box<dyn Error>> {
    if input.is_empty() {
        return Ok(());
    }
    let backup = air;

    let tests = input.split(MAIN_DELIMITER);
    for test in tests {
        let split_err = format!("file {file_name}, case ({test}): invalid test case format");
        let (i, o) = test.split_once(SUB_DELIMITER).expect(&split_err);
        let src = parse(i).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): input ({i}) parse failed\n{e}");
            e
        })?;
        let mut air = backup.clone();
        let ret = air.interpret(src);

        let ret_expected = parse(o).map_err(|e| {
            eprintln!("file {file_name}, case ({test}): output ({o}) parse failed\n{e}");
            e
        })?;
        let mut ret_cell = backup.clone();
        let ret_expected = ret_cell.interpret(ret_expected);
        assert_eq!(
            ret, ret_expected,
            "file {file_name}, case({test}): interpreting output is not as expected! real output: {ret:#?}, \
            current context: {air:#?}",
        );
    }
    Ok(())
}

mod trivial;
