
use crate::particles::Particle;
use crate::constants::{FALLBACK_TEMPERATURE, BOLTZMANN_CONST, TIME_STEP};

/**
 * Gets the scaling factor for the velocities to achieve a set temperature
 * 
 * @param &Vec<Particle> particles The full set of particles in the system
 * @param f64 target The target temperature
 * @param f64 coupling Coupling constant between the system and heat bath
 *                     scale=TIME_STEP gives direct coupling (no delay)
 *                     a large value for coupling changes the temperature slowly
 * @return f64 The scaling factor to multiply all velocities by
 */
pub fn get_scale(particles: &Vec<Particle>, target: f64, coupling: f64) -> f64 {
	let mut temperature = get_temperature(particles);

	if temperature == 0.0 {
		temperature = FALLBACK_TEMPERATURE;
	}

	return (1.0 + ((target / temperature) - 1.0) * TIME_STEP / coupling).sqrt();
}

/**
 * Gets the current temperature of a system, based on the kinetic energy
 *
 * @param &Vec<Particle> particles The full set of particles in the system
 * @return f64 The temperature of the system
 */
pub fn get_temperature(particles: &Vec<Particle>) -> f64 {
	let mut double_kinetic_energy = 0.0;
	for i in 0..particles.len() {
		double_kinetic_energy += particles[i].m * particles[i].v.sqlen();
	}

	return double_kinetic_energy / (3.0 * BOLTZMANN_CONST * (particles.len() as f64));
}
