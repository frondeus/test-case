Things to do before release:

# 2.0.0
* `using path_to_function` and `with |t: T| lambda` return types
* deprecation of interpreting `inconclusive` within test case description
* `using` has to be applicable to whole function via `#[test_case_handler()]` attribute
* Users are able to manually override method names used for individual test cases
  * `#[test_case(args... => result... ; "description..." as test_case_function_name)]`
  * `#[test_case(args... => result... as test_case_function_name)]`
* `#[test_case(args...)` with no `=> ...` part can return same types as normal `#[test]` (eg. `Result<_, _>`)

# 2.1.0
* `with` result mapping asserts methods returning boolean (so users can write `with |val: Typ| body...` instead of `with |val: Typ| assert!(body...)`)
* `|val: Typ| body` can skip type signature on argument
