extern crate sys_info;

pub struct OSInfoScanner {
    hostname: String,
    os: String,
    os_release_version: String,
    kernel: String,
}

impl OSInfoScanner {
    pub fn new() -> OSInfoScanner {
        OSInfoScanner {
            hostname: sys_info::hostname().unwrap(),
            os: sys_info::linux_os_release().unwrap().name.unwrap().to_lowercase(),
            os_release_version: sys_info::linux_os_release().unwrap().version_id.unwrap(),
            kernel: sys_info::os_release().unwrap(),
        }
    }

    pub fn get_hostname(&self) -> String {
        String::from(self.hostname.as_str())
    }

    pub fn get_os(&self) -> String {
        String::from(self.os.as_str())
    }

    pub fn get_os_release_version(&self) -> String {
        String::from(self.os_release_version.as_str())
    }

    pub fn get_kernel(&self) -> String {
        String::from(self.kernel.as_str())
    }
}
