use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}


impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { e: [x , y, z] }
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


    pub fn mul_factor(self, factor: f32) -> Self {
        Vec3::new(
            self.e[0] * factor,
            self.e[1] * factor,
            self.e[2] * factor,
        )
    }

    pub fn mul_factor_mut(&mut self, factor: f32) {
        self.e[0] *= factor;
        self.e[1] *= factor;
        self.e[2] *= factor;
    }

    pub fn div_factor(self, factor: f32) -> Self {
        Vec3::new(
            self.e[0] / factor,
            self.e[1] / factor,
            self.e[2] / factor,
        )
    }

    pub fn div_factor_mut(&mut self, factor: f32) {
        let k = 1.0f32 / factor;

        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.e[0] * self.e[0] +
        self.e[1] * self.e[1] +
        self.e[2] * self.e[2]
    }

    pub fn make_unit_vector(&mut self) {
        let k = 1.0f32 / self.length();

        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }


    pub fn unit_vector(v: &Vec3) -> Self {
        v.div_factor(v.length())
    }

    pub fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.e[0] * v2.e[0] +
        v1.e[1] * v2.e[1] +
        v1.e[2] * v2.e[2]
    }

    pub fn cross(v1: &Vec3, v2: &Vec3) -> Self {
        Vec3::new(
            v1.e[1] * v2.e[2] - v1.e[2] * v2.e[1],
            -(v1.e[0] * v2.e[2] - v1.e[2] * v2.e[0]),
            v1.e[0] * v2.e[1] - v1.e[1] * v2.e[0],
        )
    }
}

// omitting unary plus operator

// unary minus operator
impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

// index operator
impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// omitting IndexMut

// addition
impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] + other.e[0],
            self.e[1] + other.e[1],
            self.e[2] + other.e[2],
        )
    }
}

// assigning addition
impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

// subtraction
impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(
            self.e[0] - other.e[0],
            self.e[1] - other.e[1],
            self.e[2] - other.e[2],
        )
    }
}

// assigning subtraction
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

// multiplication
impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] * other.e[0],
            self.e[1] * other.e[1],
            self.e[2] * other.e[2],
        )
    }
}

// assigning multiplication
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

// division
impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Vec3::new(
            self.e[0] / other.e[0],
            self.e[1] / other.e[1],
            self.e[2] / other.e[2],
        )
    }
}

// assigning division
impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}


mod tests {
    use super::*;

    #[test]
    fn accessors() {
        let v = Vec3::new(1.1, 2.2, 3.3);

        assert_eq!(v.x(), 1.1);
        assert_eq!(v.y(), 2.2);
        assert_eq!(v.z(), 3.3);

        assert_eq!(v.r(), 1.1);
        assert_eq!(v.g(), 2.2);
        assert_eq!(v.b(), 3.3);
    }

    #[test]
    fn unary_minus_operator() {
        let pos_v = Vec3::new(1.1, 2.2, 3.3);
        let neg_v = -pos_v;

        assert_eq!(-pos_v.x(), neg_v.x());
        assert_eq!(-pos_v.y(), neg_v.y());
        assert_eq!(-pos_v.z(), neg_v.z());
    }

    #[test]
    fn index_operator() {
        let v = Vec3::new(1.1, 2.2, 3.3);

        assert_eq!(v[0], 1.1);
        assert_eq!(v[1], 2.2);
        assert_eq!(v[2], 3.3);
    }

    #[test]
    fn addition_operator() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let sum = Vec3::new(3.0, 5.0, 7.0);

        assert_eq!(sum, v1 + v2);
    }

    #[test]
    fn addition_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);
        let sum = Vec3::new(3.0, 5.0, 7.0);

        v1 += v2;
        assert_eq!(sum, v1);
    }

    #[test]
    fn subtraction_operator() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 6.0, 8.0);
        let difference = Vec3::new(-1.0, -4.0, -5.0);

        assert_eq!(difference, v1 - v2);
    }

    #[test]
    fn subtraction_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 1.0, 0.0);
        let difference = Vec3::new(-1.0, 1.0, 3.0);

        v1 -= v2;
        assert_eq!(difference, v1);
    }

    #[test]
    fn multiplication_operator() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let product = Vec3::new(1.0, 4.0, 9.0);

        assert_eq!(product, v1 * v2);
    }

    #[test]
    fn multiplication_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let product = Vec3::new(1.0, 4.0, 9.0);

        v1 *= v2;
        assert_eq!(product, v1);
    }

    #[test]
    fn division_operator() {
        let v1 = Vec3::new(1.0, 2.0, 4.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        let quotient = Vec3::new(0.5, 1.0, 2.0);

        assert_eq!(quotient, v1 / v2);
    }

    #[test]
    fn division_assign_operator() {
        let mut v1 = Vec3::new(1.0, 2.0, 4.0);
        let v2 = Vec3::new(2.0, 2.0, 2.0);
        let quotient = Vec3::new(0.5, 1.0, 2.0);

        v1 /= v2;
        assert_eq!(quotient, v1);
    }

    #[test]
    fn mul_factor() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let result = Vec3::new(2.0, 4.0, 6.0);

        assert_eq!(result, v.mul_factor(2.0));
    }

    #[test]
    fn mul_factor_mut() {
        let mut v = Vec3::new(1.0, 2.0, 3.0);
        let result = Vec3::new(2.0, 4.0, 6.0);

        v.mul_factor_mut(2.0);
        assert_eq!(result, v);
    }

    #[test]
    fn div_factor() {
        let v = Vec3::new(2.0, 3.0, 9.0);
        let result = Vec3::new(1.0, 1.5, 4.5);

        assert_eq!(result, v.div_factor(2.0));
    }

    #[test]
    fn div_factor_mut() {
        let mut v = Vec3::new(2.0, 3.0, 9.0);
        let result = Vec3::new(1.0, 1.5, 4.5);

        v.div_factor_mut(2.0);
        assert_eq!(result, v);
    }

    #[test]
    fn length() {
        let v = Vec3::new(0.0, 3.0, 4.0);

        assert_eq!(5.0, v.length());
    }

    #[test]
    fn squared_length() {
        let v = Vec3::new(0.0, 3.0, 4.0);

        assert_eq!(25.0, v.squared_length());
    }

    #[test]
    fn make_unit_vector() {
        let mut v = Vec3::new(0.0, 3.0, 4.0);
        let result = Vec3::new(0.0, 0.6, 0.8);

        v.make_unit_vector();
        assert_eq!(result, v);
    }

    #[test]
    fn unit_vector() {
        let v1 = Vec3::new(0.0, 3.0, 4.0);
        let v2 = Vec3::unit_vector(&v1);
        let result = Vec3::new(0.0, 0.6, 0.8);

        assert_eq!(result, v2);
    }

    #[test]
    fn dot_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(2.0, 3.0, 4.0);

        assert_eq!(20.0, Vec3::dot(&v1, &v2));
    }

    #[test]
    fn cross_product() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let result = Vec3::new(-3.0, 6.0, -3.0);

        assert_eq!(result, Vec3::cross(&v1, &v2));
    }
}