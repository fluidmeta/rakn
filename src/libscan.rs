use crate::common::package::LibPackage;
use walkdir::DirEntry;
use crate::scanner::python::PythonScanner;
use std::collections::HashMap;

pub fn scan(files_to_scan: Vec<DirEntry>) -> HashMap<String,Vec<LibPackage>> {
    PythonScanner::new(files_to_scan).run()
}
