use regex::Regex;
use std::path::Path;
use std::{fmt, fs, io};

#[derive(Debug)]
pub struct ApkError {
    message: String,
}

impl From<io::Error> for ApkError {
    fn from(error: io::Error) -> Self {
        ApkError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for ApkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

#[derive(Builder, Clone)]
pub struct ApkPackage {
    package: String,
    version: String,
    arch: String,
}

impl ApkPackage {
    pub fn get_package(&self) -> String {
        String::from(self.package.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }
}

pub fn scan(root_dir: &Path) -> Result<Vec<ApkPackage>, ApkError> {
    let apk_db_file = format!("{}/lib/apk/db/installed", root_dir.display().to_string());
    match Path::new(apk_db_file.as_str()).exists() {
        false => Err(ApkError {
            message: format!("apk db file not found at {}", apk_db_file).to_string(),
        }),
        true => {
            let apk_db_content = fs::read_to_string(apk_db_file)?;
            Ok(parse_apk_db_content(apk_db_content.as_str()))
        }
    }
}

fn parse_apk_db_content(apk_db_content: &str) -> Vec<ApkPackage> {
    lazy_static! {
        static ref RE_PACKAGE: Regex = Regex::new(r"(?m-i)^P:(?P<package>.*)$").unwrap();
        static ref RE_VERSION: Regex = Regex::new(r"(?m-i)^V:(?P<version>.*)$").unwrap();
        static ref RE_ARCH: Regex = Regex::new(r"(?m-i)^A:(?P<arch>.*)$").unwrap();
    }

    let mut apk_packages: Vec<ApkPackage> = vec![];

    for package_block in apk_db_content.split("\n\n").collect::<Vec<&str>>().iter() {
        let package_name = match RE_PACKAGE.captures(package_block) {
            Some(s) => s.name("package").unwrap().as_str(),
            None => "",
        };

        let package_version = match RE_VERSION.captures(package_block) {
            Some(s) => s.name("version").unwrap().as_str(),
            None => "",
        };

        let package_arch = match RE_ARCH.captures(package_block) {
            Some(s) => s.name("arch").unwrap().as_str(),
            None => "",
        };

        if !package_name.is_empty() {
            apk_packages.push(
                ApkPackageBuilder::default()
                    .package(String::from(package_name))
                    .version(String::from(package_version))
                    .arch(String::from(package_arch))
                    .build()
                    .unwrap(),
            );
        }
    }

    apk_packages
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// Tests
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_apk_db_content_string() {
        let apk_db_content = "
C:Q1p78yvTLG094tHE1+dToJGbmYzQE=
P:libc-utils
V:0.7.2-r0
A:x86_64
S:1175
I:4096
T:Meta package to pull in correct libc
U:http://alpinelinux.org
L:BSD
o:libc-dev
m:Natanael Copa <ncopa@alpinelinux.org>
t:1575749004
c:97b1c2842faa3bfa30f5811ffbf16d5ff9f1a479
D:musl-utils

C:Q1jQvjNNI/AcOc2K9lxV0zpSTXYyE=
P:musl-utils
V:1.1.24-r0
A:x86_64
S:38063
I:147456
T:the musl c library (libc) implementation
U:http://www.musl-libc.org/
L:MIT BSD GPL2+
o:musl
m:Timo Teräs <timo.teras@iki.fi>
t:1573824244
c:ba05f40c20ddc515f748f205f01befbba3a88feb
D:scanelf so:libc.musl-x86_64.so.1
p:cmd:getconf cmd:getent cmd:iconv cmd:ldconfig cmd:ldd
r:libiconv
F:sbin
R:ldconfig
a:0:0:755
Z:Q1Kja2+POZKxEkUOZqwSjC6kmaED4=
F:usr
F:usr/bin
R:iconv
a:0:0:755
Z:Q1+wx+n5UhaiCWlSrsvJ3350ZdggM=
R:ldd
a:0:0:755
Z:Q1yFAhGggmL7ERgbIA7KQxyTzf3ks=
R:getconf
a:0:0:755
Z:Q1K4FfucNzJLc83pAbwJxcFwuh160=
R:getent
a:0:0:755
Z:Q1iiUg38G3mUBMutaFzQsJQ2tDvFE=
";
        let apk_packages = parse_apk_db_content(apk_db_content);
        assert_eq!(apk_packages.len(), 2);
    }
}
