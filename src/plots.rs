
//! Custom plotting library

use crate::framebuffer::FrameBuffer;
use embedded_graphics_core::pixelcolor::Rgb888;
use embedded_graphics_core::prelude::Point;

pub struct Plot {
	pub fb: FrameBuffer,
	pub coordinate_range: [[f64; 2]; 2],
	scale: [f64; 2],
	zero: [i32; 2]
}


impl Plot {
	pub fn new(width: usize, height: usize, margins: i32, x_range: [f64; 2], y_range: [f64; 2], background: Rgb888, color: Rgb888) -> Self {
		let w = width as i32;
		let h = height as i32;
		
		let scale = [(w - 2 * margins) as f64 / (x_range[1] - x_range[0]), 
		             (h - 2 * margins) as f64 / (y_range[1] - y_range[0])]; 

		let zero = [(x_range[0] * scale[0]) as i32 + margins,
		            (y_range[1] * scale[1]) as i32 + margins];

		let mut plot = Plot{
			fb: FrameBuffer::new(width, height),
			coordinate_range: [x_range, y_range],
			scale,
			zero,
		};

		plot.fb.fill_buffer(background);

		for y in margins..(h - margins) {
			plot.fb.draw_point(margins, y, color);
			plot.fb.draw_point(w - margins, y, color);
		}

		plot.fb.draw_horizontal_line([margins, margins],     [w - margins, margins], color);
		plot.fb.draw_horizontal_line([margins, h - margins], [w - margins, h - margins], color);

		plot
	}

	pub fn plot_point(&mut self, point: [f64; 2], color: Rgb888) {
		if point[0] > self.coordinate_range[0][0] && point[0] < self.coordinate_range[0][1] &&
		   point[1] > self.coordinate_range[1][0] && point[1] < self.coordinate_range[1][1] {
			self.fb.draw_point((point[0] * self.scale[0]) as i32 + self.zero[0], -(point[1] * self.scale[1]) as i32 + self.zero[1], color);
		}

	}

	pub fn plot_series(&mut self, series: TODO, color: Rgb888) {

	}
}


