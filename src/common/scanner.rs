use crate::common::package::LibPackage;
use std::collections::HashMap;

pub trait LibScannerExt {
    fn run(self) -> HashMap<String,Vec<LibPackage>>;
}
