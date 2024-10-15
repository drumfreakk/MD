
//! The electrostatic potential/force, according to Coulomb's law.

use crate::constants::{ELEMENTARY_CHARGE, PERMITTIVITY_VACUUM};

/** Gets the potential energy due to electrostatic interactions. 

Technically speaking this is the work required to assemble the given configuration. 
Calculated between the centres of the particles.
If the charges are normalised, the charges are first multiplied by ELEMENTARY_CHARGE.
*/
pub fn get_energy(charges: (f64, f64), distance: f64, normalised: bool) -> f64 {
// qi * qj / (4pi*e0*rij)
	let normalisation = if normalised { ELEMENTARY_CHARGE * ELEMENTARY_CHARGE } else { 1.0 };
	return charges.0 * charges.1 * normalisation / (4.0 * std::f64::consts::PI * PERMITTIVITY_VACUUM * distance);
}

/** Gets the magnitude of the electrostatic force between two particles.

Calculated between the centres of the particles.
If the charges are normalised, the charges are first multiplied by ELEMENTARY_CHARGE.
*/
pub fn get_force(charges: (f64, f64), distance: f64, normalised: bool) -> f64 {
	// qi * qj / (4pi*e0 * rij^2)
	return get_energy(charges, distance, normalised) / distance;
}
