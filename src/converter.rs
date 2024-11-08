use std::{fs::File, io::Write};

use image::{AnimationDecoder, EncodableLayout, GenericImageView, ImageDecoder};

#[derive(Debug, Clone)]
pub struct Converter {
    gif_name: String,
    logs: Vec<String>,
}


impl Converter {
    
    pub fn new() -> Self {
        Converter {
            gif_name: String::new(),
            logs: Vec::new(),
        }
    }
    
    pub fn convert(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        
        let gif_path = format!("{}.gif", self.gif_name);
    
        if !std::path::Path::new(&gif_path).exists() {
            eprintln!("Error: File not found - {}", gif_path);
            return Ok(());
        }
    
        let file = std::fs::File::open(gif_path.clone())?;
    
        let output_name = gif_path.split('.').next().unwrap();
    
        let decoder = image::codecs::gif::GifDecoder::new(file)?;
    
        let target_width = decoder.dimensions().0;
        let target_height = decoder.dimensions().1;
    
        let frames = decoder.into_frames().collect_frames()?;
    
        let frames_dir = create_or_get_dir("frames_cache");
    
        let total_frame_count = frames.len();
        let mut frame_count = 1;
        for frame in frames {
            let frame = frame;
    
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
    
            self.log(format!("Frame {}/{}", frame_count, total_frame_count).as_str());
            
            frame_count += 1;
        }
    
        let mut texture = create_image(target_width, target_height * (frame_count - 1));
    
        self.log("Creating texture");
        for frame in 1..frame_count {
            let image_path = frames_dir.join(format!("frame_{}.png", frame));
            if !image_path.exists() {
                self.log(format!("Frame {} does not exist", frame).as_str());
                continue;
            }
    
            let image = image::open(&image_path)?;
    
            let resized = image.resize(target_width, target_height, image::imageops::FilterType::Nearest);
    
            for (x, y, pixel) in resized.pixels() {
                texture.put_pixel(x, y + (frame - 1) * target_height, pixel);
            }
        }
    
    
        self.log("Saving output");
        texture.save(format!("{}.png", output_name))?;
    
        self.log("Creating mcmeta file");
        let output_meta_path = format!("{}.png.mcmeta", output_name);
        let frame_duration = 1;
        let mcmeta_content = format!(
            "{{\"animation\": {{\"frametime\": {}}}}}",
            frame_duration
        );
    
        let mut meta_file = File::create(output_meta_path)?;
        meta_file.write_all(mcmeta_content.as_bytes())?;
    
        self.log("Cleaning up");
        match delete_from_dir("frames_cache") {
            Ok(msg) => self.log(msg.as_str()),
            Err(e) => self.log(e.to_string().as_str())
        }
    
        self.log("Complete");
        Ok(())
    }
    
    fn log(&mut self, text: &str) {
        self.logs.push(text.to_string());
    }
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