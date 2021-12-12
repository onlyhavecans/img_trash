use std::fs::File;
use std::path::{Path, PathBuf};
use std::env;
use std::process::Command;
use image::DynamicImage;

pub fn get_files_from_args() -> Vec<PathBuf> {
    let mut files = Vec::new();
    for argument in env::args() {
        if Path::new(&argument).extension().is_some() {
            let file = Path::new("img").join(&argument);
            files.push(file);
        }
    }
    files
}

pub fn save_and_open_file(old_filename: &Path, image_data: &DynamicImage) {
    let filename = old_filename.file_stem().unwrap().to_str().unwrap();
    let new_file = format!("img_out/{}.png", filename);
    let new_file = new_file.as_str();
    println!("Writing file {}", new_file);
    let file_out = &mut File::create(&Path::new(new_file)).unwrap();

    image_data.save(file_out, image::PNG).unwrap();
    Command::new("open").arg(new_file).spawn().expect("couldn't open");
}
