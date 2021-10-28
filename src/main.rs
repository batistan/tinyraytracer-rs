mod geometry;
mod material;
mod object;
mod light;

use std::io::{BufWriter, Write};
use crate::geometry::{Vec2f, Vec3f};
use crate::light::Light;
use crate::material::Material;
use crate::object::{Object, Sphere};

static BG_COLOR: [f32; 3] = [0.2f32, 0.7f32, 0.8f32];
static GREEN: [f32; 3] = [0f32, 1f32, 0f32];
static WIDTH: u32 = 1024;
static HEIGHT: u32 = 768;
// we could compute this from the viewport size and its distance to the camera
// but this way we don't have to
static FOV: f32 = std::f32::consts::PI / 3.0;
static MAX_REFLECT_BOUNCES: u32 = 4;

pub struct RayIntersectInfo {
    intersects_with_scene: bool,
    closest_material: Material,
    first_intersect_point: Vec3f,
    // vector perpendicular to point of intersection with object
    // you know how in blender, flat shading makes each face have a single normal vector which is the average of the normal vectors of all points on the face?
    // this is that normal vector for a given point
    // direction from object origin to the point of ray intersection, normalized because all "direction" vectors are normalized for convenience
    first_intersect_dir: Vec3f,
}

// initially, get properties of the first intersection the ray has with any object in the scene
fn scene_intersect<T>(orig: &Vec3f, dir: &Vec3f, objs: &[Box<T>]) -> RayIntersectInfo
    where T: Object {
    let mut distance = f32::MAX;
    let mut closest_material: Material = Material::new(&Vec3f::new(3), &Vec2f::new(2), 0.0);
    let mut first_intersect_point = Vec3f::new(3);
    let mut first_intersect_dir = Vec3f::new(3);

    for obj in objs {
        let (intersects, dist_i) = obj.ray_intersect(orig, dir);
        // objects closer to the camera will block further away ones
        if intersects && dist_i < distance {
            distance = dist_i;
            closest_material = (*obj.get_material()).clone();
            first_intersect_point = orig + &(dir * dist_i);
            first_intersect_dir = (&first_intersect_point - obj.get_position()).normalize();
        }
    }

    RayIntersectInfo {
        intersects_with_scene: distance < f32::MAX,
        closest_material,
        first_intersect_point,
        first_intersect_dir,
    }
}

// cast a ray into the scene from point orig, get back the color of that point on the canvas
// we need lights to determine how bright the point of intersection with the scene is, and thus to know how bright the pixel should be
// we need objs to know where our objects are in space and what their surface properties (color, roughness, etc) are
// depth is used for reflection
// we get the color contributed by the reflection of the object surface by recursively casting a ray from that point
// this ray may strike another object, and that other object may in turn have its own reflection, contributing the object's color and sending off another ray
// this continues until MAX_REFLECT_BOUNCES is reached
// i mean, realistically nobody will ever notice reflections more than 2 layers deep, but whatever
fn cast_ray<T>(orig: &Vec3f, dir: &Vec3f, lights: &[Light], objs: &[Box<T>], depth: u32) -> Vec3f
    where T: Object {
    let intersect_info = scene_intersect(orig, dir, objs);

    if !intersect_info.intersects_with_scene && depth < MAX_REFLECT_BOUNCES {
        Vec3f::from(&BG_COLOR)
    } else {
        let (diffuse_light_intensity, specular_light_intensity) = lights.iter().fold((0.0, 0.0), |val, light| {
            let light_vec = light.get_position() - &intersect_info.first_intersect_point;
            // direction of light onto intersection point (position of light source - point of intersect)
            // angle of incidence, i guess you could call it
            let light_dir = light_vec.normalize();
            let distance_to_light = light_vec.magnitude();

            // cast a "shadow ray" from the intersection point towards the light source
            // if the ray hits an object in the scene before reaching the light source, the light source doesn't illuminate this point (the point is in the shadow of that object)
            // origin is exactly the intersection point, moved a tiny bit along the normal
            // he says it's so that the shadow point doesn't lie exactly on the object surface, but i'm not sure
            let shadow_origin = if light_dir.dot(&intersect_info.first_intersect_dir) < 0.0 {
                &intersect_info.first_intersect_point - &(&intersect_info.first_intersect_dir * 0.001)
            } else {
                &intersect_info.first_intersect_point + &(&intersect_info.first_intersect_dir * 0.001)
            };

            // TODO shouldn't this be -light_dir, since we're going the opposite way?
            let shadow_intersect_info = scene_intersect(&shadow_origin, &light_dir, objs);

            // point lies in shadow of some object with regard to this light, don't contribute any color from the light
            if shadow_intersect_info.intersects_with_scene && (&shadow_intersect_info.first_intersect_point - &shadow_origin).magnitude() < distance_to_light {
                return (val.0, val.1)
            }

            // add contribution of this light source to this intersection point's diffuse intensity
            // light intensity is affected by how "head on" the surface is to the light source
            // e.g. if the normal of a plane is exactly parallel to the light ray, the plane will get the full force of that light and thus be brighter
            // if the normal of a plane is perpendicular to (or facing away from) the light ray, the plane isn't being illuminated at all, so the dot product is 0 (or negative, but negative brightness is out of scope)
            // and that gets multiplied by the light intensity
            // we know the "normal" of our "plane" here because it's the first_intersect_dir
            (val.0 + light.get_intensity() * geometry::max(0.0, light_dir.dot(&intersect_info.first_intersect_dir)),
             // i know this looks insane, but i have done the math, and it does work out. check my notes
             val.1 + f32::powf(geometry::max(0.0, geometry::reflect(&light_dir, &intersect_info.first_intersect_dir).dot(dir)),
                               intersect_info.closest_material.specular_exponent()) * light.get_intensity())
        });

        // we need this monstrosity of refs because of how we implemented multiplication on vecs as taking refs
        // in hindsight, whoops.
        let diffuse_color = &(intersect_info.closest_material.color() * diffuse_light_intensity)
            * intersect_info.closest_material.albedo()[0];
        let specular_color = &(&Vec3f::new3f(1.0, 1.0, 1.0) * specular_light_intensity)
            * intersect_info.closest_material.albedo()[1];

        &diffuse_color + &specular_color
    }
}

