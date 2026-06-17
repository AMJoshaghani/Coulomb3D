pub mod electrostatics {
    /*
    In this module, all physical calculations are defined, for more info please refer to each function.
    All equations need vector-algebra which is defined in algebra.rs mod.
    Functions defined:
    I.      Electric Potential ϕ
    II.     Electric Field E
    III.    Electric Dipole Moment p
    IV.     Field Line generation (RK4 integration)
    */
    use crate::Charge;
    use crate::math::{K, Vector3D};

    struct ChargeVector {
        charge: f64,
        position: Vector3D,
    }

    pub fn electric_potential(charges: &Vec<Charge>, s: &(f32, f32, f32)) -> f64 {
        /*
        Electric Potential
        ϕ(r′)= K ∑ (q_i / |r' - r|)
        */
        let mut phi = 0.0;
        for charge in charges {
            let c = charge_translator(charge);
            let r = separation_calculator(c.position, *s, true);
            let w = c.charge / r.magnitude();
            phi += w;
        }
        K * phi
    }

    pub fn electric_field(charges: &Vec<Charge>, s: &(f32, f32, f32)) -> Vector3D {
        /*
        Electric Field
        E(r′) = K ∑ (q_i / |r' - r|^3) . (r' - r)
        */
        let mut e = Vector3D::new(0.0, 0.0, 0.0);
        for charge in charges {
            let c = charge_translator(charge);
            let r = separation_calculator(c.position, *s, true);
            let w = r.scalar_product(&(c.charge / r.magnitude().powf(3.0)));
            e += w;
        }
        e = e.scalar_product(&K);
        e
    }

    // pub fn electric_dipole_moment(charges: &Vec<Charge>, s: &(f32, f32, f32)) -> Vector3D {
    //     /*
    //     Electric Dipole Moment,
    //     p = ∑ q_i (r_i − r′)
    //     */
    //     let mut p = Vector3D::new(0.0, 0.0, 0.0);
    //     for charge in charges {
    //         let c = charge_translator(charge);
    //         let w = separation_calculator(c.position, *s, false);
    //         let y = w.scalar_product(&c.charge);
    //         p += y;
    //     }
    //     p
    // }

    /// Multipole moments up to quadrupole (ℓ = 0, 1, 2),
    /// all computed relative to the expansion centre `s`.
    pub struct MultipoleMoments {
        /// ℓ=0  Monopole   Q   = ∑ qᵢ            [C]
        pub monopole: f64,
        /// ℓ=1  Dipole     pᵢ  = ∑ qₖ dᵢ         [C·m]
        pub dipole: Vector3D,
        /// ℓ=2  Quadrupole Qᵢⱼ = ∑ qₖ(3dᵢdⱼ − |d|²δᵢⱼ)  [C·m²]
        ///      Symmetric and traceless: Q_xx + Q_yy + Q_zz = 0 always.
        pub quadrupole: [[f64; 3]; 3],
    }

    /// Compute monopole, dipole, and (traceless) quadrupole moments
    /// for the given charge configuration relative to the reference point `s`.
    pub fn multipole_moments(charges: &Vec<Charge>, s: &(f32, f32, f32)) -> MultipoleMoments {
        let (sx, sy, sz) = (s.0 as f64, s.1 as f64, s.2 as f64);
        let mut monopole = 0.0f64;
        let mut dipole   = Vector3D::new(0.0, 0.0, 0.0);
        let mut quad     = [[0.0f64; 3]; 3];

        for ch in charges {
            let q = ch.charge;
            // displacement vector from expansion centre to this charge
            let d = [
                ch.position.0 as f64 - sx,
                ch.position.1 as f64 - sy,
                ch.position.2 as f64 - sz,
            ];
            let r2 = d[0]*d[0] + d[1]*d[1] + d[2]*d[2];

            // ℓ = 0 — monopole
            monopole += q;

            // ℓ = 1 — dipole
            dipole += Vector3D::new(q * d[0], q * d[1], q * d[2]);

            // ℓ = 2 — traceless quadrupole tensor
            for i in 0..3 {
                for j in 0..3 {
                    let kd = if i == j { 1.0 } else { 0.0 }; // Kronecker delta
                    quad[i][j] += q * (3.0 * d[i] * d[j] - r2 * kd);
                }
            }
        }

        MultipoleMoments { monopole, dipole, quadrupole: quad }
    }

    fn charge_translator(charge: &Charge) -> ChargeVector {
        ChargeVector {
            charge: charge.charge,
            position: Vector3D::new(
                charge.position.0 as f64,
                charge.position.1 as f64,
                charge.position.2 as f64,
            ),
        }
    }

    fn separation_calculator(pos: Vector3D, sv: (f32, f32, f32), reverse: bool) -> Vector3D {
        /*
        A separation vector between r and r`:
        * (r′ − r_i) – points from charge to reference  (reverse = true)
        * (r_i − r′) – points from reference to charge  (reverse = false)
        */
        let sv_vector = Vector3D::new(sv.0 as f64, sv.1 as f64, sv.2 as f64);
        if reverse { sv_vector - pos } else { pos - sv_vector }
    }

    // ── Field Line Tracing ────────────────────────────────────────────────────

    /// Return the normalised E-field direction at `pos`, scaled by `fwd` (+1 / −1).
    /// Returns None when the field magnitude is negligibly small.
    fn eval_dir(
        charges: &Vec<Charge>,
        pos: (f32, f32, f32),
        fwd: f32,
    ) -> Option<(f32, f32, f32)> {
        let e = electric_field(charges, &pos);
        let m = e.magnitude();
        if m < 1e-20 {
            return None;
        }
        let (ex, ey, ez) = e.components();
        Some((
            (ex / m * fwd as f64) as f32,
            (ey / m * fwd as f64) as f32,
            (ez / m * fwd as f64) as f32,
        ))
    }

    #[inline]
    fn step_pos(p: (f32, f32, f32), d: (f32, f32, f32), s: f32) -> (f32, f32, f32) {
        (p.0 + d.0 * s, p.1 + d.1 * s, p.2 + d.2 * s)
    }

    /// Trace a single electric field line using 4th-order Runge–Kutta integration.
    ///
    /// `fwd = +1.0` follows the field (away from +charges),
    /// `fwd = −1.0` runs against it (away from −charges, tracing where lines come from).
    fn trace_rk4(
        charges: &Vec<Charge>,
        start: (f32, f32, f32),
        step: f32,
        max_steps: usize,
        bounds: f32,
        fwd: f32,
    ) -> Vec<(f32, f32, f32)> {
        let mut pts = vec![start];
        let mut p = start;

        for _ in 0..max_steps {
            // RK4 slopes
            let k1 = match eval_dir(charges, p, fwd) {
                Some(k) => k,
                None => break,
            };
            let k2 = match eval_dir(charges, step_pos(p, k1, step * 0.5), fwd) {
                Some(k) => k,
                None => break,
            };
            let k3 = match eval_dir(charges, step_pos(p, k2, step * 0.5), fwd) {
                Some(k) => k,
                None => break,
            };
            let k4 = match eval_dir(charges, step_pos(p, k3, step), fwd) {
                Some(k) => k,
                None => break,
            };

            // Weighted average
            let d = (
                (k1.0 + 2.0 * k2.0 + 2.0 * k3.0 + k4.0) / 6.0,
                (k1.1 + 2.0 * k2.1 + 2.0 * k3.1 + k4.1) / 6.0,
                (k1.2 + 2.0 * k2.2 + 2.0 * k3.2 + k4.2) / 6.0,
            );
            let np = step_pos(p, d, step);

            // Stop if the new point is outside the bounding box
            if np.0.abs() > bounds || np.1.abs() > bounds || np.2.abs() > bounds {
                break;
            }

            // Stop if the line gets absorbed by any charge (avoids numerical blow-up)
            let absorbed = charges.iter().any(|c| {
                let dx = np.0 - c.position.0;
                let dy = np.1 - c.position.1;
                let dz = np.2 - c.position.2;
                (dx * dx + dy * dy + dz * dz).sqrt() < 0.12
            });
            if absorbed {
                break;
            }

            p = np;
            pts.push(p);
        }
        pts
    }

    /// Generate electric field lines for the given charge configuration.
    ///
    /// Seeds are placed on a small Fibonacci sphere around each charge.
    /// Positive charges emit lines forward along E; negative charges emit
    /// lines backward (showing the paths that terminate there).
    ///
    /// Returns `(polylines, half_size_of_bounding_cube)`.
    pub fn generate_field_lines(
        charges: &Vec<Charge>,
    ) -> (Vec<Vec<(f32, f32, f32)>>, f32) {
        if charges.is_empty() {
            return (Vec::new(), 0.0);
        }

        const SEEDS: usize = 12;   // seed points per charge
        const SR:    f32   = 0.35; // seed sphere radius (metres)
        const STEP:  f32   = 0.1;  // RK4 integration step
        const MAX:   usize = 300;  // max steps per line

        // Bounding half-size: large enough to contain all charges plus a margin
        let bounds = charges.iter().fold(4.0f32, |acc, c| {
            acc.max(
                c.position.0.abs()
                    .max(c.position.1.abs())
                    .max(c.position.2.abs())
                    + 3.0,
            )
        });

        // Fibonacci sphere golden angle
        let ga = std::f32::consts::PI * (3.0 - 5.0f32.sqrt());
        let mut lines: Vec<Vec<(f32, f32, f32)>> = Vec::new();

        for ch in charges {
            let (cx, cy, cz) = ch.position;
            // Forward along E for positive charges, backward for negative
            let fwd = if ch.charge >= 0.0 { 1.0f32 } else { -1.0 };

            for i in 0..SEEDS {
                // Fibonacci sphere distribution
                let t  = if SEEDS > 1 { i as f32 / (SEEDS - 1) as f32 } else { 0.5 };
                let y  = 1.0 - 2.0 * t;
                let r  = (1.0 - y * y).sqrt();
                let th = ga * i as f32;

                let seed = (
                    cx + SR * r * th.cos(),
                    cy + SR * y,
                    cz + SR * r * th.sin(),
                );

                let line = trace_rk4(charges, seed, STEP, MAX, bounds, fwd);
                if line.len() > 3 {
                    lines.push(line);
                }
            }
        }

        (lines, bounds)
    }
}