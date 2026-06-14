/*
MATH MODULE, CONTAINS MATH/PHYSICS-RELATED FUNCTIONALITY:
* VECTOR ALGEBRA
* ELECTRODYNAMICS
+ also a `helpers` module
*/

// DEFINING CONSTANTS
const PI: f64 = std::f64::consts::PI; // π
const EPSILON: f64 = 8.8541878128E-12; // ε₀ vacuum permittivity
pub const K: f64 = 1f64 / (4f64 * PI * EPSILON); // Coulomb's Constant 1/4πε₀

// IMPORTING MODS
pub mod algebra;
pub mod helpers;
pub mod physics;

// EXPORTING FUNCTIONS
pub use algebra::vector::*;
pub use helpers::*;
pub use physics::electrostatics::*; // includes generate_field_lines