use std::{fs::File, io::BufReader};

use obj::load_obj;

pub fn load_obj_model(path: &str) -> Result<obj::Obj, Box<dyn std::error::Error>> {
    let buffer = BufReader::new(File::open(path)?);
    let model = load_obj(buffer)?;
    Ok(model)
}
