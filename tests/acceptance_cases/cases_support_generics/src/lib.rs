#![cfg(test)]
use test_case::test_case;

struct Target { i: i64 }

struct Source1;
struct Source2;

impl From<Source1> for Target {
    fn from(_: Source1) -> Self {
        Self { i: 1 }
    }
}

impl From<Source2> for Target {
    fn from(_: Source2) -> Self {
        Self { i: 2 }
    }
}

#[test_case(Source1 => 1)]
#[test_case(Source2 => 2)]
fn test_generics<T: Into<Target>>(input: T) -> i64 {
    let t: Target = input.into();
    t.i
}

#[test_case(Source1 => 1)]
#[test_case(Source2 => 2)]
fn test_impl(input: impl Into<Target>) -> i64 {
    let t: Target = input.into();
    t.i
}
