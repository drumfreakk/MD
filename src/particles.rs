
//! A particle, with associated quantities and functions

use std::fmt;
use crate::vectors::Vector;

/// A particle, with associated v, a, radius, mass, and charge
pub struct Particle {
	pub pos: Vector,	// Position
	pub r: f64,			// Radius
	pub m: f64,			// Mass
	pub q: f64,			// Charge
	pub v: Vector,		// Velocity
	pub a: Vector,		// Accelleration
}

#[allow(dead_code)]
impl Particle {
	/// Create a new particle with given values
	pub fn new(pos: &Vector, r: f64, m: f64, q: f64) -> Self {
		Particle{
			pos: *pos,
			r,
			m,
			q,
			v: Vector::zero(),
			a: Vector::zero(),
		}
	}

/* Position functions */
	/// Separation vector from self to other
	pub fn separation(&self, other: &Self) -> Vector {
		return other.pos - self.pos;
	}
	/// Normalized direction vector from self to other
	pub fn direction(&self, other: &Self) -> Vector {
		return self.separation(other).norm(); 
	}
	/// Separation distance between self and other
	pub fn distance(&self, other: &Self) -> f64 {
		return self.separation(other).len();
	}
	/// Collision distance between self and other
	pub fn collision_dist(&self, other: &Self) -> f64 {
		return self.distance(other) - self.r - other.r;
	}

/* Update functions */
	/// Update position based on a step in time
	pub fn update_pos(&mut self, dt: f64) {
		self.pos = self.pos + self.v * dt
	}

	/// Update velocity based on a step in time
	pub fn update_v(&mut self, dt: f64) {
		self.v = self.v + self.a * dt;
	}

	/// Step in time, updating v and pos
	pub fn update(&mut self, dt: f64){
		self.update_v(dt);
		self.update_pos(dt);
	}
}

impl fmt::Display for Particle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "(pos: {}\tr: {}\tm: {}\tv: {}\ta: {})", self.pos, self.r, self.m, self.v, self.a)
	}
}
