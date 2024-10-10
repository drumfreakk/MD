
use crate::particles::Particle;

const LJ_4_EPSILON: f64 = 4.0;


/**
 * Gets the Lennard-Jones 12-6 potential between two particles
 *
 * @param f64 radius_minimal The distance at which the energy is minimal. Should be 2^(1/6) * (r_0 + r_1)/2
 * @param f64 total_distance The distance between the centres of mass of the particles
 * @return f64 The potential of the configuration
 */
#[allow(non_snake_case)]
pub fn LJ_VdW_pot(radius: f64, total_distance: f64) -> f64 {
	//4e ( (s/r)^12 - (s/r)^6 )
	// r is dist, s is radius, e is well depth
  
//TODO: cutoff
	
	let distance = total_distance - radius * 2_f64.powf(1.0/6.0);

	let attraction = (radius / distance).powf(6.0);
	let repulsion = attraction * attraction;

	return LJ_4_EPSILON * (repulsion - attraction); 
}

/**
 * Gets the force between two particles due to the (Lennard-Jones 12-6) Van der Waals force
 *
 * @param f64 radius_minimal The distance at which the force is 0. Should be 2^(1/6) * (r_0 + r_1)/2
 * @param f64 total_distance The distance between the centres of mass of the particles
 * @return f64 The force between the particles
 */
#[allow(non_snake_case)]
pub fn LJ_VdW_F(radius: f64, total_distance: f64) -> f64 {
	// 4e ( -12 s^12 r^-13 + 6 s^6 r^-7)
	// 4e ( 6 s^6 (r^-7 - 2 s^6 r^-13 )
	// 6 * 4e s^6 r^-7 (1 - 2 s^6 r^-6)
	let s6 = radius.powf(6.0);
	let distance = total_distance - radius * 2_f64.powf(1.0/6.0);

	return 6.0 * LJ_4_EPSILON * s6 * distance.powf(-7.0) * (1.0 - (2.0 * s6 * distance.powf(-6.0)));
}

