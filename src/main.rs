

//! Run a molecular dynamics simulation

//TODO: Error checking/handling
#![warn(missing_docs)]

mod constants;
mod vectors;
mod particles;
mod buffer;
mod forcefield;
mod log_data;

use crate::constants::{W, H, FRAME_RATE, SIM_LEN, TIME_STEP};

use crate::vectors::Vector;
use crate::particles::Particle;
use crate::log_data::DataLog;
use crate::buffer::BufferWrapper;
use crate::forcefield::{temperature, vanderwaals, electrostatic};

use minifb::{Window, WindowOptions, Key};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::time::SystemTime;

fn main () -> Result<(), Box<dyn Error>> {
	let mut buf = BufferWrapper(vec![0u32; W * H]);

	let mut window = Window::new("Molecular Dynamics Simulation",W,H,WindowOptions::default(),)?;
	let cs = {
		let root =
			BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?.into_drawing_area();
		root.fill(&BLACK)?;

		let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_2d(0.0..SIM_LEN, -10.0..10.0)?;

		chart.configure_mesh().label_style(("sans-serif", 15).into_font().color(&GREEN)).axis_style(&GREEN).draw()?;

		let cs = chart.into_chart_state();
		root.present()?;
		cs
	};

	let start_ts = SystemTime::now();
	let mut last_flushed = 0.0;

	let mut t = 0.0;

	let mut p = vec![Particle::new(&(Vector::unit_x() * -10.0), 1.00, 1.0,  1.0, None, None),
			 		 Particle::new(&(Vector::unit_x() *  10.0), 1.00, 1.0, -1.0, None, None)];

	let mut data = DataLog::new(p.len());

	data.add_particle_vector_series("position");
	data.add_particle_vector_series("velocity");
	data.add_particle_vector_series("accelleration");
	data.add_particle_vector_series("force_electric");
	data.add_particle_vector_series("force_vdw");
	data.add_particle_vector_series("force_total");
	data.add_particle_series("energy_electric");
	data.add_particle_series("energy_vdw");
	data.add_particle_series("energy_kinetic");
	data.add_particle_series("energy_total");
	data.global.add_series("separation");
	data.global.add_series("temperature");
	data.global.add_series("temperature_scale");


	while window.is_open() && !window.is_key_down(Key::Escape) {
		let epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		if epoch - last_flushed <= 1.0 / FRAME_RATE && t < SIM_LEN {

			// Calculate all relevant quantities
			let separation = p[0].separation(&p[1]);
			let sep_dist = separation.len();

			let r = (p[0].r + p[1].r)/2.0;

			let vdw_force = vanderwaals::get_force(r, sep_dist);
			let vdw_pot = vanderwaals::get_potential(r, sep_dist);
			let vdw_dir = separation/sep_dist;

			let temp = temperature::get_temperature(&p);
			let scale = temperature::get_scale(&p, 0.0, 0.5);

			let elec_f = electrostatic::get_force((p[0].q, p[1].q), sep_dist, true);
			let elec_v = electrostatic::get_energy((p[0].q, p[1].q), sep_dist, true);

			// Log all relevant quantities	
			data.time.push(t);
	
			data.insert_particle_vector_len("force_electric", 	0, vdw_dir * (-elec_f));
			data.insert_particle_vector_len("force_electric", 	1, vdw_dir * ( elec_f));
			data.insert_particle_vector_len("force_vdw", 		0, vdw_dir * ( vdw_force));
			data.insert_particle_vector_len("force_vdw", 		1, vdw_dir * (-vdw_force));
			data.insert_particle_vector_len("force_total", 		0, vdw_dir * ( vdw_force - elec_f));
			data.insert_particle_vector_len("force_total", 		1, vdw_dir * (-vdw_force + elec_f));
			for i in 0..p.len() {
				data.insert_particle_vector_len("position", i, p[i].pos);
				data.insert_particle_vector_len("velocity", i, p[i].v);	
				data.insert_particle_vector_len("accelleration", i, p[i].a);	

				data.insert_particle_add("energy_kinetic", i, p[i].m * p[i].v.sqlen() / 2.0);
				data.insert_particle_add("energy_electric", i, elec_v);
				data.insert_particle_add("energy_vdw", i, vdw_pot);
				data.insert_particle_add("energy_total", i, elec_v + vdw_pot + p[i].m * p[i].v.sqlen() / 2.0);
			}
			
			data.global.insert_into("separation", sep_dist);
			data.global.insert_into("temperature", temp);
			data.global.insert_into("temperature_scale", scale);
			

			// Update accelerations, velocities & positions
			p[0].a =  vdw_dir * ( vdw_force - elec_f) / p[0].m;
			p[1].a =  vdw_dir * (-vdw_force + elec_f) / p[1].m;
			p[0].v = p[0].v * scale;
			p[1].v = p[1].v * scale;

			p[0].update(TIME_STEP);
			p[1].update(TIME_STEP);
			
			t += TIME_STEP;
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

					
					chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 0).map(|(t, v)| {(t, v.x)}), &RED,))?;
					chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 1).map(|(t, v)| {(t, v.x)}), &GREEN,))?;
					//chart.draw_series(LineSeries::new(data.global_as_iter("temperature"), &MAGENTA,))?;
					chart.draw_series(LineSeries::new(data.particle_as_iter("force_electric", 0), &CYAN,))?;
					chart.draw_series(LineSeries::new(data.particle_as_iter("force_vdw", 0), &BLUE,))?;
				}
				root.present()?;
			}

			window.update_with_buffer(buf.borrow(), W, H)?;
			last_flushed = epoch;
		}
	}
	
	data.to_file("out.csv")?;

	Ok(())
}


