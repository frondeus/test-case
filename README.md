[![Crates.io](https://img.shields.io/crates/v/test-case.svg)](https://crates.io/crates/test-case)
[![Crates.io](https://img.shields.io/crates/d/test-case.svg)](https://crates.io/crates/test-case)
[![Docs.rs](https://docs.rs/test-case/badge.svg)](https://docs.rs/test-case)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rust-lang/docs.rs/master/LICENSE)
[![Build Status](https://github.com/frondeus/test-case/workflows/Test/badge.svg)](https://github.com/frondeus/test-case/actions)
![Maintenance](https://img.shields.io/badge/maintenance-activly--developed-brightgreen.svg)

# Test Case

## Overview
`test_case` crate provides procedural macro attribute that generates parametrized test instances.

## Getting Started

Crate has to be added as a dependency to `Cargo.toml`:

```toml
[dev-dependencies]
test-case = "*"
```

and imported to the scope of a block where it's being called
(since attribute name collides with rust's built-in `custom_test_frameworks`) via:

```rust
use test_case::test_case;
```

## Example usage:

```rust
#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(-2, -4 ; "when both operands are negative")]
    #[test_case(2,  4  ; "when both operands are positive")]
    #[test_case(4,  2  ; "when operands are swapped")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = (x * y).abs();

        assert_eq!(8, actual)
    }
}
```

Output from `cargo test` for this example:

```sh
$ cargo test

running 3 tests
test tests::multiplication_tests::when_both_operands_are_negative ... ok
test tests::multiplication_tests::when_both_operands_are_positive ... ok
test tests::multiplication_tests::when_operands_are_swapped ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### Test Matrix

The `#[test_matrix(...)]` macro allows generating multiple test cases from the
Cartesian product of one or more possible values for each test function argument. The
number of arguments to the `test_matrix` macro must be the same as the number of arguments to
the test function. Each macro argument can be:

    1. A list in array (`[x, y, ...]`) or tuple (`(x, y, ...)`) syntax. The values can be any
       valid [expression](https://doc.rust-lang.org/reference/expressions.html).
    2. A closed numeric range expression (e.g. `0..100` or `1..=99`), which will generate
       argument values for all integers in the range.
    3. A single expression, which can be used to keep one argument constant while varying the
       other test function arguments using a list or range.

#### Example usage:

```rust
#[cfg(test)]
mod tests {
    use test_case::test_matrix;

    #[test_matrix(
        [-2, 2],
        [-4, 4]
    )]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = (x * y).abs();

        assert_eq!(8, actual)
    }
}
```

## MSRV Policy

Starting with version 3.0 and up `test-case` introduces policy of only supporting latest stable Rust.
These changes may happen overnight, so if your stack is lagging behind current stable release,
it may be best to consider locking `test-case` version with `=` in your `Cargo.toml`.

## Documentation

Most up to date documentation is available in our [wiki](https://github.com/frondeus/test-case/wiki).

# License

Licensed under of MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

# Contributing

Project roadmap is available at [link](https://github.com/frondeus/test-case/issues/74). All contributions are welcome.

Recommended tools:
* `cargo readme` - to regenerate README.md based on template and lib.rs comments
* `cargo insta`  - to review test snapshots
* `cargo edit`   - to add/remove dependencies
* `cargo fmt`    - to format code
* `cargo clippy` - for all insights and tips
* `cargo fix`    - for fixing warnings
