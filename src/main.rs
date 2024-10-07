
//TODO: Error checking/handling

mod vectors;
mod particles;
mod output;

use crate::vectors::Vector;
use crate::particles::Particle;


use minifb::{Key, KeyRepeat, Window, WindowOptions};
use plotters::prelude::*;
use plotters_bitmap::bitmap_pixel::BGRXPixel;
use plotters_bitmap::BitMapBackend;
use std::collections::VecDeque;
use std::error::Error;
use std::time::SystemTime;
use std::borrow::{Borrow, BorrowMut};
const W: usize = 800;
const H: usize = 600;

const SAMPLE_RATE: f64 = 10_000.0;
const FRAME_RATE: f64 = 30.0;

struct BufferWrapper(Vec<u32>);
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

fn get_window_title(fx: f64, fy: f64, iphase: f64) -> String {
    format!(
        "x={:.1}Hz, y={:.1}Hz, phase={:.1} +/-=Adjust y 9/0=Adjust x <Esc>=Exit",
        fx, fy, iphase
    )
}

// Gives the Lennard-Jones Van der Waals potential for particle at distance
// Distance should be between the center of particle and the edge of the second particle?
fn LJ_VdW_pot(particle: &Particle, distance: f64) -> f64 {
    //4e ( (s/r)^12 - (s/r)^6 )
    // r is dist, s is radius, e is well depth
  
//TODO: cutoff, differentiate manually to get force & save a step

    let epsilon = 5_f64;

    let attraction = (particle.r / distance).powf(6.0);
    let repulsion = attraction * attraction;

    return 4_f64 * epsilon * (repulsion - attraction); 
}



