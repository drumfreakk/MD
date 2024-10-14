
use crate::Vector;

use std::collections::HashMap;
use std::iter::{zip, Zip};
use std::vec::IntoIter;
use std::fs::File;
use std::io::prelude::*;

pub struct LinearData<'a> {
	pub map: HashMap::<&'a str, Vec::<f64>>
}

pub struct ParticleData<'a> {
	pub map: HashMap::<&'a str, Vec::<Vec::<f64>>>,
	particles: usize
}

pub struct ParticleVectorData<'a> {
	pub map: HashMap::<&'a str, Vec::<Vec::<Vector>>>,
	particles: usize
}


pub struct DataLog<'a> {
	pub time: Vec::<f64>,
	pub global: LinearData<'a>,
	pub particle: ParticleData<'a>,
	pub particle_vector: ParticleVectorData<'a>,
}

impl<'a> DataLog<'a> {
	pub fn new(particles: usize) -> Self {
		DataLog{
			time: Vec::new(),
			global: LinearData::new(),
			particle: ParticleData::new(particles),
			particle_vector: ParticleVectorData::new(particles),
		}
	}

	pub fn global_as_iter(&self, name: &str) -> Zip<IntoIter<f64>, IntoIter<f64>> {
		zip(self.time.clone(), self.global.get(name).clone())
	}
	
	pub fn particle_as_iter(&self, name: &str, index: usize) -> Zip<IntoIter<f64>, IntoIter<f64>> {
		zip(self.time.clone(), self.particle.get(name)[index].clone())
	}
	
	pub fn particle_vector_as_iter(&self, name: &str, index: usize) -> Zip<IntoIter<f64>, IntoIter<Vector>> {
		zip(self.time.clone(), self.particle_vector.get(name)[index].clone())
	}

	pub fn add_particle_series(&mut self, name: &'a str) {
		self.particle.add_series(name);
		self.global.add_series(name);
	}
	
	pub fn insert_particle_add(&mut self, name: &str, index: usize, value: f64) {
		self.particle.insert_into(name, index, value);
		let l = self.global.get(name).len();
		if l < self.particle.get(name)[index].len() {
			// If there isnt a value for the global part yet
			self.global.insert_into(name, value);
		} else {
			self.global.map.get_mut(name).expect("Invalid key")[l - 1] += value;
		}
	}

	pub fn add_particle_vector_series(&mut self, name: &'a str) {
		self.particle_vector.add_series(name);
		self.particle.add_series(name);
	}

	pub fn insert_particle_vector_len(&mut self, name: &str, index: usize, value: Vector) {
		self.particle_vector.insert_into(name, index, value);
		self.particle.insert_into(name, index, value.len());
	}

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
	pub fn to_file(&self, filename: &str) -> std::io::Result<()> {
		let mut file = File::create(filename)?;

		let k_global = self.global.map.keys();
		let k_particle = self.particle.map.keys();
		let k_vector = self.particle_vector.map.keys();

		// Write the header line
		let mut line = String::from("t");
		for k in k_global.clone() {
			line.push(',');
			line.push_str(&k.to_string());
		}
//		for k in k_global.clone() {
//			for i in 0..self.global.particles{
//				line.push(',');
//				line.push_str(concat!(&k.to_string(), "_", i));
//			}
//		}
		line.push('\n');
		file.write_all(line.as_bytes())?;

//	
//		let keys = data.keys();
//	
//		let mut line = String::from("t");
//		for key in keys.clone() {
//			line.push(',');
//			line.push_str(&key.to_string());
//		}
//		line.push('\n');
//		file.write_all(line.as_bytes())?;
//		
//		for i in 0..data["pos0"].len() {
//			let mut line = String::from(&data["pos0"][i].map.to_string());
//			for key in keys.clone() {
//				line.push(',');
//				line.push_str(&data[key][i].1.to_string());
//			}
//			line.push('\n');
//			file.write_all(line.as_bytes())?;
//		}
		
		Ok(())
	}
}


impl<'a> LinearData<'a> {
	pub fn new() -> Self {
		LinearData{
			map: HashMap::new()
		}
	}

	pub fn add_series(&mut self, name: &'a str) -> Option<Vec::<f64>> {
		self.map.insert(name, Vec::new())
	}

	pub fn insert_into(&mut self, name: &str, value: f64) {
		self.map.get_mut(name).expect("Invalid key").push(value)
	}

	pub fn get(&self, name: &str) -> &Vec::<f64> {
		self.map.get(name).expect("invalid key")
	}
}

impl<'a> ParticleData<'a> {
	pub fn new(particles: usize) -> Self {
		ParticleData{
			map: HashMap::new(),
			particles
		}
	}

	pub fn add_series(&mut self, name: &'a str) -> Option<Vec::<Vec::<f64>>> {
		let out = self.map.insert(name, Vec::new());
		if out.is_none() {
			let s = self.map.get_mut(name).expect("Invalid key");
			for _i in 0..self.particles {
				s.push(Vec::new());
			}
		}
		out
	}
	
	pub fn insert_into(&mut self, name: &str, index: usize, value: f64) {
		self.map.get_mut(name).expect("Invalid key")[index].push(value)
	}

	pub fn get(&self, name: &str) -> &Vec::<Vec::<f64>> {
		self.map.get(name).expect("invalid key")
	}
}

impl<'a> ParticleVectorData<'a> {
	pub fn new(particles: usize) -> Self {
		ParticleVectorData{
			map: HashMap::new(),
			particles
		}
	}

	pub fn add_series(&mut self, name: &'a str) -> Option<Vec::<Vec::<Vector>>> {
		let out = self.map.insert(name, Vec::new());
		if out.is_none() {
			let s = self.map.get_mut(name).expect("Invalid key");
			for _i in 0..self.particles {
				s.push(Vec::new());
			}
		}
		out
	}
	
	pub fn insert_into(&mut self, name: &str, index: usize, value: Vector) {
		self.map.get_mut(name).expect("Invalid key")[index].push(value)
	}

	pub fn get(&self, name: &str) -> &Vec::<Vec::<Vector>> {
		self.map.get(name).expect("invalid key")
	}
}


