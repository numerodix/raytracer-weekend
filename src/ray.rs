use crate::vec3::Vec3;


pub struct Ray {
    a: Vec3,    // origin
    b: Vec3,    // direction
}


impl Ray {
    pub fn new(a: Vec3, b: Vec3) -> Self {
        Ray { a , b }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.a
    }

    pub fn direction(&self) -> &Vec3 {
        &self.b
    }

    pub fn point_at_parameter(&self, t: f32) -> Vec3 {
        self.a + self.b.mul_factor(t)
    }
}


mod tests {
    use super::*;

    #[test]
    fn accessors() {
        let a = Vec3::new(1.1, 2.2, 3.3);
        let b = Vec3::new(4.4, 5.5, 6.6);
        let ray = Ray::new(a, b);

        assert_eq!(&a, ray.origin());
        assert_eq!(&b, ray.direction());
    }

    #[test]
    fn point_at_parameter() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(a, b);
        let result = Vec3::new(11.0, 14.5, 18.0);

        let point = ray.point_at_parameter(2.5);
        assert_eq!(result, point);
    }
}