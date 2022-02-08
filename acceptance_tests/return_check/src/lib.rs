
#[cfg(test)]
mod tests {
    use test_case::test_case;

    #[test_case(1 => panics "abc")]
    fn return_invalid_with_panic(a: i32) -> Result<i32, String> {
        panic!("abc")
    }

    #[test_case(0 => 0)]
    #[test_case(1 => panics "abc")]
    fn return_with_panic(a: i32) -> i32 {
        if a != 0 { panic!("abc") } else { a }
    }

    #[test_case(1)]
    fn return_no_expect_single(a: i32) -> Result<i32, String> {
        Ok(a)
    }

    #[test_case(1 => inconclusive 2)]
    #[test_case(2 => 2; "inconclusive")]
    #[test_case(1 => 1)]
    fn return_inconclusive(a: i32) -> i32 {
        a
    }

    #[test_case(2, 2; "Wrong")]
    #[test_case(2, 3; "Right")]
    fn return_no_expect(a: i32, b: i32) -> Result<i32, String> {
        let result = a + b;
        return if result == 5 {
            Ok(result)
        } else {
            Err("Not equal 5".to_owned())
        }
    }

    #[test_case(2, 2 => Err("Not equal 5".to_owned()); "Wrong")]
    #[test_case(2, 3 => Ok(5); "Right")]
    fn return_with_expect(a: i32, b: i32) -> Result<i32, String> {
        let result = a + b;
        return if result == 5 {
            Ok(result)
        } else {
            Err("Not equal 5".to_owned())
        }
    }
}
