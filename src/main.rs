use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Encode and parse plus codes (https://plus.codes/)")]
enum Opt {
    Encode {
        #[structopt(long, default_value = "10")]
        length: usize,
        latitude: String,
        longitude: String,
    },
    Decode {
        code: String,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode {
            length,
            latitude,
            longitude,
        } => {
            let coord = pluscodes::Coordinate {
                latitude: latitude.parse().unwrap(),
                longitude: longitude.parse().unwrap(),
            };
            if let Some(code) = pluscodes::encode(&coord, length) {
                println!("{}", code);
            }
        }
        Opt::Decode { code } => {
            if let Some(coords) = pluscodes::decode(&code) {
                println!(
                    "Latitude: {}, Longitude: {}",
                    coords.latitude, coords.longitude
                );
            }
        }
    }
}
