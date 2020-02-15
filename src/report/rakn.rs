use std::collections::HashMap;
use crate::common::package::{LibPackage, BinaryPackage, SourcePackage};
use crate::scanner::osinfo::OSInfoScanner;
use crate::common::report::ReportExt;

pub struct RaknReport {
    os_info: OSInfoScanner,
    os_packages: Vec<BinaryPackage>,
    source_packages: Vec<SourcePackage>,
    lib_packages: HashMap<String, Vec<LibPackage>>,
}

impl ReportExt for RaknReport {
    fn get_report(&self, _pretty: &bool) -> String {
        let mut report = String::from("=== Rakn Scan Report ===\n\n");
        report = [report, "Hostname:\t\t".to_string(), self.os_info.get_hostname(), "\n".to_string()].join(" ");
        report = [report, "OS:\t\t\t".to_string(), self.os_info.get_os(), "\n".to_string()].join(" ");
        report = [report, "OS Release:\t".to_string(), self.os_info.get_os_release_version(), "\n".to_string()].join(" ");
        report = [report, "Kernel:\t\t".to_string(), self.os_info.get_kernel(), "\n\n".to_string()].join(" ");

        report = [report, "Installed OS Packages".to_string(), "\n".to_string()].join(" ");
        for os_package in self.os_packages.iter() {
            report = [report, "--".to_string(), os_package.get_name(), ":".to_string(), os_package.get_version(), "\n".to_string()].join(" ");
        }

        report = [report, "\nInstalled libraries".to_string(), "\n".to_string()].join(" ");
        for (path, packages) in self.lib_packages.iter() {
            report = [report, "\n".to_string(), "path:".to_string(), String::from(path), "\n".to_string()].join(" ");
            for package in packages.iter() {
                report = [report, "--".to_string(), package.get_name(), ":".to_string(), package.get_version(), "\n".to_string()].join(" ");
            }
        }

        report
    }
}

impl RaknReport {
    pub fn new(os_info: OSInfoScanner,
               os_packages: Vec<BinaryPackage>,
               source_packages: Vec<SourcePackage>,
               lib_packages: HashMap<String, Vec<LibPackage>>) -> RaknReport {
        RaknReport {
            os_info,
            os_packages,
            source_packages,
            lib_packages,
        }
    }
}