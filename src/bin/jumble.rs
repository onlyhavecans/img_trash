use std::fs::File;
use std::path::Path;
use std::process::Command;

use image::GenericImage;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    for file in img_trash::get_files_from_args() {

        let img = match image::open(file) {
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

        let fout = &mut File::create(&Path::new("test.png")).unwrap();

        // Write the contents of this image to the Writer in PNG format.
        new_img.save(fout, image::PNG).unwrap();
        Command::new("open").arg("test.png").spawn().expect("couldn't open");
    }
}
