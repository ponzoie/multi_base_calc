use calc_core::{parse, CalcError};

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

#[test]
fn parse_binary_number() {
    assert!(parse("0b0101") == Ok(5));
}

#[test]
fn parse_binary_number1() {
    assert!(parse("0b11111111") == Ok(255));
}

#[test]
fn parse_invalid_decimal_underscore_leading() {
    assert!(parse("_1") == Err(CalcError::InvalidLiteral));
}

#[test]
fn parse_invalid_decimal_underscore_double() {
    assert!(parse("1__2") == Err(CalcError::InvalidLiteral));
}

#[test]
fn parse_invalid_decimal_underscore_trailing() {
    assert!(parse("1_") == Err(CalcError::InvalidLiteral));
}

#[test]
fn parse_invalid_binary_literal_empty() {
    assert!(parse("0b") == Err(CalcError::InvalidLiteral));
}

#[test]
fn parse_invalid_binary_literal_digit() {
    assert!(parse("0b102") == Err(CalcError::InvalidLiteral));
}

#[test]
fn parse_invalid_binary_literal_trailing_underscore() {
    assert!(parse("0b1_") == Err(CalcError::InvalidLiteral));
}

#[test]
fn parse_invalid_token() {
    assert!(parse("1a") == Err(CalcError::InvalidToken('a')));
}
