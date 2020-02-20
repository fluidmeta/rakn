build:
	cargo build
	cargo test

test-host-debian-stretch:
	cp target/debug/rakn ext-tests/debian/stretch/rakn
	docker build -t debian-stretch-test:latest ext-tests/debian/stretch/
	docker run -it debian-stretch-test:latest

test-host-ubuntu-bionic:
	cp target/debug/rakn ext-tests/ubuntu/bionic/rakn
	docker build -t ubuntu-bionic-test:latest ext-tests/ubuntu/bionic/
	docker run -it ubuntu-bionic-test:latest

test-host-centos-8:
	cp target/debug/rakn ext-tests/centos/8/rakn
	docker build -t centos-8-test:latest ext-tests/centos/8/
	docker run -it centos-8-test:latest

test-host-python-3.6:
	cp target/debug/rakn ext-tests/python/3.6/rakn
	docker build -t python-3.6-test:latest ext-tests/python/3.6/
	docker run -it python-3.6-test:latest

test-docker-ubuntu-bionic:
	docker pull ubuntu:bionic-20200112
	target/debug/rakn -i ubuntu:bionic-20200112 | tee /dev/tty | grep "apt:1.6.12"
	target/debug/rakn -i ubuntu:bionic-20200112 | grep "Release: 18.04"

test-docker-debian-stretch:
	docker pull debian:stretch-20200130-slim
	target/debug/rakn -i debian:stretch-20200130-slim | tee /dev/tty | grep "shadow:1:4.4-4.1"
	target/debug/rakn -i debian:stretch-20200130-slim | grep "Release: 9"

test-docker-alpine-3.11:
	docker pull alpine:3.11.3
	target/debug/rakn -i alpine:3.11.3 | tee /dev/tty | grep "libc-utils:0.7.2-r0"
	target/debug/rakn -i alpine:3.11.3 | grep "Release: 3.11.3"

test-docker-python-3.6:
	docker pull tiangolo/uwsgi-nginx-flask:python3.6
	target/debug/rakn -i tiangolo/uwsgi-nginx-flask:python3.6 | tee /dev/tty | grep "Flask:1.1.1"
