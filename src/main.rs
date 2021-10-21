mod geometry;
mod material;
mod object;

use std::io::{BufWriter, Write};
use crate::geometry::{Vec2f, Vec3f};
use crate::material::{Material, IVORY, RUBBER};
use crate::object::{Object, Sphere};

static BG_COLOR: [f32; 3] = [0.2f32, 0.7f32, 0.8f32];
static GREEN: [f32; 3] = [0f32, 1f32, 0f32];
static WIDTH: u32 = 1024;
static HEIGHT: u32 = 768;
static FOV: f32 = std::f32::consts::PI / 3.0;

fn scene_intersect(orig: &Vec3f, dir: &Vec3f, objs: &Vec<&Box<dyn Object>>, hit: &Vec3f, N: &Vec3f) -> (bool, Material) {
    let mut distance = f32::MAX;
    let mut closest_mat: Material = Material::new(&Vec3f::new(3));

    for obj in objs {
        let (intersects, dist_i) = obj.ray_intersect(orig, dir);
        // objects closer to the camera will block further away ones
        if intersects && dist_i < distance {
            distance = dist_i;
            closest_mat = *(obj.get_material().clone());
        }
    }

    return (distance < f32::MAX, closest_mat.clone());
}

fn cast_ray(orig: &Vec3f, dir: &Vec3f, objs: &Vec<&Box<dyn Object>>) -> Vec3f {
    let (intersects, material) = scene_intersect(orig, dir, objs, &Vec3f::new(3), &Vec3f::new(3));
    if !intersects {
        Vec3f::from_slice(&BG_COLOR)
    } else {
        println!("yes!");
        // TODO compute color using shaders et al
        let color = [0.4f32, 0.4f32, 0.3f32];
        material.color()
    }
}

/* camera defined as:
*  dimensions of the image
*  field of view angle
*  location of camera object in 3d space (as Vec3f)
*  camera orientation, default is directly along the negative z direction
*/
fn render(objs: &Vec<Box<dyn Object>>) {
    let mut frame_buf: Vec<Vec3f> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    let tan_fov = (FOV / 2 as f32).tan();
    let width_div_height = (WIDTH / HEIGHT) as f32;

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2f32 * (i as f32 + 0.5) / WIDTH as f32 - 1f32) * tan_fov * width_div_height;
            let y = -(2f32 * (j as f32 + 0.5) / HEIGHT as f32 - 1f32) * tan_fov;
            let dir = Vec3f::from_slice(&[x, y, -1f32]).normalize();
            // TODO we only render the first object
            // rendering anything more requires us to compute rays after being casted through multiple objects
            // so cast_ray should probably be changed to accommodate that, for each ray it might need to pass through several objs
            frame_buf.push(cast_ray(&Vec3f::from_slice(&[0f32, 0f32, 0f32]), &dir, objs))
        }
    }

    write_buf_to_file(&frame_buf,
                      &Vec2f::from_slice(&[WIDTH as f32, HEIGHT as f32]),
                      "./out.ppm");
}

fn write_buf_to_file(framebuffer: &Vec<Vec3f>, dimensions: &Vec2f, filename: &str) {
    let mut file = std::fs::File::create(filename).expect("Could not create file out.ppm");
    let mut stream = BufWriter::new(file);

    let width = dimensions[0];
    let height = dimensions[1];

    stream.write(format!("P6\n{} {}\n255\n", width, height).as_bytes());
    for i in 0..(width * height) as usize {
        for j in 0..3 {
            let byte = (255f32 * geometry::max(0f32, geometry::min(1f32, framebuffer[i][j]))) as u8;
            // println!("Writing byte {}", byte);
            stream.write(&[byte]).unwrap();
        }
    }

    // files are automatically closed when they go out of scope
    // consider using sync_all if we want to catch any issues with closing
}

fn main() {
    let spheres = vec![Box::new(Sphere::new(Vec3f::from_slice(&[0f32, 0f32, -5f32]), 1f32, IVORY)),
                       Box::new(Sphere::new(Vec3f::from_slice(&[-3f32, 0f32, -16f32]), 2f32, IVORY)),
                       Box::new(Sphere::new(Vec3f::from_slice(&[-1f32, -1.5f32, -12f32]), 2f32, RUBBER)),
                       Box::new(Sphere::new(Vec3f::from_slice(&[1.5f32, -0.5f32, -18f32]), 3f32, RUBBER)),
                       Box::new(Sphere::new(Vec3f::from_slice(&[7f32, 5f32, -18f32]), 4f32, IVORY))];
    render(&spheres);
    return;
}
