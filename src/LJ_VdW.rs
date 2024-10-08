
use crate::particles::Particle;

const LJ_4_EPSILON: f64 = 20.0;


// Gives the Lennard-Jones Van der Waals potential for particle at distance
// Distance should be between the center of particle and the edge of the second particle?
#[allow(non_snake_case)]
pub fn LJ_VdW_pot(particle: &Particle, distance: f64) -> f64 {
	//4e ( (s/r)^12 - (s/r)^6 )
	// r is dist, s is radius, e is well depth
  
//TODO: cutoff, differentiate manually to get force & save a step

	let attraction = (particle.r / distance).powf(6.0);
	let repulsion = attraction * attraction;

	return LJ_4_EPSILON * (repulsion - attraction); 
}

#[allow(non_snake_case)]
pub fn LJ_VdW_F(particle: &Particle, distance: f64) -> f64 {
	// 4e ( -12 s^12 r^-13 + 6 s^6 r^-7)
	// 4e ( 6 s^6 (r^-7 - 2 s^6 r^-13 )
	// 6 * 4e s^6 r^-7 (1 - 2 s^6 r^-6)
	let s6 = particle.r.powf(6.0);

	return 6.0 * LJ_4_EPSILON * s6 * distance.powf(-7.0) * (1.0 - 2.0 * s6 * distance.powf(-6.0));
}
