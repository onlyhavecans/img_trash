use std::fs::read_dir;

fn main() {
    let files = read_dir("img").unwrap().filter_map(|f| f.ok());

    for file in files {
        let path = file.path();
        let filename = file.file_name().into_string().unwrap();
        let image = match image::open(&path) {
            Ok(i) => i,
            Err(e) => {
                println!("!! Skipping {} because of {:?}", filename, e);
                continue;
            }
        };

        println!("Scrambling {}", filename);
        let new_img = image.thumbnail(200, 200);
        let new_img = new_img.blur(10.0);

        println!("Done Scrambing {}, Saving!", filename);
        img_trash::save_and_open_file(&path, &new_img);

        println!("Done with {}", filename);
    }
}
