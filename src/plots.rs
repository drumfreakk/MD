
//! Custom plotting library

use crate::framebuffer::FrameBuffer;
use embedded_graphics_core::pixelcolor::Rgb888;

/// An instance of a plot, including framebuffer.
pub struct Plot {
	pub fb: FrameBuffer,
	pub coordinate_range: [[f64; 2]; 2],
	scale: [f64; 2],
	zero: [i32; 2],
	margin: i32,
}


impl Plot {
	/// Initializes a new plot.
	pub fn new(width: usize, height: usize, margin: i32, x_range: [f64; 2], y_range: [f64; 2], background: Rgb888, color: Rgb888) -> Self {
		let w = width as i32;
		let h = height as i32;
		
		let scale = [(w - 2 * margin) as f64 / (x_range[1] - x_range[0]), 
		             (h - 2 * margin) as f64 / (y_range[1] - y_range[0])]; 

		let zero = [(x_range[0] * scale[0]) as i32 + margin,
		            (y_range[1] * scale[1]) as i32 + margin];

		let mut plot = Plot{
			fb: FrameBuffer::new(width, height),
			coordinate_range: [x_range, y_range],
			scale,
			zero,
			margin,
		};

		plot.fb.fill_buffer(background);

		for y in margin..(h - margin) {
			plot.fb.draw_point((margin, y), color);
			plot.fb.draw_point((w - margin, y), color);
		}

		plot.fb.draw_horizontal_line([margin, margin],     [w - margin, margin], color);
		plot.fb.draw_horizontal_line([margin, h - margin], [w - margin, h - margin], color);

		plot
	}

	/// Plot a point in the graph. Checks whether the point is within the bounds of the graph.
	pub fn plot_point(&mut self, point: (f64, f64), color: Rgb888) {
		if point.0 > self.coordinate_range[0][0] && point.0 < self.coordinate_range[0][1] &&
		   point.1 > self.coordinate_range[1][0] && point.1 < self.coordinate_range[1][1] {
			self.fb.draw_point(self.coordinate_to_pixel(point), color);
		}
	}

	/// Plot a pixel in the graph. Checks whether the point is within the bounds of the graph.
	fn plot_pixel(&mut self, point: (i32, i32), color: Rgb888) {
		if point.0 > self.margin && point.0 < self.fb.width  as i32 - self.margin &&
		   point.1 > self.margin && point.1 < self.fb.height as i32 - self.margin {
			self.fb.draw_point(point, color);
		}
	}

	/// Transform a coordinate to a pixel position in the framebuffer.
	fn coordinate_to_pixel(&self, point: (f64, f64)) -> (i32, i32) {
		((point.0 * self.scale[0]) as i32 + self.zero[0], -(point.1 * self.scale[1]) as i32 + self.zero[1])
	}

	/// Plot a line segment between two points.
	pub fn plot_segment(&mut self, p1: (f64, f64), p2: (f64, f64), color: Rgb888) {
		for p in line_drawing::Bresenham::new(self.coordinate_to_pixel(p1), self.coordinate_to_pixel(p2)) {
			self.plot_pixel(p, color);
		}
	}

	/// The amount of distance in x per pixel.
	pub fn max_frequency(&self) -> f64 {
		1.0 / self.scale[0]
	}
}


