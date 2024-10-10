
/** FORCEFIELD **/
// Temperature
pub const BOLTZMANN_CONST: f64 = 1.0; 
pub const FALLBACK_TEMPERATURE: f64 = 0.001;

// Van der Waals
pub const LJ_4_EPSILON: f64 = 4.0;

// Electrostatic
// This should result in the relative strengths of the electrostatic and VdWs forces being roughly correct
// Even though the actual values are totally wrong
pub const ELECTRON_CHARGE: f64 = 130.0; 
pub const PERMITTIVITY_VACUUM: f64 = 1.0;


/** MAIN **/
// Display
pub const W: usize = 800;
pub const H: usize = 600;
pub const FRAME_RATE: f64 = 1.0;

// Simulation
pub const SIM_LEN: f64 = 10.0;
pub const TIME_STEP: f64 = 0.001;
