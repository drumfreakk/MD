
//TODO: Error checking/handling
// Either do the "simple" algebraic method with just elastic collisions,
// or start doing things with energy landscapes

mod vectors;
mod particles;
mod output;

use crate::vectors::Vector;
use crate::particles::Particle;

use plotters::prelude::*;

// Gives the Lennard-Jones Van der Waals potential for particle at distance
// Distance should be between the center of particle and the edge of the second particle?
fn LJ_VdW_pot(particle: &Particle, distance: f32) -> f32 {
    //4e ( (s/r)^12 - (s/r)^6 )
    // r is dist, s is radius, e is well depth
   
    let epsilon = 5_f32;

    let attraction = (particle.r / distance).powf(6.0);
    let repulsion = attraction * attraction;

    return 4_f32 * epsilon * (repulsion - attraction); 
}



fn main () -> Result<(), Box<dyn std::error::Error>> {
	let mut p = [Particle::new(&Vector::zero(), 1.0, 1.0, None, None),
                 Particle::new(&Vector::new(3.0,0.0,0.0), 1.0, 1.0, None, None)];


    //let root = BitMapBackend::new("out.png", (640, 480)).into_drawing_area();
    let root = BitMapBackend::gif("out.gif", (640, 480), 5).unwrap().into_drawing_area();
    let mut positions = Vec::new();
    let mut vels = Vec::new();
    let mut accs = Vec::new();
    let mut E = Vec::new();

	for i in 0..1000 {
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..4f32, -5f32..1f32)?;

    chart.configure_mesh().draw()?;


        positions.push((i as f32/500.0, p[0].pos.len()));
        vels.push((i as f32/500.0, p[0].v.len()));
        accs.push((i as f32/500.0, p[0].a.len()));


        let separation = p[0].separation(&p[1]);
        let sep_dist = separation.len();
        let sep_dist_r = sep_dist - p[0].r;
        let v_d = LJ_VdW_pot(&p[1], sep_dist_r);
        let v_d_dx = LJ_VdW_pot(&p[1], sep_dist_r * 0.99);
        let a = separation * (v_d - v_d_dx) / ( p[0].m * sep_dist_r * 0.99 * sep_dist );
        p[0].a = a;
        p[0].v = p[0].v * 0.99;
        //		p[0].v = p[0].v * (1_f32 - (i as f32/8000.0)); // Damping, make sure it doesn't endlessly
                                                       // oscillate around
        E.push((i as f32/500.0, v_d));
        println!("{}", v_d);
        p[0].update(0.05);

    chart.draw_series(LineSeries::new(positions.clone(), &RED,))?;
    chart.draw_series(LineSeries::new(vels.clone(), &GREEN,))?;
    chart.draw_series(LineSeries::new(accs.clone(), &BLUE,))?;
    chart.draw_series(LineSeries::new(E.clone(), &BLACK,))?;

    root.present()?;
 
    }

	output::log_position(&p, "out.txt");

   Ok(())
}

