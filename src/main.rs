use std::env;
use std::process;

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: fotosave <output-file>");
        eprintln!("Example: fotosave screenshot.png");
        process::exit(1);
    });

    let mut clipboard = arboard::Clipboard::new().unwrap_or_else(|e| {
        eprintln!("Failed to open clipboard: {e}");
        process::exit(1);
    });

    let img_data = clipboard.get_image().unwrap_or_else(|e| {
        eprintln!("No image in clipboard: {e}");
        process::exit(1);
    });

    let img = image::RgbaImage::from_raw(
        img_data.width as u32,
        img_data.height as u32,
        img_data.bytes.into_owned(),
    )
    .unwrap_or_else(|| {
        eprintln!("Clipboard image data is invalid");
        process::exit(1);
    });

    image::DynamicImage::ImageRgba8(img)
        .save(&path)
        .unwrap_or_else(|e| {
            eprintln!("Failed to save image to '{path}': {e}");
            process::exit(1);
        });

    println!("Saved to {path}");
}
