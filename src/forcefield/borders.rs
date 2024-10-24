
//! Enforce borders of the system

use crate::vectors::Vector;
use crate::constants::{BORDER_X, BORDER_Y, BORDER_Z, BORDER_4_EPSILON, BORDER_RANGE};

/// Calculate the potential at a given range.
fn get_potential_range(radius: f64, range: f64) -> f64 {
	BORDER_4_EPSILON * (radius / range).powf(12.0)
}

/// Calculate the magnitude of the force at a given range.
fn get_force_range(radius: f64, range: f64) -> f64 {
	BORDER_4_EPSILON * radius.powf(12.0) * 12.0 * range.powf(-13.0)
}

/** Gets the potential for a particle near the border.

The radius is the radius of the particle.
The position is the position of the particle.
*/
pub fn get_potential(radius: f64, position: &Vector) -> f64 {
	//4e ( (s/r)^12 - (s/r)^6 )
	// r is dist, s is radius, e is well depth
  
	let mut potential = 0.0;
//TODO: cutoff
	// TODO double check signs
	if position.x < BORDER_RANGE {
		potential += get_potential_range(radius, position.x);
	} else if position.x > BORDER_X - BORDER_RANGE {
		potential += get_potential_range(radius, BORDER_X - position.x);
	}
	if position.y < BORDER_RANGE {
		potential += get_potential_range(radius, position.y);
	} else if position.y > BORDER_Y - BORDER_RANGE {
		potential += get_potential_range(radius, BORDER_Y - position.y);
	}
	if position.z < BORDER_RANGE {
		potential += get_potential_range(radius, position.z);
	} else if position.z > BORDER_Z - BORDER_RANGE {
		potential += get_potential_range(radius, BORDER_Z - position.z);
	}
	potential
//	let distance = total_distance - radius * 2_f64.powf(1.0/6.0);
//
//	let attraction = (radius / distance).powf(6.0);
//	let repulsion = attraction * attraction;
//
//	return LJ_4_EPSILON * (repulsion - attraction); 
}

/** Gets the force on a particle near the border.

The radius is the radius of the particle.
The position is the position of the particle.
*/
pub fn get_force(radius: f64, position: &Vector) -> Vector {
	// 4e ( -12 s^12 r^-13 + 6 s^6 r^-7)
	// 4e ( 6 s^6 (r^-7 - 2 s^6 r^-13 )
	// 6 * 4e s^6 r^-7 (1 - 2 s^6 r^-6)

	let mut force = Vector::zero();
//TODO: cutoff
	// TODO double check signs
	if position.x < BORDER_RANGE {
		force.x += get_force_range(radius, position.x);
	} else if position.x > BORDER_X - BORDER_RANGE {
		force.x -= get_force_range(radius, BORDER_X - position.x);
	}
	if position.y < BORDER_RANGE {
		force.y += get_force_range(radius, position.y);
	} else if position.y > BORDER_Y - BORDER_RANGE {
		force.y -= get_force_range(radius, BORDER_Y - position.y);
	}
	if position.z < BORDER_RANGE {
		force.z += get_force_range(radius, position.z);
	} else if position.z > BORDER_Z - BORDER_RANGE {
		force.z -= get_force_range(radius, BORDER_Z - position.z);
	}
	force

//let s6 = radius.powf(6.0);
//	let distance = total_distance - radius * 2_f64.powf(1.0/6.0);
//
//	return 6.0 * LJ_4_EPSILON * s6 * distance.powf(-7.0) * (1.0 - (2.0 * s6 * distance.powf(-6.0)));
}


