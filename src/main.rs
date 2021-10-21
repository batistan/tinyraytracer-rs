mod geometry;

use std::io::{BufWriter, Write};

fn render() {
    let width = 1024;
    let height = 768;

    let mut frame_buf: Vec<geometry::Vec3f> = Vec::with_capacity(width * height);

    for j in 0..height {
        for i in 0..width {
            frame_buf.push(geometry::Vec3f::from_slice(&[(j as f32 / height as f32), (i as f32 / width as f32), 0f32]));
        }
    }

    let mut file = std::fs::File::create("./out.ppm").expect("Could not create file out.ppm");
    let mut stream = BufWriter::new(file);

    stream.write(format!("P6\n{} {}\n255\n", width, height).as_bytes());
    for i in 0..height * width {
        for j in 0..3 {
            let byte = (255f32 * geometry::max(0f32, geometry::min(1f32, frame_buf[i][j]))) as u8;
            // println!("Writing byte {}", byte);
            stream.write(&[byte]).unwrap();
        }
    }

    // files are automatically closed when they go out of scope
    // consider using sync_all if we want to catch any issues with closing
}

fn main() {
    render();
    return;
}
