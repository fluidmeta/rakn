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
use crate::scanner::{os, lib, pkg};

mod extract;
mod report;
pub mod scanner;
pub mod util;

#[derive(Builder, Clone)]
pub struct ScanResult {
    pub os_info: OSInfo,
    pub dpkg_binary_packages: Vec<pkg::dpkg::DpkgBinary>,
    pub dpkg_source_packages: Vec<pkg::dpkg::DpkgSource>,
    pub apk_packages: Vec<pkg::apk::ApkPackage>,
    pub rpm_packages: Vec<pkg::rpm::RpmPackage>,
    pub node_packages: Vec<lib::nodejs::NodeJsPackage>,
    pub python_packages: Vec<lib::python::PythonPackage>,
    pub ruby_packages: Vec<lib::ruby::RubyPackage>
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
        Some(i) => extract::docker::extract_image(i, &tmp_dir_alloc).unwrap(),
        None => "/".to_string(),
    };

    // get OS info
    let os_info = scan_os_info(Path::new(scan_root_dir.as_str()));

    // collect files eligible for scanning in scan root
    let files_to_scan = util::get_files_to_scan(Path::new(format!("{}/{}", scan_root_dir, scan_dir).as_str()), &excluded_dirs);

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

    // try parsing /var/lib/rpm/Packages
    let rpm_packages =
        match pkg::rpm::scan(Path::new(scan_root_dir.as_str())) {
            Err(_) => vec![],
            Ok(p) => p,
        };

    // get python libraries
    let python_packages = match lib::python::scan(&files_to_scan) {
        Err(_) => vec![],
        Ok(p) => p,
    };

    // get node libraries
    let node_packages = match lib::nodejs::scan(&files_to_scan) {
        Err(e) => vec![],
        Ok(p) => p,
    };

    // get ruby libraries
    let ruby_packages = match lib::ruby::scan(&files_to_scan) {
        Err(e) => vec![],
        Ok(p) => p,
    };

    let scan_result = ScanResultBuilder::default()
        .os_info(os_info)
        .dpkg_binary_packages(dpkg_binary_packages)
        .dpkg_source_packages(dpkg_source_packages)
        .apk_packages(apk_packages)
        .rpm_packages(rpm_packages)
        .node_packages(node_packages)
        .python_packages(python_packages)
        .ruby_packages(ruby_packages)
        .build()
        .unwrap();

    report::rakn::print(&scan_result);
}

#[derive(Builder, Clone)]
pub struct OSInfo {
    pub id: String,
    pub release: String,
    pub codename: String,
}

pub fn scan_os_info(root_dir: &Path) -> OSInfo {
    let debian_info = os::debian::scan(root_dir);
    let alpine_info = os::alpine::scan(root_dir);
    let redhat_info = os::redhat::scan(root_dir);

    match debian_info {
        Ok(info) => {
            return OSInfoBuilder::default()
                .id(info.get_id())
                .release(info.get_release())
                .codename(info.get_codename())
                .build()
                .unwrap()
        }
        Err(_) => {}
    }

    match alpine_info {
        Ok(info) => {
            return OSInfoBuilder::default()
                .id(info.get_id())
                .release(info.get_release())
                .codename(String::from(""))
                .build()
                .unwrap()
        }
        Err(_) => {}
    }

    match redhat_info {
        Ok(info) => {
            return OSInfoBuilder::default()
                .id(info.get_id())
                .release(info.get_release())
                .codename(String::from(""))
                .build()
                .unwrap()
        }
        Err(_) => {}
    }

    return OSInfoBuilder::default()
        .id(String::from(""))
        .release(String::from(""))
        .codename(String::from(""))
        .build()
        .unwrap();
}
