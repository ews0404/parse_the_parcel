/// contains sanitized parcel params, with sorted dimensions
#[derive(Debug)]
pub struct Parcel {
    pub dimensions_mm: Vec<f32>,
    pub weight_kg: f32,
}

/// returns Parcel with sorted and scaled dimensions / weight
pub fn new(x: f32, y: f32, z: f32, w: f32, to_kg: f32, to_mm: f32) -> Parcel {
    
    // sort the dimensions smallest -> largest
    let mut dims: Vec<f32> = vec![x, y, z];
    dims.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    // apply scaling factors
    for mm in dims.iter_mut() { (*mm) = (*mm) * to_mm; }
    let kg = w * to_kg;

    // outputs
    println!("parcel: {:?} [mm], weight {} [kg]", dims, w);
    Parcel {
        dimensions_mm:dims,
        weight_kg:kg
    }
}
