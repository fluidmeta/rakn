use rakn;
use common::docker;

mod common;

#[test]
fn test_ruby() {
    let (_tmp_dir_alloc, files_to_scan) = docker::extract_docker_image("ruby:2.7.0");

    let pkgs = rakn::scanner::lib::ruby::scan(&files_to_scan).expect("Could not scan ruby libraries");
    assert_eq!(pkgs.len(), 55);
}