/* camera defined as:
*  dimensions of the image
*  field of view angle
*  location of camera object in 3d space (as Vec3f)
*  camera orientation, default is directly along the negative z direction
*/
fn render<T>(lights: &[Light], objs: &[Box<T>], filename: &str) where T: Object {
    let tan_fov = (FOV / 2.0).tan();
    let aspect_ratio = (WIDTH as f32) / (HEIGHT as f32);

    // TODO not sure this is the most idiomatic way to do this
    // but it makes sense given the whole "iterating over each pixel in the viewport" procedure
    let mut frame_buf: Vec<Vec3f> = Vec::with_capacity((WIDTH * HEIGHT) as usize);

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2f32 * (i as f32 + 0.5) / WIDTH as f32 - 1f32) * tan_fov * aspect_ratio;
            let y = -(2f32 * (j as f32 + 0.5) / HEIGHT as f32 - 1f32) * tan_fov;
            let dir = Vec3f::from(&[x, y, -1f32]).normalize();
            frame_buf.push(cast_ray(&Vec3f::from(&[0f32, 0f32, 0f32]), &dir, lights, objs, 1))
        }
    }

    write_buf_to_file(&frame_buf,
                      &Vec2f::from(&[WIDTH as f32, HEIGHT as f32]),
                      filename);
}

fn write_buf_to_file(framebuffer: &[Vec3f], dimensions: &Vec2f, filename: &str) {
    let file = std::fs::File::create(filename).unwrap_or_else(|err| { panic!("Could not create file {} due to {}", filename, err)});

    // definitely want buffered writer; we're talking about height*width writes of single pixels
    let mut stream = BufWriter::new(file);

    let width = dimensions[0];
    let height = dimensions[1];

    // ppm file header
    stream.write_all(format!("P6\n{} {}\n255\n", width, height).as_bytes()).unwrap_or_else(|err| { panic!("Failed to write header to file {} due to {}", filename, err)});

    for px in framebuffer {
        for channel in 0..3 {
            // TODO gamma/color correction
            // this may be done by some magical process we don't know about; figure it out
            // whatever you do for this keep in mind we're looking to support other filetypes, which may have their own approach
            let byte = (255f32 * geometry::max(0f32, geometry::min(1f32, px[channel]))) as u8;
            stream.write_all(&[byte]).unwrap_or_else(|err| { panic!("Failed to write pixel {:?} to file {} due to {}", px, filename, err)});
        }
    }

    stream.flush().unwrap();

    // files are automatically closed when they go out of scope
    // consider using sync_all if we want to catch any issues with closing
}

/* TODO for the whole project:
 * do we want to remain dependency-free?
 * Pros:
 *  - minimize scope: we set out to make a ray tracer; no need to start pulling in fluff
 *  - potential to learn about other aspects of computer graphics: parsing model and shader files
 *  - avoid the hell of not finishing any of our projects by keeping each one as lean as can be
 *  - flexing on the haters
 * Cons:
 *  - we have to implement everything ourselves
 *  - miss out on opportunity to learn about rust libs for computer graphics and file parsing
 */
fn main() {
    // TODO import materials/objects so we don't have to do these monstrosities
    // no need to do any fancy .obj or .mtl or PBR materials parsing; just a yaml for now is ok
    let ivory: Material = Material::new(&Vec3f::new3f(0.4f32, 0.4f32, 0.3f32), &Vec2f::from(&[0.6, 0.3]), 50.0);
    let rubber: Material = Material::new(&Vec3f::new3f(0.3f32, 0.1f32, 0.1f32), &Vec2f::from(&[0.9, 0.1]), 10.0);

    let lights = [Light::new(&Vec3f::new3f(-20.0, 20.0, 20.0), 1.5),
        Light::new(&Vec3f::new3f(30.0, 50.0, -25.0), 1.8),
        Light::new(&Vec3f::new3f(30.0, 20.0, 30.0), 1.7)];

    let spheres = [
        Box::new(Sphere::new(Vec3f::new3f(0f32, 0f32, -5f32), 1f32, &ivory)),
        Box::new(Sphere::new(Vec3f::new3f(-3f32, 0f32, -16f32), 2f32, &ivory)),
        Box::new(Sphere::new(Vec3f::new3f(-1f32, -1.5f32, -12f32), 2f32, &rubber)),
        Box::new(Sphere::new(Vec3f::new3f(1.5f32, -0.5f32, -18f32), 3f32, &rubber)),
        Box::new(Sphere::new(Vec3f::new3f(7f32, 5f32, -18f32), 4f32, &ivory))];

    // TODO render more than one frame
    // TODO support non-PPM files
    render(&lights, &spheres[..], "./out.ppm");
}
