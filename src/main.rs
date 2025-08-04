use clap::Parser;
use std::iter::zip;
use config::Config;
use std::collections::HashMap;

use crate::package::Package;

mod package;
mod parcel;

const MAX_WEIGHT_KG: f32 = 25.0;

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

fn main() {

    /* 
    // get command line inputs
    let cli = Cli::parse();
    println!("\n{:?}", cli);

    let to_kg = match cli.to_kg {
        Some(x) => x,
        None => 1.0,
    };

    let to_mm = match cli.to_mm {
        Some(x) => x,
        None => 1.0,
    };

    // make a parcel with sorted dimensions
    let parcel = parcel::new(cli.x, cli.y, cli.z, cli.w, to_kg, to_mm);
*/

    let config = Config::builder()
        .add_source(config::File::with_name("config/config.json"))
        .build()
        .unwrap();
    println!("\n{:?}\n", config);

    let max_weight:f32 = config.get("max_weight").unwrap();
    println!("max_weight: {}\n", max_weight);


    let y = config.get_array("test_arr").unwrap();
    println!("y: {:?}\n", y);



    let x = config.get_array("packages").unwrap();
    println!("x: {:?}", x);

    // for p in config.get_array("packages").unwrap() {
    //     println!("p: {:?}", p);
    // }

    /* 
    // get a list of available shipping packages
    let packages = packages::build();

    // reject if parcel weighs too much
    if parcel.weight_kg > MAX_WEIGHT_KG {
        println!(
            "cannot ship this parcel, {} kg is greater than max allowable weight of {} kg",
            parcel.weight_kg, MAX_WEIGHT_KG
        );
        return;
    }

    // try to find the smallest package that fits
    for mut p in packages {
        // borrow checker says we need to sort package dimensions here(why?)
        p.dimensions_mm
            .sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        // make an iterator that acts on both parcel and Package dimensions
        let mut dims = zip(parcel.dimensions_mm.clone(), p.dimensions_mm);

        // check that the parcel fits into the Package in every dimension
        let it_fits = dims.all(|d| d.0 <= d.1);

        // exit if we found a working solution, else try next Package
        if it_fits {
            println!(
                "this parcel can ship in a {} container for ${:.2}\n",
                p.name, p.shipping_cost
            );
            return;
        }
    }

    // we did not find a working package
    println!("Sorry, we don't have a package that fits the parcel!\n")
    */
}
