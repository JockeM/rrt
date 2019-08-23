extern crate cgmath;
extern crate image;

mod shape;
use shape::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
    let mut buffer: Vec<u8> = std::vec::Vec::new();

    let spheres = vec![
        shape::Sphere {
            origin: Vec3 {
                x: 100.0,
                y: 100.0,
                z: 100.0,
            },
            radius: 10.0,
            color: [50, 255, 255, 255],
        },
        shape::Sphere {
            origin: Vec3 {
                x: 200.0,
                y: 200.0,
                z: 500.0,
            },
            radius: 100.0,
            color: [0, 100, 0, 255],
        },
    ];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let color = get_color_from_cord(x as f64, y as f64, &spheres);
            buffer.push(color[0]);
            buffer.push(color[1]);
            buffer.push(color[2]);
        }
    }

    image::save_buffer("output.png", &buffer, WIDTH, HEIGHT, image::RGB(8)).unwrap();
}

fn get_color_from_cord(x: f64, y: f64, spheres: &Vec<Sphere>) -> [u8; 4] {
    let ray = Ray {
        origin: Vec3 { x, y, z: 0.0 },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };
    for s in spheres.iter() {
        let hit = ray.intersects_sphere(s);
        match hit {
            Some(_) => return s.color,
            None => (),
        }
    }

    return [10, 10, 10, 0];
}
