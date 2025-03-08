use crate::common::{random, random_in_range};

/// A three-dimensional vector of floats used to represent colors, coordinates, etc

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    /// Generates a random vector with each component in the range [0, 1)
    pub fn random() -> Self {
        Vec3(random(), random(), random())
    }

    /// Generates a random vector with each component in the range [x_min, x_max)
    pub fn random_in_range(x_min: f64, x_max: f64) -> Self {
        Vec3(
            random_in_range(x_min, x_max),
            random_in_range(x_min, x_max),
            random_in_range(x_min, x_max),
        )
    }

    /// Generates a vector that lies within the unit sphere (i.e has length < 1)
    pub fn in_unit_sphere() -> Self {
        loop {
            let candidate = Vec3::random_in_range(-1.0, 1.0);
            if candidate.length_squared() < 1.0 {
                return candidate;
            }
        }
    }

    /// Generates a unit vector (i.e lies on the unit sphere) that points towards the normal
    /// direction
    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let unit_random_vec = Vec3::in_unit_sphere().into_unit();
        if normal.dot(unit_random_vec) > 0.0 {
            unit_random_vec
        } else {
            -unit_random_vec
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.zip_with(other, core::ops::Mul::mul)
            .reduce(core::ops::Add::add)
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            -(self.0 * other.2 - self.2 * other.0),
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn length(&self) -> f64 {
        self.dot(*self).sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.dot(*self)
    }

    pub fn into_unit(self) -> Self {
        self / self.length()
    }

    /// Applies f onto each component of the vector
    pub fn map(self, mut f: impl FnMut(f64) -> f64) -> Self {
        Vec3(f(self.0), f(self.1), f(self.2))
    }

    pub fn zip_with(self, other: Vec3, mut f: impl FnMut(f64, f64) -> f64) -> Self {
        Vec3(f(self.0, other.0), f(self.1, other.1), f(self.2, other.2))
    }

    pub fn reduce(self, f: impl Fn(f64, f64) -> f64) -> f64 {
        f(f(self.0, self.1), self.2)
    }
}

impl From<f64> for Vec3 {
    fn from(v: f64) -> Self {
        Vec3(v, v, v)
    }
}

/// Vec3 * Vec3
impl std::ops::Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Mul::mul)
    }
}

/// `scalar * vector`
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::from(self) * rhs
    }
}

/// `vector * scalar`
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        self.map(|x| -> f64 { x * rhs })
    }
}

/// Element-wise division.
impl std::ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Div::div)
    }
}

/// `vector / scalar`
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self.map(|x| x / rhs)
    }
}

/// `vector + vector`
impl std::ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Add::add)
    }
}

/// `scalar + vector`
impl std::ops::Add<Vec3> for f64 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        rhs.map(|x| self + x)
    }
}

/// `vector - vector`
impl std::ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self.zip_with(rhs, std::ops::Sub::sub)
    }
}

/// `-vector`
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        self.map(std::ops::Neg::neg)
    }
}
