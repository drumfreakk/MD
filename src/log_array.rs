
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

pub fn log_array(data: &HashMap<&str, Vec<(f64,f64)>>, filename: &str) -> std::io::Result<()> {
	let mut file = File::create(filename)?;

	let keys = data.keys();

	let mut line = String::from("t");
	for key in keys.clone() {
		line.push(',');
		line.push_str(&key.to_string());
	}
	line.push('\n');
	file.write_all(line.as_bytes())?;
	
	for i in 0..data["pos0"].len() {
		let mut line = String::from(&data["pos0"][i].0.to_string());
		for key in keys.clone() {
			line.push(',');
			line.push_str(&data[key][i].1.to_string());
		}
		line.push('\n');
		file.write_all(line.as_bytes())?;
	}
	
	Ok(())
}
