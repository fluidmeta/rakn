extern crate sys_info;

pub struct OSInfo {
    pub hostname: String,
    pub os: String,
    pub os_release_version: String,
    pub kernel: String,
}

impl OSInfo {
    pub fn new() -> OSInfo {
        OSInfo {
            hostname: sys_info::hostname().unwrap(),
            os: sys_info::linux_os_release().unwrap().name.unwrap().to_lowercase(),
            os_release_version: sys_info::linux_os_release().unwrap().version_id.unwrap(),
            kernel: sys_info::os_release().unwrap(),
        }
    }
}
