mod vec3;


fn write(s: &str) {
    print!("{}", s);
}


fn main() {
    let nx: i32 = 200;
    let ny: i32 = 100;

    write(&format!("P3\n{} {}\n255\n", nx, ny));

    for j in (0 .. ny - 1).rev() {
        for i in 0 .. nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2f32;

            let ir = (255.99f32 * r) as i32;
            let ig = (255.99f32 * g) as i32;
            let ib = (255.99f32 * b) as i32;

            //              R  G  B
            write(&format!("{} {} {}\n", ir, ig, ib));
        }
    }
}
