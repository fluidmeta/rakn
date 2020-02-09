extern crate clap;
extern crate walkdir;
extern crate regex;

use walkdir::{WalkDir, DirEntry};
use clap::{Arg, App};
use crate::core::scanner::Scanner;

mod scanner;
mod core;
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
        .get_matches();

    let dir = matches.value_of("dir").unwrap();

    // collect list of all files
    let metadata_files:Vec<DirEntry> = WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();

    // scan files for python packages
    let py_scan = scanner::python::PythonScanner::new(metadata_files);
    let py_package_groups = py_scan.run();

    // print vuls.io report
    let vulsio_report = report::vulsio::VulsIOReport::new(py_package_groups);
    let json_str = serde_json::to_string_pretty(&vulsio_report).unwrap();
    println!("{}", json_str);

    // TODO: scan OS packages
    // TODO: scan golang packages
    // TODO: scan files for nodejs packages
    // TODO: scan ruby gems
}
