use anyhow::{Context, Result};

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

fn main() -> Result<()> {
    let opt = Opt::from_args();

    match opt {
        Opt::Encode { length, latlon } => {
            let coord =
                pluscodes::parse_coordinate(latlon).context("Failed to parse coordinates")?;
            let code = pluscodes::encode(&coord, length).context("Failed to encode")?;
            println!("{}", code);
        }
        Opt::Decode { code } => {
            let coords = pluscodes::decode(&code).context("Failed to decode")?;
            println!("{:.6},{:.6}", coords.latitude, coords.longitude);
        }
    }
    Ok(())
}
