pub mod electrostatics {
    /*
            In this module, all physical calculations are defined, for more info please refer to each function.
        All equations need vector-algebra which is defined in algebra.rs mod.
        Functions defined:
            I.      Electric Potential ϕ
            II.     Electric Field E
            III.    Electric Dipole Moment p
     */
    use crate::Charge;
    use crate::math::{K, Vector3D};

    struct ChargeVector { // for transforming ui-friendly charge definition to a math-friendly one
        charge: f64,
        position: Vector3D
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
        let mut e = Vector3D::new(0.0,0.0,0.0);
        for charge in charges {
            let c = charge_translator(charge);
            let r = separation_calculator(c.position, *s, true);
            let w = r.scalar_product(&(c.charge / r.magnitude().powf(3.0)));
            e += w;
        }
        e = e.scalar_product(&K);
        e
    }
    pub fn electric_dipole_moment(charges: &Vec<Charge>, s: &(f32, f32, f32)) -> Vector3D {
        /*
            Electric Dipole Moment,
            p = ∑ q_i (r_i − r′)
         */
        let mut p = Vector3D::new(0.0,0.0,0.0);
        for charge in charges {
            let c = charge_translator(charge);
            let w = separation_calculator(c.position, *s, false);
            let y = w.scalar_product(&c.charge);
            p += y;
        }
        p
    }
    fn charge_translator(charge: &Charge) -> ChargeVector { // translating UI Charge to MATH charge
        let c = charge;
        ChargeVector { charge: c.charge, position: Vector3D::new(c.position.0 as f64, c.position.1 as f64, c.position.2 as f64) }
    }

    fn separation_calculator(pos: Vector3D, sv: (f32, f32, f32), reverse: bool) -> Vector3D {
        /*
            A separation vector is the sum between r and r`:
                * the term (r_i − r′) measures the position relative to the reference point.
                * the term (r′ − r_i) measures the position relative to the charges.
         */
        let sv_vector = Vector3D::new(sv.0 as f64, sv.1 as f64, sv.2 as f64);
        if reverse {
            sv_vector - pos
        } else {
            pos - sv_vector
        }
    }
}