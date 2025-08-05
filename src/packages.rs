use serde::Deserialize;
use crate::Configuration;

/// holds all information related to one size Package
#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub shipping_cost: f32,
    pub dimensions_mm: Vec<f32>,
}

/// build list of sorted packages from Configuration object
pub fn build_from_config(conf: &Configuration) -> Vec<Package> {
    
    let mut retval: Vec<Package> = Vec::new();

    for p in conf.packages.iter() {
        let mut q = Package {
            name: p.name.clone(),
            shipping_cost: p.shipping_cost,
            dimensions_mm: p.dimensions_mm.clone(),
        };
        q.dimensions_mm
            .sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        retval.push(q);
    }

    retval
}
