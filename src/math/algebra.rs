pub mod vector {
    use std::ops::{Add, AddAssign, Sub};

    // #[derive(Debug)]
    #[derive(Debug)]
    pub struct Vector3D {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vector3D {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self { x, y, z }
        }
        pub fn dot_product(&self, vec2: &Vector3D) -> f64 {
            self.x * vec2.x + self.y * vec2.y + self.z * vec2.z
        }
        // pub fn cross_product(&self, vec2: &Vector3D) -> Vector3D {
        //     Vector3D {
        //         x: self.y * vec2.z - self.z * vec2.y,
        //         y: self.z * vec2.x - self.x * vec2.z,
        //         z: self.x * vec2.y - self.y * vec2.x,
        //     }
        // }
        pub fn scalar_product(&self, s: &f64) -> Self {
            Self {
                x: s * self.x,
                y: s * self.y,
                z: s * self.z,
            }
        }
        pub fn magnitude(&self) -> f64 {
            // ((self.x.powf(2.)) + (self.y.powf(2.)) + (self.z.powf(2.))).powf(0.5)
            self.dot_product(self).powf(0.5) // |r| = sqrt( r . r )
        }
        // pub fn separation_with(&self, vec2: &Vector3D) -> Self {
        //     Self {
        //         x: vec2.x - self.x,
        //         y: vec2.y - self.y,
        //         z: vec2.z - self.z,
        //     }
        // }
    }
    impl Sub for Vector3D {
        type Output = Vector3D;

        fn sub(self, rhs: Self) -> Self::Output {
            Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
        }
    }

    impl Add for Vector3D {
        type Output = Vector3D;
        fn add(self, rhs: Self) -> Self::Output {
            Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
        }
    }

    impl AddAssign for Vector3D {
        fn add_assign(&mut self, rhs: Self) {
            self.x += rhs.x; self.y += rhs.y; self.z += rhs.z
        }
    }
    impl std::fmt::Display for Vector3D {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "( {}, \n{}, \n{})", self.x, self.y, self.z)
        }
    }
}
