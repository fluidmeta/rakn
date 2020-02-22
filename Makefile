HOST_TESTS := host-tests

test:
	cargo test

test-host-debian-stretch:
	cp target/debug/rakn $(HOST_TESTS)/debian/stretch/rakn
	docker build -t debian-stretch-test:latest $(HOST_TESTS)/debian/stretch/
	docker run -it debian-stretch-test:latest

test-host-ubuntu-bionic:
	cp target/debug/rakn $(HOST_TESTS)/ubuntu/bionic/rakn
	docker build -t ubuntu-bionic-test:latest $(HOST_TESTS)/ubuntu/bionic/
	docker run -it ubuntu-bionic-test:latest

test-host-centos-8:
	cp target/debug/rakn $(HOST_TESTS)/centos/8/rakn
	docker build -t centos-8-test:latest $(HOST_TESTS)/centos/8/
	docker run -it centos-8-test:latest

test-host-python-3.6:
	cp target/debug/rakn $(HOST_TESTS)/python/3.6/rakn
	docker build -t python-3.6-test:latest $(HOST_TESTS)/python/3.6/
	docker run -it python-3.6-test:latest
