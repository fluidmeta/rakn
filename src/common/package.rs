pub enum LibType {
    Python,
}

pub struct LibPackage {
    name: String,
    version: String,
    lib_type: LibType,
}

impl LibPackage {
    pub fn new(lib_type: LibType) -> LibPackage {
        LibPackage {
            name: "".to_string(),
            version: "".to_string(),
            lib_type,
        }
    }

    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }

    pub fn with_version(self, version: String) -> LibPackage {
        LibPackage {
            name: self.name,
            version,
            lib_type: self.lib_type,
        }
    }

    pub fn with_name(self, name: String) -> LibPackage {
        LibPackage {
            name,
            version: self.version,
            lib_type: self.lib_type,
        }
    }
}
