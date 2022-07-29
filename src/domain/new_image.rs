use anyhow::{anyhow, Result};
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
        let path = f;
        let filename = match path.file_name() {
            Some(i) => i
                .to_str()
                .ok_or(anyhow!("Cannot convert to string {:?}", i))?
                .to_string(),
            None => anyhow::bail!("Not a file .; {:?}", path),
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
    use claim::{assert_err, assert_ok};
    use std::path::PathBuf;

    #[test]
    fn valid_file_converts() {
        let f: PathBuf = ["img", "me.jpg"].iter().collect();
        let i = NewImage::parse(f);
        assert_ok!(i);
    }

    #[test]
    fn not_an_image() {
        let f: PathBuf = ["src", "lib.rs"].iter().collect();
        let i = NewImage::parse(f);
        assert_err!(i);
    }

    #[test]
    fn file_does_not_exist() {
        let f: PathBuf = ["does", "not.exist"].iter().collect();
        let i = NewImage::parse(f);
        assert_err!(i);
    }
}
