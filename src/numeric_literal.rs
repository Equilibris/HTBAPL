use crate::errors::BaseErr;

#[derive(Debug, PartialEq, Clone)]
pub enum NumericLiteral {
    Complex(f64, f64),
    Float(u8, f64),

    SysUint(u64),
    Uint(u8, u64),
    SysInt(i64),
    Int(u8, i64),

    Auto(f64),
    Boolean(bool),
}

fn parse_atomic_floating_point(s: &str) -> Result<f64, BaseErr<'static>> {
    let mut chars = s.chars();
    let sign = chars.next().unwrap();

    if sign == '¯' {
        match chars.as_str().parse::<f64>() {
            Ok(v) => Ok(-v),
            Err(_) => Err(BaseErr::new("invalid f64 float value")),
        }
    } else {
        match s.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(_) => Err(BaseErr::new("invalid f64 value")),
        }
    }
}

fn parse_atomic_integer(s: &str) -> Result<i64, BaseErr<'static>> {
    let mut chars = s.chars();
    let sign = chars.next().unwrap();

    if sign == '¯' {
        match chars.as_str().parse::<i64>() {
            Ok(v) => Ok(-v),
            Err(_) => Err(BaseErr::new("invalid i64 value")),
        }
    } else {
        match s.parse::<i64>() {
            Ok(v) => Ok(v),
            Err(_) => Err(BaseErr::new("invalid i64 value")),
        }
    }
}

fn parse_atomic_unsigned(s: &str) -> Result<u64, BaseErr<'static>> {
    match s.parse::<u64>() {
        Ok(v) => Ok(v),
        Err(_) => Err(BaseErr::new("invalid u64 value")),
    }
}

fn parse_exponentiated_float(s: &str) -> Result<f64, BaseErr<'static>> {
    Ok(if let Some((a, b)) = s.split_once('E') {
        let a = parse_atomic_floating_point(a)?;
        let b = parse_atomic_integer(b)?;
        a * 10f64.powi(b as i32)
    } else {
        parse_atomic_floating_point(s)?
    })
}
fn parse_exponentiated_unsigned(s: &str) -> Result<u64, BaseErr<'static>> {
    Ok(if let Some((a, b)) = s.split_once('E') {
        let a = parse_atomic_unsigned(a)?;
        let b = parse_atomic_unsigned(b)?;
        a * 10u64.pow(b as u32)
    } else {
        parse_atomic_unsigned(s)?
    })
}
fn parse_exponentiated_int(s: &str) -> Result<i64, BaseErr<'static>> {
    Ok(if let Some((a, b)) = s.split_once('E') {
        let a = parse_atomic_integer(a)?;
        let b = parse_atomic_unsigned(b)?;
        a * 10i64.pow(b as u32)
    } else {
        parse_atomic_integer(s)?
    })
}

enum ExtractSignatureAndVolumeResult {
    Auto,
    Signature(char),
    SignatureAndVolume(char, u8),
    Err(BaseErr<'static>),
}

fn extract_signature_and_volume_and_base(s: &str) -> (String, ExtractSignatureAndVolumeResult) {
    use ExtractSignatureAndVolumeResult::*;
    let mut signature = None;
    let mut volume = String::new();
    let mut return_string = String::new();

    for char in s.chars() {
        if let Some(_) = signature {
            volume.push(char);
        } else if char == 'b' {
            return (return_string, Signature('b'));
        } else if char == 'd' {
            return (return_string, SignatureAndVolume('f', 6));
        } else if char == 'h' {
            return (return_string, SignatureAndVolume('f', 4));
        } else if char == 'c' {
            return (return_string, SignatureAndVolume('c', 7));
        } else if char == 'f' {
            signature = Some('f');
        } else if char == 'i' {
            signature = Some('i');
        } else if char == 'u' {
            signature = Some('u');
        } else {
            return_string.push(char)
        }
    }

    if let Some(signature) = signature {
        return (
            return_string,
            if signature == 'f' && volume.len() == 0 {
                SignatureAndVolume('f', 5)
            } else if volume.len() == 0 {
                Signature(signature)
            } else {
                match volume.parse::<u8>() {
                    Ok(volume) => SignatureAndVolume(signature, volume),
                    _ => Err(BaseErr::new("Failed to parse volume")),
                }
            },
        );
    }
    // if (let Some('f') = signature ) && volume.len() == 0{}

    (return_string, Auto)
}

impl std::str::FromStr for NumericLiteral {
    type Err = BaseErr<'static>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((a, b)) = s.split_once('J') {
            Ok(NumericLiteral::Complex(
                parse_exponentiated_float(a)?,
                parse_exponentiated_float(b)?,
            ))
        } else {
            let (s, vol_sig_auto) = extract_signature_and_volume_and_base(s);

            match vol_sig_auto {
                ExtractSignatureAndVolumeResult::Auto => {
                    Ok(NumericLiteral::Auto(parse_exponentiated_float(s.as_str())?))
                }
                ExtractSignatureAndVolumeResult::Signature(signature) => match signature {
                    'u' => Ok(NumericLiteral::SysUint(parse_exponentiated_unsigned(
                        s.as_str(),
                    )?)),
                    'i' => Ok(NumericLiteral::SysInt(parse_exponentiated_int(s.as_str())?)),
                    'b' => Ok(NumericLiteral::Boolean(s != "0")),
                    _ => panic!("Un implemented literal"),
                },
                ExtractSignatureAndVolumeResult::SignatureAndVolume(signature, volume) => {
                    match signature {
                        'u' => Ok(NumericLiteral::Uint(
                            volume,
                            parse_exponentiated_unsigned(s.as_str())?,
                        )),
                        'i' => Ok(NumericLiteral::Int(
                            volume,
                            parse_exponentiated_int(s.as_str())?,
                        )),
                        'f' => Ok(NumericLiteral::Float(
                            volume,
                            parse_exponentiated_float(s.as_str())?,
                        )),
                        _ => panic!("Un implemented literal"),
                    }
                }
                ExtractSignatureAndVolumeResult::Err(e) => Err(e),
            }
        }
    }
}

