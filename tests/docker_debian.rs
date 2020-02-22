use rakn;
use common::docker;

mod common;

#[test]
fn test_ubuntu_bionic() {
    let (tmp_dir_alloc, _) = docker::extract_docker_image("ubuntu:bionic-20200112");
    let os_info = rakn::scanner::os::debian::scan(tmp_dir_alloc.path()).expect("Could not scan Ubuntu Bionic");
    assert_eq!(os_info.get_id(), "ubuntu");
    assert_eq!(os_info.get_release(), "18.04");
    assert_eq!(os_info.get_codename(), "bionic");

    let (bin, src) = rakn::scanner::pkg::dpkg::scan(tmp_dir_alloc.path()).expect("Could not scan dpkg");
    assert_eq!(bin.len(), 89);
    assert_eq!(src.len(), 62);
}

#[test]
fn test_debian_stretch() {
    let (tmp_dir_alloc, _) = docker::extract_docker_image("debian:stretch-20200130-slim");
    let os_info = rakn::scanner::os::debian::scan(tmp_dir_alloc.path()).expect("Could not scan Ubuntu Bionic");
    assert_eq!(os_info.get_id(), "debian");
    assert_eq!(os_info.get_release(), "9");
    assert_eq!(os_info.get_codename(), "stretch");

    let (bin, src) = rakn::scanner::pkg::dpkg::scan(tmp_dir_alloc.path()).expect("Could not scan dpkg");
    assert_eq!(bin.len(), 76);
    assert_eq!(src.len(), 51);
}
