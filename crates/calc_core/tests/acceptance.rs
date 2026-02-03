use calc_core::{eval_expression, CalcError};

// 受け入れテスト
#[test]
fn acceptance_precedence() {
    assert_eq!(eval_expression("1+2*3"), Ok(7));
}

#[test]
fn acceptance_parentheses() {
    assert_eq!(eval_expression("(1+2)*3"), Ok(9));
}

#[test]
fn acceptance_mixed_bases() {
    assert_eq!(eval_expression("0x10 + 0b11"), Ok(19));
}

#[test]
fn acceptance_negative_remainder() {
    assert_eq!(eval_expression("-5 % 3"), Ok(1));
}

#[test]
fn acceptance_addition_range_error() {
    assert_eq!(
        eval_expression("2147483647 + 1"),
        Err(CalcError::RangeError)
    );
}

#[test]
fn acceptance_min_subtraction_range_error() {
    assert_eq!(
        eval_expression("-2147483648 - 1"),
        Err(CalcError::RangeError)
    );
}

#[test]
fn acceptance_literal_range_error() {
    assert_eq!(
        eval_expression("0x1_0000_0000"),
        Err(CalcError::LiteralOutOfRange)
    );
}