fn main () -> Result<(), Box<dyn Error>> {
	let mut p = [Particle::new(&Vector::zero(), 1.0, 1.0, None, None),
                 Particle::new(&Vector::new(5.0,0.0,0.0), 1.0, 1.0, None, None)];


    let mut positions_f = Vec::new();
    let mut positions_s = Vec::new();

    let mut positions_f_r = Vec::new();
    let mut positions_s_r = Vec::new();
 
	let mut buf = BufferWrapper(vec![0u32; W * H]);

    let mut fx: f64 = 1.0;
    let mut fy: f64 = 1.1;
    let mut xphase: f64 = 0.0;
    let mut yphase: f64 = 0.1;

    let mut window = Window::new(
        &get_window_title(fx, fy, yphase - xphase),
        W,
        H,
        WindowOptions::default(),
    )?;
    let cs = {
        let root =
            BitMapBackend::<BGRXPixel>::with_buffer_and_format(buf.borrow_mut(), (W as u32, H as u32))?
                .into_drawing_area();
        root.fill(&BLACK)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .set_all_label_area_size(30)
            .build_cartesian_2d(-0.0..10.0, -0.0..5.0)?;

        chart
            .configure_mesh()
            .label_style(("sans-serif", 15).into_font().color(&GREEN))
            .axis_style(&GREEN)
            .draw()?;

        let cs = chart.into_chart_state();
        root.present()?;
        cs
    };

    let mut data = VecDeque::new();
    let start_ts = SystemTime::now();
    let mut last_flushed = 0.0;

	let mut phase_y = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let epoch = SystemTime::now()
            .duration_since(start_ts)
            .unwrap()
            .as_secs_f64();

	positions_f.push((phase_y as f64/500.0, p[0].pos.x));
        positions_s.push((phase_y as f64/500.0, p[1].pos.x));
//        vels.push((phase_y as f64/500.0, p[0].v.x));
        positions_f_r.push((phase_y as f64/500.0, p[0].pos.x + p[0].r));
        positions_s_r.push((phase_y as f64/500.0, p[1].pos.x - p[1].r));
//        accs.push((phase_y as f64/500.0, p[0].a.x));

        //positions.push((phase_y as f64/500.0, p[0].pos.len()));
        //vels.push((phase_y as f64/500.0, p[0].v.len()));
        //accs.push((phase_y as f64/500.0, p[0].a.len()));

        let separation = p[0].separation(&p[1]);
        let sep_dist = separation.len();
        let sep_dist_r_first = sep_dist - p[0].r;
        let sep_dist_r_second = sep_dist - p[1].r;

        let v_d_f = LJ_VdW_pot(&p[1], sep_dist_r_first);
        let v_d_dx_f = LJ_VdW_pot(&p[1], sep_dist_r_first * 0.99);
        let a_f = separation * (v_d_f - v_d_dx_f) / ( p[0].m * sep_dist_r_first * 0.01 * sep_dist );
        
		let v_d_s = LJ_VdW_pot(&p[1], sep_dist_r_second);
        let v_d_dx_s = LJ_VdW_pot(&p[1], sep_dist_r_second * 0.99);
        let a_s = p[1].separation(&p[0]) * (v_d_s - v_d_dx_s) / ( p[0].m * sep_dist_r_second * 0.01 * sep_dist );
        p[0].a = a_f;
        p[1].a = a_s;
        p[0].v = p[0].v * 0.99;
        p[1].v = p[1].v * 0.99;
        //		p[0].v = p[0].v * (1_f64 - (phase_y as f64/8000.0)); // Damping, make sure it doesn't endlessly
                                                       // oscillate around
//        E_pot.push((phase_y as f64/500.0, v_d));
 //       E_kin.push((phase_y as f64/500.0, p[0].v.sqlen() * p[0].m * 0.5));
  //      E.push((phase_y as f64/500.0, v_d + E_kin[E_kin.len() - 1].1));
        p[0].update(0.01);
        p[1].update(0.01);


		let phase_x = positions_f[positions_f.len() - 1].1;
		phase_y += 0.01;
   //     let phase_x = 2.0 * epoch * std::f64::consts::PI * fx + xphase;
   //     let phase_y = 2.0 * epoch * std::f64::consts::PI * fy + yphase;
        data.push_back((epoch, phase_x, phase_y));

        if epoch - last_flushed > 1.0 / FRAME_RATE {
            {
                let root = BitMapBackend::<BGRXPixel>::with_buffer_and_format(
                    buf.borrow_mut(),
                    (W as u32, H as u32),
                )?
                .into_drawing_area();
                {
                    let mut chart = cs.clone().restore(&root);
                    chart.plotting_area().fill(&BLACK)?;

                    chart
                        .configure_mesh()
                        .bold_line_style(&GREEN.mix(0.2))
                        .light_line_style(&TRANSPARENT)
                        .draw()?;

                    chart.draw_series(data.iter().zip(data.iter().skip(1)).map(
                        |(&(e, x0, y0), &(_, x1, y1))| {
                            PathElement::new(
                                vec![(x0, y0), (x1, y1)],
                                &GREEN,//.mix(((e - epoch) * 20.0).exp()),
                            )
                        },
                    ))?;
                }
                root.present()?;

                let keys = window.get_keys_pressed(KeyRepeat::Yes);
                for key in keys {
                    let old_fx = fx;
                    let old_fy = fy;
                    match key {
                        Key::Equal => {
                            fy += 0.1;
                        }
                        Key::Minus => {
                            fy -= 0.1;
                        }
                        Key::Key0 => {
                            fx += 0.1;
                        }
                        Key::Key9 => {
                            fx -= 0.1;
                        }
                        _ => {
                            continue;
                        }
                    }
                    xphase += 2.0 * epoch * std::f64::consts::PI * (old_fx - fx);
                    yphase += 2.0 * epoch * std::f64::consts::PI * (old_fy - fy);
                    window.set_title(&get_window_title(fx, fy, yphase - xphase));
                }
            }

            window.update_with_buffer(buf.borrow(), W, H)?;
            last_flushed = epoch;
        }

        while let Some((e, _, _)) = data.front() {
            if ((e - epoch) * 20.0).exp() > 0.1 {
                break;
            }
            data.pop_front();
        }
    }
