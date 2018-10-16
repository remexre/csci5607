mod to_ray;

use std::f32;

use cgmath::{Angle, InnerSpace, Vector3};
use float_ord::FloatOrd;
use image::Rgb;

use light::Light;
use ray::Ray;
use renderable::{DynamicRenderable, Renderable};
use scene::Scene;
use util::{add_colors, mul_colors, scale_color, BLACK};

const MIN_INTENSITY: f32 = 0.0001;

impl Scene {
    pub(crate) fn trace_pixel(&self, x: u32, y: u32) -> [u8; 3] {
        fn f32_to_u8(x: f32) -> u8 {
            debug_assert!(x <= 1.0);
            debug_assert!(x >= 0.0);
            (x * 255.0) as u8
        }
        let px = self
            .trace_ray(self.make_camera_ray(x, y), self.max_collisions)
            .data;
        [f32_to_u8(px[0]), f32_to_u8(px[1]), f32_to_u8(px[2])]
    }

    fn trace_ray(&self, ray: Ray, remaining_collisions: usize) -> Rgb<f32> {
        if let Some((obj, dist)) = self.closest_collision(ray) {
            let amb = mul_colors(obj.material().ambient, self.ambient_light);
            // TODO: This is not actually diffuse; this is just light bouncing off it...
            let dif = mul_colors(
                obj.material().diffuse,
                self.light_on(obj, ray.project(dist)),
            );
            add_colors(amb, dif)
        } else {
            self.background
        }
    }

    fn closest_collision(&self, ray: Ray) -> Option<(&DynamicRenderable, f32)> {
        self.objects
            .iter()
            .filter_map(|o| o.collides_with(ray).map(|d| (o, d)))
            .min_by_key(|&(_, d)| FloatOrd(d))
    }

    fn light_on<R: Renderable>(&self, obj: &R, pos: Vector3<f32>) -> Rgb<f32> {
        let norm = obj.normal_at(pos);
        self.lights
            .iter()
            .filter_map(|l| self.one_light(l, obj, pos, norm))
            .fold(BLACK, add_colors)
    }
    fn one_light<L, R>(
        &self,
        light: &L,
        obj: &R,
        pos: Vector3<f32>,
        norm: Vector3<f32>,
    ) -> Option<Rgb<f32>>
    where
        L: Light,
        R: Renderable,
    {
        let intensity = light.intensity_at(pos);
        if intensity < MIN_INTENSITY {
            None
        } else {
            let (dir, dist) = light.direction_from(pos);
            let ray = Ray {
                origin: pos + 0.0001 * norm,
                direction: dir,
            };
            let collision_dist = self
                .closest_collision(ray)
                .map(|(_, d)| d)
                .unwrap_or(f32::INFINITY);

            // Check for shadow.
            if collision_dist >= dist {
                let c = norm.angle(dir).cos();
                if c >= 0.0 {
                    Some(scale_color(light.color(), c * intensity))
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}
