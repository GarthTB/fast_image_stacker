use image::ImageBuffer;
use rayon::prelude::*;
use std::path::PathBuf;

pub(crate) fn stack_and_save(
    image_paths: Vec<PathBuf>,
    result_path: PathBuf,
    mode: u8,
) -> Result<(), &'static str> {
    let mut buffer: Vec<f64> = Vec::new();
    let mut dimensions = None; // throw an error when dimensions mismatch

    for (index, path) in image_paths.iter().enumerate() {
        let image = image::open(path).map_err(|_| "Failed to open image")?;
        let image = image.into_rgb16();
        let (width, height) = image.dimensions();
        let pixels = image.as_raw();

        // Check if dimensions match and set buffer if first image
        if let Some((expected_w, expected_h)) = dimensions {
            if width != expected_w || height != expected_h {
                return Err("Dimensions of images do not match");
            }
        } else {
            buffer = pixels.iter().map(|&v| v as f64).collect();
            dimensions = Some((width, height));
            continue;
        }

        // Stack the image
        buffer
            .par_iter_mut()
            .zip(pixels.par_iter())
            .for_each(|(old_val, &new_val)| {
                *old_val = update_pixel(*old_val, new_val, mode, index);
            });
        println!("\rStacked image {}/{}", index + 1, image_paths.len());
    }

    let result_pixels: Vec<u16> = buffer.par_iter().map(|v| v.round() as u16).collect();
    let (width, height) = dimensions.ok_or("No images found")?;
    let result_image: ImageBuffer<image::Rgb<u16>, Vec<u16>> =
        ImageBuffer::from_raw(width, height, result_pixels).ok_or("Invalid image buffer")?;
    result_image
        .save(result_path.as_path())
        .map_err(|_| "Failed to save image")?;
    println!("Saved result to {}", result_path.display());

    Ok(())
}

fn update_pixel(old_val: f64, new_val: u16, mode: u8, index: usize) -> f64 {
    match mode {
        0 => old_val * ((index - 1) as f64 / index as f64) + new_val as f64 / index as f64,
        1 => old_val.max(new_val as f64),
        2 => old_val.min(new_val as f64),
        _ => panic!("Invalid mode"),
    }
}
