mod test_cases {
    use test_case::test_case;

    #[test_case(Some("str") => is some())]
    #[test_case(None        => is none())]
    #[test_case(Some("str") => is all!(some(), not(none())))]
    #[test_case(Some("str") => is has("str"))]
    #[test_case(Some("str") => is some() ; "can be followed by comment")]
    #[test_case(Some("str") => is none() ; "can be followed by inconclusive comment")]
    fn hamcrest_feature_works(v: Option<&str>) -> Option<&str> {
        v
    }

    #[test_case(&[1, 3] => is empty())]
    #[test_case(&[2, 3] => it contains(2))]
    #[test_case(&[2, 3] => it not(contains(3)))]
    #[test_case(&[2, 4] => it contains(vec!(2, 4)))]
    #[test_case(&[2, 3] => is len(1))]
    fn removes_odd_numbers(collection: &[u8]) -> &Vec<u8> {
        Box::leak(Box::new(collection.into_iter().filter(|x| *x % 2 == 0).map(|v| *v).collect()))
    }
}
