#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate walkdir;
extern crate regex;

use walkdir::{WalkDir, DirEntry};
use clap::{Arg, App};
use std::fs;
use std::path::PathBuf;
use common::report::OutputType;
use crate::common::report::ReportExt;
use crate::scanner::osinfo::OSInfoScanner;

mod scanner;
mod common;
mod report;
mod libscan;
mod osscan;

fn main() {
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
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("TYPE")
            .help("Allowed are 'vulsio' and 'rakn' (default)")
            .default_value("rakn")
            .takes_value(true))
        .arg(Arg::with_name("pretty")
            .short("p")
            .long("pretty")
            .takes_value(false))
        .get_matches();

    // ***************
    // Parse arguments
    // ***************
    let dir = matches.value_of("dir").unwrap();
    let mut excluded_dirs: Vec<String> = Vec::new();
    if let Some(excluded) = matches.values_of("exclude") {
        for exclude in excluded.into_iter() {
            let os_dir = fs::canonicalize(PathBuf::from(exclude));
            match os_dir {
                Ok(os_dir) => excluded_dirs.push(String::from(os_dir.to_str().unwrap())),
                _ => (),
            }
        }
    }

    let output = match matches.value_of("output").unwrap() {
        "vulsio" => OutputType::VulsIO,
        _ => OutputType::Rakn,
    };

    let pretty = matches.is_present("pretty");

    // collect list of all files
    let files_to_scan: Vec<DirEntry> = WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| !is_excluded_dir(e, &excluded_dirs))
        .filter_map(|v| v.ok())
        .collect();

    // ******
    // Scans
    // ******
    // OS
    let os_info = OSInfoScanner::new();
    let (os_packages, source_packages) = osscan::scan(&os_info);

    // Lib packages
    let py_package_groups = libscan::scan(files_to_scan.clone());

    // *******
    // Report
    // *******
    match output {
        OutputType::VulsIO => {
            let vulsio_report = report::vulsio::VulsIOReport::new(os_info, os_packages, source_packages, py_package_groups);
            println!("{}", vulsio_report.get_report(&pretty));
        }
        OutputType::Rakn => {
            let rakn_report = report::rakn::RaknReport::new(os_info, os_packages, source_packages, py_package_groups);
            println!("{}", rakn_report.get_report(&pretty));
        }
    }

    // TODO: scan golang packages
    // TODO: scan nodejs packages
    // TODO: scan ruby gems
}

fn is_excluded_dir(dir: &DirEntry, excluded_dirs: &Vec<String>) -> bool {
    excluded_dirs.contains(&dir.path().to_str().unwrap().to_string())
}
