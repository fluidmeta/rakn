build:
	cargo build

test-debian-stretch:
	cp target/debug/rakn tests/debian/stretch/rakn
	docker build -t debian-stretch-test:latest tests/debian/stretch/
	docker run -it debian-stretch-test:latest

test-ubuntu-bionic:
	cp target/debug/rakn tests/ubuntu/bionic/rakn
	docker build -t ubuntu-bionic-test:latest tests/ubuntu/bionic/
	docker run -it ubuntu-bionic-test:latest

test-centos-8:
	cp target/debug/rakn tests/centos/8/rakn
	docker build -t centos-8-test:latest tests/centos/8/
	docker run -it centos-8-test:latest

test-python-3.6:
	cp target/debug/rakn tests/python/3.6/rakn
	docker build -t python-3.6-test:latest tests/python/3.6/
	docker run -it python-3.6-test:latest
