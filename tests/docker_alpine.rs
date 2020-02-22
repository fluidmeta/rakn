use rakn;
use common::docker;

mod common;

#[test]
fn test_alpine() {
    let (tmp_dir_alloc, _) = docker::extract_docker_image("alpine:3.11.3");
    let os_info = rakn::scanner::os::alpine::scan(tmp_dir_alloc.path()).expect("Could not scan Alpine");
    assert_eq!(os_info.get_id(), "alpine");
    assert_eq!(os_info.get_release(), "3.11.3");

    let pkgs = rakn::scanner::pkg::apk::scan(tmp_dir_alloc.path()).expect("Could not scan apk");
    assert_eq!(pkgs.len(), 14);
}
