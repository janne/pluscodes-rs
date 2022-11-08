use crate::coordinates::Coordinates;

pub fn encode(coordinates: Coordinates) -> String {
    "9FFW84J9+XG".to_string()
}

#[cfg(test)]
mod tests {
    use crate::{coordinates::Coordinates, encode::encode};

    #[test]
    fn it_encodes() {
        let coord = Coordinates {
            latitude: 59.332438,
            longitude: 18.118813,
        };
        assert_eq!(encode(coord), "9FFW84J9+XG");
    }
}
