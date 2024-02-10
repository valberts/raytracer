mod window;
use glam::*;
use window::Window;

const CANVAS_WIDTH: usize = 600;
const CANVAS_HEIGHT: usize = 600;
const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;
const DISTANCE_CAMERA_VIEWPORT: f64 = 1.0;

#[derive(Clone, Debug)]
struct Scene {
    background_color: u32,
    entities: Vec<SceneEntity>,
}

#[derive(Clone, Copy, Debug)]
enum SceneEntity {
    Sphere(SphereEntity),
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SphereEntity {
    center: DVec3,
    radius: f64,
    color: u32,
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn canvas_to_viewport(x: f64, y: f64) -> DVec3 {
    DVec3::new(
        x * VIEWPORT_WIDTH / CANVAS_WIDTH as f64,
        y * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f64,
        DISTANCE_CAMERA_VIEWPORT,
    )
}

fn trace_ray(origin: DVec3, direction: DVec3, t_min: f64, t_max: f64, scene: &Scene) {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = Option::<&SphereEntity>::None;
}

fn create_scene() -> Scene {
    Scene {
        background_color: from_u8_rgb(255, 255, 255),
        entities: vec![SceneEntity::Sphere(SphereEntity {
            center: DVec3::new(0.0, -1.0, 3.0),
            radius: 1.0,
            color: from_u8_rgb(255, 0, 0),
        })],
    }
}

fn main() {
    let mut window = Window::new("raytracer", CANVAS_WIDTH, CANVAS_HEIGHT);
    window.limit_60_fps();

    let cw = CANVAS_WIDTH as i32;
    let ch = CANVAS_HEIGHT as i32;

    let scene = create_scene();
    let origin = DVec3::new(0.0, 0.0, 0.0);

    while !window.should_close() {
        let framebuffer = window.framebuffer();
        for x in -cw / 2..cw / 2 {
            for y in -ch / 2..ch / 2 {
                let direction = canvas_to_viewport(x as f64, y as f64);
                framebuffer.put_pixel(x as usize, y as usize, from_u8_rgb(255, 0, 0));
            }
        }
        window.update();
    }
}