/*
	let mut p = [Particle::new(&Vector::zero(), 1.0, 1.0, None, None),
                 Particle::new(&Vector::new(5.0,0.0,0.0), 1.0, 1.0, None, None)];


    let root = BitMapBackend::new("out.png", (1000, 1000)).into_drawing_area();
    //let root = BitMapBackend::gif("out.gif", (640, 480), 5).unwrap().into_drawing_area();
    let mut positions_f = Vec::new();
    let mut positions_s = Vec::new();

    let mut positions_f_r = Vec::new();
    let mut positions_s_r = Vec::new();
    

	root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..10f64, 0f64..5f64)?;

    chart.configure_mesh().draw()?;
	for phase_y in 0..5000 {


        positions_f.push((phase_y as f64/500.0, p[0].pos.x));
        positions_s.push((phase_y as f64/500.0, p[1].pos.x));
//        vels.push((phase_y as f64/500.0, p[0].v.x));
        positions_f_r.push((phase_y as f64/500.0, p[0].pos.x + p[0].r));
        positions_s_r.push((phase_y as f64/500.0, p[1].pos.x - p[1].r));
//        accs.push((phase_y as f64/500.0, p[0].a.x));

        //positions.push((phase_y as f64/500.0, p[0].pos.len()));
        //vels.push((phase_y as f64/500.0, p[0].v.len()));
        //accs.push((phase_y as f64/500.0, p[0].a.len()));

        let separation = p[0].separation(&p[1]);
        let sep_dist = separation.len();
        let sep_dist_r_first = sep_dist - p[0].r;
        let sep_dist_r_second = sep_dist - p[1].r;

        let v_d_f = LJ_VdW_pot(&p[1], sep_dist_r_first);
        let v_d_dx_f = LJ_VdW_pot(&p[1], sep_dist_r_first * 0.99);
        let a_f = separation * (v_d_f - v_d_dx_f) / ( p[0].m * sep_dist_r_first * 0.01 * sep_dist );
        
		let v_d_s = LJ_VdW_pot(&p[1], sep_dist_r_second);
        let v_d_dx_s = LJ_VdW_pot(&p[1], sep_dist_r_second * 0.99);
        let a_s = p[1].separation(&p[0]) * (v_d_s - v_d_dx_s) / ( p[0].m * sep_dist_r_second * 0.01 * sep_dist );
        p[0].a = a_f;
        p[1].a = a_s;
        p[0].v = p[0].v * 0.99;
        p[1].v = p[1].v * 0.99;
        //		p[0].v = p[0].v * (1_f64 - (phase_y as f64/8000.0)); // Damping, make sure it doesn't endlessly
                                                       // oscillate around
//        E_pot.push((phase_y as f64/500.0, v_d));
 //       E_kin.push((phase_y as f64/500.0, p[0].v.sqlen() * p[0].m * 0.5));
  //      E.push((phase_y as f64/500.0, v_d + E_kin[E_kin.len() - 1].1));
        p[0].update(0.01);
        p[1].update(0.01);

 
    }
    chart.draw_series(LineSeries::new(positions_f, &RED,))?;
    chart.draw_series(LineSeries::new(positions_s, &GREEN,))?;
//    chart.draw_series(LineSeries::new(vels, &GREEN,))?;
    chart.draw_series(LineSeries::new(positions_f_r, &RED,))?;
    chart.draw_series(LineSeries::new(positions_s_r, &GREEN,))?;
//    chart.draw_series(LineSeries::new(accs, &BLUE,))?;
//    chart.draw_series(LineSeries::new(E_pot, &BLACK,))?;
//    chart.draw_series(LineSeries::new(E_kin, &MAGENTA,))?;
//    chart.draw_series(LineSeries::new(E, &YELLOW,))?;
    root.present()?;

	output::log_position(&p, "out.txt");
*/
   Ok(())
}

