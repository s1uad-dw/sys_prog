use image;
use num_complex;
use rand::Rng;
fn main() {
    let imgx = 800;
    let imgy = 600;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    let mut rng = rand::thread_rng();
    let mut x = 0.0;
    let mut y = 0.0;

    for _i in 0..800*600{
        let r: f32 = rng.gen();
        let (dx, dy, color) = if r < 0.01 {
            x = 0.0;
            y = 0.16 * y;
            (0, 0, image::Rgb([0, 255, 0]))
        } else if r < 0.86 {
            let new_x = 0.85 * x + 0.04 * y;
            let new_y = -0.04 * x + 0.85 * y + 1.6;
            x = new_x;
            y = new_y;
            (400 + (new_x * 50.0) as i32, imgy as i32 - (new_y * 50.0) as i32, image::Rgb([0, 0, 0]))
        } else if r < 0.93 {
            let new_x = 0.2 * x - 0.26 * y;
            let new_y = 0.23 * x + 0.22 * y + 1.6;
            x = new_x;
            y = new_y;
            (400 + (new_x * 50.0) as i32, imgy as i32 - (new_y * 50.0) as i32, image::Rgb([255, 0, 0]))
        } else {
            let new_x = -0.15 * x + 0.28 * y;
            let new_y = 0.26 * x + 0.24 * y + 0.44;
            x = new_x;
            y = new_y;
            (400 + (new_x * 50.0) as i32, imgy as i32 - (new_y * 50.0) as i32, image::Rgb([0, 0, 255]))
        };
        let pixel = imgbuf.get_pixel_mut(dx as u32, dy as u32);
        *pixel = color;
    }
    imgbuf.save("fractal.png").unwrap();
    

    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    
}