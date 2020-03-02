use regex::Regex;
use std::{fmt, fs, io};
use walkdir::DirEntry;
use serde_json::Value;

#[derive(Builder, Clone)]
pub struct RubyPackage {
    name: String,
    version: String,
    lib_path: String,
}

impl RubyPackage {
    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }

    pub fn get_lib_path(&self) -> String {
        String::from(self.lib_path.as_str())
    }
}

#[derive(Debug)]
pub struct RubyError {
    message: String,
}

impl From<io::Error> for RubyError {
    fn from(error: io::Error) -> Self {
        RubyError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for RubyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

fn data_from_gem_dir(f: &DirEntry) -> (String, String) {
    lazy_static! {
        static ref GEM_DIR: Regex = Regex::new(r"(?m-i)^.*gems/(?P<name>[^/]*)-(?P<version>\d+\.\d+\.\d+)$").unwrap();
    }

    match GEM_DIR.captures(f.path().display().to_string().as_str()) {
        Some(s) => {
            (String::from(s.name("name").unwrap().as_str()),
             String::from(s.name("version").unwrap().as_str()))
        },
        None => (String::from(""), String::from("")),
    }
}

pub fn is_relevant_file(f: &DirEntry) -> bool {
    match f.file_type().is_dir() {
        false => false,
        true => {
            let (name, version) = data_from_gem_dir(f);
            !name.is_empty() && !version.is_empty()
        },
    }
}

pub fn scan(files: &Vec<DirEntry>) -> Result<Vec<RubyPackage>, RubyError> {
    let mut ruby_packages: Vec<RubyPackage> = vec![];
    let gem_dirs: Vec<DirEntry> = files
        .clone()
        .into_iter()
        .filter(|e| is_relevant_file(e))
        .collect();

    for gem_dir in gem_dirs.into_iter() {
        let (name, version) = data_from_gem_dir(&gem_dir);
        ruby_packages.push(RubyPackageBuilder::default()
            .name(String::from(name))
            .version(String::from(version))
            .lib_path(String::from(gem_dir.path().display().to_string()))
            .build()
            .unwrap())
    }

    Ok(ruby_packages)
}

// TODO: unit test
