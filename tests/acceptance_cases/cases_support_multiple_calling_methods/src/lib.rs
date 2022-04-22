#![cfg(test)]
mod import {
    use test_case::test_case;

    #[test_case(2)]
    fn can_import_test_case_attribute(_: u8) {}
}

mod short_version {
    use test_case::case;

    #[case(12u8 => 12u16)]
    fn can_use_case_attribute_same_as_test_case(i: u8) -> u16 {
        i as u16
    }
}

#[test_case::test_case(1; "first test")]
#[test_case::test_case(1; "second test")]
fn can_use_qualified_attribute(_: u8) {}
