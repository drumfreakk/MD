use std::fmt;
use std::ops;

#[derive(Copy, Clone)]
pub struct Vector {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Vector {
	pub fn len(&self) -> f32 {
		return self.sqlen().sqrt();
	}
	
	pub fn sqlen(&self) -> f32 {
		return self.dot(self);
	}

	pub fn dot(&self, other: &Vector) -> f32 {
		return self.x * other.x + self.y * other.y + self.z * other.z;
	}

	pub fn cross(&self, other: &Vector) -> Vector {
		return Vector{
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		};
	}

	pub fn norm (&self) -> Vector {
		return *self / self.len();
	}
}

macro_rules! vector {
	( $x:expr, $y:expr ) => {
		vectors::Vector{x: $x as f32, y: $y as f32, z: 0.0}
	};
	( $x:expr, $y:expr, $z:expr  ) => {
		vectors::Vector{x: $x as f32, y: $y as f32, z: $z as f32}
	};
}


/* Implementations of methods for Vector */
impl fmt::Display for Vector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "({}, {}, {})", self.x, self.y, self.z)
	}
}

impl ops::Add<Vector> for Vector {
	type Output = Vector;
	fn add(self, other: Vector) -> Vector {
		Vector{
			x: self.x + other.x,
			y: self.y + other.y,
			z: self.z + other.z,
		}
	}
}

impl ops::Sub<Vector> for Vector {
	type Output = Vector;
	fn sub(self, other: Vector) -> Vector {
		Vector{
			x: self.x - other.x,
			y: self.y - other.y,
			z: self.z - other.z,
		}
	}
}

impl ops::Mul<f32> for Vector {
	type Output = Vector;
	fn mul(self, other: f32) -> Vector {
		Vector{
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
		}
	}
}

impl ops::Div<f32> for Vector {
	type Output = Vector;
	fn div(self, other: f32) -> Vector {
		Vector{
			x: self.x / other,
			y: self.y / other,
			z: self.z / other,
		}
	}
}

