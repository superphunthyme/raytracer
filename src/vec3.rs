use std::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    // Use Option to allow for empty contructor

    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn zero() -> Vector3 {
        Vector3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
    pub fn r(&self) -> f32 {
        self.x
    }
    pub fn g(&self) -> f32 {
        self.y
    }
    pub fn b(&self) -> f32 {
        self.z
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y() * other.z() - self.z() * other.y(),
            y: self.z() * other.x() - self.x() * other.z(),
            z: self.x() * other.y() - self.y() * other.x(),
        }
    }

    pub fn dot(&self, other: &Vector3) -> f32 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Useful for comparisons, including with 0,1
    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vector(&mut self) {
        let len = self.length();
        if len > 0.0 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
        }
    }

    pub fn get_unit_vector(&self) -> Vector3 {
        let len = self.length();
        if len > 0.0 {
            Vector3 {
                x: self.x / len,
                y: self.y / len,
                z: self.z / len,
            }
        } else {
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
        }
    }
}

impl std::ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add for &Vector3 {
    type Output = Vector3;

    fn add(self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, t: f32) -> Vector3 {
        Vector3 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl std::ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, t: f32) -> Vector3 {
        Vector3 {
            x: self.x / t,
            y: self.y / t,
            z: self.z / t,
        }
    }
}

impl std::ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, v: Vector3) -> Vector3 {
        Vector3 {
            x: self * v.x(),
            y: self * v.y(),
            z: self * v.z(),
        }
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// Could add tests for overflows
#[cfg(test)]
mod tests {
    use super::*;

    // These tests are a bit bland, but the point of this is learning anyway

    // Thanks https://mathinsight.org/cross_product_examples
    #[test]
    fn cross_product() {
        let vec1 = Vector3::new(3.0, -3.0, 1.0);
        let vec2 = Vector3::new(4.0, 9.0, 2.0);
        let vec_test = Vector3::new(-15.0, -2.0, 39.0);
        let vec_result = vec1.cross(&vec2);

        // Using f32 equality because these are small integers
        assert_eq!(vec_result.x, vec_test.x);
        assert_eq!(vec_result.y, vec_test.y);
        assert_eq!(vec_result.z, vec_test.z);
    }

    #[test]
    fn dot_product() {
        let vec1 = Vector3::new(3.0, -3.0, 1.0);
        let vec2 = Vector3::new(4.0, 9.0, 2.0);
        let f_test = -13.0;
        let f_result = vec1.dot(&vec2);

        // Using f32 equality because these all ops are adds/mults on small inetegers
        assert_eq!(f_test, f_result);
    }

    #[test]
    fn test_add_postive() {
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(1.5, 2.5, 3.5);
        let vec_result = vec1 + vec2;

        assert_eq!(vec_result.x, 1.0 + 1.5);
        assert_eq!(vec_result.y, 2.0 + 2.5);
        assert_eq!(vec_result.z, 3.0 + 3.5);
    }

    #[test]
    fn test_add_zero() {
        let vec1 = Vector3::new(1.0, 2.0, 3.0);
        let vec2 = Vector3::new(0.0, 0.0, 0.0);
        let vec_result = vec1 + vec2;

        assert_eq!(vec_result.x, vec1.x);
        assert_eq!(vec_result.y, vec1.y);
        assert_eq!(vec_result.z, vec1.z);
    }

    #[test]
    fn test_add_negative() {
        let vec1 = Vector3::new(-1.0, -2.0, -3.0);
        let vec2 = Vector3::new(-1.5, -2.5, -3.5);
        let vec_result = vec1 + vec2;

        assert_eq!(vec_result.x, -1.0 - 1.5);
        assert_eq!(vec_result.y, -2.0 - 2.5);
        assert_eq!(vec_result.z, -3.0 - 3.5);
    }

    #[test]
    fn test_f32_mult() {
        let vec1 = Vector3::new(-1.0, -2.0, -3.0);
        let f_test = -2.0;
        let vec_test = Vector3::new(2.0, 4.0, 6.0);
        let vec_result = f_test * vec1;

        // Using f32 equality because these all ops are adds/mults on small inetegers
        assert_eq!(vec_result.x, vec_test.x);
        assert_eq!(vec_result.y, vec_test.y);
        assert_eq!(vec_result.z, vec_test.z);

        let vec_result = vec1 * f_test;
        assert_eq!(vec_result.x, vec_test.x);
        assert_eq!(vec_result.y, vec_test.y);
        assert_eq!(vec_result.z, vec_test.z);
    }
}
