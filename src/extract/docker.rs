extern crate docker_extract;

use std::io;
use tempdir::TempDir;

pub fn extract_image(image: &str, tmp_dir_alloc: &TempDir) -> io::Result<String> {
    let tag = image.rsplit(":").collect::<Vec<&str>>()[0];
    let image = image.split_at(image.len() - tag.len() - 1).0;
    docker_extract::extract_image(image, tag, tmp_dir_alloc.path())?;
    Ok(tmp_dir_alloc.path().display().to_string())
}
