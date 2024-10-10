
use crate::constants::{ELECTRON_CHARGE, PERMITTIVITY_VACUUM};

/**
 * Gets the potential energy due to electrostatic interactions
 * Technically speaking this is the work required to assemble a configuration
 *
 * @param (f64, f64) charges The charges of the two particles
 *		Can be absolute charges or in terms of the charge of an electron.
 * @param f64 distance The distance between the centres of the particles
 * @param bool normalised Whether the charges are absolute (false), or in terms of the charge of an electron (true)
 * @return f64 The electrostatic energy potential
 */
pub fn get_energy(charges: (f64, f64), distance: f64, normalised: bool) -> f64 {
// qi * qj / (4pi*e0*rij)
	let normalisation = if normalised { ELECTRON_CHARGE * ELECTRON_CHARGE } else { 1.0 };
	return charges.0 * charges.1 * normalisation / (4.0 * std::f64::consts::PI * PERMITTIVITY_VACUUM * distance);
}

/**
 * Gets the magnitude of the electrostatic force between two particles
 *
 * @param (f64, f64) charges The charges of the two particles
 *		Can be absolute charges or in terms of the charge of an electron.
 * @param f64 distance The distance between the centres of the particles
 * @param bool normalised Whether the charges are absolute (false), or in terms of the charge of an electron (true)
 * @return f64 The electrostatic energy potential
 */
pub fn get_force(charges: (f64, f64), distance: f64, normalised: bool) -> f64 {
	// qi * qj / (4pi*e0 * rij^2)
	return get_energy(charges, distance, normalised) / distance;
}
