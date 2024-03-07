

use std::fs::File;
use std::io::Write;

use particles::Particle;

pub fn log_position(particles: &[Particle], path: &str) {
	let mut output = File::create(path).unwrap();
	writeln!(output, "x,y,z");
	for p in particles {
		writeln!(output, "{},{},{}", p.pos.x, p.pos.y, p.pos.z);
	}
}
