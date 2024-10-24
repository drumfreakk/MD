
//! Several (natural) constants used in several places

/* FORCEFIELD */
// Temperature 
/// Boltzmann Constant
pub const BOLTZMANN_CONST: f64 = 1.0; 
/// Fallback temperature, to avoid divide by 0 
pub const FALLBACK_TEMPERATURE: f64 = 0.001;

// Van der Waals
/// 4 * the well depth of the Lennard-Jones potential. 
pub const LJ_4_EPSILON: f64 = 4.0;

// Electrostatic
// This should result in the relative strengths of the electrostatic and VdWs forces being roughly correct
// Even though the actual values are totally wrong
/// Elementary charge
pub const ELEMENTARY_CHARGE: f64 = 130.0;
/// Dielectric permittivity of a vacuum
pub const PERMITTIVITY_VACUUM: f64 = 1.0;

// Borders
/// The border of the system in the x direction. The other border is at the origin.
pub const BORDER_X: f64 = 10.0;
/// The border of the system in the y direction. The other border is at the origin.
pub const BORDER_Y: f64 = 10.0;
/// The border of the system in the z direction. The other border is at the origin.
pub const BORDER_Z: f64 = 10.0;
/// 4 times the repulsion strength of the borders.
pub const BORDER_4_EPSILON: f64 = 0.1;
/// The range at which the border potential is calculated.
pub const BORDER_RANGE: f64 = 1.0;

/* MAIN */
// Display
/// Width of the graph
pub const W: usize = 800;
/// Height of the graph
pub const H: usize = 600;
/// Frame rate of the graph
pub const FRAME_RATE: f64 = 5.0;

// Simulation
/// Length of the simulation (time)
pub const SIM_LEN: f64 = 150.0;
/// Timestep of the simulation
pub const TIME_STEP: f64 = 0.001;
