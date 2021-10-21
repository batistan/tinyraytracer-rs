mod geometry;

use std::io::{BufWriter, Write};
use crate::geometry::{Object, Sphere};
use crate::geometry::{Vec2f, Vec3f};

static BG_COLOR: [f32; 3] = [0.2f32, 0.7f32, 0.8f32];
static WIDTH: u32 = 1024;
static HEIGHT: u32 = 768;
static FOV: f32 = std::f32::consts::PI / 3.0;

fn cast_ray(orig: &Vec3f, dir: &Vec3f, obj: &dyn Object) -> Vec3f {
    let distance = f32::MAX;

    let (intersects, dist) = obj.ray_intersect(orig, dir);
    if !intersects {
        Vec3f::from_slice(&BG_COLOR)
    } else {
        println!("yes!");
        // TODO compute color using shaders et al
        let color = [0.4f32, 0.4f32, 0.3f32];
        Vec3f::from_slice(&color)
    }
}

fn render_sphere(sphere: &Sphere) {}

/* camera defined as:
*  dimensions of the image
*  field of view angle
*  location of camera object in 3d space (as Vec3f)
*  camera orientation, default is directly along the negative z direction
*/
fn render(objs: &Vec<Box<dyn Object>>) {
    let mut frame_buf: Vec<Vec3f> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    let tan_fov = (FOV/2 as f32).tan();
    let width_div_height = (WIDTH/HEIGHT) as f32;

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2f32 * (i as f32 + 0.5) / WIDTH as f32- 1f32) * tan_fov * width_div_height;
            let y = -(2f32 * (j as f32 + 0.5)/ HEIGHT as f32- 1f32) * tan_fov;
            let dir = Vec3f::from_slice(&[x, y, -1f32]).normalize();
            // TODO we only render the first object
            // rendering anything more requires us to compute rays after being casted through multiple objects
            // so cast_ray should probably be changed to accommodate that, for each ray it might need to pass through several objs
            frame_buf.push(cast_ray(&Vec3f::from_slice(&[0f32, 0f32, 0f32]), &dir, objs[0].as_ref()))
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
    render(&vec![Box::new(Sphere::new(Vec3f::from_slice(&[0f32, 0f32, -5f32]),1f32))]);
    return;
}
