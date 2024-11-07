use std::{fs::File, io::{stdin, Write}};

use image::{AnimationDecoder, EncodableLayout, GenericImageView, ImageDecoder};


//TODO: TUI Version
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_std = String::new();

    let _ = stdin().read_line(&mut input_std);

    let gif_path = format!("{}.gif", input_std.trim());

    if !std::path::Path::new(&gif_path).exists() {
        eprintln!("Error: File not found - {}", gif_path);
        return Ok(());
    }
    
    let file = std::fs::File::open(gif_path.clone())?;

    let output_name = gif_path.split('.').next().unwrap();

    let decoder = image::codecs::gif::GifDecoder::new(file)?;

    let target_width = decoder.dimensions().0;
    let target_height = decoder.dimensions().1;

    let frames = decoder.into_frames();

    let frames_dir = create_or_get_dir("frames_cache");

    let mut frame_count = 1;
    for frame in frames {
        let frame = frame.unwrap();

        let image = create_image(frame.buffer().width(), frame.buffer().height());
        let mut image = image.into_raw();
        let buffer = frame.buffer().as_bytes();
        for (i, pixel) in buffer.chunks(4).enumerate() {
            image[i * 4] = pixel[0];
            image[i * 4 + 1] = pixel[1];
            image[i * 4 + 2] = pixel[2];
            image[i * 4 + 3] = pixel[3];
        }
        
        let image = image::RgbaImage::from_raw(frame.buffer().width(), frame.buffer().height(), image).unwrap();
        image.save(frames_dir.join(format!("frame_{}.png", frame_count)))?;

        println!("Frame: {} | {} | {}", frame_count, frame.buffer().width(), frame.buffer().height());
        frame_count += 1;
    }

    let mut texture = create_image(target_width, target_height * (frame_count - 1));

    println!("Creating texture");
    for frame in 1..frame_count {
        let image_path = frames_dir.join(format!("frame_{}.png", frame));
        if !image_path.exists() {
            println!("Frame {} does not exist", frame);
            continue;
        }

        let image = image::open(&image_path)?;
        
        let resized = image.resize(target_width, target_height, image::imageops::FilterType::Nearest);

        for (x, y, pixel) in resized.pixels() {
            texture.put_pixel(x, y + (frame - 1) * target_height, pixel);
        }
    }

    
    println!("Saving output");
    texture.save(format!("{}.png", output_name))?;

    println!("Creating mcmeta file");
    let output_meta_path = format!("{}.png.mcmeta", output_name);
    let frame_duration = 1;
    let mcmeta_content = format!(
        "{{\"animation\": {{\"frametime\": {}}}}}",
        frame_duration
    );
    
    let mut meta_file = File::create(output_meta_path)?;
    meta_file.write_all(mcmeta_content.as_bytes())?;
    
    println!("Cleaning up");
    match delete_from_dir("frames_cache") {
        Ok(msg) => println!("{}", msg),
        Err(e) => println!("Error: {}", e)
    }

    println!("Complete");
    Ok(())
}

fn create_image( width: u32, height: u32) -> image::RgbaImage {
    image::RgbaImage::new(width, height)
}

fn create_or_get_dir(name: &str) -> std::path::PathBuf {
    let dir = std::path::Path::new(name);
    if !dir.exists() {
        std::fs::create_dir(dir).unwrap();
    }
    dir.to_path_buf()
}

fn delete_from_dir(dir_name: &str) -> Result<String, std::io::Error> {
    let dir = std::path::Path::new(dir_name);
    if dir.exists() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            std::fs::remove_file(entry.path())?;
        }
        Ok(format!("All files removed from {}", dir_name))
    } else {
        Ok(format!("{} does not exist", dir_name))
    }
}