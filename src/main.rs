use save::Save;
use std::fs;

use serde_xml_rs::from_reader;

fn main() {
    println!("Start");
    let planet_file = fs::File::open("planet.plnt").unwrap();
    println!("Opened");
    let planet: Save = from_reader(planet_file).unwrap();
    println!("Deserialized!");
}
