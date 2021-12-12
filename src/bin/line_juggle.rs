use image::{GenericImage, DynamicImage};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env::args;
use std::fs::read_dir;

fn main() {
    let skip_lines: bool = args().last().unwrap() == "skip";

    let files = read_dir("img")
        .unwrap()
        .filter_map(|f| f.ok());

    for file in files {
        let path = file.path();
        let image = match image::open(&path) {
            Ok(i) => i,
            Err(e) => {
                println!("!! Skipping because of {:?}", e);
                continue;
            }
        };

        let new_img = shuffle_image_contents(image, skip_lines);
        img_trash::save_and_open_file(&path, &new_img);
    }
}

fn shuffle_image_contents(img: DynamicImage, skip_lines: bool) -> DynamicImage {
    let mut rng = thread_rng();

    let (width, height) = img.dimensions();
    println!("dimensions {} by {}", width, height);
    println!("{:?}", img.color());

    let mut new_img = img.clone();

    let mut shuffled_lines: Vec<u32> = (0..height).collect();
    shuffled_lines.shuffle(&mut rng);

    let mut place_in_old: u32 = 0;
    for line in shuffled_lines {
        if skip_lines && place_in_old % 2 == 0 {
            place_in_old += 1;
            continue;
        }
        for x in 0..width {
            new_img.put_pixel(x, place_in_old, img.get_pixel(x, line));
        }
        place_in_old += 1;
    };
    new_img
}
