use rdotenv::DotEnv;

#[test]
fn test_parse_line() {
    let line = "test_key=test_value";
    let pair = DotEnv::parse_line(line);
    assert_eq!(pair.key, "test_key");
    assert_eq!(pair.value, "test_value");
}

#[test]
fn test_loading_file_and_parse() {
    let mut dotenv = DotEnv::new();
    dotenv.load(Some("tests/.env"));
    assert_eq!(dotenv.pairs.len(), 2);
    assert_eq!(dotenv.pairs[0].key, "TEST_KEY1");
    assert_eq!(dotenv.pairs[0].value, "test_value1");
    assert_eq!(dotenv.pairs[1].key, "TEST_KEY2");
    assert_eq!(dotenv.pairs[1].value, "test_value2");
}