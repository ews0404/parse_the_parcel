use clap::Parser;
use std::iter::zip;

// settings
const MAX_WEIGHT_KG:f32 = 25.0;

// used to ensure we get positive length/weight only
fn f32_is_positive(s: &str) -> Result<f32, String> {
    let x: f32 = s.parse().map_err(|_| format!("'{}' is not a valid float", s))?; // this is a bit of copy-paste 
    if x >= 0.0 { 
         Ok(x)
    } else {
        Err(format!("value {} cannot be negative!", x))
    }
}

#[derive(Parser, Debug)]
#[command(version, about="\n\ndefault units are (mm, kg)", long_about = None)]
struct Cli{
    /// height of object
    #[clap(value_parser=f32_is_positive)]
    x:f32,

    /// width of object
    #[clap(value_parser=f32_is_positive)]
    y:f32,

    /// depth of object 
    #[clap(value_parser=f32_is_positive)]
    z:f32,

    /// weight of object
    #[clap(value_parser=f32_is_positive)]
    w:f32,

    /// optional coversion factor from your length units to mm
    #[arg(long, value_parser=f32_is_positive)]
    to_mm:Option<f32>,

    /// optional coversion factor from your weight units to kg
    #[arg(long, value_parser=f32_is_positive)]
    to_kg:Option<f32>
}

/// contains all info for one type of package
struct Package{
    name:String,
    shipping_cost:f32,
    dimensions_mm:Vec<f32>
}

fn main() {
    // parse input values
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    // make a parcel with sorted dimensions
    let mut parcel: Vec<f32>= vec![cli.x, cli.y, cli.z];
    parcel.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());

    // check for weight conversions
    let mut weight = cli.w;
    match cli.to_kg{
        Some(to_kg)=>{  weight *= to_kg; },
        None=>{}
    }

    // check for length conversions
    match cli.to_mm{
        Some(to_mm) => { for d in parcel.iter_mut() { (*d) = (*d) * to_mm; } },
        None => {}
    }

    println!("parcel: {:?} [mm], weight {} [kg]", parcel, weight);

    // reject if parcel weighs too much
    if weight>MAX_WEIGHT_KG { 
        println!("can't ship, {} kg is more than max weight of {} kg!", weight, MAX_WEIGHT_KG);
        return;
    }

    // make a vec of Packages with UNsorted dimensions [can't just declare this in vec! or sort dimensions in place?]
    let mut packages = Vec::<Package>::new();
    packages.push(Package { name: "small".to_string(),  shipping_cost: 5.00, dimensions_mm: vec![200.0, 300.0, 150.0] });
    packages.push(Package { name: "medium".to_string(), shipping_cost: 7.50, dimensions_mm: vec![300.0, 400.0, 200.0] });
    packages.push(Package { name: "large".to_string(),  shipping_cost: 8.50, dimensions_mm: vec![400.0, 600.0, 250.0] });

    // try to find the smallest package that fits
    for mut p in packages {
        // borrow checker says we need to sort package dimensions here(why?)
        p.dimensions_mm.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());  
        
        // make an iterator that acts on both parcel and Package dimensions
        let mut dims = zip(parcel.clone(), p.dimensions_mm);
        
        // check that the parcel fits into the Package in every dimension
        let it_fits = dims.all(|d| d.0<=d.1);

        // exit if we found a working solution, else try next Package
        if it_fits {
            println!("this parcel can ship in a {} container for ${:.2}", p.name, p.shipping_cost); 
            return; 
        }
    }
    
    // we did not find a working package
    println!("Sorry, we don't have a package that fits the parcel!")

}


