use std::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct vec3 {
    e: [f32; 3],
}

impl vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        vec3 { e: [x , y, z] }
    }

    pub fn x(&self) -> f32 {
        self.e[0]
    }

    pub fn y(&self) -> f32 {
        self.e[1]
    }

    pub fn z(&self) -> f32 {
        self.e[2]
    }

    pub fn r(&self) -> f32 {
        self.e[0]
    }

    pub fn g(&self) -> f32 {
        self.e[1]
    }

    pub fn b(&self) -> f32 {
        self.e[2]
    }
}

// omitting unary plus operator

// unary minus operator
impl Neg for vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

// index operator
impl Index<usize> for vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// omitting IndexMut

// addition
impl Add for vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

// assigning addition
impl AddAssign for vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

// multiplication
impl Mul for vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

// assigning multiplication
impl MulAssign for vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}


mod tests {
    use super::*;

    #[test]
    fn accessors() {
        let v = vec3::new(1.1, 2.2, 3.3);

        assert_eq!(v.x(), 1.1);
        assert_eq!(v.y(), 2.2);
        assert_eq!(v.z(), 3.3);

        assert_eq!(v.r(), 1.1);
        assert_eq!(v.g(), 2.2);
        assert_eq!(v.b(), 3.3);
    }

    #[test]
    fn unary_minus_operator() {
        let pos_v = vec3::new(1.1, 2.2, 3.3);
        let neg_v = -pos_v;

        assert_eq!(-pos_v.x(), neg_v.x());
        assert_eq!(-pos_v.y(), neg_v.y());
        assert_eq!(-pos_v.z(), neg_v.z());
    }

    #[test]
    fn index_operator() {
        let v = vec3::new(1.1, 2.2, 3.3);

        assert_eq!(v[0], 1.1);
        assert_eq!(v[1], 2.2);
        assert_eq!(v[2], 3.3);
    }

    #[test]
    fn addition_operator() {
        let v1 = vec3::new(1.0, 2.0, 3.0);
        let v2 = vec3::new(2.0, 3.0, 4.0);
        let sum = vec3::new(3.0, 5.0, 7.0);

        assert_eq!(sum, v1 + v2);
    }

    #[test]
    fn addition_assign_operator() {
        let mut v1 = vec3::new(1.0, 2.0, 3.0);
        let v2 = vec3::new(2.0, 3.0, 4.0);
        let sum = vec3::new(3.0, 5.0, 7.0);

        v1 += v2;
        assert_eq!(sum, v1);
    }

    #[test]
    fn multiplication_operator() {
        let v1 = vec3::new(1.0, 2.0, 3.0);
        let v2 = vec3::new(1.0, 2.0, 3.0);
        let product = vec3::new(1.0, 4.0, 9.0);

        assert_eq!(product, v1 * v2);
    }

    #[test]
    fn multiplication_assign_operator() {
        let mut v1 = vec3::new(1.0, 2.0, 3.0);
        let v2 = vec3::new(1.0, 2.0, 3.0);
        let product = vec3::new(1.0, 4.0, 9.0);

        v1 *= v2;
        assert_eq!(product, v1);
    }
}