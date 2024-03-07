
use std::fmt;
use vectors::Vector;

pub struct Particle {
	pub pos: Vector,	// Position
	pub r: f32,			// Radius
	pub m: f32,			// Mass
	pub v: Vector,		// Velocity
	pub a: Vector,		// Accelleration
}

impl Particle {
/* Position functions */
	// Separation vector from self to other
	pub fn separation(&self, other: &Particle) -> Vector {
		return other.pos - self.pos;
	}
	pub fn distance(&self, other: &Particle) -> f32 {
		return self.separation(other).len();
	}
	pub fn collision_dist(&self, other: &Particle) -> f32 {
		return self.distance(other) - self.r - other.r;
	}

/* Change position */
	pub fn update_pos(&mut self, dt: f32) {
		self.pos = self.pos + self.v * dt
	}

/* Change velocity */
	pub fn update_v(&mut self, dt: f32) {
		self.v = self.v + self.a * dt;
	}

/* Step in time */
	pub fn update(&mut self, dt: f32){
		self.update_v(dt);
		self.update_pos(dt);
	}
}

macro_rules! particle {
	( $pos:expr ) => {
		particles::Particle{
			pos: $pos,
			r: 0.0,
			m: 0.0,
			v: vector!(0,0),
			a: vector!(0,0)
		}
	};
}

impl fmt::Display for Particle {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "(pos: {}\tr: {}\tm: {}\tv: {}\ta: {})", self.pos, self.r, self.m, self.v, self.a)
	}
}
