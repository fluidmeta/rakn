use crate::common::package::{LibPackage, BinaryPackage, SourcePackage};
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum OSFamily {
    Unknown,
    Debian,
}

pub trait LibScannerExt {
    fn run(self) -> HashMap<String, Vec<LibPackage>>;
}

pub trait OSScannerExt {
    fn run(self) -> (Vec<BinaryPackage>, Vec<SourcePackage>);
}
