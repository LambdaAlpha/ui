use std::error::Error;

use crate::{
    test,
    testing_ctx,
};

#[ignore]
#[test]
fn test_trivial() -> Result<(), Box<dyn Error>> {
    test(
        testing_ctx(),
        include_str!("../../../tests/trivial.air"),
        "trivial.air",
    )
}
