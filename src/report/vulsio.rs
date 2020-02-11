use serde::Serialize;
use std::collections::HashMap;
use crate::common::package::LibPackage;
use crate::scanner::osinfo::OSInfoScanner;
use crate::common::report::ReportExt;

impl ReportExt for VulsIOReport {
    fn get_report(&self, pretty: &bool) -> String {
        match pretty {
            true => serde_json::to_string_pretty(self).unwrap(),
            false => serde_json::to_string(self).unwrap()
        }
    }
}

#[derive(Serialize)]
pub struct VulsIOReport {
    #[serde(rename = "serverName")]
    server_name: String,
    family: String,
    release: String,
    #[serde(rename = "runningKernel")]
    running_kernel: Kernel,
    #[serde(rename = "libScanners")]
    lib_scanners: Vec<LibScanner>,
}

impl VulsIOReport {
    pub fn new(os_info: OSInfoScanner, package_groups: HashMap<String,Vec<LibPackage>>) -> VulsIOReport {
        let mut lib_scanners: Vec<LibScanner> = Vec::new();
        for (p, packages) in package_groups.into_iter() {
            let mut libs: Vec<Lib> = Vec::new();
            for pkg in packages.into_iter() {
                let name = pkg.get_name();
                let version = pkg.get_version();
                libs.push(Lib{
                    name,
                    version,
                })
            }
            let mut path = String::from(p);
            path.push_str("/Pipfile.lock");
            lib_scanners.push(LibScanner{
                path,
                libs,
            });
        }

        VulsIOReport {
            server_name: String::from(os_info.get_hostname()),
            family: String::from(os_info.get_os()),
            release: String::from(os_info.get_os_release_version()),
            running_kernel: Kernel {
                release: String::from(os_info.get_kernel()),
                version: "".to_string(),
                reboot_required: "".to_string(),
            },
            lib_scanners,
        }
    }
}

#[derive(Serialize)]
struct Kernel {
    release: String,
    version: String,
    #[serde(rename = "rebootRequired")]
    reboot_required: String,
}

#[derive(Serialize)]
struct LibScanner {
    path: String,
    libs: Vec<Lib>,
}

#[derive(Serialize)]
struct Lib {
    name: String,
    version: String,
}
