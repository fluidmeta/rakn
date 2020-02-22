use tempdir::TempDir;
use std::process::Command;
use walkdir::{WalkDir, DirEntry};

pub fn extract_docker_image(image_raw: &str) -> (TempDir, Vec<DirEntry>) {
    assert_eq!(Command::new("docker").args(&[
            "pull",
            image_raw,
        ]).status().expect("Could not pull docker image").success(), true);

    let tmp_dir_alloc = TempDir::new(env!("CARGO_PKG_NAME")).unwrap();
    let tag = image_raw.rsplit(":").collect::<Vec<&str>>()[0];
    let image = image_raw.split_at(image_raw.len() - tag.len() - 1).0;
    docker_extract::extract_image(image, tag, tmp_dir_alloc.path())
        .expect("Could not extract image");

    let excluded_dirs = vec!["/proc/", "/dev/"];
    let files_to_scan: Vec<DirEntry> = WalkDir::new(tmp_dir_alloc.path())
        .follow_links(false)
        .into_iter()
        .filter_entry(|d| {
            !excluded_dirs.contains(&d.path().to_str().unwrap())
        })
        .filter_map(|v| v.ok())
        .collect();

    (tmp_dir_alloc, files_to_scan)
}
