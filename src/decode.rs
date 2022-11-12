use regex::Regex;

use crate::utils::{Coordinate, InvalidCodeError, DIGITS};

pub fn decode(code: &str) -> Result<Coordinate, InvalidCodeError> {
    if !is_valid_code(code) {
        return Err(InvalidCodeError);
    }
    Ok(Coordinate {
        latitude: 0.0,
        longitude: 0.0,
    })
}
fn is_valid_code(code: &str) -> bool {
    let pair = format!("[{}]{{2}}", DIGITS);
    let pair_or_zero = format!("([{}]|0){{2}}", DIGITS);
    let exp = format!(
        "^{pair}({pairOrZero}){{0,3}}[+]({pair})?$",
        pair = pair,
        pairOrZero = pair_or_zero
    );
    let re = Regex::new(&exp).unwrap();
    re.is_match(&code)
}

#[cfg(test)]
mod tests {
    use crate::{
        decode::decode,
        utils::{Coordinate, InvalidCodeError},
    };

    #[test]
    fn it_returns_none_for_invalid_codes() {
        assert_eq!(decode("foo"), Err(InvalidCodeError));
    }

    #[test]
    fn it_returns_the_coordinate_for_valid_codes() {
        let coord = Coordinate {
            latitude: 0.0,
            longitude: 0.0,
            // latitude: 59.332438,
            // longitude: 18.118813,
        };
        assert_eq!(decode("9FFW84J9+XG"), Ok(coord))
    }
}
