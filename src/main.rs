use std::error::Error;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Encode and parse plus codes (https://plus.codes/)")]
enum Opt {
    Encode {
        #[structopt(long, default_value = "10")]
        length: usize,

        #[structopt(help = "latitude,longitude")]
        latlon: Vec<String>,
    },
    Decode {
        code: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode { length, latlon } => {
            let latlon: Vec<&str> = latlon
                .iter()
                .flat_map(|latlon| latlon.split(","))
                .filter(|latlon| !latlon.is_empty())
                .collect();
            let latitude = latlon[0];
            let longitude = latlon[1];
            let coord = pluscodes::Coordinate {
                latitude: latitude.parse().unwrap(),
                longitude: longitude.parse().unwrap(),
            };
            let code = pluscodes::encode(&coord, length)?;
            println!("{}", code);
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
    Ok(())
}
