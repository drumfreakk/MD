
/*! Some helper functions for displaying the graph.

Original code from <https://github.com/plotters-rs/plotters-minifb-demo>
The other code for displaying the graph in main.rs is also adapted from this repo
*/

use std::borrow::{Borrow, BorrowMut};
use embedded_graphics_core::pixelcolor::Rgb565;

pub struct BufferWrapper(pub Vec<u32>);
impl Borrow<[u8]> for BufferWrapper {
	fn borrow(&self) -> &[u8] {
		// Safe for alignment: align_of(u8) <= align_of(u32)
		// Safe for cast: u32 can be thought of as being transparent over [u8; 4]
		unsafe {
			std::slice::from_raw_parts(
				self.0.as_ptr() as *const u8,
				self.0.len() * 4
			)
		}
	}
}
impl BorrowMut<[u8]> for BufferWrapper {
	fn borrow_mut(&mut self) -> &mut [u8] {
		// Safe for alignment: align_of(u8) <= align_of(u32)
		// Safe for cast: u32 can be thought of as being transparent over [u8; 4]
		unsafe {
			std::slice::from_raw_parts_mut(
				self.0.as_mut_ptr() as *mut u8,
				self.0.len() * 4
			)
		}
	}
}
impl Borrow<[u32]> for BufferWrapper {
	fn borrow(&self) -> &[u32] {
		self.0.as_slice()
	}
}
impl BorrowMut<[u32]> for BufferWrapper {
	fn borrow_mut(&mut self) -> &mut [u32] {
		self.0.as_mut_slice()
	}
}

impl BufferWrapper {
	pub fn clear_buffer(&mut self) {
		for i in 0..self.0.len() {
			self.0[i] = 0;
		}
	}
}
