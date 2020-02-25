use rakn;
use common::docker;

mod common;

#[test]
fn test_centos_8() {
    let (tmp_dir_alloc, _) = docker::extract_docker_image("centos:8.1.1911");

    let pkgs = rakn::scanner::pkg::rpm::scan(tmp_dir_alloc.path()).expect("Could not scan rpm");
    // TODO
    assert_eq!(pkgs.len(), 0);
}

#[test]
fn test_centos_7() {
    let (tmp_dir_alloc, _) = docker::extract_docker_image("centos:7.7.1908");

    let pkgs = rakn::scanner::pkg::rpm::scan(tmp_dir_alloc.path()).expect("Could not scan rpm");
    // TODO
    assert_eq!(pkgs.len(), 0);
}
