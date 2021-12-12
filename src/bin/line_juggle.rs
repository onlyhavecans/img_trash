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

        let filename = file.file_name().into_string().unwrap();

        println!("Scrambling {}", filename);
        let new_img = shuffle_image_contents(image, skip_lines);

        println!("Done Scrambing {}, Saving!", filename);
        img_trash::save_and_open_file(&path, &new_img);

        println!("Done with {}", filename);
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

    for y in 0..height {
        if skip_lines && y % 2 == 0 {
            continue;
        }
        let shuffled_y: u32 = shuffled_lines[y as usize];
        for x in 0..width {
            new_img.put_pixel(x, y, img.get_pixel(x, shuffled_y));
        }
    }
    new_img
}
