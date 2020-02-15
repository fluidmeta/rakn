extern crate sys_info;

use crate::common::scanner::OSFamily;

pub struct OSInfoScanner {
    hostname: String,
    os: String,
    os_release_version: String,
    kernel: String,
    os_family: OSFamily,
    arch: String,
}

impl OSInfoScanner {
    pub fn new() -> OSInfoScanner {
        OSInfoScanner {
            hostname: sys_info::hostname().unwrap(),
            os: sys_info::linux_os_release().unwrap().name.unwrap().to_lowercase(),
            os_release_version: sys_info::linux_os_release().unwrap().version_id.unwrap(),
            kernel: sys_info::os_release().unwrap(),
            os_family: {
                match sys_info::linux_os_release()
                    .unwrap().name
                    .unwrap()
                    .to_lowercase()
                    .split(" ")
                    .collect::<Vec<&str>>()[0]
                {
                    "ubuntu" | "debian" => OSFamily::Debian,
                    "centos" => OSFamily::CentOS,
                    _ => OSFamily::Unknown,
                }
            },
            arch: get_arch(),
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

    pub fn get_os_family(&self) -> OSFamily {
        OSFamily::from(self.os_family)
    }

    pub fn get_arch(&self) -> String {
        String::from(self.arch.as_str())
    }
}


fn get_arch() -> String {
    #[cfg(target_arch = "x86_64")]
        {
            return "x86_64".to_string()
        }

    #[cfg(target_arch = "x86")]
        {
            return "x86".to_string()
        }
}
