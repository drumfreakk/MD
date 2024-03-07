
//TODO: Error checking/handling
// Either do the "simple" algebraic method with just elastic collisions,
// or start doing things with energy landscapes

#[macro_use]
mod particles;
#[macro_use]
mod vectors;

mod output;

fn main () {
	let mut p = [particle!(vector!(0,0)), particle!(vector!(5,0))];

	p[0].r = 1.0;
	p[0].v = vector!(1,0);
	p[0].a = vector!(0.5,0);
	p[1].r = 1.0;

	for i in 0..300 {
		let d = p[0].collision_dist(&p[1]);
		println!("{}: {}", i, d);
		if d <= 0.0 {
			break;
		}
		p[0].update(0.05);
	}

	output::log_position(&p, "out.txt");
}

