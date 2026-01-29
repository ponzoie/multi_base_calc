use calc_core::{eval_expression, CalcError};

#[test]
fn addition_single() {
    assert_eq!(eval_expression("1+2"), Ok(3));
}

#[test]
fn addition_multiple_with_spaces() {
    assert_eq!(eval_expression(" 10 + 20 + 30 "), Ok(60));
}

#[test]
fn addition_literal_range_error() {
    assert_eq!(eval_expression("2147483648"), Err(CalcError::LiteralOutOfRange));
}

#[test]
fn addition_result_range_error() {
    assert_eq!(eval_expression("2147483647 + 1"), Err(CalcError::RangeError));
}
