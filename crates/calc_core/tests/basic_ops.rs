use calc_core::eval_expression;

// 加算
#[test]
fn basic_addition() {
    assert_eq!(eval_expression("1+2"), Ok(3));
}

// 減算
#[test]
fn basic_subtraction() {
    assert_eq!(eval_expression("5-3"), Ok(2));
}

// 乗算
#[test]
fn basic_multiplication() {
    assert_eq!(eval_expression("4*3"), Ok(12));
}

// 余り
#[test]
fn basic_remainder() {
    assert_eq!(eval_expression("5%3"), Ok(2));
}

// 減算の異常系
#[test]
fn subtraction_negative_result_error() {
    assert_eq!(eval_expression("1-2"), Err(calc_core::CalcError::RangeError));
}

// 余りの0除算
#[test]
fn remainder_divide_by_zero_error() {
    assert_eq!(
        eval_expression("5%0"),
        Err(calc_core::CalcError::RangeError)
    );
}

// 演算子の異常系
#[test]
fn trailing_plus_operator_error() {
    assert_eq!(
        eval_expression("1+"),
        Err(calc_core::CalcError::InvalidToken('+'))
    );
}

// 演算子の異常系
#[test]
fn trailing_minus_operator_error() {
    assert_eq!(
        eval_expression("1-"),
        Err(calc_core::CalcError::InvalidToken('-'))
    );
}

// 演算子の異常系
#[test]
fn trailing_multiply_operator_error() {
    assert_eq!(
        eval_expression("1*"),
        Err(calc_core::CalcError::InvalidToken('*'))
    );
}

// 演算子の異常系
#[test]
fn trailing_remainder_operator_error() {
    assert_eq!(
        eval_expression("1%"),
        Err(calc_core::CalcError::InvalidToken('%'))
    );
}

// 連続演算子の異常系
#[test]
fn consecutive_plus_operator_error() {
    assert_eq!(
        eval_expression("1++2"),
        Err(calc_core::CalcError::InvalidToken('+'))
    );
}

#[test]
fn consecutive_multiply_operator_error() {
    assert_eq!(
        eval_expression("1**2"),
        Err(calc_core::CalcError::InvalidToken('*'))
    );
}

#[test]
fn consecutive_remainder_operator_error() {
    assert_eq!(
        eval_expression("1%%2"),
        Err(calc_core::CalcError::InvalidToken('%'))
    );
}

#[test]
fn consecutive_mixed_plus_multiply_operator_error() {
    assert_eq!(
        eval_expression("1+*2"),
        Err(calc_core::CalcError::InvalidToken('*'))
    );
}

#[test]
fn consecutive_mixed_multiply_plus_operator_error() {
    assert_eq!(
        eval_expression("1*+2"),
        Err(calc_core::CalcError::InvalidToken('+'))
    );
}

#[test]
fn consecutive_mixed_minus_plus_operator_error() {
    assert_eq!(
        eval_expression("1-+2"),
        Err(calc_core::CalcError::InvalidToken('+'))
    );
}

#[test]
fn consecutive_mixed_remainder_plus_operator_error() {
    assert_eq!(
        eval_expression("1%+2"),
        Err(calc_core::CalcError::InvalidToken('+'))
    );
}

// 先頭演算子の異常系
#[test]
fn leading_plus_operator_error() {
    assert_eq!(
        eval_expression("+1"),
        Err(calc_core::CalcError::InvalidToken('+'))
    );
}

#[test]
fn leading_multiply_operator_error() {
    assert_eq!(
        eval_expression("*1"),
        Err(calc_core::CalcError::InvalidToken('*'))
    );
}

#[test]
fn leading_remainder_operator_error() {
    assert_eq!(
        eval_expression("%1"),
        Err(calc_core::CalcError::InvalidToken('%'))
    );
}

// 演算子のみの異常系
#[test]
fn minus_only_operator_error() {
    assert_eq!(
        eval_expression("-"),
        Err(calc_core::CalcError::InvalidToken('-'))
    );
}

// 単項マイナス
#[test]
fn unary_minus_literal() {
    assert_eq!(eval_expression("-1"), Ok(-1));
}

#[test]
fn unary_minus_in_addition() {
    assert_eq!(eval_expression("1+-2"), Ok(-1));
}

#[test]
fn unary_minus_in_subtraction() {
    assert_eq!(eval_expression("1--2"), Ok(3));
}

#[test]
fn unary_minus_in_multiplication() {
    assert_eq!(eval_expression("2*-3"), Ok(-6));
}
