use crate::scanner::os;
use std::path::Path;

#[derive(Builder, Clone)]
pub struct OSInfo {
    pub id: String,
    pub release: String,
    pub codename: String,
}

pub fn scan_os_info(root_dir: &Path) -> OSInfo {
    let debian_info = os::debian::scan(root_dir);
    let alpine_info = os::alpine::scan(root_dir);

    match debian_info {
        Ok(info) => {
            return OSInfoBuilder::default()
                .id(info.get_id())
                .release(info.get_release())
                .codename(info.get_codename())
                .build()
                .unwrap()
        }
        Err(_) => {}
    }

    match alpine_info {
        Ok(info) => {
            return OSInfoBuilder::default()
                .id(info.get_id())
                .release(info.get_release())
                .codename(String::from(""))
                .build()
                .unwrap()
        }
        Err(_) => {}
    }

    return OSInfoBuilder::default()
        .id(String::from(""))
        .release(String::from(""))
        .codename(String::from(""))
        .build()
        .unwrap()
}
