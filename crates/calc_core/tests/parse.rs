use calc_core::parse;

#[test]
fn parse_one() {
    assert!(parse("1") == Ok(1));
}

#[test]
fn parse_large_number() {
    assert!(parse("2147483647") == Ok(2147483647));
}

#[test]
fn parse_small_number() {
    assert!(parse("2") == Ok(2));
}
