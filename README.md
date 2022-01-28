[![Crates.io](https://img.shields.io/crates/v/test-case.svg)](https://crates.io/crates/test-case)
[![Crates.io](https://img.shields.io/crates/d/test-case.svg)](https://crates.io/crates/test-case)
[![1.49+](https://img.shields.io/badge/rust-1.49.0%2B-orange.svg)](https://img.shields.io/badge/rust-1.49.0%2B-orange.svg)
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
test-case = "2.0.0-rc2"
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

running 4 tests
test tests::multiplication_tests::when_both_operands_are_negative ... ok
test tests::multiplication_tests::when_both_operands_are_positive ... ok
test tests::multiplication_tests::when_operands_are_swapped ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Advanced use

For `#[test_case(body)]` the body is built as follows:

`body` := `$arguments ($expected_result)? ($description)?`

### Arguments

`arguments` := `$expr(,$expr)*(,)?`

Comma separated list of one or more [expressions][1], eg.:
```rust
#[test_case(a, b, c,)]
#[test_case(())]
#[test_case(a_method_that_produces_arg(1, 2, 3), "a string")]
```

### Expected result

`expected_result` := `=> ($modifier)* $validator`

Optional part that provides assertions to instantiated tests.

When using `expected_result` version of `test_case` tested function **must** return a type
that can be matched with validator. Each validator description states how to ensure
that the type returned by function can be matched.

#### Modifiers

`modifier` := `ignore | inconclusive`

Both `ignore` and `inconclusive` keywords indicate that test case should be skipped. This is equivalent to using
`#[ignore]` attribute on normal test. Eg.:

```rust
#[test_case(0.0 => ignore 0.0)] // not yet implemented
```

#### Validator

There are numerous validators provided by `test_case`:

`validator` := `$simple|$matching|$panicking|$with|$using|$complex`

##### Simple

`simple` := `$expr`

Accepts any [expression][1] that evaluates to function return type and
compares it against whatever tested block returns via `assert_eq`. Eg.:

```rust
#[test_case("2.0" => 2.0)]
fn parses_a_string(arg_in: &str) -> f64 {
    body omitted...
}
```
##### Matching

`matching` := `matches $pattern`

A [pattern][3] following keyword `matches`.
Result of a function is compared to `pattern` via [MatchExpression][2]. Eg.:

```rust
#[test_case("2.0" => matches Ok(_))]
#[test_case("1.0" => matches Ok(v) if v == 1.0f64)]
#[test_case("abc" => matches Err(_))]
```

##### Panicking

`panicking` := `panics ($expr)?`

Indicates that test instance should panic. Works identical to `#[should_panic]` test attribute.
Optional expression after the keyword is treated like `expected` in [should_panic][4]. Eg.:

```rust
#[test_case(0 => panics "division by zero")]
```

##### With

`with` := `with $closure`

Allows manual assertions of the result of testing function.
Closure must indicate argument type and it has to be implicitly convertible from type returned by testing function.
Eg.:

```rust
#[test_case(2.0 => 0.0)]
#[test_case(0.0 => with |i: f64| assert!(i.is_nan()))]
fn test_division(i: f64) -> f64 {
    0.0 / i
}
```

##### Using

`using` := `using $path`

Work similar to `with` attribute, with the difference being that instead of a closure
it accepts path to a function that should validate result of the testing function. Eg.:

```rust
fn is_power_of_two(input: u64) {
    assert!(input.is_power_of_two())
}

#[test_case(1 => using self::is_power_of_two)]
fn some_test(input: u64) -> u64 {
    "body omitted..."
}
```

##### Complex

`complex` := `(it|is) $complex_expression`

`complex_expression` := `not $complex_expression_inner | $complex_expression_inner (and $complex_expression_inner)* | $complex_expression_inner (or $complex_expression_inner)*`

`complex_expression_inner` := `$cmp_assertion|$path_assertion|$collection_assertion|\($complex_expression\)`

`cmp_assertion` := `$ord_assertion|$almost_eq_assertion`
`path_assertion` := `existing_path|file|dir|directory`
`collection_assertion` := `contains $expr|contains_in_order $expr`
`ord_assertion` := `(eq|equal_to|lt|less_than|gt|greater_than|leq|less_or_equal_than|geq|greater_or_equal_than) $expr`
`almost_eq_assertion` := `(almost_equal_to|almost) $expr precision $expr`

Complex assertions are created as an extension to `test_case` allowing for more flexibility in comparisons. Eg.:

```rust
#[test_case(args => is lt 2*3.14)]
fn take_piece_of_circle(...) -> f64 {
    "body omitted..."
}

#[test_case(args => is existing_path)]
fn installation_created_path(...) -> PathBuf {
    "body omitted..."
}

#[test_case(args => is almost_equal_to 2.0 precision 0.00001)]
fn some_volatile_computation(...) -> f64 {
    "body omitted..."
}

#[test_case(args => it contains "Jack")]
fn list_of_users(...) -> Vec<String> {
    "body omitted..."
}

#[test_case(args => it contains_in_order [1, 2, 3])]
fn sorts_asc(...) -> Vec<i32> {
    "body omitted..."
}
```

`it` and `is` have equivalent interpretation. Both variants exist in order to make test cases easier to read.

> complex assertions are WIP content, use at own discretion.

## Notes about async & additional attributes

If `test_case` is used with `async` tests, eg. `#[tokio::test]`, or user wants to pass other attributes to each
test instance then additional attributes have to be added past first occurrence of `#[test_case]`. Eg.:

```rust
#[test_case(...)]
#[tokio::test]
#[allow(clippy::non_camel_case_types)]
async fn xyz() { }
```

[1]: https://doc.rust-lang.org/reference/expressions.html
[2]: https://doc.rust-lang.org/reference/expressions/match-expr.html
[3]: https://doc.rust-lang.org/reference/patterns.html
[4]: https://doc.rust-lang.org/book/ch11-01-writing-tests.html#checking-for-panics-with-should_panic

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
