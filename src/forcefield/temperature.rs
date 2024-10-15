
//! Functions to set the temperature of a system.

use crate::particles::Particle;
use crate::constants::{FALLBACK_TEMPERATURE, BOLTZMANN_CONST, TIME_STEP};

/** Gets the scaling factor for the velocities to achieve a set temperature
 
A large coupling constant results in a slow change in temperature, 
whereas a coupling constant equal to TIME_STEP results in instant adjustments.
All velocities should be multiplied by the resulting scaling factor.
*/
pub fn get_scale(particles: &Vec<Particle>, target: f64, coupling: f64) -> f64 {
	let mut temperature = get_temperature(particles);

	if temperature == 0.0 {
		temperature = FALLBACK_TEMPERATURE;
	}

	return (1.0 + ((target / temperature) - 1.0) * TIME_STEP / coupling).sqrt();
}

/// Gets the current temperature of a system, based on the kinetic energy
pub fn get_temperature(particles: &Vec<Particle>) -> f64 {
	let mut double_kinetic_energy = 0.0;
	for i in 0..particles.len() {
		double_kinetic_energy += particles[i].m * particles[i].v.sqlen();
	}

	return double_kinetic_energy / (3.0 * BOLTZMANN_CONST * (particles.len() as f64));
}
