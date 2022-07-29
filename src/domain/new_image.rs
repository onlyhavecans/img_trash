use anyhow::Result;
use image::DynamicImage;
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(Debug)]
pub struct NewImage {
    pub path: PathBuf,
    pub filename: String,
    pub image: DynamicImage,
}

impl NewImage {
    pub fn parse(f: DirEntry) -> Result<NewImage> {
        let path = f.path();
        let filename = match f.file_name().into_string() {
            Ok(i) => i,
            Err(e) => anyhow::bail!("Couldn't turn filename into_string {:?}", e),
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
    use anyhow::Result;
    use claim::{assert_err, assert_ok};
    use std::fs;

    #[test]
    fn valid_file_converts() -> Result<()> {
        for x in fs::read_dir("img")? {
            let f = x?;
            assert_ok!(NewImage::parse(f));
        }
        Ok(())
    }

    #[test]
    fn not_an_image() -> Result<()> {
        for x in fs::read_dir("src")? {
            let f = x?;
            assert_err!(NewImage::parse(f));
        }
        Ok(())
    }
}
