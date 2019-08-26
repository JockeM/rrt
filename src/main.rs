extern crate cgmath;
extern crate image;

mod test;

mod ray;
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
                x: 100.0,
                y: 100.0,
                z: 100.0,
            },
            radius: 10.0,
            color: [50, 255, 255, 255],
        },
        Sphere {
            origin: Vec3 {
                x: 200.0,
                y: 200.0,
                z: 500.0,
            },
            radius: 100.0,
            color: [10, 200, 10, 255],
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

fn get_color_from_cord(x: f64, y: f64, spheres: &Vec<Sphere>, lights: &Vec<Light>) -> [u8; 4] {
    use cgmath::InnerSpace;
    let ray = Ray {
        origin: Vec3 { x, y, z: 0.0 },
        direction: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    };

    for sphere in spheres.iter() {
        let hit = ray.intersects_sphere(*sphere);
        match hit {
            Some(time) => {
                let mut color = sphere.color;
                let hit = ray.get_hitpoint(time - 0.1);
                let normal = sphere.get_normal(&hit);
                let facing_ratio = normal.dot(ray.direction) * 0.8;
                for light in lights.iter() {
                    let direction = (hit - light.origin).normalize();
                    let ray = Ray {
                        origin: hit,
                        direction,
                    };

                    fn did_hit(ray: Ray, spheres: &Vec<Sphere>) -> bool {
                        for sphere in spheres.iter() {
                            let hit = ray.intersects_sphere(*sphere);
                            match hit {
                                Some(_) => return true,
                                None => (),
                            }
                        }
                        return false;
                    }

                    if !did_hit(ray, spheres) {
                        color[0] /= 2;
                        color[1] /= 2;
                        color[2] /= 2;
                    }
                }
                color[0] = (color[0] as f64 * facing_ratio) as u8; //make a color struct or somethint o easier do math ops
                color[1] = (color[1] as f64 * facing_ratio) as u8;
                color[2] = (color[2] as f64 * facing_ratio) as u8;

                return color;
            }
            None => (),
        }
    }

    return [10, 10, 10, 0];
}
