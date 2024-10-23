use embedded_graphics_core::prelude::Point;
use embedded_graphics_core::pixelcolor::Rgb888;
use crate::DrawPrimitive;
use crate::framebuffer::FrameBuffer;
use crate::constants::{W, H};

#[inline]
pub fn draw(primitive: DrawPrimitive, fb: &mut FrameBuffer) {
	match primitive {
		DrawPrimitive::Line([p1, p2], color) => {
			for (x, y) in line_drawing::Bresenham::new((p1.x, p1.y), (p2.x, p2.y)) {
				fb.draw_point(x, y, color);
			}
		}
		DrawPrimitive::ColoredPoint(p, c) => {
			fb.draw_point(p.x, p.y, c);
		}
		DrawPrimitive::ColoredTriangle(mut vertices, color) => {
			//sort vertices by y
			vertices.sort_by(|a, b| a.y.cmp(&b.y));

			let [p1, p2, p3] = vertices
				.iter()
				.map(|p| embedded_graphics_core::geometry::Point::new(p.x, p.y))
				.collect::<Vec<embedded_graphics_core::geometry::Point>>()
				.try_into()
				.unwrap();

			if p2.y == p3.y {
				fill_bottom_flat_triangle(p1, p2, p3, color, fb);
			} else if p1.y == p2.y {
				fill_top_flat_triangle(p1, p2, p3, color, fb);
			} else {
				let p4 = Point::new(
					(p1.x as f64
						+ ((p2.y - p1.y) as f64 / (p3.y - p1.y) as f64) * (p3.x - p1.x) as f64)
						as i32,
					p2.y,
				);

				fill_bottom_flat_triangle(p1, p2, p4, color, fb);
				fill_top_flat_triangle(p2, p4, p3, color, fb);
			}
		}
	}
}

fn fill_bottom_flat_triangle(p1: Point, p2: Point, p3: Point, color: Rgb888, fb: &mut FrameBuffer){
	let invslope1 = (p2.x - p1.x) as f64 / (p2.y - p1.y) as f64;
	let invslope2 = (p3.x - p1.x) as f64 / (p3.y - p1.y) as f64;

	let mut curx1 = p1.x as f64;
	let mut curx2 = p1.x as f64;

	for scanline_y in p1.y..=p2.y {
		//TODO: make these not use points either
		fb.draw_horizontal_line(
			[curx1 as i32, scanline_y],
			[curx2 as i32, scanline_y],
			color,
		);

		curx1 += invslope1;
		curx2 += invslope2;
	}
}

fn fill_top_flat_triangle(p1: Point, p2: Point, p3: Point, color: Rgb888, fb: &mut FrameBuffer){
	let invslope1 = (p3.x - p1.x) as f64 / (p3.y - p1.y) as f64;
	let invslope2 = (p3.x - p2.x) as f64 / (p3.y - p2.y) as f64;

	let mut curx1 = p3.x as f64;
	let mut curx2 = p3.x as f64;

	for scanline_y in (p1.y..=p3.y).rev() {
		fb.draw_horizontal_line(
			[curx1 as i32, scanline_y],
			[curx2 as i32, scanline_y],
			color,
		);

		curx1 -= invslope1;
		curx2 -= invslope2;
	}
}

