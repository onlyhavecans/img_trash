use img_trash::domain::NewImage;
use std::fs;

fn main() {
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
        let new_img = n.image.thumbnail(200, 200);
        let new_img = new_img.blur(10.0);

        println!("Done Scrambling {}, Saving!", n.filename);
        if let Err(e) = img_trash::save_and_open_file(&n.path, &new_img) {
            println!("Unable to save and open {}! Err: {}", n.filename, e)
        }

        println!("Done with {}", n.filename);
    }
}
