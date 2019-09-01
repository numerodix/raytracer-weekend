mod vec3;
mod ray;

use crate::ray::Ray;
use crate::vec3::Vec3;


fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = *r.origin() - *center;

    let a = Vec3::dot(r.direction(), r.direction());

    let b = 2.0f32 * Vec3::dot(&oc, r.direction());

    let c = Vec3::dot(&oc, &oc) - radius * radius;

    let discriminant = b * b - 4.0f32 * a * c;

    match discriminant {
       d if d < 0.0f32 => -1.0,
       _ => (-b - discriminant.sqrt()) / (2.0f32 * a),
    }
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5f32, &r);

    if t > 0.0f32 {
        let N = Vec3::unit_vector(
            &(r.point_at_parameter(t) - Vec3::new(0.0, 0.0, -1.0))
        );

        return 0.5f32 * Vec3::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }

    // r has a direction vector like:           [2.0, 0.4, -2.0]
    // make it into a unit vector:              [1.0, 0.2, -1.0]
    let unit_direction: Vec3 = Vec3::unit_vector(r.direction());

    // 1) grab the y and add 1:                 1.2
    // 2) halve it:                             0.6
    let t = 0.5f32 * (unit_direction.y() + 1.0f32);

    // t is now a number in the range [0, 1]

    // [1.0, 1.0, 1.0] represents white
    // [0.5, 0.7, 1.0] represents light blue

    // blend: white at intensity 1-t *with* blue at intensity t
    (1.0f32 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}


fn write(s: &str) {
    print!("{}", s);
}


fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    write(&format!("P3\n{} {}\n255\n", nx, ny));

    // screen coordinate
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    // screen width
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    // screen height
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    // position of the camera / eye
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0 .. ny - 1).rev() {
        for i in 0 .. nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical,
            );
            let col = color(&r);

            let ir = (255.99f32 * col[0]) as i32;
            let ig = (255.99f32 * col[1]) as i32;
            let ib = (255.99f32 * col[2]) as i32;

            //              R  G  B
            write(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
}
