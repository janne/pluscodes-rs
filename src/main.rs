fn usage() {
    println!("pluscodes");
}

fn main() {
    usage();
    let coord = pluscodes::Coordinate {
        latitude: 59.332438,
        longitude: 18.118813,
    };
    if let Some(code) = pluscodes::encode(&coord, 10) {
        println!("{}", code);
    }
}
