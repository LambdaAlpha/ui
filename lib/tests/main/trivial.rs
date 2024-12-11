use std::error::Error;

use crate::{
    test,
    testing_air_cell,
};

#[ignore]
#[test]
fn test_trivial() -> Result<(), Box<dyn Error>> {
    test(
        testing_air_cell(),
        include_str!("../../../tests/trivial.air"),
        "trivial.air",
    )
}
