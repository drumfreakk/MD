#![allow(dead_code)]
#![allow(unused_imports)]

//! Run a molecular dynamics simulation

//TODO: Error checking/handling
#![warn(missing_docs)]

mod constants;
mod vectors;
mod particles;
mod forcefield;
mod log_data;

mod embedded_gfx;

mod icosphere;

use crate::constants::{W, H, FRAME_RATE, SIM_LEN, TIME_STEP};

use crate::vectors::Vector;
use crate::particles::Particle;
use crate::log_data::DataLog;
use crate::forcefield::{temperature, vanderwaals, electrostatic};

use minifb::{Window, WindowOptions, Key, KeyRepeat};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::time::SystemTime;

use std::time::Instant;


use crate::embedded_gfx::framebuffer::BufferWrapper;
use crate::embedded_gfx::mesh::K3dMesh;
use crate::embedded_gfx::{
	draw::draw,
	mesh::{Geometry, RenderMode},
	K3dengine,
};
use embedded_graphics::Drawable;
use embedded_graphics::{
	geometry::Point,
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	text::Text,
};
use embedded_graphics_core::pixelcolor::{Rgb888, WebColors};
use nalgebra::Point3;

use crate::embedded_gfx::DrawPrimitive;
use embedded_graphics_core::pixelcolor::{RgbColor, IntoStorage};


fn make_xz_plane() -> Vec<[f64; 3]> {
	let step = 1.0;
	let nsteps = 10;

	let mut vertices = Vec::new();
	for i in 0..nsteps {
		for j in 0..nsteps {
			vertices.push([
				(i as f64 - nsteps as f64 / 2.0) * step,
				0.0,
				(j as f64 - nsteps as f64 / 2.0) * step,
			]);
		}
	}

	vertices
}

