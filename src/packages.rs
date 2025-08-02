pub struct Package{
    pub name:String,
    pub shipping_cost:f32,
    pub dimensions_mm:Vec<f32>
}

pub fn build() -> Vec<Package>{
    let mut packages = Vec::<Package>::new();
    packages.push(Package { name: "small".to_string(),  shipping_cost: 5.00, dimensions_mm: vec![200.0, 300.0, 150.0] });
    packages.push(Package { name: "medium".to_string(), shipping_cost: 7.50, dimensions_mm: vec![300.0, 400.0, 200.0] });
    packages.push(Package { name: "large".to_string(),  shipping_cost: 8.50, dimensions_mm: vec![400.0, 600.0, 250.0] });
    packages
}