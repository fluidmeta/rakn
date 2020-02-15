use crate::common::package::{BinaryPackage, BinaryPackageBuilder, SourcePackage};
use std::process::Command;

pub struct RpmScanner {}

impl RpmScanner {
    pub fn new() -> RpmScanner {
        RpmScanner {}
    }

    pub fn run(self) -> (Vec<BinaryPackage>, Vec<SourcePackage>) {
        let mut binary_packages:Vec<BinaryPackage> = vec![];
        let output = Command::new("rpm")
            .args(&["-qa", "--queryformat", "%{NAME} , %{EPOCHNUM} , %{VERSION} , %{RELEASE} , %{ARCH}\n"])
            .output()
            .expect("failed to execute process");
        let output_string = String::from_utf8_lossy(&output.stdout);
        for line in output_string.to_string().lines().into_iter() {
            let line_split = line.split(',').collect::<Vec<_>>();
            if line_split.len() == 5 {
                let v = line_split.iter().map(|s| String::from(*s))
                    .collect::<Vec<_>>();
                let (name, epoch_num, version, release, arch) = (
                    v[0].as_str(), v[1].as_str(), v[2].as_str(), v[3].as_str(), v[4].as_str());
                binary_packages.push(
                    BinaryPackageBuilder::default()
                        .name(String::from(name.trim()))
                        .version(String::from(version.trim()))
                        .build()
                        .unwrap()
                );
            }
        }
        (binary_packages, vec![])
    }
}
