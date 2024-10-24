
//! Gets the van der Waals potential and force, according to the Lennard-Jones 12-6 expression

use crate::constants::LJ_4_EPSILON;

/** Gets the potential of a given configuration of two particles.

The radius is the average radius of the two particles.
The total distance is the distance between the centres of the particles
*/
pub fn get_potential(radius: f64, total_distance: f64) -> f64 {
	//4e ( (s/r)^12 - (s/r)^6 )
	// r is dist, s is radius, e is well depth
  
//TODO: cutoff
//TODO: is this right like this?	
	let distance = total_distance - radius;// * 2_f64.powf(1.0/6.0);

	let attraction = (radius / distance).powf(6.0);
	let repulsion = attraction * attraction;

	return LJ_4_EPSILON * (repulsion - attraction); 
}

/** Gets the magnitude of the force between two particles.

The radius is the average radius of the two particles.
The total distance is the distance between the centres of the particles
*/
pub fn get_force(radius: f64, total_distance: f64) -> f64 {
	// 4e ( -12 s^12 r^-13 + 6 s^6 r^-7)
	// 4e ( 6 s^6 (r^-7 - 2 s^6 r^-13 )
	// 6 * 4e s^6 r^-7 (1 - 2 s^6 r^-6)
	let s6 = radius.powf(6.0);
	let distance = total_distance - radius; //* 2_f64.powf(1.0/6.0);

	return 6.0 * LJ_4_EPSILON * s6 * distance.powf(-7.0) * (1.0 - (2.0 * s6 * distance.powf(-6.0)));
}

