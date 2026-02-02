mod error;

pub use crate::error::{CalcError, CalcResult};

const MIN_I32: i64 = i32::MIN as i64;
const MAX_I32: i64 = i32::MAX as i64;

pub fn parse(input: &str) -> CalcResult<i64> {
    let bytes = input.as_bytes();
    let mut idx = 0;

    skip_ws(bytes, &mut idx);
    let start = idx;
    let value = parse_literal(input, bytes, &mut idx)?;
    skip_ws(bytes, &mut idx);
    if idx < bytes.len() {
        let ch = input[idx..].chars().next().unwrap_or('\0');
        if ch == '-' && is_hex_prefix(bytes, start) {
            return Err(CalcError::LiteralOutOfRange);
        }
        return Err(CalcError::InvalidToken(ch));
    }
    Ok(value)
}

pub fn eval_expression(input: &str) -> CalcResult<i64> {
    let bytes = input.as_bytes();
    let mut idx = 0;

    let value = parse_expression(input, bytes, &mut idx)?;
    skip_ws(bytes, &mut idx);
    if idx < bytes.len() {
        let ch = input[idx..].chars().next().unwrap_or('\0');
        return Err(CalcError::InvalidToken(ch));
    }
    Ok(value)
}

fn parse_expression(input: &str, bytes: &[u8], idx: &mut usize) -> CalcResult<i64> {
    let mut acc = parse_term(input, bytes, idx)?;
    acc = check_range(acc)?;

    loop {
        skip_ws(bytes, idx);
        if *idx >= bytes.len() {
            break;
        }

        match bytes[*idx] {
            b'+' => {
                *idx += 1;
                ensure_operand_after_operator('+', bytes, idx)?;
                let rhs = parse_term(input, bytes, idx)?;
                let sum = acc.checked_add(rhs).ok_or(CalcError::RangeError)?;
                acc = check_range(sum)?;
            }
            b'-' => {
                *idx += 1;
                ensure_operand_after_operator('-', bytes, idx)?;
                let rhs = parse_term(input, bytes, idx)?;
                let diff = acc.checked_sub(rhs).ok_or(CalcError::RangeError)?;
                acc = check_range(diff)?;
                if acc < 0 {
                    return Err(CalcError::RangeError);
                }
            }
            _ => break,
        }
    }

    Ok(acc)
}

fn parse_term(input: &str, bytes: &[u8], idx: &mut usize) -> CalcResult<i64> {
    let mut acc = parse_factor(input, bytes, idx)?;
    acc = check_range(acc)?;

    loop {
        skip_ws(bytes, idx);
        if *idx >= bytes.len() {
            break;
        }

        match bytes[*idx] {
            b'*' => {
                *idx += 1;
                ensure_operand_after_operator('*', bytes, idx)?;
                let rhs = parse_factor(input, bytes, idx)?;
                let product = acc.checked_mul(rhs).ok_or(CalcError::RangeError)?;
                acc = check_range(product)?;
            }
            b'%' => {
                *idx += 1;
                ensure_operand_after_operator('%', bytes, idx)?;
                let rhs = parse_factor(input, bytes, idx)?;
                if rhs == 0 {
                    return Err(CalcError::RangeError);
                }
                let rem = acc.rem_euclid(rhs);
                acc = check_range(rem)?;
            }
            _ => break,
        }
    }

    Ok(acc)
}

fn parse_factor(input: &str, bytes: &[u8], idx: &mut usize) -> CalcResult<i64> {
    skip_ws(bytes, idx);
    if *idx < bytes.len() && bytes[*idx] == b'-' {
        *idx += 1;
        ensure_operand_after_operator('-', bytes, idx)?;
        let value = parse_factor(input, bytes, idx)?;
        let negated = value.checked_neg().ok_or(CalcError::RangeError)?;
        return check_range(negated);
    }
    parse_literal(input, bytes, idx)
}

fn ensure_operand_after_operator(op: char, bytes: &[u8], idx: &mut usize) -> CalcResult<()> {
    skip_ws(bytes, idx);
    if *idx >= bytes.len() {
        return Err(CalcError::InvalidToken(op));
    }
    Ok(())
}

fn parse_literal(input: &str, bytes: &[u8], idx: &mut usize) -> CalcResult<i64> {
    skip_ws(bytes, idx);
    if *idx >= bytes.len() {
        return Err(CalcError::InvalidLiteral);
    }

    let value = if bytes[*idx] == b'0' && matches!(bytes.get(*idx + 1), Some(b'b') | Some(b'B')) {
        *idx += 2;
        let start = *idx;
        while *idx < bytes.len() {
            let b = bytes[*idx];
            if b == b'0' || b == b'1' || b == b'_' {
                *idx += 1;
            } else if b.is_ascii_alphanumeric() {
                return Err(CalcError::InvalidLiteral);
            } else {
                break;
            }
        }

        if *idx == start {
            return Err(CalcError::InvalidLiteral);
        }

        let token = &input[start..*idx];
        parse_binary(token)?
    } else if bytes[*idx] == b'0' && matches!(bytes.get(*idx + 1), Some(b'x') | Some(b'X')) {
        *idx += 2;
        let start = *idx;
        while *idx < bytes.len() {
            let b = bytes[*idx];
            if b.is_ascii_hexdigit() || b == b'_' {
                *idx += 1;
            } else if b == b'-' || b.is_ascii_alphanumeric() {
                return Err(CalcError::InvalidLiteral);
            } else {
                break;
            }
        }

        if *idx == start {
            return Err(CalcError::InvalidLiteral);
        }

        let token = &input[start..*idx];
        parse_hex(token)?
    } else {
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
        parse_decimal(token)?
    };
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

fn parse_binary(token: &str) -> CalcResult<i64> {
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

        let digit = match ch {
            '0' => 0,
            '1' => 1,
            _ => return Err(CalcError::InvalidLiteral),
        };
        seen_digit = true;
        prev_underscore = false;

        value = value
            .checked_mul(2)
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

fn parse_hex(token: &str) -> CalcResult<i64> {
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

        let digit = ch.to_digit(16).ok_or(CalcError::InvalidLiteral)? as i64;
        seen_digit = true;
        prev_underscore = false;

        value = value
            .checked_mul(16)
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

fn is_hex_prefix(bytes: &[u8], start: usize) -> bool {
    bytes.get(start) == Some(&b'0') && matches!(bytes.get(start + 1), Some(b'x') | Some(b'X'))
}

fn skip_ws(bytes: &[u8], idx: &mut usize) {
    while *idx < bytes.len() {
        match bytes[*idx] {
            b' ' | b'\t' | b'\n' | b'\r' => *idx += 1,
            _ => break,
        }
    }
}
