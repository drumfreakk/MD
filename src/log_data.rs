
//! Structs and methods to save and log data about the system

use crate::Vector;

use std::collections::HashMap;
use std::iter::{zip, Zip};
use std::vec::IntoIter;
use std::fs::File;
use std::io::prelude::*;

/// A map, containing named elements that have f64 values for each time step.
pub struct LinearData<'a> {
	pub map: HashMap::<&'a str, Vec::<f64>>
}

/// A map, containing named elements that have f64 values for each particle for each timestep.
pub struct ParticleData<'a> {
	pub map: HashMap::<&'a str, Vec::<Vec::<f64>>>,
	particles: usize
}

/// A map, containing named elements that have Vector values for each particle for each timestep.
pub struct ParticleVectorData<'a> {
	pub map: HashMap::<&'a str, Vec::<Vec::<Vector>>>,
	particles: usize
}


/// A collection of a LinearData, ParticleData, ParticleVectorData, and a time series. 
/// Used to store all data about a system over time.
pub struct DataLog<'a> {
	//TODO: insert system properties
	pub time: Vec::<f64>,
	pub global: LinearData<'a>,
	pub particle: ParticleData<'a>,
	pub particle_vector: ParticleVectorData<'a>,
}

impl<'a> DataLog<'a> {
	/// Create a new DataLog, for a given number of particles.
	pub fn new(particles: usize) -> Self {
		DataLog{
			time: Vec::new(),
			global: LinearData::new(),
			particle: ParticleData::new(particles),
			particle_vector: ParticleVectorData::new(particles),
		}
	}

	/// Get an iterator of the form (time: f64, value: f64) over a global value.
	pub fn global_as_iter(&self, name: &str) -> Zip<IntoIter<f64>, IntoIter<f64>> {
		zip(self.time.clone(), self.global.get(name).clone())
	}
	
	/// Get an iterator of the form (time: f64, value: f64) over a value of a particle.
	pub fn particle_as_iter(&self, name: &str, index: usize) -> Zip<IntoIter<f64>, IntoIter<f64>> {
		zip(self.time.clone(), self.particle.get(name)[index].clone())
	}
	
	/// Get an iterator of the form (time: f64, value: Vector) over a value of a particle.
	pub fn particle_vector_as_iter(&self, name: &str, index: usize) -> Zip<IntoIter<f64>, IntoIter<Vector>> {
		zip(self.time.clone(), self.particle_vector.get(name)[index].clone())
	}

	/// Create a global series and a particle series with a given name. 
	pub fn add_particle_series(&mut self, name: &'a str) {
		self.particle.add_series(name);
		self.global.add_series(name);
	}
	
	/// Simultaneously insert a value into a particle series, and add the value to the corresponding global series.
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

	/// Create a particle series and a particle vector series with the same name.
	pub fn add_particle_vector_series(&mut self, name: &'a str) {
		self.particle_vector.add_series(name);
		self.particle.add_series(name);
	}

	/// Simultaneously insert a vector into a ParticleVector series, and insert the length of the vector into the corresponing global series.
	pub fn insert_particle_vector_len(&mut self, name: &str, index: usize, value: Vector) {
		self.particle_vector.insert_into(name, index, value);
		self.particle.insert_into(name, index, value.len());
	}

	/// Logs all data to a csv file.
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
		for k in k_particle.clone() {
			for i in 0..self.particle.particles{
				line.push_str(&format!(",{}_{}", &k.to_string(), i).to_string());
			}
		}
		for k in k_vector.clone() {
			for i in 0..self.particle_vector.particles{
				line.push_str(&format!(",{}_{}_x", &k.to_string(), i).to_string());
				line.push_str(&format!(",{}_{}_y", &k.to_string(), i).to_string());
				line.push_str(&format!(",{}_{}_z", &k.to_string(), i).to_string());
			}
		}
		line.push('\n');
		file.write_all(line.as_bytes())?;

		for t in 0..self.time.len() {
			let mut line = String::from(&self.time[t].to_string());
		
			for k in k_global.clone() {
				line.push(',');
				line.push_str(&self.global.get(k)[t].to_string());
			}
			for k in k_particle.clone() {
				for i in 0..self.particle.particles{
					line.push(',');
					line.push_str(&self.particle.get(k)[i][t].to_string());
				}
			}
			for k in k_vector.clone() {
				for i in 0..self.particle_vector.particles{
					line.push(',');
					line.push_str(&self.particle_vector.get(k)[i][t].x.to_string());
					line.push(',');
					line.push_str(&self.particle_vector.get(k)[i][t].y.to_string());
					line.push(',');
					line.push_str(&self.particle_vector.get(k)[i][t].z.to_string());
				}
			}

			line.push('\n');
			file.write_all(line.as_bytes())?;
		}
		
		Ok(())
	}
}


impl<'a> LinearData<'a> {
	/// Create a new empty LinearData.
	pub fn new() -> Self {
		LinearData{
			map: HashMap::new()
		}
	}

	/// Insert a series into the set.
	pub fn add_series(&mut self, name: &'a str) -> Option<Vec::<f64>> {
		self.map.insert(name, Vec::new())
	}

	/// Insert a value into a series.
	pub fn insert_into(&mut self, name: &str, value: f64) {
		self.map.get_mut(name).expect("Invalid key").push(value)
	}

	/// Get a given series, panics if the name is invalid.
	pub fn get(&self, name: &str) -> &Vec::<f64> {
		self.map.get(name).expect("invalid key")
	}
}

impl<'a> ParticleData<'a> {
	/// Create a new empty ParticleData, with a given number of particles.
	pub fn new(particles: usize) -> Self {
		ParticleData{
			map: HashMap::new(),
			particles
		}
	}

	/// Insert a series into the set.
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
	
	/// Insert a value for a given particle into a given series.
	pub fn insert_into(&mut self, name: &str, index: usize, value: f64) {
		self.map.get_mut(name).expect("Invalid key")[index].push(value)
	}

	/// Get a given series, with all particles. Panics if the name is invalid.
	pub fn get(&self, name: &str) -> &Vec::<Vec::<f64>> {
		self.map.get(name).expect("invalid key")
	}
}

impl<'a> ParticleVectorData<'a> {
	/// Create a new empty ParticleVectorData, with a given number of particles.
	pub fn new(particles: usize) -> Self {
		ParticleVectorData{
			map: HashMap::new(),
			particles
		}
	}

	/// Insert a series into the set.
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
	
	/// Insert a value for a given particle into a given series.
	pub fn insert_into(&mut self, name: &str, index: usize, value: Vector) {
		self.map.get_mut(name).expect("Invalid key")[index].push(value)
	}

	/// Get a given series, with all particles. Panics if the name is invalid.
	pub fn get(&self, name: &str) -> &Vec::<Vec::<Vector>> {
		self.map.get(name).expect("invalid key")
	}
}


