#[cfg(test)]
mod tests {
    use crate::shape::*;

    #[test]
    fn test_should_hit_with_5_distance() {
        let sphere = Sphere {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 5.0,
            color: [255, 255, 255, 255],
        };

        let ray = Ray {
            origin: Vec3 {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let hit = ray.intersects_sphere(sphere);
        assert_eq!(hit, Some(5.0));
    }

    #[test]
    fn test_should_miss() {
        let sphere = Sphere {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 9.0,
            color: [255, 255, 255, 255],
        };

        let ray = Ray {
            origin: Vec3 {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let hit = ray.intersects_sphere(sphere);
        assert_eq!(hit, None);
    }

    #[test]
    fn test_starting_at_edge_pointing_to_other_edge() {
        //is this correct behavior?
        let sphere = Sphere {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 10.0,
            color: [255, 255, 255, 255],
        };

        let ray = Ray {
            origin: Vec3 {
                x: 10.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let hit = ray.intersects_sphere(sphere);
        assert_eq!(hit, None);
    }
}
