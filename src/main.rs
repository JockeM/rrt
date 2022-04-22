extern crate cgmath;
extern crate image;

mod test;

mod ray;

use cgmath::InnerSpace;
use ray::*;

mod objects;
use objects::*;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

pub type Vec3 = cgmath::Vector3<f64>;

fn main() {
    let spheres = vec![
        Sphere {
            origin: Vec3 {
                x: 120.0,
                y: 120.0,
                z: 100.0,
            },
            radius: 12.0,
            color: [50, 200, 200, 255],
        },
        Sphere {
            origin: Vec3 {
                x: 200.0,
                y: 200.0,
                z: 10000.0,
            },
            radius: 100.0,
            color: [10, 200, 10, 255],
        },
        Sphere {
            origin: Vec3 {
                x: 300.0,
                y: 300.0,
                z: 1000.0,
            },
            radius: 20.0,
            color: [200, 0, 10, 255],
        },
    ];

    let lights = vec![Light {
        origin: Vec3 {
            x: 0.0,
            y: 500.0,
            z: 50.0,
        },
        color: [255, 255, 255, 255],
    }];

    let mut buffer: Vec<u8> = vec![];

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let color = get_color_from_cord(x as f64, y as f64, &spheres, &lights);

            buffer.push(color[0]);
            buffer.push(color[1]);
            buffer.push(color[2]);
        }
    }

    image::save_buffer("output.png", &buffer, WIDTH, HEIGHT, image::RGB(8)).unwrap();
}

fn get_color_from_cord(
    x: f64,
    y: f64,
    spheres: &Vec<Sphere>,
    _lights: &Vec<Light>,
) -> [u8; 4] {
    let ray = Ray {
        origin: Vec3 { x, y, z: 0.0 },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    const BACKROUND_COLOR: [u8; 4] = [10, 10, 10, 255];

    for sphere in spheres.iter() {
        let hit = ray.intersects_sphere(*sphere);
        if let Some(time) = hit {
            let hit = ray.get_hitpoint(time - 0.1);
            let normal = sphere.get_normal(&hit);
            let facing_ratio = normal.dot(ray.direction) * 0.8;
            return color_lerp(sphere.color, BACKROUND_COLOR, facing_ratio);
        }
    }

    BACKROUND_COLOR
}

fn _get_color_with_shadow(
    lights: &Vec<Light>,
    hit: cgmath::Vector3<f64>,
    spheres: &Vec<Sphere>,
    color: [u8; 4],
) -> [u8; 4] {
    for light in lights.iter() {
        let direction = (hit - light.origin).normalize();
        let ray = Ray {
            origin: hit,
            direction,
        };

        fn is_hit(ray: Ray, spheres: &Vec<Sphere>) -> bool {
            spheres.iter().any(|sphere| {
                ray.intersects_sphere(*sphere).is_some()
            })
        }

        if !is_hit(ray, spheres) {
            return [color[0] + 50, color[1] + 50, color[2] + 50, color[3]];
        }
    }

    color
}

fn color_lerp(a: [u8; 4], b: [u8; 4], ratio: f64) -> [u8; 4] {
    [
        (a[0] as f64 * ratio + b[0] as f64 * (1.0 - ratio)) as u8,
        (a[1] as f64 * ratio + b[1] as f64 * (1.0 - ratio)) as u8,
        (a[2] as f64 * ratio + b[2] as f64 * (1.0 - ratio)) as u8,
        a[3],
    ]
}
