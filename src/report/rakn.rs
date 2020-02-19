use crate::ScanResult;

pub fn print(report: &ScanResult) -> String {
    let mut result = "".to_string();

    result.push_str("Dpkg binary packages\n------\n\n");
    for p in report.dpkg_binary_packages.iter() {
        result.push_str(format!("{}:{}\n", p.get_package(), p.get_version()).as_str());
    }
    result.push_str("\n");

    result.push_str("Dpkg source packages\n------\n\n");
    for p in report.dpkg_source_packages.iter() {
        result.push_str(format!("{}:{}\n", p.get_package().as_str(), p.get_version().as_str()).as_str());
    }
    result.push_str("\n");

    result.push_str("Python packages\n------\n\n");
    for p in report.python_packages.iter() {
        result.push_str(format!("{} | {}:{}\n", p.get_lib_path(), p.get_name(), p.get_version()).as_str());
    }
    result.push_str("\n");

    println!("{}", result);
    result
}
