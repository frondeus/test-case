#[cfg(test)]
mod tests {
    use std::error::Error;
    use test_case::test_case;

    #[test_case(12)]
    #[test_case(13)]
    fn is_even(value: u64) -> Result<(), String> {
        if value % 2 == 0 {
            Ok(())
        } else {
            Err("is odd".to_string())
        }
    }

    #[test_case(12)]
    #[test_case(13)]
    fn is_odd_boxed(value: u64) -> Result<(), Box<dyn Error>> {
        if value % 2 == 1 {
            Ok(())
        } else {
            Err("is even".to_string().into())
        }
    }
}
