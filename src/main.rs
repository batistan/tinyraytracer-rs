mod geometry;
mod material;
mod object;

use std::io::{BufWriter, Write};
use crate::geometry::{Vec2f, Vec3f};
use crate::material::Material;
use crate::object::{Object, Sphere};

static BG_COLOR: [f32; 3] = [0.2f32, 0.7f32, 0.8f32];
static GREEN: [f32; 3] = [0f32, 1f32, 0f32];
static WIDTH: u32 = 1024;
static HEIGHT: u32 = 768;
static FOV: f32 = std::f32::consts::PI / 3.0;

fn scene_intersect<T>(orig: &Vec3f, dir: &Vec3f, objs: &[Box<T>], hit: &Vec3f, N: &Vec3f) -> (bool, Material)
    where T: Object {
    let mut distance = f32::MAX;
    let mut closest_mat: Material = Material::new(&Vec3f::new(3));

    for obj in objs {
        let (intersects, dist_i) = obj.ray_intersect(orig, dir);
        // objects closer to the camera will block further away ones
        if intersects && dist_i < distance {
            distance = dist_i;
            closest_mat = (*obj.get_material()).clone();
        }
    }

    return (distance < f32::MAX, closest_mat);
}

fn cast_ray<T>(orig: &Vec3f, dir: &Vec3f, objs: &[Box<T>]) -> Vec3f
    where T: Object {
    let (intersects, material) = scene_intersect(orig, dir, objs, &Vec3f::new(3), &Vec3f::new(3));
    if !intersects {
        Vec3f::from(&BG_COLOR)
    } else {
        // TODO compute color using shaders et al
        let color = [0.4f32, 0.4f32, 0.3f32];
        material.color().clone()
    }
}

/* camera defined as:
*  dimensions of the image
*  field of view angle
*  location of camera object in 3d space (as Vec3f)
*  camera orientation, default is directly along the negative z direction
*/
fn render<T>(objs: &[Box<T>]) where T: Object {
    let mut frame_buf: Vec<Vec3f> = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    let tan_fov = (FOV / 2 as f32).tan();
    let width_div_height = (WIDTH as f32) / (HEIGHT as f32);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2f32 * (i as f32 + 0.5) / WIDTH as f32 - 1f32) * tan_fov * width_div_height;
            let y = -(2f32 * (j as f32 + 0.5) / HEIGHT as f32 - 1f32) * tan_fov;
            let dir = Vec3f::from(&[x, y, -1f32]).normalize();
            // TODO we only render the first object
            // rendering anything more requires us to compute rays after being casted through multiple objects
            // so cast_ray should probably be changed to accommodate that, for each ray it might need to pass through several objs
            frame_buf.push(cast_ray(&Vec3f::from(&[0f32, 0f32, 0f32]), &dir, objs))
        }
    }

    write_buf_to_file(&frame_buf,
                      &Vec2f::from(&[WIDTH as f32, HEIGHT as f32]),
                      "./out.ppm");
}

fn write_buf_to_file(framebuffer: &[Vec3f], dimensions: &Vec2f, filename: &str) {
    let mut file = std::fs::File::create(filename).expect(&format!("Could not create file {}", filename));

    // definitely want buffered writer; we're talking about height*width writes of single pixels
    let mut stream = BufWriter::new(file);

    let width = dimensions[0];
    let height = dimensions[1];

    // ppm file header
    stream.write(format!("P6\n{} {}\n255\n", width, height).as_bytes());

    for i in 0..(width * height) as usize {
        for j in 0..3 {
            let byte = (255f32 * geometry::max(0f32, geometry::min(1f32, framebuffer[i][j]))) as u8;
            // println!("Writing byte {}", byte);
            stream.write(&[byte]).unwrap();
        }
    }

    stream.flush();

    // files are automatically closed when they go out of scope
    // consider using sync_all if we want to catch any issues with closing
}

fn main() {
    let ivory: Material = Material::new(&Vec3f::new3f(0.4f32, 0.4f32, 0.3f32));
    let rubber: Material = Material::new(&Vec3f::new3f(0.3f32, 0.1f32, 0.1f32));

    let spheres = [
        Box::new(Sphere::new(Vec3f::new3f(0f32, 0f32, -5f32), 1f32, &ivory)),
        Box::new(Sphere::new(Vec3f::new3f(-3f32, 0f32, -16f32), 2f32, &ivory)),
        Box::new(Sphere::new(Vec3f::new3f(-1f32, -1.5f32, -12f32), 2f32, &rubber)),
        Box::new(Sphere::new(Vec3f::new3f(1.5f32, -0.5f32, -18f32), 3f32, &rubber)),
        Box::new(Sphere::new(Vec3f::new3f(7f32, 5f32, -18f32), 4f32, &ivory))];
    render(&spheres[..]);
    return;
}
