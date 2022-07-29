use anyhow::{anyhow, bail, Result};
use image::DynamicImage;
use std::path::PathBuf;

#[derive(Debug)]
pub struct NewImage {
    pub path: PathBuf,
    pub filename: String,
    pub image: DynamicImage,
}

impl NewImage {
    pub fn parse(f: PathBuf) -> Result<NewImage> {
        if !f.is_file() {
            bail!("{} is not a file!", f.to_string_lossy())
        }

        let path = f;
        let filename = match path.file_name() {
            Some(i) => i
                .to_os_string()
                .into_string()
                .map_err(|e| anyhow!("to_string failed on {}", e.to_string_lossy()))?,
            None => anyhow::bail!("{} is not a file", path.to_string_lossy()),
        };
        let image = image::open(&path)?;
        let e = NewImage {
            path,
            filename,
            image,
        };
        Ok(e)
    }
}

#[cfg(test)]
mod tests {
    use super::NewImage;
    use std::path::PathBuf;

    #[test]
    fn valid_file_converts() {
        let f: PathBuf = ["img", "me.jpg"].iter().collect();
        let i = NewImage::parse(f);

        assert!(i.is_ok());
        let u = i.unwrap();

        assert_eq!(["img", "me.jpg"].iter().collect::<PathBuf>(), u.path);
        assert_eq!("me.jpg", u.filename);
        assert_eq!(1024, u.image.width());
    }

    #[test]
    fn not_an_image() {
        let f: PathBuf = ["src", "lib.rs"].iter().collect();
        assert!(NewImage::parse(f).is_err())
    }

    #[test]
    fn file_does_not_exist() {
        let f: PathBuf = ["does", "not.exist"].iter().collect();
        assert!(NewImage::parse(f).is_err())
    }
}
