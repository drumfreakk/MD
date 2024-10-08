
//TODO: Error checking/handling

mod constants;
mod vectors;
mod particles;
mod buffer;
mod log_array;
mod forcefield;

use crate::constants::{W, H, FRAME_RATE, SIM_LEN, TIME_STEP};

use crate::vectors::Vector;
use crate::particles::Particle;
use crate::buffer::BufferWrapper;
use crate::forcefield::{temperature, vanderwaals, electrostatic};

use minifb::{Window, WindowOptions, Key};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::time::SystemTime;
use std::collections::HashMap;

fn main () -> Result<(), Box<dyn Error>> {
	let mut buf = BufferWrapper(vec![0u32; W * H]);

	let mut window = Window::new("Molecular Dynamics Simulation",W,H,WindowOptions::default(),)?;
	let cs = {
		let root =
			BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?.into_drawing_area();
		root.fill(&BLACK)?;

		let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_2d(0.0..SIM_LEN, -20.0..20.0)?;

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

	let mut global_data = Vec::new();
	global_data.push(("time", Vec::<f64>::new()));

	let mut particle_data = Vec::new();
	particle_data.push(("pos", Vec::new()));

	for _i in 0..p.len() {
		particle_data[0].1.push(Vec::<f64>::new());
	}

	let mut pos = [Vec::new(), Vec::new()];
	let mut pos_r = [Vec::new(), Vec::new()];
	let mut v = [Vec::new(), Vec::new()];
	let mut a = [Vec::new(), Vec::new()];
	let mut ekin = [Vec::new(), Vec::new()];
	let mut epot = [Vec::new(), Vec::new()];
	let mut eelec = [Vec::new(), Vec::new()];
	let mut e = [Vec::new(), Vec::new()];
	let mut force_elec = [Vec::new(), Vec::new()];
	let mut force_vdw = [Vec::new(), Vec::new()];
	let mut force = [Vec::new(), Vec::new()];
	let mut etot = Vec::new();
	let mut sep = Vec::new();
	let mut temperature = Vec::new();
	let mut temp_scale = Vec::new();


	while window.is_open() && !window.is_key_down(Key::Escape) {
		let epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		if epoch - last_flushed <= 1.0 / FRAME_RATE && t < SIM_LEN {
			global_data[get_index_global(&global_data, "time")?].1.push(t);

			pos[0].push((  t, p[0].pos.x));
			pos[1].push((  t, p[1].pos.x));
			sep.push((  t, p[1].pos.x - p[0].pos.x));
			pos_r[0].push((t, p[0].pos.x + p[0].r));
			pos_r[1].push((t, p[1].pos.x - p[1].r));
			v[0].push((	t, p[0].v.x));
			v[1].push((	t, p[1].v.x));
			a[0].push((	t, p[0].a.x));
			a[1].push((	t, p[1].a.x));
			
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

			epot[0].push(( t, vdw_pot));
			epot[1].push(( t, vdw_pot));
			ekin[0].push(( t, p[0].m * p[0].v.sqlen() / 2.0));
			ekin[1].push(( t, p[1].m * p[1].v.sqlen() / 2.0));
			eelec[0].push(( t, elec_v));
			eelec[1].push(( t, elec_v));
			let index = epot[0].len() - 1;
			e[0].push((	t, epot[0][index].1 + ekin[0][index].1 + eelec[0][index].1));
			e[1].push((	t, epot[1][index].1 + ekin[1][index].1 + eelec[1][index].1));
			etot.push((	t, e[0][index].1 + e[1][index].1));
			temperature.push((	   t, temp));
			temp_scale.push((	   t, scale));
			force_vdw[0].push((t,  vdw_force));
			force_vdw[1].push((t, -vdw_force));
			force_elec[0].push((t,  -elec_f/100.0));
			force_elec[1].push((t,  elec_f/100.0));
			force[0].push((t, vdw_force - elec_f));
			force[1].push((t, -vdw_force + elec_f));



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

					//chart.draw_series(LineSeries::new(data.get_mut("positions").expect("Invalid key")[0].clone(), &GREEN,))?;
					//chart.draw_series(LineSeries::new(data.get_mut("positions").expect("Invalid key")[1].clone(), &RED,))?;
					//chart.draw_series(LineSeries::new(pos[0].clone(), &GREEN,))?;
					//chart.draw_series(LineSeries::new(pos[1].clone(), &RED,))?;
					chart.draw_series(LineSeries::new(pos_r[0].clone(), &GREEN,))?;
					chart.draw_series(LineSeries::new(pos_r[1].clone(), &RED,))?;
					//chart.draw_series(LineSeries::new(sep.clone(), &MAGENTA,))?;
					//chart.draw_series(LineSeries::new(v[0].clone(), &CYAN,))?;
					//chart.draw_series(LineSeries::new(v[1].clone(), &BLUE,))?;
					//chart.draw_series(LineSeries::new(a[0].clone(), &WHITE,))?;
					//chart.draw_series(LineSeries::new(a[1].clone(), &YELLOW,))?;
					//chart.draw_series(LineSeries::new(epot[0].clone(),   &BLUE,))?;
					//chart.draw_series(LineSeries::new(epot[1].clone(),   &MAGENTA,))?;
					//chart.draw_series(LineSeries::new(ekin[0].clone(),   &BLUE,))?;
					//chart.draw_series(LineSeries::new(ekin[1].clone(),   &CYAN,))?;
					//chart.draw_series(LineSeries::new(eelec[0].clone(),   &BLUE,))?;
					//chart.draw_series(LineSeries::new(eelec[1].clone(),   &CYAN,))?;
					//chart.draw_series(LineSeries::new(e[0].clone(),   &WHITE,))?;
					//chart.draw_series(LineSeries::new(e[1].clone(),   &YELLOW,))?;
					//chart.draw_series(LineSeries::new(etot.clone(),   &MAGENTA,))?;
					//chart.draw_series(LineSeries::new(force_vdw[0].clone(),   &YELLOW,))?;
					//chart.draw_series(LineSeries::new(force_vdw[1].clone(),   &WHITE,))?;
					chart.draw_series(LineSeries::new(force_elec[0].clone(),   &YELLOW,))?;
					chart.draw_series(LineSeries::new(force_elec[1].clone(),   &WHITE,))?;
					//chart.draw_series(LineSeries::new(force[0].clone(),   &YELLOW,))?;
					//chart.draw_series(LineSeries::new(force[1].clone(),   &WHITE,))?;
					//chart.draw_series(LineSeries::new(temperature.clone(), &MAGENTA,))?;
					//chart.draw_series(LineSeries::new(temp_scale.clone(), &YELLOW,))?;
				}
				root.present()?;
			}

			window.update_with_buffer(buf.borrow(), W, H)?;
			last_flushed = epoch;
		}
	}

	let mut data = HashMap::new();

	data.insert("pos0", pos[0].clone());
	data.insert("pos1", pos[1].clone());
	data.insert("posr0", pos_r[0].clone());
	data.insert("posr1", pos_r[1].clone());
	data.insert("a0", a[0].clone());
	data.insert("a1", a[1].clone());
	data.insert("v0", v[0].clone());
	data.insert("v1", v[1].clone());
	data.insert("E0", e[0].clone());
	data.insert("E1", e[1].clone());
	data.insert("E",  etot.clone());
	data.insert("force0", force[0].clone());
	data.insert("force1", force[1].clone());
	data.insert("force_elec0", force_elec[0].clone());
	data.insert("force_elec1", force_elec[1].clone());
	data.insert("force_vdw0", force_vdw[0].clone());
	data.insert("force_vdw1", force_vdw[1].clone());
	data.insert("Ekin0", ekin[0].clone());
	data.insert("Ekin1", ekin[1].clone());
	data.insert("Epot0", epot[0].clone());
	data.insert("Epot1", epot[1].clone());
	data.insert("Eelec0", eelec[0].clone());
	data.insert("Eelec1", eelec[1].clone());
	data.insert("sep", sep.clone());
	data.insert("temperature", temperature.clone());
	data.insert("temp_scale", temp_scale.clone());
	
	crate::log_array::log_array(&data, "out.csv")?;

	Ok(())
}

fn get_index_global(data: &Vec<(&str, Vec<f64>)>, key: &str) -> Result<usize, usize> {
	for i in 0..data.len() {
		if key == data[i].0 {
			return Ok(i);
		}
	}
	return Err(0);
}

fn get_index_particle(data: &Vec<(&str, Vec<Vec<f64>>)>, key: &str) -> Result<usize, ()> {
	for i in 0..data.len() {
		if key == data[i].0 {
			return Ok(i);
		}
	}
	return Err(());
}
