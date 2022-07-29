use img_trash::image::save_and_open_file;
use img_trash::image::NewImage;
use std::fs;

fn main() {
    let dir = fs::read_dir("img").expect("Unable to read from img");
    let files = dir.filter_map(|f| f.ok());

    for file in files {
        let n = match NewImage::parse(file.path()) {
            Ok(i) => i,
            Err(e) => {
                println!("!! Skipping file: {}", e);
                continue;
            }
        };

        println!("Scrambling {}", n.filename);
        let new_img = n.image.thumbnail(200, 200);
        let new_img = new_img.blur(10.0);

        println!("Done Scrambling {}, Saving!", n.filename);
        if let Err(e) = save_and_open_file(&n.path, &new_img) {
            println!("Unable to save and open {}! Err: {}", n.filename, e)
        }

        println!("Done with {}", n.filename);
    }
}
