
//! The mesh to store 3D objects.

use embedded_graphics_core::pixelcolor::{Rgb888, WebColors};
use nalgebra::{Point3, Similarity3, UnitQuaternion, Vector3};

/// How to render a Geometry object.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RenderMode {
	Points,
	Lines,
	Solid,
	SolidLightDir(Vector3<f64>),
}

/// A 3D object, stored as vertices with lines and faces connecting them.
#[derive(Debug, Default, Copy, Clone)]
pub struct Geometry<'a> {
	pub vertices: &'a [[f64; 3]],
	pub faces: &'a [[usize; 3]],
	pub colors: &'a [Rgb888],
	pub lines: &'a [[usize; 2]],
	pub normals: &'a [[f64; 3]],
}

impl<'a> Geometry<'a> {
	/// Check if a Geometry object is valid.
	fn check_validity(&self) -> bool {
		if self.vertices.is_empty() {
			println!("Vertices are empty");
			return false;
		}

		for face in self.faces {
			if face[0] >= self.vertices.len()
				|| face[1] >= self.vertices.len()
				|| face[2] >= self.vertices.len()
			{
				println!("Face vertices are out of bounds");
				return false;
			}
		}

		for line in self.lines {
			if line[0] >= self.vertices.len() || line[1] >= self.vertices.len() {
				println!("Line vertices are out of bounds");
				return false;
			}
		}

		if !self.colors.is_empty() && self.colors.len() != self.vertices.len() {
			println!("Colors are not the same length as vertices");
			return false;
		}

		true
	}

	/// Create lines along the edges of faces (?).
	pub fn lines_from_faces(faces: &[[usize; 3]]) -> Vec<(usize, usize)> {
		let mut lines = Vec::new();
		for face in faces {
			for line in &[(face[0], face[1]), (face[1], face[2]), (face[2], face[0])] {
				let (a, b) = if line.0 < line.1 {
					(line.0, line.1)
				} else {
					(line.1, line.0)
				};
				if !lines.contains(&(a, b)) {
					lines.push((a, b));
				}
			}
		}

		lines
	}
}

/// A mesh representing a 3D object and how it should be displayed.
#[derive(Copy, Clone)]
pub struct K3dMesh<'a> {
	pub similarity: Similarity3<f64>,
	pub model_matrix: nalgebra::Matrix4<f64>,

	pub color: Rgb888,
	pub render_mode: RenderMode,
	pub geometry: Geometry<'a>,
}

impl<'a> K3dMesh<'a> {
	/// Create a new mesh based on a Geometry object.
	pub fn new(geometry: Geometry) -> K3dMesh {
		assert!(geometry.check_validity());
		let sim = Similarity3::new(Vector3::new(0.0, 0.0, 0.0), nalgebra::zero(), 1.0);
		K3dMesh {
			model_matrix: sim.to_homogeneous(),
			similarity: sim,
			color: Rgb888::CSS_WHITE,
			render_mode: RenderMode::Points,
			geometry,
		}
	}

	/// Set the color of the mesh.
	pub fn set_color(&mut self, color: Rgb888) {
		self.color = color;
	}

	/// Set the render mode of the mesh.
	pub fn set_render_mode(&mut self, mode: RenderMode) {
		self.render_mode = mode;
	}

	/// Set the position of the object.
	pub fn set_position(&mut self, x: f64, y: f64, z: f64) {
		self.similarity.isometry.translation.x = x;
		self.similarity.isometry.translation.y = y;
		self.similarity.isometry.translation.z = z;
		self.update_model_matrix();
	}

	/// Get the position of the object.
	pub fn get_position(&self) -> Point3<f64> {
		self.similarity.isometry.translation.vector.into()
	}

	/// Set the rotation of the object.
	pub fn set_attitude(&mut self, roll: f64, pitch: f64, yaw: f64) {
		self.similarity.isometry.rotation = UnitQuaternion::from_euler_angles(roll, pitch, yaw);
		self.update_model_matrix();
	}

	/// ?
	pub fn set_target(&mut self, target: Point3<f64>) {
		let view = Similarity3::look_at_rh(
			&self.similarity.isometry.translation.vector.into(),
			&target,
			&Vector3::y(),
			1.0,
		);

		self.similarity = view;
		self.update_model_matrix();
	}

	/// Set the size of the object.
	pub fn set_scale(&mut self, s: f64) {
		if s == 0.0 {
			return;
		}
		self.similarity.set_scaling(s);
		self.update_model_matrix();
	}

	/// ?
	fn update_model_matrix(&mut self) {
		self.model_matrix = self.similarity.to_homogeneous();
	}
}
