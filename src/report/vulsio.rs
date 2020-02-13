use serde::Serialize;
use std::collections::HashMap;
use crate::common::package::{LibPackage, BinaryPackage, SourcePackage};
use crate::scanner::osinfo::OSInfoScanner;
use crate::common::report::ReportExt;

/// Example .json
/// {
///  "serverName": "fishi0x01",
///  "family": "ubuntu",
///  "release": "18.04",
///  "runningKernel": {
///    "release": "4.15.0-76-generic",
///    "rebootRequired": false
///  },
///  "packages": {
///    "ntp": {
///      "name": "ntp",
///      "version": "4.2.6p5",
///    },
///    "openssh": {
///      "name": "openssh",
///      "version": "5.3p1",
///    }
///  },
/// "srcPackages": {
///    "bind9": {
///      "name": "bind9",
///      "version": "1:9.9.5.dfsg-9+deb8u15",
///      "binaryNames": [
///        "bind9-host"
///      ]
///    }
///  },
///  "libScanners": [
///    {
///      "path": "/home/fishi/py-venv/lib/python2.7/site-packages/Pipfile.lock",
///      "libs": [
///        {
///          "name": "cryptography",
///          "version": "2.8"
///        },
///        {
///          "name": "pip",
///          "version": "20.0.2"
///        }
///      ]
///    }
///   ]
/// }
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
    #[serde(rename = "packages")]
    binary_packages: HashMap<String, ReportBinaryPackage>,
    #[serde(rename = "srcPackages")]
    source_packages: HashMap<String, ReportSourcePackage>,
    #[serde(rename = "libScanners")]
    lib_scanners: Vec<LibScanner>,
}

impl VulsIOReport {
    pub fn new(os_info: OSInfoScanner,
               os_binary_packages: Vec<BinaryPackage>,
               os_source_packages: Vec<SourcePackage>,
               package_groups: HashMap<String, Vec<LibPackage>>) -> VulsIOReport {
        let mut binary_packages: HashMap<String, ReportBinaryPackage> = HashMap::new();
        for pkg in os_binary_packages.iter() {
            binary_packages.entry(pkg.get_name())
                .or_insert(ReportBinaryPackage {
                    name: pkg.get_name(),
                    version: pkg.get_version(),
                });
        }

        let mut lib_scanners: Vec<LibScanner> = Vec::new();
        for (p, packages) in package_groups.into_iter() {
            let mut libs: Vec<Lib> = Vec::new();
            for pkg in packages.into_iter() {
                let name = pkg.get_name();
                let version = pkg.get_version();
                libs.push(Lib {
                    name,
                    version,
                })
            }
            let mut path = String::from(p);
            path.push_str("/Pipfile.lock");
            lib_scanners.push(LibScanner {
                path,
                libs,
            });
        }

        let mut source_packages: HashMap<String, ReportSourcePackage> = HashMap::new();
        for source_package in os_source_packages.iter() {
            source_packages.entry(source_package.get_name())
                .or_insert(ReportSourcePackage{
                    name: source_package.get_name(),
                    version: source_package.get_version(),
                    binary_names: {
                        source_package
                            .get_binary_packages()
                            .iter()
                            .map(|x|x.get_name()).collect()
                    },
                });
        }

        VulsIOReport {
            server_name: String::from(os_info.get_hostname()),
            family: String::from(os_info.get_os()),
            release: String::from(os_info.get_os_release_version()),
            running_kernel: Kernel {
                release: String::from(os_info.get_kernel()),
                reboot_required: false,
            },
            binary_packages,
            source_packages,
            lib_scanners,
        }
    }
}

#[derive(Serialize)]
struct Kernel {
    release: String,
    #[serde(rename = "rebootRequired")]
    reboot_required: bool,
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

#[derive(Serialize)]
struct ReportBinaryPackage {
    name: String,
    version: String,
}

#[derive(Serialize)]
struct ReportSourcePackage {
    name: String,
    version: String,
    #[serde(rename = "binaryNames")]
    binary_names: Vec<String>,
}
