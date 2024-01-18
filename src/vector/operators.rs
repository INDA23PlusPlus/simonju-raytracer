use std::ops::*;

use super::Vec3D;

impl Neg for Vec3D {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Add for Vec3D {
    type Output = Self;

    #[inline]
    fn add(self, vector: Self) -> Self {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        }
    }
}

impl Sub for Vec3D {
    type Output = Self;

    #[inline]
    fn sub(self, vector: Self) -> Self {
        Self {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        }
    }
}

// Hadamard product
impl Mul for Vec3D {
    type Output = Self;

    #[inline]
    fn mul(self, vector: Self) -> Self {
        Self {
            x: self.x * vector.x,
            y: self.y * vector.y,
            z: self.z * vector.z,
        }
    }
}

impl Mul<f32> for Vec3D {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3D> for f32 {
    type Output = Vec3D;

    #[inline]
    fn mul(self, vector: Vec3D) -> Vec3D {
        Vec3D {
            x: self * vector.x,
            y: self * vector.y,
            z: self * vector.z,
        }
    }
}

// Hadamard product
impl Div for Vec3D {
    type Output = Self;

    #[inline]
    fn div(self, vector: Self) -> Self {
        Self {
            x: self.x / vector.x,
            y: self.y / vector.y,
            z: self.z / vector.z,
        }
    }
}

impl Div<f32> for Vec3D {
    type Output = Self;

    #[inline]
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl AddAssign for Vec3D {
    #[inline]
    fn add_assign(&mut self, vector: Self) {
        *self = Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
            z: self.z + vector.z,
        };
    }
}

impl SubAssign for Vec3D {
    #[inline]
    fn sub_assign(&mut self, vector: Self) {
        *self = Self {
            x: self.x - vector.x,
            y: self.y - vector.y,
            z: self.z - vector.z,
        };
    }
}

// Hadamard product
impl MulAssign for Vec3D {
    #[inline]
    fn mul_assign(&mut self, vector: Self) {
        *self = Self {
            x: self.x * vector.x,
            y: self.y * vector.y,
            z: self.z * vector.z,
        };
    }
}

impl MulAssign<f32> for Vec3D {
    #[inline]
    fn mul_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        };
    }
}

// Hadamard quotient
impl DivAssign for Vec3D {
    #[inline]
    fn div_assign(&mut self, vector: Self) {
        *self = Self {
            x: self.x / vector.x,
            y: self.y / vector.y,
            z: self.z / vector.z,
        };
    }
}

impl DivAssign<f32> for Vec3D {
    #[inline]
    fn div_assign(&mut self, scalar: f32) {
        *self = Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        };
    }
}