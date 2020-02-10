use serde::Serialize;
use std::collections::HashMap;
use crate::common::package::LibPackage;
use crate::scanner::osinfo::OSInfo;

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
    pub fn new(os_info: OSInfo, package_groups: HashMap<String,Vec<LibPackage>>) -> VulsIOReport {
        let mut lib_scanners: Vec<LibScanner> = Vec::new();
        for (p, packages) in package_groups.into_iter() {
            let mut libs: Vec<Lib> = Vec::new();
            for pkg in packages.into_iter() {
                let (name, version) = pkg.get();
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
            server_name: String::from(os_info.hostname),
            family: String::from(os_info.os),
            release: String::from(os_info.os_release_version),
            running_kernel: Kernel {
                release: String::from(os_info.kernel),
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
