use image::{GenericImage, GenericImageView};
use img_trash::domain::NewImage;
use rand::{thread_rng, Rng};
use std::fs;

fn main() {
    let mut rng = thread_rng();
    let dir = fs::read_dir("img").expect("Unable to read from img");
    let files = dir.filter_map(|f| f.ok());

    for file in files {
        println!("Opening {}", file.file_name().to_string_lossy());

        let n = match NewImage::parse(file.path()) {
            Ok(i) => i,
            Err(e) => {
                println!("!! Skipping file: {}", e);
                continue;
            }
        };
        let img = n.image;

        let (width, height) = img.dimensions();
        println!(
            "dimensions {} by {}; color {:?}",
            width,
            height,
            img.color()
        );

        let mut new_img = img.clone();

        let pixel_count = img.pixels().count();
        println!("Editing {} pixels", &pixel_count);

        /* This was is pure nose, cool but not as much as the other one
        for x in 0..width {
            for y in 0..height {
                let source_width = rng.gen_range(0..width);
                let source_height = rng.gen_range(0..height);

                new_img.put_pixel(x, y, img.get_pixel(source_width, source_height));
            }
        }
        */

        for _ in 0..pixel_count {
            let source_width = rng.gen_range(0..width);
            let source_height = rng.gen_range(0..height);
            let destination_width = rng.gen_range(0..width);
            let destination_height = rng.gen_range(0..height);

            let pixel = img.get_pixel(source_width, source_height);
            new_img.put_pixel(destination_width, destination_height, pixel);
        }

        if let Err(e) = img_trash::save_and_open_file(&n.path, &new_img) {
            println!("Unable to save and open {}! Err: {}", n.filename, e)
        }
    }
}
