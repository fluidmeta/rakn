extern crate regex;

use regex::Regex;
use std::path::Path;
use std::{fmt, fs, io};

#[derive(Builder, Clone)]
pub struct DpkgBinary {
    package: String,
    version: String,
    source: String,
    arch: String,
}

impl DpkgBinary {
    pub fn get_package(&self) -> String {
        String::from(self.package.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }
}

#[derive(Builder, Clone)]
pub struct DpkgSource {
    package: String,
    version: String,
}

impl DpkgSource {
    pub fn get_package(&self) -> String {
        String::from(self.package.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }
}

#[derive(Debug)]
pub struct DpkgError {
    message: String,
}

impl From<io::Error> for DpkgError {
    fn from(error: io::Error) -> Self {
        DpkgError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for DpkgError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

pub fn scan(root_dir: &Path) -> Result<(Vec<DpkgBinary>, Vec<DpkgSource>), DpkgError> {
    let dpkg_status_file = format!("{}/var/lib/dpkg/status", root_dir.display().to_string());
    match Path::new(dpkg_status_file.as_str()).exists() {
        false => Err(DpkgError {
            message: format!("dpkg status file not found at {}", dpkg_status_file).to_string(),
        }),
        true => {
            let dpkg_status_content = fs::read_to_string(dpkg_status_file)?;
            Ok(parse_dpkg_status_content(dpkg_status_content.as_str()))
        }
    }
}

fn parse_dpkg_status_content(dpkg_status_content: &str) -> (Vec<DpkgBinary>, Vec<DpkgSource>) {
    lazy_static! {
        static ref RE_PACKAGE: Regex = Regex::new(r"(?m-i)^Package: (?P<package>.*)$").unwrap();
        static ref RE_STATUS: Regex = Regex::new(r"(?m-i)^Status: (?P<status>[\w\s]+)$").unwrap();
        static ref RE_VERSION: Regex = Regex::new(r"(?m-i)^Version: (?P<version>.*)$").unwrap();
        static ref RE_SOURCE: Regex = Regex::new(r"(?m-i)^Source: (?P<source>.*)$").unwrap();
        static ref RE_ARCH: Regex = Regex::new(r"(?m-i)^Architecture: (?P<arch>.*)$").unwrap();
    }

    let mut binary_packages: Vec<DpkgBinary> = vec![];
    let mut source_packages: Vec<DpkgSource> = vec![];

    for package_block in dpkg_status_content
        .split("\n\n")
        .collect::<Vec<&str>>()
        .iter()
    {
        let status = match RE_STATUS.captures(package_block) {
            Some(s) => s.name("status").unwrap().as_str(),
            None => "",
        };

        // check if package is marked as installed
        if String::from(status).contains("install ") {
            let package_name = match RE_PACKAGE.captures(package_block) {
                Some(s) => {
                    s.name("package")
                        .unwrap()
                        .as_str()
                        // remove :amd64 :i368 suffix from package name
                        .split(":")
                        .collect::<Vec<&str>>()[0]
                        .trim()
                }
                None => "",
            };

            let package_version = match RE_VERSION.captures(package_block) {
                Some(s) => s.name("version").unwrap().as_str(),
                None => "",
            };

            let package_source = match RE_SOURCE.captures(package_block) {
                Some(s) => {
                    s.name("source")
                        .unwrap()
                        .as_str()
                        // remove potential suffix e.g., 'gcc-8 (8.3.0-6ubuntu1~18.04.1)'
                        .split(" ")
                        .collect::<Vec<&str>>()[0]
                        .trim()
                }
                None => "",
            };

            let package_arch = match RE_ARCH.captures(package_block) {
                Some(s) => s.name("arch").unwrap().as_str(),
                None => "",
            };

            let binary_package = DpkgBinaryBuilder::default()
                .package(String::from(package_name))
                .version(String::from(package_version))
                .source(String::from(package_source))
                .arch(String::from(package_arch))
                .build()
                .unwrap();

            binary_packages.push(binary_package.clone());

            if !package_source.is_empty() && package_source != package_name {
                source_packages.push(
                    DpkgSourceBuilder::default()
                        .package(String::from(package_source))
                        .version(String::from(package_version))
                        .build()
                        .unwrap(),
                )
            }
        }
    }

    (binary_packages, source_packages)
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// Tests
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_dpkg_content_string() {
        let dpkg_content = "
Package: keybase
Status: install ok installed
Priority: optional
Section: misc
Installed-Size: 436172
Maintainer: Keybase Bugs <bugs@keybase.io>
Architecture: amd64
Version: 5.2.0-20200130211428.cf82db8320
Depends: libappindicator1, fuse, libgconf-2-4, psmisc, lsof, procps, libasound2, libnss3, libxss1, libxtst6, libgtk-3-0
Description: The Keybase Go client, filesystem, and GUI

Package: binutils-x86-64-linux-gnu
Status: install ok installed
Priority: optional
Section: devel
Installed-Size: 11758
Maintainer: Ubuntu Core developers <ubuntu-devel-discuss@lists.ubuntu.com>
Architecture: amd64
Multi-Arch: foreign
Source: binutils
Version: 2.30-21ubuntu1~18.04.2
Replaces: binutils (<< 2.29-6)
Depends: binutils-common (= 2.30-21ubuntu1~18.04.2), libbinutils (= 2.30-21ubuntu1~18.04.2), libc6 (>= 2.27), zlib1g (>= 1:1.1.4)
Suggests: binutils-doc (= 2.30-21ubuntu1~18.04.2)
Breaks: binutils (<< 2.29-6)
Description: GNU binary utilities, for x86-64-linux-gnu target
 This package provides GNU assembler, linker and binary utilities
 for the x86-64-linux-gnu target.
 .
 You don't need this package unless you plan to cross-compile programs
 for x86-64-linux-gnu and x86-64-linux-gnu is not your native platform.
Homepage: https://www.gnu.org/software/binutils/
Original-Maintainer: Matthias Klose <doko@debian.org>
";
        let (binary_packages, source_packages) = parse_dpkg_status_content(dpkg_content);
        assert_eq!(binary_packages.len(), 2);
        assert_eq!(source_packages.len(), 1);
    }
}
