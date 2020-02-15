#[derive(Copy, Clone)]
pub enum LibType {
    Unknown,
    Python,
}

impl Default for LibType {
    fn default() -> Self {
        LibType::Unknown
    }
}

#[derive(Default, Builder)]
pub struct LibPackage {
    name: String,
    version: String,
    lib_type: LibType,
}

impl LibPackage {
    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }
}

#[derive(Clone, Default, Builder)]
pub struct BinaryPackage {
    name: String,
    version: String,
}

impl BinaryPackage {
    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }
}

pub struct SourcePackage {
    name: String,
    version: String,
    binary_packages: Vec<BinaryPackage>,
}

impl SourcePackage {
    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }

    pub fn get_binary_packages(&self) -> Vec<BinaryPackage> {
        self.binary_packages.clone()
    }
}

pub struct SourcePackageBuilder {
    name: String,
    version: String,
    binary_packages: Vec<BinaryPackage>,
}

impl SourcePackageBuilder {
    pub fn new() -> SourcePackageBuilder {
        SourcePackageBuilder {
            name: "".to_string(),
            version: "".to_string(),
            binary_packages: Vec::new(),
        }
    }

    pub fn with_name(&self, name: &str) -> SourcePackageBuilder {
        SourcePackageBuilder {
            name: String::from(name),
            version: String::from(self.version.as_str()),
            binary_packages: self.binary_packages.clone(),
        }
    }

    pub fn with_version(&self, version: &str) -> SourcePackageBuilder {
        SourcePackageBuilder {
            name: String::from(self.name.as_str()),
            version: String::from(version),
            binary_packages: self.binary_packages.clone(),
        }
    }

    pub fn add_binary_package(&self, binary_package: &BinaryPackage) -> SourcePackageBuilder {
        SourcePackageBuilder {
            name: String::from(self.name.as_str()),
            version: String::from(self.version.as_str()),
            binary_packages: {
                let mut pkgs = self.binary_packages.clone();
                pkgs.push(binary_package.clone());
                pkgs
            },
        }
    }

    pub fn finish(&self) -> SourcePackage {
        SourcePackage {
            name: String::from(self.name.as_str()),
            version: String::from(self.version.as_str()),
            binary_packages: self.binary_packages.clone(),
        }
    }
}
