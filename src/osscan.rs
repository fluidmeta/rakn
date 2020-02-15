use crate::common::package::{BinaryPackage, SourcePackage};
use crate::scanner::osinfo::OSInfoScanner;
use crate::common::scanner::OSFamily;
use crate::scanner::dpkg::DpkgScanner;
use crate::scanner::rpm::RpmScanner;

pub fn scan(os_info: &OSInfoScanner) -> (Vec<BinaryPackage>, Vec<SourcePackage>) {
    match os_info.get_os_family() {
        OSFamily::Debian => DpkgScanner::new().run(),
        OSFamily::CentOS => RpmScanner::new().run(),
        // TODO: handle unknown case
        OSFamily::Unknown => (vec![], vec![]),
    }
}
