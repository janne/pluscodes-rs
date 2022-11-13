use std::error;

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

fn main() -> Result<(), Box<dyn error::Error>> {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode { length, latlon } => {
            let flattened: Vec<&str> = latlon
                .iter()
                .flat_map(|latlon| latlon.split(","))
                .filter(|latlon| !latlon.is_empty())
                .collect();
            if flattened.len() != 2 {
                return Err(pluscodes::Error::InvalidCoordinate(latlon).into());
            }
            let latitude = flattened[0].parse();
            let longitude = flattened[1].parse();
            if latitude.is_err() || longitude.is_err() {
                return Err(pluscodes::Error::InvalidCoordinate(latlon).into());
            }
            let coord = pluscodes::Coordinate {
                latitude: latitude.unwrap(),
                longitude: longitude.unwrap(),
            };
            let code = pluscodes::encode(&coord, length)?;
            println!("{}", code);
        }
        Opt::Decode { code } => {
            let coords = pluscodes::decode(&code)?;
            println!("{:.6},{:.6}", coords.latitude, coords.longitude);
        }
    }
    Ok(())
}
