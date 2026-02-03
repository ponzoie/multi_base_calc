use calc_core::{eval_expression, CalcError};

// 括弧の評価
#[test]
fn parentheses_override_precedence() {
    assert_eq!(eval_expression("(1+2)*3"), Ok(9));
}

#[test]
fn parentheses_with_whitespace() {
    assert_eq!(eval_expression(" ( 1 + 2 ) * 3 "), Ok(9));
}

#[test]
fn nested_parentheses() {
    assert_eq!(eval_expression("((1+2)*3)"), Ok(9));
}

// (数字) の許容
#[test]
fn parentheses_single_literal() {
    assert_eq!(eval_expression("(1)"), Ok(1));
}

// 括弧の不正パターン
#[test]
fn parentheses_empty_error() {
    assert_eq!(eval_expression("()"), Err(CalcError::InvalidToken(')')));
}

#[test]
fn parentheses_unclosed_error() {
    assert_eq!(eval_expression("(1+2"), Err(CalcError::InvalidToken(')')));
}

#[test]
fn parentheses_extra_closing_error() {
    assert_eq!(eval_expression("1+2)"), Err(CalcError::InvalidToken(')')));
}

#[test]
fn parentheses_inverted_error() {
    assert_eq!(eval_expression(")("), Err(CalcError::InvalidToken(')')));
}
