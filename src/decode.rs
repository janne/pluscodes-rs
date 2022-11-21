use regex::Regex;

use crate::utils::{Coordinate, Error, DIGITS};

/// Decode a plus code, returns a [Coordinate] struct
///
/// # Example
///  
/// ```
/// if let Ok(coord) = pluscodes::decode("9FFW83PH+94") {
///   assert_eq!(format!("{:.6}", coord.latitude), "59.335938");
///   assert_eq!(format!("{:.6}", coord.longitude), "18.077813");
/// }
///  ```
pub fn decode(code: &str) -> Result<Coordinate, Error> {
    if !is_valid_code(code) {
        return Err(Error::InvalidCode(String::from(code)));
    }

    let (lat, lon) = code
        .chars()
        .filter(|c| c != &'+')
        .enumerate()
        .fold((Vec::new(), Vec::new()), split_reducer);

    let res = resolution(code);
    let lat = decode_axis(lat) + res / 2.0 - 90.0;
    let lon = decode_axis(lon) + res / 2.0 - 180.0;

    Ok(Coordinate {
        latitude: lat,
        longitude: lon,
    })
}

fn decode_axis(axis: Vec<char>) -> f64 {
    let (result, _) = axis
        .iter()
        .map(|c| digit_to_value(c))
        .fold((0.0, 20.0), axis_reducer);

    result
}

fn digit_to_value(x: &char) -> usize {
    DIGITS.chars().position(|c| &c == x).unwrap()
}

fn axis_reducer(memo: (f64, f64), value: usize) -> (f64, f64) {
    let (result, pos_value) = memo;
    let result = result + pos_value * value as f64;
    let pos_value = pos_value / 20.0;
    (result, pos_value)
}

fn split_reducer(axes: (Vec<char>, Vec<char>), digit: (usize, char)) -> (Vec<char>, Vec<char>) {
    let (idx, c) = digit;
    let (mut lat, mut lon) = axes;

    if idx % 2 == 0 {
        lat.push(c);
    } else {
        lon.push(c)
    }

    return (lat, lon);
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

fn resolution(code: &str) -> f64 {
    let length = code.chars().filter(|c| c != &'+' && c != &'0').count();
    return 20.0 / (20_u32.pow((length as u32 / 2) - 1) as f64);
}

#[cfg(test)]
mod tests {
    use crate::{
        decode::decode,
        utils::{Coordinate, Error},
    };

    #[test]
    fn it_returns_none_for_invalid_codes() {
        assert_eq!(decode("foo"), Err(Error::InvalidCode("foo".to_string())));
    }

    #[test]
    fn it_returns_the_coordinate_for_valid_codes() {
        if let Ok(Coordinate {
            latitude,
            longitude,
        }) = decode("9FFW84J9+XG")
        {
            assert_eq!(format!("{:.6}", latitude), "59.332438");
            assert_eq!(format!("{:.6}", longitude), "18.118813");
        }
    }
}
