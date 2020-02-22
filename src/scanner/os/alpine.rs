use regex::Regex;
use std::path::Path;
use std::{fmt, fs, io};

#[derive(Builder, Clone)]
pub struct AlpineInfo {
    id: String,
    release: String,
}

impl AlpineInfo {
    pub fn get_id(&self) -> String {
        String::from(self.id.as_str())
    }

    pub fn get_release(&self) -> String {
        String::from(self.release.as_str())
    }
}

#[derive(Debug)]
pub struct AlpineError {
    message: String,
}

impl From<io::Error> for AlpineError {
    fn from(error: io::Error) -> Self {
        AlpineError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for AlpineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

pub fn scan(root_dir: &Path) -> Result<AlpineInfo, AlpineError> {
    let os_release_file = format!("{}/etc/os-release", root_dir.display().to_string());
    match Path::new(os_release_file.as_str()).exists() {
        false => Err(AlpineError {
            message: format!("os-release file not found at {}", os_release_file).to_string(),
        }),
        true => {
            let os_release_content = fs::read_to_string(os_release_file)?;
            parse_os_release_file(os_release_content.as_str())
        }
    }
}

fn parse_os_release_file(os_release_content: &str) -> Result<AlpineInfo, AlpineError> {
    lazy_static! {
        static ref RE_NAME: Regex = Regex::new(r#"(?m-i)^NAME="(?P<name>.*)"$"#).unwrap();
        static ref RE_ID: Regex = Regex::new(r#"(?m-i)^ID=(?P<id>.*)$"#).unwrap();
        static ref RE_RELEASE: Regex = Regex::new(r#"(?m-i)^VERSION_ID=(?P<release>.*)$"#).unwrap();
    }

    let os_name = match RE_NAME.captures(os_release_content) {
        Some(s) => s.name("name").unwrap().as_str(),
        None => "",
    };

    match os_name.contains("Alpine") {
        false => Err(AlpineError {
            message: String::from("'Alpine' not in os-release file"),
        }),
        true => {
            let os_id = match RE_ID.captures(os_release_content) {
                Some(s) => s.name("id").unwrap().as_str(),
                None => "",
            };

            let os_release = match RE_RELEASE.captures(os_release_content) {
                Some(s) => s.name("release").unwrap().as_str(),
                None => "",
            };

            Ok(AlpineInfoBuilder::default()
                .id(String::from(os_id))
                .release(String::from(os_release))
                .build()
                .unwrap())
        }
    }
}

// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
// Tests
// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_alpine_os_release_valid_content_string() {
        let os_release_content = "
NAME=\"Alpine Linux\"
ID=alpine
VERSION_ID=3.11.3
PRETTY_NAME=\"Alpine Linux v3.11\"
HOME_URL=\"https://alpinelinux.org/\"
BUG_REPORT_URL=\"https://bugs.alpinelinux.org/\"
";
        let debian_info = parse_os_release_file(os_release_content).unwrap_or(AlpineInfo {
            id: String::from(""),
            release: String::from(""),
        });
        assert_eq!(debian_info.get_id(), "alpine");
        assert_eq!(debian_info.get_release(), "3.11.3");
    }

    #[test]
    fn test_parse_alpine_os_release_invalid_content_string() {
        let os_release_content = "
PRETTY_NAME=\"Debian GNU/Linux 9 (stretch)\"
NAME=\"Debian GNU/Linux\"
VERSION_ID=\"9\"
VERSION=\"9 (stretch)\"
VERSION_CODENAME=stretch
ID=debian
HOME_URL=\"https://www.debian.org/\"
SUPPORT_URL=\"https://www.debian.org/support\"
BUG_REPORT_URL=\"https://bugs.debian.org/\"
";
        let alpine_info = parse_os_release_file(os_release_content);
        assert!(alpine_info.is_err());
    }
}
