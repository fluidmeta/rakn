#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate regex;
extern crate tempdir;
extern crate walkdir;

use clap::{App, Arg};
use std::path::Path;
use tempdir::TempDir;
use walkdir::{DirEntry, WalkDir};
use crate::scanner::{lib, pkg};
use crate::os::OSInfo;

mod docker;
mod report;
mod scanner;
mod os;

#[derive(Builder, Clone)]
pub struct ScanResult {
    pub os_info: OSInfo,
    pub dpkg_binary_packages: Vec<pkg::dpkg::DpkgBinary>,
    pub dpkg_source_packages: Vec<pkg::dpkg::DpkgSource>,
    pub apk_packages: Vec<pkg::apk::ApkPackage>,
    pub python_packages: Vec<lib::python::PythonPackage>,
}

fn main() {
    let matches = App::new("rakn")
        .version("0.1.0")
        .author("Karl Fischer <fishi0x01@gmail.com>")
        .about("Simple version scanner")
        .arg(
            Arg::with_name("docker_image")
                .short("i")
                .long("docker-image")
                .value_name("IMAGE")
                .help("Which docker image to scan")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("dir")
                .short("d")
                .long("dir")
                .value_name("DIR")
                .help("Which dir to scan recursively")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("exclude")
                .short("e")
                .long("exclude-dir")
                .value_name("DIR")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("TYPE")
                .help("Allowed are 'vulsio' and 'rakn' (default)")
                .default_value("rakn")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("pretty")
                .short("p")
                .long("pretty")
                .takes_value(false),
        )
        .get_matches();

    // ***************
    // Parse arguments
    // ***************
    let docker_image = matches.value_of("docker_image");
    let scan_dir = match matches.value_of("dir") {
        Some(d) => d,
        // Default
        None => "/",
    };
    let excluded_dirs = match matches.values_of("exclude") {
        Some(values) => values.into_iter().collect::<Vec<&str>>(),
        // Defaults
        None => vec!["/dev", "/proc", "/sys"],
    };

    // *********
    // Execution
    // *********
    let tmp_dir_alloc = TempDir::new(env!("CARGO_PKG_NAME")).unwrap();

    // determine scan root
    let scan_root_dir = match docker_image {
        Some(i) => docker::extract_image(i, &tmp_dir_alloc).unwrap(),
        None => "/".to_string(),
    };

    // collect files eligible for scanning in scan root
    let files_to_scan: Vec<DirEntry> = WalkDir::new(format!("{}/{}", scan_root_dir, scan_dir))
        .follow_links(false)
        .into_iter()
        .filter_entry(|d| {
            // TODO: remove from scan_root_dir prefix
            !excluded_dirs.contains(&d.path().to_str().unwrap())
        })
        .filter_map(|v| v.ok())
        .collect();

    let os_info = os::scan_os_info(Path::new(scan_root_dir.as_str()));

    // try parsing /var/lib/dpkg/status
    let (dpkg_binary_packages, dpkg_source_packages) =
        match pkg::dpkg::scan(Path::new(scan_root_dir.as_str())) {
            Err(_) => (vec![], vec![]),
            Ok(p) => p,
        };

    // try parsing /lib/apk/db/installed
    let apk_packages =
        match pkg::apk::scan(Path::new(scan_root_dir.as_str())) {
            Err(_) => vec![],
            Ok(p) => p,
        };

    // get python libraries
    let python_packages = match lib::python::scan(&files_to_scan) {
        Err(_) => vec![],
        Ok(p) => p,
    };

    let scan_result = ScanResultBuilder::default()
        .os_info(os_info)
        .dpkg_binary_packages(dpkg_binary_packages)
        .dpkg_source_packages(dpkg_source_packages)
        .apk_packages(apk_packages)
        .python_packages(python_packages)
        .build()
        .unwrap();

    report::rakn::print(&scan_result);
}
