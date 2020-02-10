extern crate clap;
extern crate walkdir;
extern crate regex;

use walkdir::{WalkDir, DirEntry};
use clap::{Arg, App};
use crate::common::scanner::LibScannerExt;
use std::fs;
use std::path::PathBuf;

mod scanner;
mod common;
mod report;

fn main() {
    // parse user input
    let matches = App::new("rakn")
        .version("0.1.0")
        .author("Karl Fischer <fishi0x01@gmail.com>")
        .about("Simple package version scanner")
        .arg(Arg::with_name("dir")
            .short("d")
            .long("dir")
            .value_name("DIR")
            .help("Which dir to scan recursively")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("exclude")
            .short("e")
            .long("exclude-dir")
            .value_name("DIR")
            .takes_value(true)
            .multiple(true))
        .get_matches();

    let dir = matches.value_of("dir").unwrap();
    let mut excluded_dirs:Vec<String>= Vec::new();
    if let Some(excluded) = matches.values_of("exclude") {
        for exclude in excluded.into_iter() {
            excluded_dirs.push(
                String::from(fs::canonicalize(PathBuf::from(exclude))
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
                )
            );
        }
    }

    // collect list of all files
    let metadata_files:Vec<DirEntry> = WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e|!is_excluded_dir(e, &excluded_dirs))
        .filter_map(|v| v.ok())
        .collect();

    // basic host info
    let os_info = scanner::osinfo::OSInfo::new();

    // scan files for python packages
    let py_scan = scanner::python::PythonScanner::new(metadata_files);
    let py_package_groups = py_scan.run();

    // print vuls.io report
    let vulsio_report = report::vulsio::VulsIOReport::new(os_info, py_package_groups);
    let json_str = serde_json::to_string_pretty(&vulsio_report).unwrap();
    println!("{}", json_str);

    // TODO: scan OS packages
    // TODO: scan golang packages
    // TODO: scan files for nodejs packages
    // TODO: scan ruby gems
}

fn is_excluded_dir(dir: &DirEntry, excluded_dirs: &Vec<String>) -> bool {
    excluded_dirs.contains(&dir.path().to_str().unwrap().to_string())
}
