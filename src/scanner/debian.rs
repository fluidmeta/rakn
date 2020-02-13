use crate::common::package::{BinaryPackage, BinaryPackageBuilder, SourcePackage, SourcePackageBuilder};
use crate::common::scanner::OSScannerExt;
use std::process::Command;
use std::collections::HashMap;

pub struct DebianScanner {}

impl OSScannerExt for DebianScanner {
    fn run(self) -> (Vec<BinaryPackage>, Vec<SourcePackage>) {
        let mut binary_packages: Vec<BinaryPackage> = Vec::new();
        let mut source_packages_map: HashMap<String, SourcePackageBuilder> = HashMap::new();
        let output = Command::new("dpkg-query")
            .args(&["-W", "-f=${binary:Package} , ${db:Status-Abbrev} , ${Version} , ${Source} , ${source:Version} \n"])
            .output()
            .expect("failed to execute process");
        let output_string = String::from_utf8_lossy(&output.stdout);

        // https://github.com/future-architect/vuls/blob/master/scan/debian.go#L377-L455
        for line in output_string.to_string().lines().into_iter() {
            let line_split = line.split(',').collect::<Vec<_>>();
            if line_split.len() == 5 {
                let v = line_split.iter().map(|s| String::from(*s))
                    .collect::<Vec<_>>();
                let (pkg_raw, db_abrv, version, src_raw, src_version) = (
                    v[0].as_str(), v[1].as_str(), v[2].as_str(), v[3].as_str(), v[4].as_str());
                // remove :amd64 :i368
                let pkg: &str = pkg_raw.split(":").collect::<Vec<&str>>()[0];
                // if package is installed
                if db_abrv.contains('i') {
                    let binary_package = BinaryPackageBuilder::new()
                        .with_name(pkg.trim())
                        .with_version(version.trim())
                        .finish();
                    binary_packages.push(binary_package.clone());

                    // remove funky suffixes, e.g., " (libx for linux)"
                    let src: &str = src_raw.trim().split(" ").collect::<Vec<&str>>()[0];
                    if src.trim() != "" && src.trim() != pkg && src.trim() != "linux" {
                        let pkg_builder = source_packages_map.entry(String::from(src.trim()))
                            .or_insert(SourcePackageBuilder::new()
                                .with_name(src.trim())
                                .with_version(src_version.trim()))
                            .add_binary_package(&binary_package.clone());
                        source_packages_map.insert(String::from(src.trim()), pkg_builder);
                    }
                }
            }
        }

        (binary_packages,
         source_packages_map.values().map(|x|x.finish()).collect())
    }
}

impl DebianScanner {
    pub fn new() -> DebianScanner {
        DebianScanner {}
    }
}
