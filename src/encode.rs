use crate::utils::{max, min, Coordinate, Error, DIGITS};

/// Encode a plus code, given latitude and longitude in a [Coordinate] struct
///
/// # Example
///  
/// ```
/// let coord = pluscodes::Coordinate { latitude: 59.335938, longitude: 18.077813 };
///
/// if let Ok(pluscode) = pluscodes::encode(&coord, 10) {
///   assert_eq!(pluscode, "9FFW83PH+94");
/// }
/// if let Ok(pluscode) = pluscodes::encode(&coord, 8) {
///   assert_eq!(pluscode, "9FFW83PH+");
/// }
/// if let Ok(pluscode) = pluscodes::encode(&coord, 4) {
///   assert_eq!(pluscode, "9FFW0000+");
/// }
///  ```
pub fn encode(coordinates: &Coordinate, length: usize) -> Result<String, Error> {
    if length < 2 || length > 10 || length % 2 != 0 {
        return Err(Error::InvalidLength(length));
    }

    let latitude = normalize_latitude(coordinates.latitude);
    let longitude = normalize_longitude(coordinates.longitude);

    let mut pluscode = interleave(
        encode_axis(length >> 1, latitude),
        encode_axis(length >> 1, longitude),
    );

    pluscode = format!("{:0<8}", pluscode);
    pluscode.insert(8, '+');

    Ok(pluscode)
}

fn normalize_latitude(lat: f64) -> f64 {
    min(180.0, max(0.0, lat + 90.0))
}

fn normalize_longitude(lon: f64) -> f64 {
    if (lon + 180.0) > 360.0 {
        return lon - 180.0;
    }
    lon + 180.0
}

fn value_to_digit(x: usize) -> char {
    DIGITS.chars().nth(x).unwrap()
}

fn digit_reducer(accumulator: Accumulator, _: usize) -> Accumulator {
    let q = (accumulator.value / accumulator.pos_value).floor();
    let mut result = accumulator.result;
    result.push(value_to_digit(q as usize));
    Accumulator {
        value: accumulator.value - (q * accumulator.pos_value) as f64,
        pos_value: accumulator.pos_value / 20.0,
        result,
    }
}

struct Accumulator {
    value: f64,
    pos_value: f64,
    result: Vec<char>,
}

fn encode_axis(length: usize, value: f64) -> Vec<char> {
    (0..length)
        .into_iter()
        .fold(
            Accumulator {
                value,
                pos_value: 20.0,
                result: vec![],
            },
            digit_reducer,
        )
        .result
}

fn interleave(xs: Vec<char>, ys: Vec<char>) -> String {
    let zipped: Vec<(&char, &char)> = xs.iter().zip(ys.iter()).collect();
    zipped.iter().flat_map(|(a, b)| vec![*a, *b]).collect()
}

#[cfg(test)]
mod tests {
    use crate::{
        encode::encode,
        utils::{Coordinate, Error},
    };

    #[test]
    fn it_returns_error_for_invalid_lengths() {
        let coord = Coordinate {
            latitude: 59.332438,
            longitude: 18.118813,
        };

        assert_eq!(encode(&coord, 0), Err(Error::InvalidLength(0)));
        assert_eq!(encode(&coord, 1), Err(Error::InvalidLength(1)));
        assert_eq!(encode(&coord, 3), Err(Error::InvalidLength(3)));
        assert_eq!(encode(&coord, 11), Err(Error::InvalidLength(11)));
    }

    #[test]
    fn it_encodes() {
        let coord = Coordinate {
            latitude: 59.332438,
            longitude: 18.118813,
        };

        if let Ok(pluscode) = encode(&coord, 10) {
            assert_eq!(pluscode, "9FFW84J9+XG");
        }
    }
}