impl ToString for NumericLiteral {
    fn to_string(&self) -> String {
        match *self {
            NumericLiteral::Complex(a, b) => format!("{}J{}", a, b),
            NumericLiteral::Float(size, n) => format!("{}f{}", n, size),
            NumericLiteral::SysUint(n) => format!("{}u", n),
            NumericLiteral::SysInt(n) => format!("{}i", n),
            NumericLiteral::Uint(size, n) => format!("{}u{}", n, size),
            NumericLiteral::Int(size, n) => format!("{}i{}", n, size),
            NumericLiteral::Auto(n) => format!("{}", n),
            NumericLiteral::Boolean(true) => "1b".to_string(),
            NumericLiteral::Boolean(false) => "0b".to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::numeric_literal::BaseErr;

    use super::NumericLiteral;
    #[test]
    fn it_parses_integer_complex() {
        assert_eq!(
            NumericLiteral::Complex(10.0, 10.0),
            "10J10".parse::<NumericLiteral>().unwrap()
        );

        assert_eq!(
            NumericLiteral::Complex(10.0, 10.0),
            "10.0J10.0".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Complex(10.1, -10.1),
            "10.1J¯10.1".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Complex(-10.123, -10.123),
            "¯10.123J¯10.123".parse::<NumericLiteral>().unwrap()
        );

        assert_eq!(
            NumericLiteral::Complex(10.0e2, 10.0),
            "10.0E2J10.0".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Complex(2345e-2, 2345.0),
            "2345E¯2J2345".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Complex(-2345e-2, 2345.0),
            "¯2345E¯2J2345".parse::<NumericLiteral>().unwrap()
        );
    }

    #[test]
    fn it_parses_simple_auto() {
        assert_eq!(
            NumericLiteral::Auto(1234.0),
            "1234".parse::<NumericLiteral>().unwrap()
        );
    }

    #[test]
    fn it_parses_simple_float() {
        assert_eq!(
            NumericLiteral::Float(5, 1234f64),
            "1234f".parse::<NumericLiteral>().unwrap()
        );

        assert_eq!(
            NumericLiteral::Float(5, 1234e2f64),
            "1234E2f".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Float(5, 1234e-2f64),
            "1234E¯2f".parse::<NumericLiteral>().unwrap()
        );

        assert_eq!(
            NumericLiteral::Float(4, 1234e-2f64),
            "1234E¯2f4".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Float(5, 1234e-2f64),
            "1234E¯2f5".parse::<NumericLiteral>().unwrap()
        );
    }

    #[test]
    fn it_parses_unsigned() {
        assert_eq!(
            NumericLiteral::Uint(4, 123400u64),
            "1234E2u4".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Uint(4, 12340u64),
            "1234E1u4".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Uint(3, 12330u64),
            "1233E1u3".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            BaseErr::new("invalid u64 value"),
            "1233E¯2u3".parse::<NumericLiteral>().unwrap_err()
        );
    }

    #[test]
    fn it_parses_ints() {
        assert_eq!(
            NumericLiteral::Int(4, 123400i64),
            "1234E2i4".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Int(4, 12340i64),
            "1234E1i4".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Int(3, 12330i64),
            "1233E1i3".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            BaseErr::new("invalid u64 value"),
            "1233E¯2i3".parse::<NumericLiteral>().unwrap_err()
        );
        assert_eq!(
            NumericLiteral::Int(4, -123400i64),
            "¯1234E2i4".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Int(4, -12340i64),
            "¯1234E1i4".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Int(3, -12330i64),
            "¯1233E1i3".parse::<NumericLiteral>().unwrap()
        );
    }
    #[test]
    fn it_parses_system_types() {
        assert_eq!(
            NumericLiteral::SysInt(123300i64),
            "1233E2i".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::SysUint(123300u64),
            "1233E2u".parse::<NumericLiteral>().unwrap()
        );
    }

    #[test]
    fn it_parses_bools() {
        assert_eq!(
            NumericLiteral::Boolean(true),
            "1b".parse::<NumericLiteral>().unwrap()
        );
        assert_eq!(
            NumericLiteral::Boolean(true),
            "2b".parse::<NumericLiteral>().unwrap()
        );

        assert_eq!(
            NumericLiteral::Boolean(false),
            "0b".parse::<NumericLiteral>().unwrap()
        );
    }
}
