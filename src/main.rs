
//! Run a molecular dynamics simulation

//TODO: Error checking/handling
#![warn(missing_docs)]

mod constants;
mod vectors;
mod particles;
mod forcefield;
mod framebuffer;
mod log_data;
mod plots;

mod embedded_gfx;

mod icosphere;

use crate::constants::{W, H, FRAME_RATE, SIM_LEN, TIME_STEP, BORDER_X, BORDER_Y, BORDER_Z};

use crate::vectors::Vector;
use crate::particles::Particle;
use crate::log_data::DataLog;
use crate::forcefield::{temperature, vanderwaals, electrostatic, borders};
use crate::framebuffer::FrameBuffer;
use crate::plots::Plot;

use minifb::{Window, WindowOptions, Key, KeyRepeat};
use std::borrow::Borrow;
use std::error::Error;
use std::time::SystemTime;

use crate::embedded_gfx::{DrawPrimitive, K3dengine};
use crate::embedded_gfx::mesh::{K3dMesh, Geometry, RenderMode};
use crate::embedded_gfx::draw::draw;
use embedded_graphics_core::pixelcolor::Rgb888;
use nalgebra::Point3;


fn main () -> Result<(), Box<dyn Error>> {
	let mut sim_fb = FrameBuffer::new(W, H);
	let mut plot = Plot::new(W, H, 10, [0.0, SIM_LEN], [-10.0, 10.0], Rgb888::new(0,0,0), Rgb888::new(0,255,0));
	
	let mut sim_window = Window::new("MD Sim", W, H, WindowOptions::default(),)?;
	let mut data_window = Window::new("MD Sim Data", W, H, WindowOptions::default(),)?;
	
	let mut t = 0.0;

	let start_ts = SystemTime::now();
	let mut last_flushed = 0.0;
	
	let mut theta: f64 = 0.0;
	let mut phi: f64 = 0.0;
	let mut zoom: f64 = 20.0;

	let mut engine = K3dengine::new(W as u16, H as u16);
	engine.camera.set_position(Point3::new(0.0, 0.0, zoom));
	engine.camera.set_target(Point3::new(5.0, 5.0, 5.0));
	engine.camera.set_fovy(3.141592 / 4.0);
	engine.camera.far = 30.0;
	
	let mut b = make_xy_plane(0.0);
	let mut z1 = make_xy_plane(BORDER_Z);
	let mut x0 = make_yz_plane(0.0);
	let mut x1 = make_yz_plane(BORDER_X);
	let mut y0 = make_xz_plane(0.0);
	let mut y1 = make_xz_plane(BORDER_Y);
	b.append(&mut z1);
	b.append(&mut x0);
	b.append(&mut x1);
	b.append(&mut y0);
	b.append(&mut y1);
	let mut borders = K3dMesh::new(Geometry {
		vertices: &b,
		faces: &[], colors: &[], lines: &[], normals: &[]
	});
	borders.set_color(Rgb888::new(255,255,255));


	//TODO: multiple graphs, split this file up

	let mut p = vec![Particle::new(&Vector::new(1.0, 1.0, 1.0),  1.0, 3.0,  0.0, None, None),
			 		 Particle::new(&Vector::new(4.0, 3.0, 1.0),     1.0, 1.0,  0.0, None, None),
			 		 Particle::new(&Vector::new(1.0, 5.0, 1.0),     1.5, 1.0,  0.0, None, None)];

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

	let s = icosphere::create_icosphere(2);
	let norms = icosphere::get_normals(&s.0, &s.1);
	let sphere = K3dMesh::new(Geometry {
		vertices: &s.0,
		faces: &s.1,
		colors: &[],
		lines: &[],
		normals: &norms,
	});

	let mut spheres = Vec::new();
	for i in 0..p.len() {
		spheres.push(sphere);
		spheres[i].set_position(p[i].pos.x, p[i].pos.y, p[i].pos.z);
		spheres[i].set_scale(p[i].r);
		spheres[i].set_render_mode(RenderMode::SolidLightDir(nalgebra::Vector3::new(0.0, 0.0, 1.0)));
	}
	spheres[0].set_color(Rgb888::new(255,0,0));
	spheres[1].set_color(Rgb888::new(0,255,0));
	spheres[2].set_color(Rgb888::new(0,0,255));

	let mut last_plotted = 0.0;

	while sim_window.is_open()  &&  !sim_window.is_key_down(Key::Escape) &&
		  data_window.is_open() && !data_window.is_key_down(Key::Escape) {
		let epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		if epoch - last_flushed <= 1.0 / FRAME_RATE && t < SIM_LEN {
			data.time.push(t);
			
			for i in 0..p.len() {
				p[i].a = Vector::zero();
				data.insert_particle_vector_len("position", i, p[i].pos);
				data.insert_particle_vector_len("velocity", i, p[i].v);	
				data.insert_particle_vector_len("accelleration", i, p[i].a);	
			}
			
			let scale = temperature::get_scale(&p, 0.0, 50.0);
			
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
				let f = borders::get_force(p[i].r, &p[i].pos);
				p[i].a += f;

				p[i].a = p[i].a / p[i].m; // Finally convert the force to accelleration
				p[i].v = p[i].v * scale; // Scale the temperature
				p[i].update(TIME_STEP);
			}

			t += TIME_STEP;
		} else {
			let keys = sim_window.get_keys_pressed(KeyRepeat::Yes);
			for key in keys {
				match key {
					Key::Up => { phi += 0.05; }
					Key::Down => { phi -= 0.05; }
					Key::Left => { theta -= 0.05; }
					Key::Right => { theta += 0.05; }
					Key::Minus => { if zoom > 0.1 { zoom += 0.1; }}
					Key::Equal => { zoom -= 0.1; }
					_ => { continue; }
				}
			}
			if phi > 3.15 {
				phi = -3.15;
			} else if phi < -3.15 {
				phi = 3.15;
			}
			if theta > 3.15 {
				theta = -3.15;
			} else if theta < -3.15 {
				theta = 3.15;
			}		

			sim_fb.clear_buffer();
			for i in 0..p.len(){
				spheres[i].set_position(p[i].pos.x, p[i].pos.y, p[i].pos.z);
			}
			let x = zoom * theta.cos() * phi.sin();
			let y = zoom * theta.sin() * phi.sin();
			let z = zoom * phi.cos();
		
			// not the nicest way to do things but it works
			// TODO: fr now use one type of vector
			engine.camera.set_position(Point3::new(x, y, z));
			engine.render(&[borders], |p| draw(p, &mut sim_fb));
			
			let camera = Vector::new(x, y, z);
			let mut r = Vec::new();
			for i in 0..p.len() {
				r.push((i, (camera-p[i].pos).sqlen()));
			}
			r.sort_unstable_by(|a,b| b.1.partial_cmp(&a.1).unwrap());
			for i in r {
				engine.render(&[spheres[i.0]], |p| draw(p, &mut sim_fb));
			}
			
			sim_window.update_with_buffer(sim_fb.borrow(), W, H)?;

			data.plot_global("temperature", last_plotted, plot.max_frequency(), |p1, p2| plot.plot_segment(p1, p2, Rgb888::new(255,0,0)));
			last_plotted = t;

//			chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 0).map(|(t, v)| {(t, v.x)}), &RED,))?;
			
//			{
//				let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
//					data_fb.borrow_mut(),
//					(W as u32, H as u32),
//				)?
//				.into_drawing_area();
//				{
//					let mut chart = cs.clone().restore(&root);
//					
//					chart.plotting_area().fill(&BLACK)?;
//
//					chart.configure_mesh().bold_line_style(&GREEN.mix(0.2)).light_line_style(&TRANSPARENT).draw()?;
//					
//					chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 1).map(|(t, v)| {(t, v.x)}), &GREEN,))?;
//					chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 2).map(|(t, v)| {(t, v.x)}), &MAGENTA,))?;
//					chart.draw_series(LineSeries::new(data.global_as_iter("temperature"), &YELLOW,))?;
//	//				chart.draw_series(LineSeries::new(data.particle_as_iter("force_electric", 0), &CYAN,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("force_vdw", 0).map(|(t, v)| {(t, v.x)}), &BLUE,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("force_vdw", 1).map(|(t, v)| {(t, v.x)}), &CYAN,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("force_vdw", 2).map(|(t, v)| {(t, v.x)}), &YELLOW,))?;
//					//chart.draw_series(LineSeries::new(data.particle_as_iter("energy_total", 0), &YELLOW,))?;
//					//chart.draw_series(LineSeries::new(data.particle_as_iter("energy_total", 1), &WHITE,))?;
//					//chart.draw_series(LineSeries::new(data.particle_as_iter("energy_total", 2), &CYAN,))?;
////					chart.draw_series(LineSeries::new(data.global_as_iter("energy_total"), &WHITE,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 0).map(|(t, v)| {(v.x, v.y, t)}), &RED,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 1).map(|(t, v)| {(v.x, v.y, t)}), &GREEN,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 2).map(|(t, v)| {(v.x, v.y, t)}), &MAGENTA,))?;
//
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_circles("position", 0, p[0].r, 200), &RED.mix(0.5),))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_circles("position", 1, p[1].r, 200), &GREEN.mix(0.5),))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_circles("position", 2, p[2].r, 200), &MAGENTA.mix(0.5),))?;
//					//chart.draw_series(LineSeries::new(f, &YELLOW,))?;
//				}
//				root.present()?;
//			}
			data_window.update_with_buffer(plot.fb.borrow(), W, H)?;

			last_flushed = epoch;
		}
	}
	
//	data.to_file("out.csv")?;

	Ok(())
}

fn make_xz_plane(y: f64) -> Vec<[f64; 3]> {
	let step = 1.0;
	let nsteps = 10;

	let mut vertices = Vec::new();
	for i in 0..nsteps {
		for j in 0..nsteps {
			vertices.push([
				i as f64 * step,
				y,
				j as f64 * step,
			]);
		}
	}

	vertices
}
fn make_xy_plane(z: f64) -> Vec<[f64; 3]> {
	let step = 1.0;
	let nsteps = 10;

	let mut vertices = Vec::new();
	for i in 0..nsteps {
		for j in 0..nsteps {
			vertices.push([
				i as f64 * step,
				j as f64 * step,
				z,
			]);
		}
	}

	vertices
}
fn make_yz_plane(x: f64) -> Vec<[f64; 3]> {
	let step = 1.0;
	let nsteps = 10;

	let mut vertices = Vec::new();
	for i in 0..nsteps {
		for j in 0..nsteps {
			vertices.push([
				x,
				i as f64 * step,
				j as f64 * step,
			]);
		}
	}

	vertices
}
