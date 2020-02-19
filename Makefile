build:
	cargo build

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

test-docker-python-3.6:
	docker pull tiangolo/uwsgi-nginx-flask:python3.6
	target/debug/rakn -i tiangolo/uwsgi-nginx-flask:python3.6 | tee /dev/tty | grep "Flask:1.1.1"
