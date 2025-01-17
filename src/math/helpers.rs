use rand::Rng;
use crate::Charge;

pub fn string_to_tuple(input: &str) -> Result<(f32, f32, f32), &'static str> {
    /*
        This function converts input strings (of tuples) to actual tuples.
     */
    // Remove the parentheses
    let trimmed = input.trim_matches(|c| c == '(' || c == ')');

    // Split the string by commas
    let parts: Vec<&str> = trimmed.split(',').collect();

    // Ensure there are exactly 3 components
    if parts.len() != 3 {
        return Err("Input string must have exactly 3 components");
    }

    // Parse each component into a f32
    let x = parts[0].trim().parse::<f32>().map_err(|_| "Failed to parse x")?;
    let y = parts[1].trim().parse::<f32>().map_err(|_| "Failed to parse y")?;
    let z = parts[2].trim().parse::<f32>().map_err(|_| "Failed to parse z")?;

    // Return the tuple
    Ok((x, y, z))
}

pub fn generate_random_rgba() -> (f32, f32, f32, f32) {
    /*
        This function generates random color for each charge.
     */
    let mut rng = rand::rng();

    // Generate random values for r, g, b, a in the range [0.0, 1.0]
    let r = rng.random_range(0.0..=1.0);
    let g = rng.random_range(0.0..=1.0);
    let b = rng.random_range(0.0..=1.0);
    let a = rng.random_range(0.5..=1.0);

    // Return the RGBA tuple
    (r, g, b, a)
}

pub fn charge_to_radius(charge: f64) -> f64 {
    /*
            This function, scales charges into radii for their representative sphere.
        The radius grows by growth of charge in a gradual but slow rate.
     */

    // Define the original and new ranges
    let min_old = 1e-9; // Minimum charge (1e-9 C)
    let max_old = 1.0;  // Maximum charge (1 C)
    let min_new = 0.1;  // Minimum radius (1 unit)
    let max_new = 1f64; // Maximum radius (50 units)

    // Ensure the charge is within the original range
    let charge = (charge).abs().clamp(min_old, max_old);

    // Perform logarithmic scaling
    let log_charge = (charge / min_old).ln(); // Natural logarithm of normalized charge
    let log_max = (max_old / min_old).ln();  // Natural logarithm of max normalized charge

    min_new + (log_charge / log_max) * (max_new - min_new)
}

pub fn is_position_unique(position: &String, charge: &Vec<Charge>) -> bool {
    let p = string_to_tuple(&position).unwrap();
    for charge in charge {
        if charge.position == p {
            return false;
        }
    }
    true
}