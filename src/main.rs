mod camera;
mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;


fn color(r: &Ray, world: &Hitable) -> Vec3 {
    let opt_rec = world.hit(r, 0.0, std::f32::MAX);

    if opt_rec.is_some() {
        let rec = opt_rec.unwrap();
        return 0.5 * Vec3::new(
            rec.normal.x() + 1.0,
            rec.normal.y() + 1.0,
            rec.normal.z() + 1.0,
        );
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}


fn write(s: &str) {
    print!("{}", s);
}


fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    write(&format!("P3\n{} {}\n255\n", nx, ny));

    let sphere1 = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let sphere2 = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);
    let world = HitableList::new(vec![
        Box::new(sphere1),
        Box::new(sphere2),
    ]);
    let cam = Camera::new();

    for j in (0 .. ny - 1).rev() {
        for i in 0 .. nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = cam.get_ray(u, v);
            let col = color(&r, &world);

            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            //              R  G  B
            write(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
}
