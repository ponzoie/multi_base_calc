use calc_core::{format_all, CalcError, FormattedValue};

// フォーマット
#[test]
fn format_zero() {
    assert_eq!(
        format_all(0),
        Ok(FormattedValue {
            bin: "0b0".to_string(),
            dec: "0".to_string(),
            hex: "0x0".to_string(),
        })
    );
}

#[test]
fn format_positive_value() {
    assert_eq!(
        format_all(26),
        Ok(FormattedValue {
            bin: "0b1_1010".to_string(),
            dec: "26".to_string(),
            hex: "0x1A".to_string(),
        })
    );
}

#[test]
fn format_negative_value() {
    assert_eq!(
        format_all(-26),
        Ok(FormattedValue {
            bin: "-0b1_1010".to_string(),
            dec: "-26".to_string(),
            hex: "-0x1A".to_string(),
        })
    );
}

#[test]
fn format_out_of_range_error() {
    assert_eq!(format_all(2147483648), Err(CalcError::RangeError));
}
