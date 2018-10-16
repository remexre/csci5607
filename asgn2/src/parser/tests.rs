use cgmath::Vector3;
use image::Pixel;
use material::Material;
use renderable::{DynamicRenderable, Sphere};
use scene::Scene;

#[test]
fn integration() {
    let scene: Scene = include_str!("../../scenes/sphere.scn").parse().unwrap();

    assert_eq!(
        scene,
        Scene {
            ambient_light: Pixel::from_channels(0.1, 0.1, 0.1, 1.0),
            background: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
            camera_direction: Vector3::new(0.0, 0.0, 1.0),
            camera_half_angle_tan: 1.0,
            camera_position: Vector3::new(0.0, 0.0, 0.0),
            camera_up: Vector3::new(0.0, 1.0, 0.0),
            height: 1080,
            lights: vec![],
            max_collisions: 5,
            objects: vec![DynamicRenderable::Sphere(Sphere {
                material: Material {
                    ambient: Pixel::from_channels(1.0, 1.0, 1.0, 1.0),
                    diffuse: Pixel::from_channels(1.0, 1.0, 1.0, 1.0),
                    specular: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
                    phong: 5.0,
                    transmissive: Pixel::from_channels(0.0, 0.0, 0.0, 1.0),
                    ior: 1.0,
                },
                position: Vector3::new(0.0, 0.0, 2.0),
                radius: 1.0,
            }),],
            output_image: Some("example.png".into()),
            width: 1920,
        }
    );
}
