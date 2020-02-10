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
}
