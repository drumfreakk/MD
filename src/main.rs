
//TODO: Error checking/handling
// Either do the "simple" algebraic method with just elastic collisions,
// or start doing things with energy landscapes

mod vectors;
mod particles;
mod output;

use crate::vectors::Vector;
use crate::particles::Particle;

fn main () {
	let mut p = [Particle::new(&Vector::zero(), 1.0, 0.0, Some(Vector::unit_x()), Some(Vector::unit_x() * 0.5)), 
                Particle::new(&Vector::new(5.0,0.0,0.0), 1.0, 0.0, None, None)];

	for i in 0..300 {
		let d = p[0].collision_dist(&p[1]);
		println!("{}: {}", i, d);
		if d <= 0.0 {
			break;
		}
		p[0].update(0.05);
	}

	output::log_position(&p, "out.txt");

}

