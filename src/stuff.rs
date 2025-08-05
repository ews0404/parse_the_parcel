use crate::packages::Package;
use crate::parcel::Parcel;
use std::iter::zip;

pub enum ShippingType {
    ItFits(String, f32),
    Overweight(f32),
    DoesntFit,
}

pub fn find_best_shipping(parcel: Parcel, packages: Vec<Package>, max_weight: f32) -> ShippingType {
    // check for overweight packages
    if parcel.weight_kg > max_weight {
        return ShippingType::Overweight(parcel.weight_kg);
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
            return ShippingType::ItFits(p.name.clone(), p.shipping_cost);
        }
    }

    // no packages fit
    return ShippingType::DoesntFit;
}
