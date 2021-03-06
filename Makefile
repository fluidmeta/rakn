HOST_TESTS := host-tests
RAKN_BIN := target/x86_64-unknown-linux-musl/debug/rakn

test:
	cargo test --target=x86_64-unknown-linux-musl

test-host-debian-stretch:
	cp $(RAKN_BIN) $(HOST_TESTS)/debian/stretch/rakn
	docker build -t debian-stretch-test:latest $(HOST_TESTS)/debian/stretch/
	docker run debian-stretch-test:latest

test-host-ubuntu-bionic:
	cp $(RAKN_BIN) $(HOST_TESTS)/ubuntu/bionic/rakn
	docker build -t ubuntu-bionic-test:latest $(HOST_TESTS)/ubuntu/bionic/
	docker run ubuntu-bionic-test:latest

test-host-centos-7:
	cp $(RAKN_BIN) $(HOST_TESTS)/centos/7/rakn
	docker build -t centos-7-test:latest $(HOST_TESTS)/centos/7/
	docker run centos-7-test:latest

test-host-centos-8:
	cp $(RAKN_BIN) $(HOST_TESTS)/centos/8/rakn
	docker build -t centos-8-test:latest $(HOST_TESTS)/centos/8/
	docker run centos-8-test:latest

test-host-python-3.6:
	cp $(RAKN_BIN) $(HOST_TESTS)/python/3.6/rakn
	docker build -t python-3.6-test:latest $(HOST_TESTS)/python/3.6/
	docker run python-3.6-test:latest

test-host-nodejs-6:
	cp $(RAKN_BIN) $(HOST_TESTS)/nodejs/6/rakn
	docker build -t nodejs-6-test:latest $(HOST_TESTS)/nodejs/6/
	docker run nodejs-6-test:latest

test-host-ruby-2.7:
	cp $(RAKN_BIN) $(HOST_TESTS)/ruby/2.7/rakn
	docker build -t ruby-2.7-test:latest $(HOST_TESTS)/ruby/2.7/
	docker run ruby-2.7-test:latest
