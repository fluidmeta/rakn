use crate::common::package::{BinaryPackage, SourcePackage};
use std::process::Command;

pub struct RpmScanner {}

impl RpmScanner {
    pub fn new() -> RpmScanner {
        RpmScanner {}
    }

    pub fn run(self) -> (Vec<BinaryPackage>, Vec<SourcePackage>) {
        let output = Command::new("rpm")
            .args(&["-qa", "--queryformat", "%{NAME} %{EPOCHNUM} %{VERSION} %{RELEASE} %{ARCH}"])
            .output()
            .expect("failed to execute process");
        let output_string = String::from_utf8_lossy(&output.stdout);
        println!{"{}", output_string};
        (vec![], vec![])
    }
}