fn main () -> Result<(), Box<dyn Error>> {
	let mut fb = BufferWrapper(vec![0u32; W * H]);
	let mut window = Window::new("MD Sim", W, H, WindowOptions::default(),)?;
	
	let mut t = 0.0;

//	let cs = {
//		let root =
//			BitMapBackend::<BGRXPixel>::with_buffer_and_format(fb.borrow_mut(), (W as u32, H as u32))?.into_drawing_area();
//		root.fill(&BLACK)?;
//
//		//let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_2d(0.0..SIM_LEN, -100.0..100.0)?;
//		//chart.configure_mesh().label_style(("sans-serif", 15).into_font().color(&GREEN)).axis_style(&GREEN).draw()?;
//		//let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_3d(-5.0..5.0, -5.0..5.0, 0.0..SIM_LEN)?;
//		let mut chart = ChartBuilder::on(&root).margin(10).set_all_label_area_size(30).build_cartesian_3d(-5.0..5.0, -5.0..5.0, -5.0..5.0)?;
//		chart.configure_axes().label_style(("sans-serif", 15).into_font().color(&GREEN)).axis_panel_style(&GREEN).draw()?;
//		chart.with_projection(|mut pb| {
//			pb.yaw = yaw;
//			pb.pitch = pitch;
//			pb.scale = scale;
//			pb.into_matrix()
//		});
//
//		let cs = chart.into_chart_state();
//		root.present()?;
//		cs
//	};

	let start_ts = SystemTime::now();
	let mut last_flushed = 0.0;

	let mut engine = K3dengine::new(W as u16, H as u16);
	engine.camera.set_position(Point3::new(10.0, 10.0, 0.0));
	engine.camera.set_target(Point3::new(0.0, 0.0, 0.0));
	engine.camera.set_fovy(3.141592 / 4.0);
	
	let mut angle: f64 = 0.0;


	//TODO: multiple graphs, split this file up

	//let mut p = vec![Particle::new(&(Vector::unit_x() * -5.0), 1.00, 1.0,  0.0, None, None),
	//		 		 Particle::new(&Vector::zero(),			1.00, 1.0,  0.0, None, None),
	//		 		 Particle::new(&(Vector::unit_x() *  5.0), 1.00, 1.0,  0.0, None, None)];
	let mut p = vec![Particle::new(&Vector::new(-3.0, -2.0, -2.0),	1.0, 3.0,  0.0, None, None),
			 		 Particle::new(&Vector::zero(),					1.0, 1.0,  0.0, None, None),
			 		 Particle::new(&Vector::new(0.0, 5.0, 0.0),		1.5, 1.0,  0.0, None, None)];

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
	let mut sphere = K3dMesh::new(Geometry {
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
	}
	spheres[0].set_color(Rgb888::new(255,0,0));
	spheres[1].set_color(Rgb888::new(0,255,0));
	spheres[2].set_color(Rgb888::new(0,0,255));

	while window.is_open() && !window.is_key_down(Key::Escape) {
		let mut epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		if epoch - last_flushed <= 1.0 / FRAME_RATE && t < SIM_LEN {
			data.time.push(t);
			
			for i in 0..p.len() {
				p[i].a = Vector::zero();
				data.insert_particle_vector_len("position", i, p[i].pos);
				data.insert_particle_vector_len("velocity", i, p[i].v);	
				data.insert_particle_vector_len("accelleration", i, p[i].a);	
			}
			
			let scale = temperature::get_scale(&p, 0.0, 5.0);
			
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
			epoch = SystemTime::now().duration_since(start_ts).unwrap().as_secs_f64();
		} else {
			println!("Rendering t={}", t);
			let now = Instant::now();
			fb.clear_buffer();
			for i in 0..p.len(){
				spheres[i].set_position(p[i].pos.x, p[i].pos.y, p[i].pos.z);
				spheres[i].set_render_mode(RenderMode::SolidLightDir(nalgebra::Vector3::new(angle.cos(),0.0,angle.sin())));
			}
			//engine.camera.set_position(Point3::new(5.0 * angle.cos(), 8.0, 5.0 * angle.sin()));
			engine.render(&spheres, |p| draw(p, &mut fb));
			window.update_with_buffer(fb.borrow(), W, H)?;
			//angle += 0.005;

//			{
//				let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
//					fb.borrow_mut(),
//					(W as u32, H as u32),
//				)?
//				.into_drawing_area();
//				{
//					let mut chart = cs.clone().restore(&root);
//					
//					chart.plotting_area().fill(&BLACK)?;
//
//					//chart.configure_mesh().bold_line_style(&GREEN.mix(0.2)).light_line_style(&TRANSPARENT).draw()?;
//					
//					chart.with_projection(|mut pb| {
//							pb.yaw = yaw;
//							pb.pitch = pitch;
//							pb.scale = scale;
//							pb.into_matrix()
//						});
//					chart.configure_axes().bold_grid_style(&GREEN.mix(0.2)).light_grid_style(&TRANSPARENT).draw()?;
//
//					
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 0).map(|(t, v)| {(t, v.x)}), &RED,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 1).map(|(t, v)| {(t, v.x)}), &GREEN,))?;
//					//chart.draw_series(LineSeries::new(data.particle_vector_as_iter("position", 2).map(|(t, v)| {(t, v.x)}), &MAGENTA,))?;
//					//chart.draw_series(LineSeries::new(data.global_as_iter("temperature"), &YELLOW,))?;
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
//					chart.draw_series(LineSeries::new(data.particle_vector_as_sphere("position", 0, p[0].r, data.time.len() - 1), &RED.mix(0.5),))?;
//					chart.draw_series(LineSeries::new(data.particle_vector_as_sphere("position", 1, p[1].r, data.time.len() - 1), &GREEN.mix(0.5),))?;
//					chart.draw_series(LineSeries::new(data.particle_vector_as_sphere("position", 2, p[2].r, data.time.len() - 1), &MAGENTA.mix(0.5),))?;
//					//chart.draw_series(LineSeries::new(f, &YELLOW,))?;
//				}
//				root.present()?;
//
//				let keys = window.get_keys_pressed(KeyRepeat::Yes);
//				for key in keys {
//					match key {
//						Key::Up => {
//							pitch += 0.05;
//						}
//						Key::Down => {
//							pitch -= 0.05;
//						}
//						Key::Left => {
//							yaw -= 0.05;
//						}
//						Key::Right => {
//							yaw += 0.05;
//						}
//						Key::Minus => {
//							if scale > 0.1 {
//								scale -= 0.1;
//							}
//						}
//						Key::Equal => {
//							scale += 0.1;
//						}
//						_ => {
//							continue;
//						}
//					}
//				}
//				if pitch > 3.15 {
//					pitch = -3.15;
//				} else if pitch < -3.15 {
//					pitch = 3.15;
//				}
//				if yaw > 3.15 {
//					yaw = -3.15;
//				} else if yaw < -3.15 {
//					yaw = 3.15;
//				}
//				window.set_title(&get_window_title(t, pitch, yaw));
//			}
//
//			window.update_with_buffer(fb.borrow(), W, H)?;
			println!("Elapsed: {:.2?}", now.elapsed());
		
			
		

			last_flushed = epoch;
		}
	}
	
	data.to_file("out.csv")?;

	Ok(())
}

