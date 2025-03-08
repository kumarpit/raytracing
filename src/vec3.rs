#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
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
