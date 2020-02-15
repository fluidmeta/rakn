extern crate clap;
extern crate walkdir;
extern crate regex;

use walkdir::DirEntry;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;
use crate::common::package::{LibPackage, LibPackageBuilder, LibType};

pub struct PythonScanner {
    pub package_groups: HashMap<String, Vec<LibPackage>>,
    scan_files: Vec<DirEntry>,
}

impl PythonScanner {
    pub fn new(scan_files: Vec<DirEntry>) -> PythonScanner {
        PythonScanner {
            package_groups: HashMap::new(),
            scan_files,
        }
    }

    pub fn run(mut self) -> HashMap<String, Vec<LibPackage>> {
        let metadata_files: Vec<DirEntry> = self.scan_files
            .clone()
            .into_iter()
            .filter(|e| is_py_metadata_file(e))
            .collect();

        for entry in metadata_files.iter() {
            // parse package version and name
            let (name, version) = parse_py_metadata_file(&entry);
            let pkg = LibPackageBuilder::default()
                .name(name)
                .version(version)
                .lib_type(LibType::Python)
                .build()
                .unwrap();

            // get package path
            let mut path_buf = PathBuf::from(entry.path());
            path_buf.pop();
            path_buf.pop();
            let dir = fs::canonicalize(&path_buf)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();

            // add package to list of packages in same path
            self.package_groups
                .entry(dir)
                .or_insert(Vec::new())
                .push(pkg);
        }

        self.package_groups
    }
}

fn is_py_metadata_file(entry: &DirEntry) -> bool {
    entry.path()
        .to_str()
        .map(|s| s.ends_with(".dist-info/METADATA"))
        .unwrap_or(false)
}

fn parse_py_metadata_file(p: &DirEntry) -> (String, String) {
    lazy_static! {
        static ref RE_VERSION: Regex = Regex::new(r"(?m-i)^Version: ([\d\\.]+)$").unwrap();
        static ref RE_PACKAGE: Regex = Regex::new(r"(?m-i)^Name: ([\w_-]+)$").unwrap();
    }

    let path = p.path()
        .to_str()
        .unwrap();

    let contents = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let (mut name, mut version) = ("".to_string(), "".to_string());
    for cap in RE_VERSION.captures_iter(contents.as_str()) {
        version = cap[1].to_string();
    }
    for cap in RE_PACKAGE.captures_iter(contents.as_str()) {
        name = cap[1].to_string().to_lowercase();
    }

    (name, version)
}
