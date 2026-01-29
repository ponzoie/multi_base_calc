mod error;

pub use crate::error::{CalcError, CalcResult};

const MIN_I32: i64 = i32::MIN as i64;
const MAX_I32: i64 = i32::MAX as i64;

pub fn parse(input: &str) -> CalcResult<i64> {
    let bytes = input.as_bytes();
    let mut idx = 0;

    let value = parse_literal(input, bytes, &mut idx)?;
    skip_ws(bytes, &mut idx);
    if idx < bytes.len() {
        let ch = input[idx..].chars().next().unwrap_or('\0');
        return Err(CalcError::InvalidToken(ch));
    }
    Ok(value)
}

pub fn eval_expression(input: &str) -> CalcResult<i64> {
    let bytes = input.as_bytes();
    let mut idx = 0;

    skip_ws(bytes, &mut idx);
    let mut acc = parse_literal(input, bytes, &mut idx)?;
    acc = check_range(acc)?;

    loop {
        skip_ws(bytes, &mut idx);
        if idx >= bytes.len() {
            break;
        }

        match bytes[idx] {
            b'+' => {
                idx += 1;
                let rhs = parse_literal(input, bytes, &mut idx)?;
                let sum = acc.checked_add(rhs).ok_or(CalcError::RangeError)?;
                acc = check_range(sum)?;
            }
            _ => {
                let ch = input[idx..].chars().next().unwrap_or('\0');
                return Err(CalcError::InvalidToken(ch));
            }
        }
    }

    Ok(acc)
}

fn parse_literal(input: &str, bytes: &[u8], idx: &mut usize) -> CalcResult<i64> {
    skip_ws(bytes, idx);
    if *idx >= bytes.len() {
        return Err(CalcError::InvalidLiteral);
    }

    let start = *idx;
    while *idx < bytes.len() {
        let b = bytes[*idx];
        if b.is_ascii_digit() || b == b'_' {
            *idx += 1;
        } else {
            break;
        }
    }

    if *idx == start {
        let ch = input[*idx..].chars().next().unwrap_or('\0');
        return Err(CalcError::InvalidToken(ch));
    }

    let token = &input[start..*idx];
    let value = parse_decimal(token)?;
    if value > MAX_I32 {
        return Err(CalcError::LiteralOutOfRange);
    }
    Ok(value)
}

fn parse_decimal(token: &str) -> CalcResult<i64> {
    let mut value: i64 = 0;
    let mut seen_digit = false;
    let mut prev_underscore = false;

    for ch in token.chars() {
        if ch == '_' {
            if !seen_digit || prev_underscore {
                return Err(CalcError::InvalidLiteral);
            }
            prev_underscore = true;
            continue;
        }

        let digit = ch.to_digit(10).ok_or(CalcError::InvalidLiteral)? as i64;
        seen_digit = true;
        prev_underscore = false;

        value = value
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
            .ok_or(CalcError::LiteralOutOfRange)?;

        if value > MAX_I32 {
            return Err(CalcError::LiteralOutOfRange);
        }
    }

    if !seen_digit || prev_underscore {
        return Err(CalcError::InvalidLiteral);
    }

    Ok(value)
}

fn check_range(value: i64) -> CalcResult<i64> {
    if value < MIN_I32 || value > MAX_I32 {
        Err(CalcError::RangeError)
    } else {
        Ok(value)
    }
}

fn skip_ws(bytes: &[u8], idx: &mut usize) {
    while *idx < bytes.len() {
        match bytes[*idx] {
            b' ' | b'\t' | b'\n' | b'\r' => *idx += 1,
            _ => break,
        }
    }
}
