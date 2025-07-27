use clap::Parser;
use std::iter::zip;

// settings
const MAX_WEIGHT_KG:u32 = 25;


#[derive(Parser, Debug)]
#[command(version, about="\n\ndefault units are (mm, kg)", long_about = None)]
struct Cli{
    /// height of object
    x:u32,

    /// width of object
    y:u32,

    /// depth of object 
    z:u32,

    /// weight of object
    w:u32,

    /// optional length conversion to mm
    #[arg(long)]
    to_mm:Option<u32>,

    /// optional weight conversion to kg
    #[arg(long)]
    to_kg:Option<u32>
}

struct Package{
    name:String,
    cost:f32,
    dimensions:Vec<u32>
}

fn main() {

    // parse input values
    let cli = Cli::parse();
    println!("cli: {:?}", cli);

    // make a parcel with sorted dimensions
    let mut parcel: Vec<u32>= vec![cli.x, cli.y, cli.z];
    parcel.sort_unstable();

    // check for weight conversions
    let mut weight = cli.w;
    match cli.to_kg{
        Some(to_kg)=>{  weight *= to_kg; },
        None=>{}
    }

    // check for length conversions
    match cli.to_mm{
        Some(to_mm) => { for d in parcel.iter_mut() { *d *= to_mm; } },
        None => {}
    }

    println!("parcel: {:?}, weight {}", parcel, weight);

    // reject if parcel weighs too much
    if weight>MAX_WEIGHT_KG { 
        println!("can't ship, {} kg is more than max weight of {} kg!", weight, MAX_WEIGHT_KG);
        return;
    }

    // make a vec of Packages with UNsorted dimensions
    let mut packages = Vec::<Package>::new();
    packages.push(Package { name: "small".to_string(),  cost: 5.00, dimensions: vec![200, 300, 150] });
    packages.push(Package { name: "medium".to_string(), cost: 7.50, dimensions: vec![300 ,400, 200] });
    packages.push(Package { name: "large".to_string(),  cost: 8.50, dimensions: vec![400, 600, 250] });

    // try to find the smallest package that fits
    for mut p in packages {
        // borrow checker says we need to sort package dimensions here(why?)
        p.dimensions.sort_unstable();   
        
        // make an iterator that acts on both parcel and Package dimensions
        let mut dims = zip(parcel.clone(), p.dimensions);
        
        // check that the parcel fits into the Package in every dimension
        let it_fits = dims.all(|d| d.0<=d.1);

        // exit if we found a working solution, else try next Package
        if it_fits {
            println!("this parcel can ship in a {} container for ${:.2}", p.name, p.cost); 
            return; 
        }
    }
    
    // we did not find a working package
    println!("Sorry, we can't ship this!")

}


