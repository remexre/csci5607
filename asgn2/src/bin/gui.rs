use std::time::Instant;

use cgmath::{Deg, Matrix3, Vector3};
use sdl2;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::TextureAccess;

use raytracer::Scene;

const PAN_SPEED: f32 = 0.25;
const PITCH_SPEED: Deg<f32> = Deg(10.0);
const ROLL_SPEED: Deg<f32> = Deg(10.0);
const YAW_SPEED: Deg<f32> = Deg(10.0);

pub fn run(mut scene: Scene) {
    let sdl = sdl2::init().expect("failed to init SDL");
    let video = sdl.video().expect("failed to init SDL video");

    let mut events = sdl.event_pump().expect("couldn't get events");
    let mut canvas = video
        .window(crate_name!(), scene.width, scene.height)
        .position_centered()
        .build()
        .expect("failed to create window")
        .into_canvas()
        .present_vsync()
        .build()
        .expect("failed to get renderer");
    let texture_builder = canvas.texture_creator();

    loop {
        let render_start = Instant::now();

        for event in events.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::W) => {
                        scene.camera_position += scene.camera_direction * PAN_SPEED;
                    }
                    Some(Keycode::S) => {
                        scene.camera_position -= scene.camera_direction * PAN_SPEED;
                    }
                    Some(Keycode::A) => {
                        let vec = scene.camera_up.cross(scene.camera_direction);
                        scene.camera_position -= vec * PAN_SPEED;
                    }
                    Some(Keycode::D) => {
                        let vec = scene.camera_up.cross(scene.camera_direction);
                        scene.camera_position += vec * PAN_SPEED;
                    }
                    Some(Keycode::Q) => {
                        scene.camera_position += scene.camera_up * PAN_SPEED;
                    }
                    Some(Keycode::E) => {
                        scene.camera_position -= scene.camera_up * PAN_SPEED;
                    }
                    Some(Keycode::J) => {
                        let rot_mat = Matrix3::from_axis_angle(scene.camera_up, -YAW_SPEED);
                        scene.camera_direction = rot_mat * scene.camera_direction;
                    }
                    Some(Keycode::L) => {
                        let rot_mat = Matrix3::from_axis_angle(scene.camera_up, YAW_SPEED);
                        scene.camera_direction = rot_mat * scene.camera_direction;
                    }
                    Some(Keycode::I) => {
                        let rot_mat = Matrix3::from_axis_angle(scene.camera_right(), -PITCH_SPEED);
                        scene.camera_direction = rot_mat * scene.camera_direction;
                        scene.camera_up = rot_mat * scene.camera_up;
                    }
                    Some(Keycode::K) => {
                        let rot_mat = Matrix3::from_axis_angle(scene.camera_right(), PITCH_SPEED);
                        scene.camera_direction = rot_mat * scene.camera_direction;
                        scene.camera_up = rot_mat * scene.camera_up;
                    }
                    Some(Keycode::U) => {
                        let rot_mat = Matrix3::from_axis_angle(scene.camera_direction, ROLL_SPEED);
                        scene.camera_up = rot_mat * scene.camera_up;
                    }
                    Some(Keycode::O) => {
                        let rot_mat = Matrix3::from_axis_angle(scene.camera_direction, -ROLL_SPEED);
                        scene.camera_up = rot_mat * scene.camera_up;
                    }
                    Some(Keycode::Space) => {
                        scene.camera_direction = Vector3::new(0.0, 0.0, 1.0);
                        scene.camera_up = Vector3::new(0.0, 1.0, 0.0);
                    }
                    Some(Keycode::Escape) | Some(Keycode::X) => return,
                    _ => {}
                },
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Close => return,
                    WindowEvent::Resized(w, h) => {
                        scene.width = w as u32;
                        scene.height = h as u32;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        let buf = scene.render().into_raw();
        assert_eq!(buf.len() as u32, scene.width * scene.height * 3);
        let rect = Rect::new(0, 0, scene.width, scene.height);
        let mut texture = texture_builder
            .create_texture(
                Some(PixelFormatEnum::RGB24),
                TextureAccess::Streaming,
                scene.width,
                scene.height,
            ).expect("failed to create image");

        texture
            .update(Some(rect), &buf, scene.width as usize * 3)
            .expect("failed to render");
        canvas
            .copy(&texture, Some(rect), Some(rect))
            .expect("failed to render");
        canvas.present();

        let render_time = render_start.elapsed();
        let render_time_secs = render_time.as_secs();
        let render_time_ns = render_time.subsec_nanos();
        trace!(
            "Rendering took {}ms",
            render_time_secs * 1000 + (render_time_ns / 1_000_000) as u64
        );
    }
}
