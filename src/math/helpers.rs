use rand::Rng;
use crate::Charge;

pub fn string_to_tuple(input: &str) -> Result<(f32, f32, f32), &'static str> {
    /*
    This function converts input strings (of tuples) to actual tuples.
    */
    let trimmed = input.trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = trimmed.split(',').collect();

    if parts.len() != 3 {
        return Err("Input string must have exactly 3 components");
    }

    let x = parts[0].trim().parse::<f32>().map_err(|_| "Failed to parse x")?;
    let y = parts[1].trim().parse::<f32>().map_err(|_| "Failed to parse y")?;
    let z = parts[2].trim().parse::<f32>().map_err(|_| "Failed to parse z")?;

    Ok((x, y, z))
}

pub fn generate_random_rgba() -> (f32, f32, f32, f32) {
    let mut rng = rand::rng();
    let r = rng.random_range(0.0..=1.0);
    let g = rng.random_range(0.0..=1.0);
    let b = rng.random_range(0.0..=1.0);
    let a = rng.random_range(0.5..=1.0);
    (r, g, b, a)
}

pub fn charge_to_radius(charge: f64) -> f64 {
    let min_old = 1e-9;
    let max_old = 1.0;
    let min_new = 0.1;
    let max_new = 1f64;

    let charge = (charge).abs().clamp(min_old, max_old);
    let log_charge = (charge / min_old).ln();
    let log_max   = (max_old / min_old).ln();

    min_new + (log_charge / log_max) * (max_new - min_new)
}

pub fn is_position_unique(position: &String, charge: &Vec<Charge>) -> bool {
    let p = string_to_tuple(&position).unwrap();
    for c in charge {
        if c.position == p {
            return false;
        }
    }
    true
}