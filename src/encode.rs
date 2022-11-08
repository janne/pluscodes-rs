use crate::coordinates::Coordinates;

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        return a;
    }
    b
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        return a;
    }
    b
}

const DIGITS: &'static str = "23456789CFGHJMPQRVWX";

fn digit_to_value(x: char) -> usize {
    DIGITS.chars().position(|c| c == x).unwrap()
}

fn value_to_digit(x: usize) -> char {
    if x >= DIGITS.len() {
        print!("Invalid value: {}", x);
        panic!();
    }
    DIGITS.chars().nth(x).unwrap()
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

fn digit_reducer(accumulator: Accumulator, _: usize) -> Accumulator {
    println!(
        "Acc value: {}, pos: {}",
        accumulator.value, accumulator.pos_value
    );
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

fn interleave(length: usize, xs: Vec<char>, ys: Vec<char>) -> String {
    let zipped: Vec<(&char, &char)> = xs.iter().zip(ys.iter()).collect();
    let flattened: String = zipped.iter().flat_map(|(a, b)| vec![*a, *b]).collect();

    //   const padding = length > 8 ? [] : arrayOf(8 - length, '0')
    //   const digits = [...flatten(zipped), ...padding]

    //   return [...digits.slice(0, 8), '+', ...digits.slice(8)].join('')
    return flattened;
}

pub fn encode(coordinates: &Coordinates, length: usize) -> Option<String> {
    if length < 2 || length > 10 || length % 2 != 0 {
        return None;
    }

    let latitude = normalize_latitude(coordinates.latitude);
    let longitude = normalize_longitude(coordinates.longitude);

    return Some(interleave(
        length,
        encode_axis(length >> 1, latitude),
        encode_axis(length >> 1, longitude),
    ));

    // Some("9FFW84J9+XG".to_string())
}

#[cfg(test)]
mod tests {
    use crate::{coordinates::Coordinates, encode::encode};

    #[test]
    fn it_returns_none_for_invalid_lengths() {
        let coord = Coordinates {
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
        let coord = Coordinates {
            latitude: 59.332438,
            longitude: 18.118813,
        };

        if let Some(pluscode) = encode(&coord, 10) {
            assert_eq!(pluscode, "9FFW84J9+XG");
        }
    }
}
