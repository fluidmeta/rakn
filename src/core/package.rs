pub enum PackageType {
    Python
}

pub struct Package {
    name: String,
    version: String,
    package_type: PackageType,
}

impl Package {
    pub fn new(package_type: PackageType) -> Package {
        Package {
            name: "".to_string(),
            version: "".to_string(),
            package_type,
        }
    }

    pub fn get(self) -> (String,String) {
        (String::from(self.name), String::from(self.version))
    }

    pub fn with_version(self, version: String) -> Package {
        Package {
            name: self.name,
            version,
            package_type: self.package_type,
        }
    }

    pub fn with_name(self, name: String) -> Package {
        Package {
            name,
            version: self.version,
            package_type: self.package_type,
        }
    }
}
