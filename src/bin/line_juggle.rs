use image::{DynamicImage, GenericImage, GenericImageView};
use img_trash::domain::NewImage;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::env::args;
use std::fs;

fn main() {
    let skip_lines: bool = args().last() == Some("skip".to_string());

    let dir = fs::read_dir("img").expect("Unable to read from img");
    let files = dir.filter_map(|f| f.ok());

    for file in files {
        let n = match NewImage::parse(file) {
            Ok(i) => i,
            Err(e) => {
                println!("!! Skipping file: {}", e);
                continue;
            }
        };

        println!("Scrambling {}", n.filename);
        let new_img = shuffle_image_contents(n.image, skip_lines);

        println!("Done Scrambling {}, Saving!", n.filename);
        if let Err(e) = img_trash::save_and_open_file(&n.path, &new_img) {
            println!("Unable to save and open {}! Err: {}", n.filename, e)
        }

        println!("Done with {}", n.filename);
    }
}

fn shuffle_image_contents(img: DynamicImage, skip_lines: bool) -> DynamicImage {
    let mut rng = thread_rng();

    let (width, height) = img.dimensions();
    println!(
        "Dimensions {} by {} with color {:?}",
        width,
        height,
        img.color()
    );

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
