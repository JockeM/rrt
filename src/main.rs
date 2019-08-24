extern crate cgmath;
extern crate image;

mod test;

mod shape;
use shape::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

fn main() {
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
    let mut buffer: Vec<u8> = vec![];
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
    use cgmath::InnerSpace;
    let ray = Ray {
        origin: Vec3 { x, y, z: 0.0 },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    for s in spheres.iter() {
        let hit = ray.intersects_sphere(*s);
        match hit {
            Some(time) => {
                let hit = ray.get_hitpoint(time);
                let normal = s.get_normal(&hit);
                let facing_ratio = normal.dot(ray.direction) * 0.8;
                return [
                    (s.color[0] as f64 * facing_ratio) as u8,
                    (s.color[1] as f64 * facing_ratio) as u8,
                    (s.color[2] as f64 * facing_ratio) as u8,
                    255,
                ];
            }
            None => (),
        }
    }

    return [10, 10, 10, 0];
}
