use rakn;
use common::docker;

mod common;

#[test]
fn test_nodejs() {
    let (_tmp_dir_alloc, files_to_scan) = docker::extract_docker_image("node:13.8.0-stretch");

    let pkgs = rakn::scanner::lib::nodejs::scan(&files_to_scan).expect("Could not scan node libraries");
    assert_eq!(pkgs.len(), 430);
}
