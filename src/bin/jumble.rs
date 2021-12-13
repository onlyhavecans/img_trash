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
        let (length, width) = img.dimensions();
        println!("dimensions {} by {}", length, width);
        println!("{:?}", img.color());

        let mut new_img = img.clone();

        let size = img.pixels().count();
        let edit_count = size;
        println!("Editing {} pixels", edit_count);

        for _ in 0..edit_count {
            let source_length = rng.gen_range(1..length);
            let source_width = rng.gen_range(1..width);
            let destination_length = rng.gen_range(1..length);
            let destination_width = rng.gen_range(1..width);

            let pixel = img.get_pixel(source_length, source_width);
            new_img.put_pixel(destination_length,destination_width, pixel);
        }

        img_trash::save_and_open_file(&path, &new_img);
    }
}
