extern crate image;
extern crate rand;
extern crate img_trash;

use std::fs::read_dir;

use image::{GenericImage, DynamicImage};
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    if let Ok(files) = read_dir("img") {
        for file in files {
            let file = file.unwrap();
            let img: DynamicImage = match image::open(file.path()) {
                Ok(i) => i,
                Err(e) => {
                    println!("!! Skipping because of {:?}", e);
                    continue;
                }
            };
            let (length, width) = img.dimensions();
            println!("dimensions {} by {}", length, width);
            println!("{:?}", img.color());

            let mut new_img = img.clone();

            let mut range: Vec<u32> = Vec::new();
            for n in 0..width {
                range.push(n);
            };
            let shuffled_lines = &mut range[..];
            rng.shuffle(shuffled_lines);

            let mut place: u32 = 0;
            for line in shuffled_lines {
                for scan in 0..length {
                    let pixel = img.get_pixel(scan, *line);
                    new_img.put_pixel(scan, place, pixel);
                }
                place = place + 1;
            };

            img_trash::save_and_open_file(&file.path(), new_img);
        }
    }
}
