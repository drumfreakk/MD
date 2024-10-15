
//! A 3D vector, with associated functions

use std::fmt;
use std::ops;

/// A 3-dimensional vector.
#[derive(Copy, Clone)]
pub struct Vector {
	pub x: f64,
	pub y: f64,
	pub z: f64,
}

#[allow(dead_code)]
impl Vector {
	/// The length of the vector.
	pub fn len(&self) -> f64 {
		self.sqlen().sqrt()
	}
	
	/// The squared length of the vector.
	pub fn sqlen(&self) -> f64 {
		self.dot(self)
	}

	/// The inner (dot) product of the vector with other.
	pub fn dot(&self, other: &Self) -> f64 {
		self.x * other.x + self.y * other.y + self.z * other.z
	}
	
	/// The outer (cross) product of the vector with other.
	pub fn cross(&self, other: &Self) -> Self {
		Vector{
			x: self.y * other.z - self.z * other.y,
			y: self.z * other.x - self.x * other.z,
			z: self.x * other.y - self.y * other.x,
		}
	}

	/// The normalized version of the vector.
	pub fn norm (&self) -> Self {
		*self / self.len()
	}

	/// Create a new vector with given values.
	pub fn new(x: f64, y: f64, z: f64) -> Self {
		Vector{x,y,z}
	}

	/// Create a new 0 vector.
	pub fn zero() -> Self {
		Vector{x: 0.0, y: 0.0, z: 0.0}
	}

	/// Create the vector (1,0,0).
	pub fn unit_x() -> Self {
		Vector{x: 1.0, y: 0.0, z: 0.0}
	}

	/// Create the vector (0,1,0).
	pub fn unit_y() -> Self {
		Vector{x: 0.0, y: 1.0, z: 0.0}
	}

	/// Create the vector (0,0,1).
	pub fn unit_z() -> Self {
		Vector{x: 0.0, y: 0.0, z: 1.0}
	}
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

impl ops::AddAssign for Vector {
	fn add_assign(&mut self, other: Self) {
		self.x += other.x;
		self.y += other.y;
		self.z += other.z;
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

impl ops::Mul<f64> for Vector {
	type Output = Vector;
	fn mul(self, other: f64) -> Vector {
		Vector{
			x: self.x * other,
			y: self.y * other,
			z: self.z * other,
		}
	}
}

impl ops::Div<f64> for Vector {
	type Output = Vector;
	fn div(self, other: f64) -> Vector {
		Vector{
			x: self.x / other,
			y: self.y / other,
			z: self.z / other,
		}
	}
}

impl ops::Neg for Vector {
	type Output = Vector;
	fn neg(self) -> Vector {
		Vector{
			x: -self.x,
			y: -self.y,
			z: -self.z,
		}
	}
}
