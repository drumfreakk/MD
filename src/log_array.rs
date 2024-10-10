
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

/**
 * Prints a hashmap with stored data values to a csv file
 *	Hashmap keys: column names
 *	Hashmap values: Vec<(f64, f64)>
 *		The first value in the tuple is time
 *		The second value is the actual value
 * 	All columns should have the same length and same time steps (TODO: make this better)
 *
 * @param &HashMap<&str, Vec<(f64, f64)>> data
 * @param &str filename
 * @return std::io::Result<()>
 */
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
