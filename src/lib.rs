use std::path::{Path};
use std::env;
use std::process::Command;
use image::DynamicImage;

pub fn get_files_from_args() -> Option<Vec<String>> {
    let files = env::args()
        .filter(|x| Path::new(x).extension().is_some())
        .filter(|x| Path::new(x).is_file())
        .collect::<Vec<String>>();
    match files.is_empty() {
        true => None,
        false => Some(files)
    }
}

pub fn save_and_open_file(old_filename: &Path, image_data: &DynamicImage) {
    let filename = old_filename.file_stem().unwrap().to_str().unwrap();
    let new_file = format!("img_out/{}.png", filename);
    let new_file = new_file.as_str();
    let new_file_path = Path::new(new_file);

    println!("Writing file {}", new_file);

    image_data.save(new_file_path).unwrap();
    Command::new("open").arg(new_file).spawn().expect("couldn't open");
}
