
//TODO: Error checking/handling

mod vectors;
mod particles;
#[allow(non_snake_case)]
mod LJ_VdW;
mod buffer;

use crate::vectors::Vector;
use crate::particles::Particle;
use crate::LJ_VdW::{LJ_VdW_pot, LJ_VdW_F};
use crate::buffer::BufferWrapper;

use minifb::{Window, WindowOptions};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::time::SystemTime;
const W: usize = 800;
const H: usize = 600;

const FRAME_RATE: f64 = 1.0;
const SIM_LEN: f64 = 25.0;
const TIMESTEP: f64 = 0.005;

fn main () -> Result<(), Box<dyn Error>> {
	let mut buf = BufferWrapper(vec![0u32; W * H]);

	let mut window = Window::new("Molecular Dynamics Simulation",W,H,WindowOptions::default(),)?;
	let cs = {
		let root =
			BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?.into_drawing_area();
		root.fill(&BLACK)?;

		let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_2d(0.0..SIM_LEN, -5.0..5.0)?;
		//let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_2d(12.0..14.0, -4.0..3.0)?;

		chart.configure_mesh().label_style(("sans-serif", 15).into_font().color(&GREEN)).axis_style(&GREEN).draw()?;

		let cs = chart.into_chart_state();
		root.present()?;
		cs
	};

	let start_ts = SystemTime::now();
	let mut last_flushed = 0.0;

	let mut t = 0.0;

	//TODO: doesnt fucking work
	let mut p = [Particle::new(&(Vector::unit_x() * -1.2), 0.95, 1.0, None, None),
				 Particle::new(&(Vector::unit_x() *  1.2), 1.00, 1.0, None, None)];

	let mut pos = [Vec::new(), Vec::new()];
	let mut pos_r = [Vec::new(), Vec::new()];
	let mut v = [Vec::new(), Vec::new()];
	let mut a = [Vec::new(), Vec::new()];
	let mut ekin = [Vec::new(), Vec::new()];
	let mut epot = [Vec::new(), Vec::new()];
	let mut e = [Vec::new(), Vec::new()];
	let mut etot = Vec::new();


	while window.is_open() {
		let epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		if epoch - last_flushed <= 1.0 / FRAME_RATE && t < SIM_LEN {
			pos[0].push((  t, p[0].pos.x));
			pos[1].push((  t, p[1].pos.x));
			pos_r[0].push((t, p[0].pos.x + p[0].r));
			pos_r[1].push((t, p[1].pos.x - p[1].r));
			v[0].push((    t, p[0].v.x*25.0));
			v[1].push((    t, p[1].v.x*25.0));
			a[0].push((    t, p[0].a.x*0.25));
			a[1].push((    t, p[1].a.x*0.25));
			
			let separation = p[0].separation(&p[1]);
			let sep_dist = separation.len();
			
			epot[0].push(( t, LJ_VdW_pot(&p[1], sep_dist - p[0].r)));
			epot[1].push(( t, LJ_VdW_pot(&p[0], sep_dist - p[1].r)));
			let pot0 = (LJ_VdW_pot(&p[1], sep_dist - p[0].r + 0.01) - LJ_VdW_pot(&p[1], sep_dist - p[0].r)) / 0.01;
			let pot1 = (LJ_VdW_pot(&p[0], sep_dist - p[1].r + 0.01) - LJ_VdW_pot(&p[0], sep_dist - p[1].r)) / 0.01;

			p[0].a = separation * pot0 / (p[0].m * sep_dist);
			p[1].a = -separation * pot1 / (p[1].m * sep_dist);
			//p[0].a =  separation * LJ_VdW_F(&p[1], sep_dist - p[0].r) / (p[0].m * sep_dist);
			//p[1].a = -separation * LJ_VdW_F(&p[0], sep_dist - p[1].r) / (p[1].m * sep_dist);
			p[0].v = p[0].v * 1.0;
			p[1].v = p[1].v * 1.0;

//			if t > 15.0 && t < 15.1 {
//				p[0].v = Vector::zero();
//				p[1].v = Vector::zero();
//				p[0].a = Vector::zero();
//				p[1].a = Vector::zero();
//			}

			p[0].update(TIMESTEP);
			p[1].update(TIMESTEP);
			
			ekin[0].push(( t, p[0].m * p[0].v.sqlen() / 2.0));
			ekin[1].push(( t, p[1].m * p[1].v.sqlen() / 2.0));
			let index = epot[0].len() - 1;
			e[0].push((    t, epot[0][index].1 + ekin[0][index].1));
			e[1].push((    t, epot[1][index].1 + ekin[1][index].1));
			etot.push((    t, e[0][index].1 + e[1][index].1));
			t += TIMESTEP;
		} else {
			{
				let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
					buf.borrow_mut(),
					(W as u32, H as u32),
				)?
				.into_drawing_area();
				{
					let mut chart = cs.clone().restore(&root);
					
					chart.plotting_area().fill(&BLACK)?;

					chart.configure_mesh().bold_line_style(&GREEN.mix(0.2)).light_line_style(&TRANSPARENT).draw()?;

					chart.draw_series(LineSeries::new(pos[0].clone(), &GREEN,))?;
					chart.draw_series(LineSeries::new(pos[1].clone(), &RED,))?;
					chart.draw_series(LineSeries::new(pos_r[0].clone(), &GREEN,))?;
					chart.draw_series(LineSeries::new(pos_r[1].clone(), &RED,))?;
					//chart.draw_series(LineSeries::new(v[0].clone(), &BLUE,))?;
					//chart.draw_series(LineSeries::new(v[1].clone(), &MAGENTA,))?;
					//chart.draw_series(LineSeries::new(a[0].clone(), &WHITE,))?;
					//chart.draw_series(LineSeries::new(a[1].clone(), &YELLOW,))?;
					chart.draw_series(LineSeries::new(epot[0].clone(),   &BLUE,))?;
					chart.draw_series(LineSeries::new(epot[1].clone(),   &MAGENTA,))?;
					chart.draw_series(LineSeries::new(ekin[0].clone(),   &GREEN,))?;
					chart.draw_series(LineSeries::new(ekin[1].clone(),   &CYAN,))?;
					chart.draw_series(LineSeries::new(e[0].clone(),   &YELLOW,))?;
					chart.draw_series(LineSeries::new(e[1].clone(),   &WHITE,))?;
					chart.draw_series(LineSeries::new(etot.clone(),   &RED,))?;
				}
				root.present()?;
			}

			window.update_with_buffer(buf.borrow(), W, H)?;
			last_flushed = epoch;
		}
	}

   Ok(())
}

