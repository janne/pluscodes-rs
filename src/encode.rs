use crate::utils::{max, min, Coordinate, DIGITS};

pub fn encode(coordinates: &Coordinate, length: usize) -> Option<String> {
    if length < 2 || length > 10 || length % 2 != 0 {
        return None;
    }

    let latitude = normalize_latitude(coordinates.latitude);
    let longitude = normalize_longitude(coordinates.longitude);

    let mut pluscode = interleave(
        encode_axis(length >> 1, latitude),
        encode_axis(length >> 1, longitude),
    );

    pluscode = format!("{:0<8}", pluscode);
    pluscode.insert(8, '+');

    Some(pluscode)
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
    use crate::{encode::encode, utils::Coordinate};

    #[test]
    fn it_returns_none_for_invalid_lengths() {
        let coord = Coordinate {
            latitude: 59.332438,
            longitude: 18.118813,
        };

        assert_eq!(encode(&coord, 0), None);
        assert_eq!(encode(&coord, 1), None);
        assert_eq!(encode(&coord, 3), None);
        assert_eq!(encode(&coord, 11), None);
    }

    #[test]
    fn it_encodes() {
        let coord = Coordinate {
            latitude: 59.332438,
            longitude: 18.118813,
        };

        if let Some(pluscode) = encode(&coord, 10) {
            assert_eq!(pluscode, "9FFW84J9+XG");
        }
    }
}
