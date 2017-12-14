extern crate csv;
use std::fs::File;
use std::iter::Skip;
use std::io::Lines;
use std::iter::Filter;
use std::rc::Rc;
use std::collections::BTreeMap;
use std::path::Path;
use error::Error;

pub struct Shape {
    pub shape_pt_lat : f64,
    pub shape_pt_lon : f64,
    pub shape_pt_sequence : u32,
    pub shape_dist_traveled : String
}

#[derive(Debug, Deserialize)]
pub struct ShapeCsv {
    shape_id: String,
    shape_pt_lat : f64,
    shape_pt_lon : f64,
    shape_pt_sequence : u32,
    shape_dist_traveled : String
}

impl Shape {
    pub fn make_shapes(shapes_path : &Path) -> Result<BTreeMap<String, Vec<Shape>>, Error> {
        let file = File::open(shapes_path)?;
        let mut reader = csv::Reader::from_reader(file);

        let mut map : BTreeMap<String, Vec<Shape>> = BTreeMap::new();

        for record in reader.deserialize() {
            let row: ShapeCsv = record?;

            let mut list = map.entry(row.shape_id).or_insert(vec![]);
            list.push(Shape {
                shape_pt_lat: row.shape_pt_lat,
                shape_pt_lon: row.shape_pt_lon,
                shape_pt_sequence: row.shape_pt_sequence,
                shape_dist_traveled: row.shape_dist_traveled,
            });
        }
        println!("Finished reading shapes");
        Ok(map)
    }
}
