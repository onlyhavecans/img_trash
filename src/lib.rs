pub mod domain;

use anyhow::{bail, Result};
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

pub fn save_and_open_file(old_file: &Path, image_data: &DynamicImage) -> Result<()> {
    // this validator saves a lot or error handling later
    if !old_file.is_file() {
        bail!("{} is not a file!", old_file.to_string_lossy())
    }

    let out_path = Path::new("img_out");
    fs::create_dir_all(out_path)?;

    let out_format = "png";
    let new_file_type = old_file.with_extension(out_format);
    let new_file_path = out_path.join(new_file_type.file_name().unwrap());
    let new_file_name = new_file_path.to_string_lossy();

    println!("Writing file {new_file_name}");

    image_data.save(new_file_path.as_os_str())?;
    Command::new("open").arg(new_file_path).spawn()?;
    Ok(())
}
