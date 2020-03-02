use walkdir::{DirEntry, WalkDir};
use crate::scanner;
use std::path::Path;

pub fn get_files_to_scan(root_dir: &Path, excludes: &Vec<&str>) -> Vec<DirEntry> {
    WalkDir::new(root_dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|d| {
            // TODO: remove from scan_root_dir prefix
            !excludes.contains(&d.path().to_str().unwrap())
        })
        .filter_map(|v| v.ok())
        .filter(|d|{
            scanner::lib::nodejs::is_relevant_file(d) ||
                scanner::lib::python::is_relevant_file(d) ||
                scanner::lib::ruby::is_relevant_file(d) ||
                scanner::pkg::dpkg::is_relevant_file(d) ||
                scanner::pkg::apk::is_relevant_file(d) ||
                scanner::pkg::rpm::is_relevant_file(d) ||
                scanner::os::redhat::is_relevant_file(d) ||
                scanner::os::alpine::is_relevant_file(d) ||
                scanner::os::debian::is_relevant_file(d)
        })
        .collect()
}
