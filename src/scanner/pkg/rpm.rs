use std::path::Path;
use std::{fmt, io};

#[derive(Debug)]
pub struct RpmError {
    message: String,
}

impl From<io::Error> for RpmError {
    fn from(error: io::Error) -> Self {
        RpmError {
            message: error.to_string(),
        }
    }
}

impl fmt::Display for RpmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

#[derive(Builder, Clone)]
pub struct RpmPackage {
    name: String,
    version: String,
}

impl RpmPackage {
    pub fn get_name(&self) -> String {
        String::from(self.name.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }
}

pub fn scan(root_dir: &Path) -> Result<Vec<RpmPackage>, RpmError> {
    let rpm_db = format!("{}/var/lib/rpm", root_dir.display().to_string());
    match Path::new(rpm_db.as_str()).exists() {
        false => Err(RpmError {
            message: format!("rpm db not found at {}", rpm_db).to_string(),
        }),
        true => read_rpm_db(Path::new(rpm_db.as_str())),
    }
}

fn read_rpm_db(rpm_db: &Path) -> Result<Vec<RpmPackage>, RpmError> {
    let mut packages: Vec<RpmPackage> = vec![];

    // TODO
//     // XXX: libtpm::set_db_path is global .. Cannot scan in parallel
// // TODO: use libdb-sys to get independent db instances in parallel
//     librpm::config::set_db_path(rpm_db)?;
//     for pkg in librpm::db::installed_packages() {
//         packages.push(RpmPackageBuilder::default()
//             .name(String::from(pkg.name.as_str()))
//             .version(String::from(pkg.version.as_str()))
//             .build()
//             .unwrap());
//     }

    Ok(packages)
}
