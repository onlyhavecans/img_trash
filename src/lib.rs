use image::DynamicImage;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub fn get_files_from_args() -> Option<Vec<String>> {
    let files = env::args()
        .filter(|x| Path::new(x).extension().is_some())
        .filter(|x| Path::new(x).is_file())
        .collect::<Vec<String>>();
    match files.is_empty() {
        true => None,
        false => Some(files),
    }
}

pub fn save_and_open_file(old_filename: &Path, image_data: &DynamicImage) {
    let filename = old_filename.file_stem().unwrap().to_str().unwrap();
    let new_file = format!("{}.png", filename);
    let new_file = new_file.as_str();
    let out_path = Path::new("img_out");
    let new_file_path = out_path.join(Path::new(new_file));

    println!("Writing file {}", new_file);

    fs::create_dir_all(out_path).expect("Could not create output directory");
    image_data
        .save(new_file_path.as_os_str())
        .expect("Could not save output");
    Command::new("open")
        .arg(new_file_path)
        .spawn()
        .expect("couldn't open");
}
