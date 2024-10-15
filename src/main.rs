
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

		let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_2d(0.0..SIM_LEN, -15.0..15.0)?;

		chart.configure_mesh().label_style(("sans-serif", 15).into_font().color(&GREEN)).axis_style(&GREEN).draw()?;

		let cs = chart.into_chart_state();
		root.present()?;
		cs
	};

	let start_ts = SystemTime::now();
	let mut last_flushed = 0.0;

	let mut t = 0.0;

	let mut p = vec![Particle::new(&(Vector::unit_x() * -5.0), 1.00, 1.0,  0.0, None, None),
			 		 Particle::new(&Vector::zero(),            1.00, 1.0, 0.0, None, None),
			 		 Particle::new(&(Vector::unit_x() *  4.0), 1.00, 1.0,  0.0, None, None)];

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
	data.global.add_series("temperature");
	data.global.add_series("temperature_scale");

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		if epoch - last_flushed <= 1.0 / FRAME_RATE && t < SIM_LEN {
			data.time.push(t);
			
			for i in 0..p.len() {
				p[i].a = Vector::zero();
				data.insert_particle_vector_len("position", i, p[i].pos);
				data.insert_particle_vector_len("velocity", i, p[i].v);	
				data.insert_particle_vector_len("accelleration", i, p[i].a);	
			}
			
			let scale = temperature::get_scale(&p, 0.0, 25.0);
			
			data.global.insert_into("temperature", temperature::get_temperature(&p));
			data.global.insert_into("temperature_scale", scale);

			// Iterate over each pair of particles
			for i in 0..(p.len()-1) {
				for j in (i+1)..(p.len()) {
					let separation = p[i].separation(&p[j]);
					let sep_dist = separation.len();

					let r = (p[i].r + p[j].r)/2.0;

					let vdw_force = vanderwaals::get_force(r, sep_dist);
					let vdw_pot = vanderwaals::get_potential(r, sep_dist);
					let vdw_dir = separation/sep_dist;

					let elec_f = electrostatic::get_force((p[i].q, p[j].q), sep_dist, true);
					let elec_v = electrostatic::get_energy((p[i].q, p[j].q), sep_dist, true);

					data.add_to_particle_vector_len("force_electric", 	i, vdw_dir * (-elec_f));
					data.add_to_particle_vector_len("force_electric", 	j, vdw_dir * ( elec_f));
					data.add_to_particle_vector_len("force_vdw", 		i, vdw_dir * ( vdw_force));
					data.add_to_particle_vector_len("force_vdw", 		j, vdw_dir * (-vdw_force));
					data.add_to_particle_vector_len("force_total", 		i, vdw_dir * ( vdw_force - elec_f));
					data.add_to_particle_vector_len("force_total", 		j, vdw_dir * (-vdw_force + elec_f));
					for k in [i,j] {
						data.add_to_particle_add("energy_kinetic", k, p[k].m * p[k].v.sqlen() / 2.0);
						data.add_to_particle_add("energy_electric", k, elec_v);
						data.add_to_particle_add("energy_vdw", k, vdw_pot);
						data.add_to_particle_add("energy_total", k, elec_v + vdw_pot + p[k].m * p[k].v.sqlen() / 2.0);
					}

					// Technically this stores the forces instead of the accellerations, but it saves dividing by the mass so often
					p[i].a +=  vdw_dir * ( vdw_force - elec_f);
					p[j].a +=  vdw_dir * (-vdw_force + elec_f);
				}
			}

			for i in 0..p.len() {
				p[i].a = p[i].a / p[i].m; // Finally convert the force to accelleration
				p[i].v = p[i].v * scale; // Scale the temperature
				p[i].update(TIME_STEP);
			}

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
					chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 2).map(|(t, v)| {(t, v.x)}), &MAGENTA,))?;
					chart.draw_series(LineSeries::new(data.global_as_iter("temperature"), &YELLOW,))?;
	//				chart.draw_series(LineSeries::new(data.particle_as_iter("force_electric", 0), &CYAN,))?;
					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("force_vdw", 0).map(|(t, v)| {(t, v.x)}), &BLUE,))?;
					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("force_vdw", 1).map(|(t, v)| {(t, v.x)}), &CYAN,))?;
					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("force_vdw", 2).map(|(t, v)| {(t, v.x)}), &YELLOW,))?;
					//chart.draw_series(LineSeries::new(data.particle_as_iter("energy_total", 0), &YELLOW,))?;
					//chart.draw_series(LineSeries::new(data.particle_as_iter("energy_total", 1), &WHITE,))?;
					//chart.draw_series(LineSeries::new(data.particle_as_iter("energy_total", 2), &CYAN,))?;
					chart.draw_series(LineSeries::new(data.global_as_iter("energy_total"), &WHITE,))?;
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


