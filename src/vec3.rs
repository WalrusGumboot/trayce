use rand::rngs::ThreadRng;

#[derive(Clone, Copy, Debug,  PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

pub type Loc3 = Vec3;
pub type Colour = Vec3;

pub mod utilities {
    use rand::{rngs::ThreadRng, Rng};

    pub fn get_normal_dist_random(rng: &mut ThreadRng) -> f64 {
        // approximation: (1.5-(R1+R2+R3))*1.88
        // see also https://stackoverflow.com/questions/75677/converting-a-uniform-distribution-to-a-normal-distribution#comment9462647_7771542
        let (a, b, c) = rng.gen::<(f64, f64, f64)>();
        (1.5 - (a + b + c)) * 1.88
    }
}

// constants
impl Vec3 {
    pub const ZERO: Self = Vec3(0.0, 0.0, 0.0);
    pub const ONES: Self = Vec3(1.0, 1.0, 1.0);

    pub const R: Self = Vec3( 1.0,  0.0,  0.0);
    pub const L: Self = Vec3(-1.0,  0.0,  0.0);
    pub const U: Self = Vec3( 0.0,  1.0,  0.0);
    pub const D: Self = Vec3( 0.0, -1.0,  0.0);
    pub const F: Self = Vec3( 0.0,  0.0,  1.0);
    pub const B: Self = Vec3( 0.0,  0.0, -1.0);
}

impl Vec3 {
    pub fn dot(a: Self, b: Self) -> f64 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Vec3(
            a.1 * b.2 - a.2 * b.1,
            a.2 * b.0 - a.0 * b.2,
            a.0 * b.1 - a.1 * b.0
        )
    }

    pub fn random_on_unit_sphere(rng: &mut ThreadRng) -> Self {
        Vec3(
            utilities::get_normal_dist_random(rng),
            utilities::get_normal_dist_random(rng),
            utilities::get_normal_dist_random(rng)
        ).normalise()
    }

    pub fn random_on_unit_hemisphere(rng: &mut ThreadRng, normal: Vec3) -> Self {
        let vec = Self::random_on_unit_sphere(rng);
        if Vec3::dot(vec, normal) > 0.0 { vec } else { -vec }
    }
}

impl Vec3 {
    pub fn magnitude_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn normalise(&self) -> Self {
        *self * (1.0 / self.magnitude())
    }
}

// ------------------------------------------
// rote operation implementations start here:
// ------------------------------------------

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

// the standard multiplication implementation is a Hadamard multiplication

impl std::ops::Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl std::ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

// but scalar multiplication is also defined.

impl std::ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl std::ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self * -1.0
    }
}