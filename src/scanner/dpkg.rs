extern crate regex;
use crate::common::package::{BinaryPackage, BinaryPackageBuilder, SourcePackage, SourcePackageBuilder};
use std::process::Command;
use std::collections::HashMap;
use walkdir::DirEntry;
use std::fs;
use regex::Regex;

pub struct DpkgScanner {}

impl DpkgScanner {
    pub fn new() -> DpkgScanner {
        DpkgScanner {}
    }

    pub fn run(self) -> (Vec<BinaryPackage>, Vec<SourcePackage>) {
        // read /var/lib/dpkg/status
        // TODO: catch exception
        let contents = fs::read_to_string("/var/lib/dpkg/status")
            .expect("Cannot read /var/lib/dpkg/status");

        lazy_static! {
            static ref RE_PACKAGE: Regex = Regex::new(r"(?m-i)^Package: (.*)$").unwrap();
            static ref RE_STATUS: Regex = Regex::new(r"(?m-i)^Status:( [\w\s]+)$").unwrap();
            static ref RE_VERSION: Regex = Regex::new(r"(?m-i)^Version: (.*)$").unwrap();
            static ref RE_SOURCE: Regex = Regex::new(r"(?m-i)^Source: (.*)$").unwrap();
            // TODO: get '^Architecture: (.*)$'
        }

        let mut binary_packages: Vec<BinaryPackage> = Vec::new();
        let mut source_packages_map: HashMap<String, SourcePackageBuilder> = HashMap::new();

        for package_block in contents.split("\n\n").collect::<Vec<&str>>().iter() {
            let mut status = "".to_string();
            let mut package = "".to_string();
            let mut source = "".to_string();
            let mut version = "".to_string();

            // TODO: make sure only single match!
            for cap in RE_STATUS.captures_iter(package_block) {
                status = cap[1].to_string();
            }

            if status.contains(" install ") {
                // package is installed
                // TODO: make sure only single match!
                for cap in RE_PACKAGE.captures_iter(package_block) {
                    package = cap[1].trim().to_string();
                }
                // remove :amd64 :i368 suffix
                package = package.split(":").collect::<Vec<&str>>()[0].trim().to_string();

                // TODO: make sure only single match!
                for cap in RE_VERSION.captures_iter(package_block) {
                    version = cap[1].trim().to_string();
                }
                // TODO: make sure only single match!
                for cap in RE_SOURCE.captures_iter(package_block) {
                    source = cap[1].trim().to_string();
                }

                let binary_package = BinaryPackageBuilder::default()
                    .name(String::from(package.as_str()))
                    .version(String::from(version.as_str()))
                    .build()
                    .unwrap();
                binary_packages.push(binary_package.clone());

                // TODO: make as_str() foo more beautiful
                if String::from(source.as_str()) != "" && String::from(source.as_str()) != String::from(package.as_str()) {
                    let pkg_builder = source_packages_map.entry(String::from(source.as_str()))
                        .or_insert(SourcePackageBuilder::new()
                            .with_name(source.as_str())
                            .with_version(version.as_str()))
                        .add_binary_package(&binary_package.clone());
                    source_packages_map.insert(String::from(source), pkg_builder);
                }
            }
        }

        (binary_packages, source_packages_map.values().map(|x|x.finish()).collect())
    }
}
