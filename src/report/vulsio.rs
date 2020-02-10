use serde::Serialize;
use std::collections::HashMap;
use crate::core::package::Package;

// TODO: convert core structure to vulsio json structure

#[derive(Serialize)]
pub struct VulsIOReport {
    #[serde(rename = "serverName")]
    server_name: String,
    family: String,
    release: String,
    #[serde(rename = "libScanners")]
    lib_scanners: Vec<LibScanner>,
}

impl VulsIOReport {
    pub fn new(package_groups: HashMap<String,Vec<Package>>) -> VulsIOReport {
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
            server_name: "fishi-test".to_string(),
            family: "ubunu".to_string(),
            release: "18.04".to_string(),
            lib_scanners,
        }
    }
}

#[derive(Serialize)]
pub struct LibScanner {
    path: String,
    libs: Vec<Lib>,
}

#[derive(Serialize)]
pub struct Lib {
    name: String,
    version: String,
}
