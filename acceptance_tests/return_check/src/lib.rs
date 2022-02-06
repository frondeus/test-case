
#[cfg(test)]
mod tests {
    use test_case::test_case;

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
