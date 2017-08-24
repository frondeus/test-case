# Overview
This crate provides `#[test_case]` procedural macro attribute that generates multiple tests using one body with different input parameters.
A test is generated for each data set passed in `test_case` attribute.
Under the hood, all test cases that share same body are grouped into `mod`, giving clear and readable test results.

[![Crates.io](https://img.shields.io/crates/v/test-case-derive.svg)](https://crates.io/crates/test-case-derive)
[![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/kbknapp/clap-rs/blob/master/LICENSE-MIT)
[![Build Status](https://travis-ci.org/synek317/test-case-derive.svg?branch=master)](https://travis-ci.org/synek317/test-case-derive)

[Documentation](https://docs.rs/test-case-derive/)

[Repository](https://github.com/synek317/test-case-derive)

# Getting Started

First of all you have to add this dependency to your `Cargo.toml`:

```toml
[dev-dependencies]
test-case-derive = "0.1.0"
```

Additionally you have to enable `proc_macro` feature and include crate. You can do this globally by adding:

```
#![feature(proc_macro)]
extern crate test_case_derive;
```

to your `lib.rs` or `main.rs` file. Optionally you may enable proc macros only for tests:

```
#![cfg_attr(test, feature(proc_macro))]
#[cfg(test)]
extern crate test_case_derive;
```

Don't forget that procedural macros are imported with `use` statement:

```
use test_case_derive::test_case;
```

# Example usage:

```
#![cfg(test)]
#![feature(proc_macro)]
extern crate test_case_derive;

use test_case_derive::test_case;

#[test_case( 2,  4 :: "when both operands are possitive")]
#[test_case( 4,  2 :: "when operands are swapped")]
#[test_case(-2, -4 :: "when both operands are negative")]
fn multiplication_tests(x: i8, y: i8) {
    let actual = (x * y).abs();

    assert_eq!(8, actual)
}
```

Output from `cargo test` for this example:

```
$ cargo test

running 3 tests
test multiplication_tests::when_both_operands_are_possitive ... ok
test multiplication_tests::when_both_operands_are_negative ... ok
test multiplication_tests::when_operands_are_swapped ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

# Examples

If your only assertion is just `assert_eq!`, you can pass the expectation as macro attribute using `=>` syntax:

```
#[test_case( 2 => 2 :: "returns given number for positive input")]
#[test_case(-2 => 2 :: "returns opposite number for non-positive input")]
#[test_case( 0 => 0 :: "returns 0 for 0")]
fn abs_tests(x: i8) -> i8 {
   if x > 0 { x } else { -x }
}
```

Which is equivalent to 

```
#[test_case( 2, 2 :: "returns given number for positive input")]
#[test_case(-2, 2 :: "returns opposite number for non-positive input")]
#[test_case( 0, 0 :: "returns 0 for 0")]
fn abs_tests(x: i8, expected: i8){
   let actual = if x > 0 { x } else { -x };

   assert_eq!(expected, actual);
}
```

Attributes and expectation may be any expresion unless they contain `=>`, e.g.

```
#[test_case(None,        None    => 0 :: "treats none as 0")]
#[test_case(Some(2),     Some(3) => 5)]
#[test_case(Some(2 + 3), Some(4) => 2 + 3 + 4)]
fn fancy_addition(x: Option<i8>, y: Option<i8>) -> i8 {
    x.unwrap_or(0) + y.unwrap_or(0)
}
```

Note: in fact, `=>` is not prohibited but the parser will always treat last `=>` sign as beginning of expectation definition.

Test case names are optional. They are set using `::` followed by string literal at the end of macro attributes.

Example generated code:

```
mod fancy_addition {
    #[allow(unused_imports)]
    use super::*;

    fn fancy_addition(x: Option<i8>, y: Option<i8>) -> i8 {
        x.unwrap_or(0) + y.unwrap_or(0)
    }

    #[test]
    fn treats_none_as_0() {
        let expected = 0;
        let actual = fancy_addition(None, None);

        assert_eq!(expected, actual);
    }

    #[test]
    fn some_2_some_3() {
        let expected = 5;
        let actual = fancy_addition(Some(2), Some(3));
        
        assert_eq!(expected, actual);
    }

    #[test]
    fn some_2_3_some_4() {
        let expected = 2 + 3 + 4;
        let actual = fancy_addition(Some(2 + 3), Some(4));

        assert_eq!(expected, actual);
    }
}
```

# Contribution

All contributions and comments are more than welcome! Don't be afraid to open an issue or PR whenever you find a bug or have an idea to improve this crate.

# License

MIT License

Copyright (c) 2017 Marcin Sas-Szymański

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.