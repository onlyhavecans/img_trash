use std::fs::read_dir;

use image::{GenericImage, GenericImageView};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    let files = read_dir("img")
        .unwrap()
        .filter_map(|f| f.ok());

    for file in files {
        let path = file.path();
        let filename = file.file_name().into_string().unwrap();
        let img = match image::open(&path) {
            Ok(i) => i,
            Err(e) => {
                println!("!! Skipping {} because of {:?}", filename, e);
                continue;
            }
        };
        let (width, height) = img.dimensions();
        println!("dimensions {} by {}; color {:?}", width, height, img.color());

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
            new_img.put_pixel(destination_width,destination_height, pixel);
        }

        img_trash::save_and_open_file(&path, &new_img);
    }
}
