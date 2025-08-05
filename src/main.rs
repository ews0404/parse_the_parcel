use crate::packages::Package;
use crate::core_logic::{ShippingType, find_best_shipping};
use clap::Parser;
use config::Config;
use serde_derive::Deserialize;

mod packages;
mod parcel;
mod core_logic;

/// contains command line inputs
#[derive(Parser, Debug)]
#[command(version, about="\n\ndefault units are (mm, kg)", long_about = None)]
pub struct Cli {
    /// height of object
    #[clap(value_parser=f32_is_positive)]
    x: f32,

    /// width of object
    #[clap(value_parser=f32_is_positive)]
    y: f32,

    /// depth of object
    #[clap(value_parser=f32_is_positive)]
    z: f32,

    /// weight of object
    #[clap(value_parser=f32_is_positive)]
    w: f32,

    /// optional coversion factor from your length units to mm
    #[arg(long, value_parser=f32_is_positive)]
    to_mm: Option<f32>,

    /// optional coversion factor from your weight units to kg
    #[arg(long, value_parser=f32_is_positive)]
    to_kg: Option<f32>,
}

// used to ensure we get positive length/weight only
fn f32_is_positive(s: &str) -> Result<f32, String> {
    let x: f32 = s
        .parse()
        .map_err(|_| format!("'{}' is not a valid float", s))?; // this is a bit of copy-paste 
    if x >= 0.0 {
        Ok(x)
    } else {
        Err(format!("value {} cannot be negative!", x))
    }
}

/// holds configuration data from config.json
#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Configuration {
    max_weight: f32,
    packages: Vec<Package>,
}

fn main() {

    println!("\n\n");

    // get command line inputs
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    // build the configuration from config/config.json
    let config = Config::builder()
        .add_source(config::File::with_name("config/config.json"))
        .build()
        .unwrap();
    let conf: Configuration = config.try_deserialize().unwrap();
    println!("conf: {:?}", conf);

    // make a parcel with converted units and sorted dimensions
    let to_kg = match cli.to_kg {
        Some(x) => x,
        None => 1.0,
    };
    let to_mm = match cli.to_mm {
        Some(x) => x,
        None => 1.0,
    };
    let parcel = parcel::new(cli.x, cli.y, cli.z, cli.w, to_kg, to_mm);
    println!("parcel: {:?}", parcel);

    // get a list of available shipping packages from config file
    let packages = packages::build_from_config(&conf);

    // find best shipping method
    print!("result: ");
    match find_best_shipping(parcel, packages, conf.max_weight) {
        ShippingType::ItFits(name, shipping_cost) => {
            println!(
                "-> this parcel can ship in a {} container for ${:.2}\n",
                name, shipping_cost
            );
        }
        ShippingType::Overweight(w) => {
            println!(
                "cannot ship this parcel, {} kg is greater than max allowable weight of {} kg",
                w, conf.max_weight
            )
        }
        ShippingType::DoesntFit => {
            println!("Sorry, we don't have a package that fits the parcel!\n")
        }
    }

    println!("");

}



