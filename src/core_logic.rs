use crate::packages::Package;
use crate::parcel::Parcel;
use std::iter::zip;

#[derive (Debug, PartialEq)]
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


#[cfg(test)]
mod tests{
    use std::vec;

    use super::*;
    use crate::parcel::Parcel;

    // check for overweight parcel
    #[test]
    fn test_overweight(){
        let max_weight = 100.0;
        let parcel = Parcel{
            dimensions_mm:vec!(100.0, 100.0, 100.0), 
            weight_kg:max_weight+1.0
        };
        let mut packages:Vec<Package> = Vec::new();
        packages.push(Package { name: "test".to_string(), shipping_cost: 1.23, dimensions_mm: parcel.dimensions_mm.clone() });

        assert_eq!(find_best_shipping(parcel, packages, max_weight), ShippingType::Overweight(max_weight+1.0));
    }

    #[test]
    fn test_it_fits(){
        // todo: test case for where a package fits
        assert!(true);
    }

    #[test]
    fn test_doesnt_fit(){
        // todo: test case for when no package fits
        assert!(true);
    }
}

