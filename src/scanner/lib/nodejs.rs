use regex::Regex;
use std::{fmt, fs, io};
use walkdir::DirEntry;
use serde_json::Value;

#[derive(Builder, Clone)]
pub struct NodeJsPackage {
    name: String,
    version: String,
    lib_path: String,
}

impl NodeJsPackage {
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
pub struct NodeJsError {
    message: String,
}

impl From<io::Error> for NodeJsError {
    fn from(error: io::Error) -> Self {
        NodeJsError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for NodeJsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

fn is_package_json_file(f: &DirEntry) -> bool {
    lazy_static! {
        static ref RE_FILE: Regex = Regex::new(r"(?m-i)^.*node_modules/[^/]*/package.json$").unwrap();
    }
    RE_FILE.is_match(f.path().display().to_string().as_str())
}

pub fn is_relevant_file(f: &DirEntry) -> bool {
    f.file_type().is_file() && is_package_json_file(f)
}

pub fn scan(files: &Vec<DirEntry>) -> Result<Vec<NodeJsPackage>, NodeJsError> {
    let mut node_js_packages: Vec<NodeJsPackage> = vec![];
    let metadata_files: Vec<DirEntry> = files
        .clone()
        .into_iter()
        .filter(|e| is_relevant_file(e))
        .collect();

    for entry in metadata_files.into_iter() {
        // parse package version and name
        if entry.file_type().is_file() {
            let contents = fs::read_to_string(entry.path().to_str().unwrap())?;
            match parse_package_json(contents.as_str()) {
                Ok((name, version)) => {
                    node_js_packages.push(
                        NodeJsPackageBuilder::default()
                            .name(String::from(name))
                            .version(String::from(version))
                            .lib_path(String::from(entry.path().to_str().unwrap()))
                            .build()
                            .unwrap());
                }
                Err(_) => (),
            }
        }
    }

    Ok(node_js_packages)
}

fn parse_package_json(json_string: &str) -> Result<(String, String), io::Error> {
    let v: Value = serde_json::from_str(json_string)?;
    let name = String::from(v["name"].as_str().unwrap_or(""));
    let version = String::from(v["version"].as_str().unwrap_or(""));

    match name.is_empty() || version.is_empty() {
        true => Err(io::Error::new(io::ErrorKind::Other, "Empty json fields")),
        false => Ok((name, version)),
    }
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// Tests
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_package_json_string() {
        let metadata_content = "
{
  \"name\": \"psl\",
  \"version\": \"1.7.0\"
}
";
        let (package_name, package_version) = parse_package_json(metadata_content).unwrap();
        assert_eq!(package_name, "psl");
        assert_eq!(package_version, "1.7.0");
    }
}
