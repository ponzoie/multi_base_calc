mod error;

pub use crate::error::{CalcError, CalcResult};

const MIN_I32: i64 = i32::MIN as i64;
const MAX_I32: i64 = i32::MAX as i64;
const MAX_I32_PLUS_ONE: i64 = (i32::MAX as i64) + 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormattedValue {
    pub bin: String,
    pub dec: String,
    pub hex: String,
}

pub fn format_all(value: i64) -> CalcResult<FormattedValue> {
    let value = check_range(value)?;
    Ok(FormattedValue {
        bin: format_binary(value),
        dec: value.to_string(),
        hex: format_hex_string(value),
    })
}

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
    parse_factor_with_literal_max(input, bytes, idx, MAX_I32)
}

fn parse_factor_with_literal_max(
    input: &str,
    bytes: &[u8],
    idx: &mut usize,
    literal_max: i64,
) -> CalcResult<i64> {
    skip_ws(bytes, idx);
    if *idx >= bytes.len() {
        return Err(CalcError::InvalidLiteral);
    }
    if *idx < bytes.len() && bytes[*idx] == b'-' {
        *idx += 1;
        ensure_operand_after_operator('-', bytes, idx)?;
        let value = parse_factor_with_literal_max(input, bytes, idx, MAX_I32_PLUS_ONE)?;
        let negated = value.checked_neg().ok_or(CalcError::RangeError)?;
        return check_range(negated);
    }
    if bytes[*idx] == b'(' {
        *idx += 1;
        skip_ws(bytes, idx);
        if *idx >= bytes.len() {
            return Err(CalcError::InvalidToken(')'));
        }
        if bytes[*idx] == b')' {
            return Err(CalcError::InvalidToken(')'));
        }
        let value = parse_expression(input, bytes, idx)?;
        skip_ws(bytes, idx);
        if *idx >= bytes.len() || bytes[*idx] != b')' {
            return Err(CalcError::InvalidToken(')'));
        }
        *idx += 1;
        return Ok(value);
    }
    parse_literal_with_max(input, bytes, idx, literal_max)
}

fn ensure_operand_after_operator(op: char, bytes: &[u8], idx: &mut usize) -> CalcResult<()> {
    skip_ws(bytes, idx);
    if *idx >= bytes.len() {
        return Err(CalcError::InvalidToken(op));
    }
    Ok(())
}

fn parse_literal(input: &str, bytes: &[u8], idx: &mut usize) -> CalcResult<i64> {
    parse_literal_with_max(input, bytes, idx, MAX_I32)
}

fn parse_literal_with_max(
    input: &str,
    bytes: &[u8],
    idx: &mut usize,
    literal_max: i64,
) -> CalcResult<i64> {
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
        parse_binary(token, literal_max)?
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
        parse_hex(token, literal_max)?
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
        parse_decimal(token, literal_max)?
    };
    if value > literal_max {
        return Err(CalcError::LiteralOutOfRange);
    }
    Ok(value)
}

fn parse_decimal(token: &str, literal_max: i64) -> CalcResult<i64> {
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

        if value > literal_max {
            return Err(CalcError::LiteralOutOfRange);
        }
    }

    if !seen_digit || prev_underscore {
        return Err(CalcError::InvalidLiteral);
    }

    Ok(value)
}

fn parse_binary(token: &str, literal_max: i64) -> CalcResult<i64> {
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

        if value > literal_max {
            return Err(CalcError::LiteralOutOfRange);
        }
    }

    if !seen_digit || prev_underscore {
        return Err(CalcError::InvalidLiteral);
    }

    Ok(value)
}

fn parse_hex(token: &str, literal_max: i64) -> CalcResult<i64> {
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

        if value > literal_max {
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

fn format_binary(value: i64) -> String {
    let (sign, mut n) = if value < 0 {
        ("-", -value)
    } else {
        ("", value)
    };

    if n == 0 {
        return format!("{sign}0b0");
    }

    let mut bits = String::new();
    while n > 0 {
        bits.push(if n & 1 == 1 { '1' } else { '0' });
        n >>= 1;
    }
    let bits: String = bits.chars().rev().collect();
    let grouped = group_from_right(&bits, 4, '_');
    format!("{sign}0b{grouped}")
}

fn format_hex_string(value: i64) -> String {
    let (sign, n) = if value < 0 {
        ("-", -value)
    } else {
        ("", value)
    };
    if n == 0 {
        return format!("{sign}0x0");
    }
    format!("{sign}0x{:X}", n)
}

fn group_from_right(input: &str, group: usize, sep: char) -> String {
    if input.len() <= group {
        return input.to_string();
    }

    let mut out = String::new();
    let mut count = 0;
    for ch in input.chars().rev() {
        if count == group {
            out.push(sep);
            count = 0;
        }
        out.push(ch);
        count += 1;
    }
    out.chars().rev().collect()
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
