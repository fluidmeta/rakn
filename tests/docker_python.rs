use rakn;
use common::docker;

mod common;

#[test]
fn test_python() {
    let (_tmp_dir_alloc, files_to_scan) = docker::extract_docker_image("tiangolo/uwsgi-nginx-flask:python3.6");

    let pkgs = rakn::scanner::lib::python::scan(&files_to_scan).expect("Could not scan python libraries");
    assert_eq!(pkgs.len(), 10);
}
