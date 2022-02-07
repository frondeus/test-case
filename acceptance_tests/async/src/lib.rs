mod test_cases {
    use test_case::test_case;

    #[test_case(100i32 => 100usize)]
    #[tokio::test]
    async fn works_seamlessly_with_tokio(arg: i32) -> usize {
        arg as usize
    }

    #[test_case(100i32 => 100usize)]
    #[async_std::test]
    async fn works_seamlessly_with_async_std(arg: i32) -> usize {
        arg as usize
    }
}
