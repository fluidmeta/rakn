use regex::Regex;
use std::path::Path;
use std::{fmt, fs, io};

#[derive(Builder, Clone)]
pub struct RedhatInfo {
    id: String,
    release: String,
}

impl RedhatInfo {
    pub fn get_id(&self) -> String {
        String::from(self.id.as_str())
    }

    pub fn get_release(&self) -> String {
        String::from(self.release.as_str())
    }
}

#[derive(Debug)]
pub struct RedhatError {
    message: String,
}

impl From<io::Error> for RedhatError {
    fn from(error: io::Error) -> Self {
        RedhatError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for RedhatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

pub fn scan(root_dir: &Path) -> Result<RedhatInfo, RedhatError> {
    let os_release_file = format!("{}/etc/os-release", root_dir.display().to_string());
    match Path::new(os_release_file.as_str()).exists() {
        false => Err(RedhatError {
            message: format!("os-release file not found at {}", os_release_file).to_string(),
        }),
        true => {
            let os_release_content = fs::read_to_string(os_release_file)?;
            parse_os_release_file(os_release_content.as_str())
        }
    }
}

fn parse_os_release_file(os_release_content: &str) -> Result<RedhatInfo, RedhatError> {
    lazy_static! {
        static ref RE_NAME: Regex = Regex::new(r#"(?m-i)^NAME="(?P<name>.*)"$"#).unwrap();
        static ref RE_ID: Regex = Regex::new(r#"(?m-i)^ID="(?P<id>.*)"$"#).unwrap();
        static ref RE_RELEASE: Regex = Regex::new(r#"(?m-i)^VERSION_ID="(?P<release>.*)"$"#).unwrap();
    }

    let os_name = match RE_NAME.captures(os_release_content) {
        Some(s) => s.name("name").unwrap().as_str(),
        None => "",
    };

    match os_name.contains("CentOS") {
        false => Err(RedhatError {
            message: String::from("'CentOS' not in os-release file"),
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

            Ok(RedhatInfoBuilder::default()
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
    fn test_parse_redhat_os_release_valid_content_string() {
        let os_release_content = "
NAME=\"CentOS Linux\"
VERSION=\"8 (Core)\"
ID=\"centos\"
ID_LIKE=\"rhel fedora\"
VERSION_ID=\"8\"
PLATFORM_ID=\"platform:el8\"
PRETTY_NAME=\"CentOS Linux 8 (Core)\"
ANSI_COLOR=\"0;31\"
CPE_NAME=\"cpe:/o:centos:centos:8\"
HOME_URL=\"https://www.centos.org/\"
BUG_REPORT_URL=\"https://bugs.centos.org/\"

CENTOS_MANTISBT_PROJECT=\"CentOS-8\"
CENTOS_MANTISBT_PROJECT_VERSION=\"8\"
REDHAT_SUPPORT_PRODUCT=\"centos\"
REDHAT_SUPPORT_PRODUCT_VERSION=\"8\"
";
        let redhat_info = parse_os_release_file(os_release_content).unwrap_or(RedhatInfo {
            id: String::from(""),
            release: String::from(""),
        });
        assert_eq!(redhat_info.get_id(), "centos");
        assert_eq!(redhat_info.get_release(), "8");
    }

    #[test]
    fn test_parse_centos_os_release_invalid_content_string() {
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
        let centos_info = parse_os_release_file(os_release_content);
        assert!(centos_info.is_err());
    }
}
