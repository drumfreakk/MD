
//TODO: Error checking/handling

mod vectors;
mod particles;
mod output;
mod buffer;


use crate::vectors::Vector;
use crate::particles::Particle;
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
const SIM_LEN: f64 = 50.0;
const TIMESTEP: f64 = 0.005;

const LJ_4_EPSILON: f64 = 20.0;

// Gives the Lennard-Jones Van der Waals potential for particle at distance
// Distance should be between the center of particle and the edge of the second particle?
fn LJ_VdW_pot(particle: &Particle, distance: f64) -> f64 {
	//4e ( (s/r)^12 - (s/r)^6 )
	// r is dist, s is radius, e is well depth
  
//TODO: cutoff, differentiate manually to get force & save a step

	let attraction = (particle.r / distance).powf(6.0);
	let repulsion = attraction * attraction;

	return LJ_4_EPSILON * (repulsion - attraction); 
}

fn LJ_VdW_F(particle: &Particle, distance: f64) -> f64 {
    // 4e ( -12 s^12 r^-13 + 6 s^6 r^-7)
    // 4e ( 6 s^6 (r^-7 - 2 s^6 r^-13 )
    // 6 * 4e s^6 r^-7 (1 - 2 s^6 r^-6)
    let s6 = particle.r.powf(6.0);

    return 6.0 * LJ_4_EPSILON * s6 * distance.powf(-7.0) * (1.0 - 2.0 * s6 * distance.powf(-6.0));
}

fn main () -> Result<(), Box<dyn Error>> {
    let mut buf = BufferWrapper(vec![0u32; W * H]);

	let mut window = Window::new("Molecular Dynamics Simulation",W,H,WindowOptions::default(),)?;
	let cs = {
		let root =
            BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?.into_drawing_area();
		root.fill(&BLACK)?;

		let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_2d(0.0..SIM_LEN, -5.0..5.0)?;

		chart.configure_mesh().label_style(("sans-serif", 15).into_font().color(&GREEN)).axis_style(&GREEN).draw()?;

		let cs = chart.into_chart_state();
		root.present()?;
		cs
	};

	let start_ts = SystemTime::now();
	let mut last_flushed = 0.0;

    let mut t = 0.0;

    //TODO: doesnt fucking work
    let mut p = [Particle::new(&(Vector::unit_x() * -2.0), 0.95, 1.0, None, None),
				 Particle::new(&(Vector::unit_x() * 2.0), 1.0, 1.0, None, None)];

	let mut positions_f = Vec::new();
	let mut positions_s = Vec::new();

	let mut positions_f_r = Vec::new();
	let mut positions_s_r = Vec::new();

    let mut v_f = Vec::new();
    let mut v_s = Vec::new();
    let mut a_f = Vec::new();
    let mut a_s = Vec::new();

	while window.is_open() {
		let epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		if epoch - last_flushed <= 1.0 / FRAME_RATE && t < SIM_LEN {
            positions_f.push((t, p[0].pos.x));
		    positions_s.push((t, p[1].pos.x));
		    positions_f_r.push((t, p[0].pos.x + p[0].r));
		    positions_s_r.push((t, p[1].pos.x - p[1].r));
            v_f.push((t, p[0].v.x));
            v_s.push((t, p[1].v.x));
            a_f.push((t, p[0].a.x));
            a_s.push((t, p[1].a.x));

		    let separation = p[0].separation(&p[1]);
		    let sep_dist = separation.len();

		    p[0].a =  separation * p[0].m * LJ_VdW_F(&p[1], sep_dist - p[0].r) / sep_dist;
		    p[1].a = -separation * p[1].m * LJ_VdW_F(&p[0], sep_dist - p[1].r) / sep_dist;
		    p[0].v = p[0].v * 1.0;
		    p[1].v = p[1].v * 1.0;

            p[0].update(TIMESTEP);
		    p[1].update(TIMESTEP);

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

                    chart.draw_series(LineSeries::new(positions_f.clone(), &GREEN,))?;
                    chart.draw_series(LineSeries::new(positions_s.clone(), &RED,))?;
                    chart.draw_series(LineSeries::new(positions_f_r.clone(), &GREEN,))?;
                    chart.draw_series(LineSeries::new(positions_s_r.clone(), &RED,))?;
                    //chart.draw_series(LineSeries::new(v_f.clone(), &BLUE,))?;
                    //chart.draw_series(LineSeries::new(v_s.clone(), &MAGENTA,))?;
                    //chart.draw_series(LineSeries::new(a_f.clone(), &WHITE,))?;
                    //chart.draw_series(LineSeries::new(a_s.clone(), &YELLOW,))?;
                }
				root.present()?;
			}

			window.update_with_buffer(buf.borrow(), W, H)?;
			last_flushed = epoch;
		}
	}

   Ok(())
}

