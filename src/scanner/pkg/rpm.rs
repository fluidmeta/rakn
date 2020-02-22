use libdb::DbType;
use std::path::Path;
use std::{fmt, io};

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

impl From<libdb::Error> for RpmError {
    fn from(error: libdb::Error) -> Self {
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
    package: String,
    version: String,
}

impl RpmPackage {
    pub fn get_package(&self) -> String {
        String::from(self.package.as_str())
    }

    pub fn get_version(&self) -> String {
        String::from(self.version.as_str())
    }
}

pub fn scan(root_dir: &Path) -> Result<Vec<RpmPackage>, RpmError> {
    let rpm_db_file = format!("{}/var/lib/rpm/Packages", root_dir.display().to_string());
    match Path::new(rpm_db_file.as_str()).exists() {
        false => Err(RpmError {
            message: format!("rpm db file not found at {}", rpm_db_file).to_string(),
        }),
        true => read_rpm_db(Path::new(rpm_db_file.as_str())),
    }
}

fn read_rpm_db(rpm_db_file: &Path) -> Result<Vec<RpmPackage>, RpmError> {
    let mut db_cursor = libdb::DatabaseBuilder::new()
        .db_type(DbType::Hash)
        .flags(libdb::DB_RDONLY)
        .file(rpm_db_file.display().to_string().as_str())
        .open()?
        .cursor()?;

    let r = db_cursor.next();
    //println!("{}", str::from_utf8(key.unwrap().as_slice()).unwrap());

    Ok(vec![RpmPackage {
        version: "".to_string(),
        package: "".to_string(),
    }])
}

// ref. https://github.com/rpm-software-management/rpm/blob/rpm-4.11.3-release/lib/header_internal.h#L13-L19
struct EntryInfo {
    tag: i32,
    etype: u32,
    offset: i32,
    count: u32,
}

// ref. https://github.com/rpm-software-management/rpm/blob/rpm-4.11.3-release/lib/header_internal.h#L27-L33
struct IndexEntry {
    info: EntryInfo,
    length: i32,
    rdlen: i32,
    data: Vec<u8>,
}
