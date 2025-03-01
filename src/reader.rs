use serde::Serialize;
use std::error::Error;
use std::fs::File;

use serde_json::Value;

pub fn write_to_file<T>(path: &str, data: Vec<T>) -> std::io::Result<()>
where
    T: Serialize,
{
    let data: Vec<Value> = data
        .iter()
        .map(|s| serde_json::to_value(s).unwrap())
        .collect();

    let mut file = File::create(path)?;
    serde_json::to_writer(&mut file, &data)?;

    Ok(())
}

pub fn read_file(path: &str) -> Result<Vec<Value>, Box<dyn Error>> {
    let file = File::open(path)?;
    let data: Vec<Value> = serde_json::from_reader(file)?;

    Ok(data)
}
