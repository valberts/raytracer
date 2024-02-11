mod color;
mod window;
use color::RGB;
use glam::*;
use window::Window;

const CANVAS_WIDTH: usize = 600;
const CANVAS_HEIGHT: usize = 600;
const VIEWPORT_WIDTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 1.0;
const DISTANCE_CAMERA_VIEWPORT: f64 = 1.0;

#[derive(Clone, Debug)]
struct Scene {
    background_color: RGB,
    entities: Vec<SceneEntity>,
}

#[derive(Clone, Copy, Debug)]
enum SceneEntity {
    Sphere(SphereEntity),
    Light(LightType),
}

#[derive(Clone, Copy, Debug)]
enum LightType {
    Ambient(AmbientLightEntity),
    Point(PointLightEntity),
    Directional(DirectionalLightEntity),
}
#[derive(Clone, Copy, Debug, PartialEq)]
struct SphereEntity {
    center: DVec3,
    radius: f64,
    color: RGB,
    specular: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct AmbientLightEntity {
    intensity: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PointLightEntity {
    intensity: f64,
    position: DVec3,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct DirectionalLightEntity {
    intensity: f64,
    direction: DVec3,
}

fn canvas_to_viewport(x: f64, y: f64) -> DVec3 {
    DVec3::new(
        x * VIEWPORT_WIDTH / CANVAS_WIDTH as f64,
        y * VIEWPORT_HEIGHT / CANVAS_HEIGHT as f64,
        DISTANCE_CAMERA_VIEWPORT,
    )
}

fn trace_ray(origin: DVec3, direction: DVec3, t_min: f64, t_max: f64, scene: &Scene) -> RGB {
    let mut closest_t = f64::INFINITY;
    let mut closest_sphere = Option::<&SphereEntity>::None;
    for entity in scene.entities.iter() {
        #[allow(irrefutable_let_patterns)]
        if let SceneEntity::Sphere(sphere) = entity {
            let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);
            if (t1 > t_min) && (t1 < t_max) && (t1 < closest_t) {
                closest_t = t1;
                closest_sphere = Some(&sphere);
            }
            if (t2 > t_min) && (t2 < t_max) && (t2 < closest_t) {
                closest_t = t2;
                closest_sphere = Some(&sphere);
            }
        }
    }
    if let Some(sphere) = closest_sphere {
        let position = origin + closest_t * direction;
        let mut normal = position - sphere.center;
        normal = normal.normalize();
        let intensity = compute_lighting(position, normal, -direction, sphere.specular, scene);
        return sphere.color.multiply_by(intensity);
    } else {
        return scene.background_color;
    }
}

fn intersect_ray_sphere(origin: DVec3, direction: DVec3, sphere: &SphereEntity) -> (f64, f64) {
    let radius = sphere.radius;
    let center_origin = origin - sphere.center;

    let a = direction.dot(direction);
    let b = 2.0 * center_origin.dot(direction);
    let c = center_origin.dot(center_origin) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (f64::INFINITY, f64::INFINITY);
    }

    let t1 = (-b + f64::sqrt(discriminant)) / (2.0 * a);
    let t2 = (-b - f64::sqrt(discriminant)) / (2.0 * a);

    return (t1, t2);
}

fn compute_lighting(point: DVec3, normal: DVec3, view: DVec3, specular: i32, scene: &Scene) -> f64 {
    let mut i = 0.0;
    for entity in scene.entities.iter() {
        if let SceneEntity::Light(light_type) = entity {
            if let LightType::Ambient(ambient_light) = light_type {
                i += ambient_light.intensity;
            } else {
                let mut light_dir = DVec3::ZERO;
                let mut light_intensity = 0.0;
                if let LightType::Point(point_light) = light_type {
                    light_dir = point_light.position - point;
                    light_intensity = point_light.intensity;
                } else if let LightType::Directional(directional_light) = light_type {
                    light_dir = directional_light.direction;
                    light_intensity = directional_light.intensity;
                }

                // Diffuse
                let n_dot_l = normal.dot(light_dir);
                if n_dot_l > 0.0 {
                    i += light_intensity * n_dot_l / (normal.length() * light_dir.length())
                }

                // Specular
                if specular != -1 {
                    let reflection = 2.0 * normal * n_dot_l - light_dir;
                    let r_dot_v = reflection.dot(view);
                    if r_dot_v > 0.0 {
                        i += light_intensity
                            * f64::powi(r_dot_v / (reflection.length() * view.length()), specular);
                    }
                }
            }
        }
    }
    return i;
}

fn create_scene() -> Scene {
    Scene {
        background_color: RGB::new(255, 255, 255),
        entities: vec![
            SceneEntity::Sphere(SphereEntity {
                center: DVec3::new(0.0, -1.0, 3.0),
                radius: 1.0,
                color: RGB::new(255, 0, 0),
                specular: 1000,
            }),
            SceneEntity::Sphere(SphereEntity {
                center: DVec3::new(2.0, 0.0, 4.0),
                radius: 1.0,
                color: RGB::new(0, 0, 255),
                specular: 500,
            }),
            SceneEntity::Sphere(SphereEntity {
                center: DVec3::new(-2.0, 0.0, 4.0),
                radius: 1.0,
                color: RGB::new(0, 255, 0),
                specular: 10,
            }),
            SceneEntity::Light(LightType::Ambient(AmbientLightEntity { intensity: 0.2 })),
            SceneEntity::Light(LightType::Point(PointLightEntity {
                intensity: 0.6,
                position: DVec3::new(2.0, 1.0, 0.0),
            })),
            SceneEntity::Light(LightType::Directional(DirectionalLightEntity {
                intensity: 0.2,
                direction: DVec3::new(1.0, 4.0, 4.0),
            })),
        ],
    }
}

fn main() {
    let mut window = Window::new("raytracer", CANVAS_WIDTH, CANVAS_HEIGHT);
    window.limit_60_fps();

    let cw = CANVAS_WIDTH as i32;
    let ch = CANVAS_HEIGHT as i32;

    let scene = create_scene();
    let mut origin = DVec3::new(0.0, 0.0, 0.0);

    while !window.should_close() {
        window.handle_input(&mut origin);
        let framebuffer = window.framebuffer();
        for x in -cw / 2..cw / 2 {
            for y in -ch / 2..ch / 2 {
                let direction = canvas_to_viewport(x as f64, y as f64);
                let color = trace_ray(origin, direction, 1.0, f64::INFINITY, &scene);
                framebuffer.put_pixel(x as usize, y as usize, color);
            }
        }
        window.update();
    }
}
