
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


/* MAIN */
// Display
/// Width of the graph
pub const W: usize = 800;
/// Height of the graph
pub const H: usize = 600;
/// Frame rate of the graph
pub const FRAME_RATE: f64 = 1.0;

// Simulation
/// Length of the simulation (time)
pub const SIM_LEN: f64 = 20.0;
/// Timestep of the simulation
pub const TIME_STEP: f64 = 0.001;
