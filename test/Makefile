venv:
	mkdir -p ./.venv
	virtualenv ./.venv
	. .venv/bin/activate; pip install -r requirements.txt

circle-ci-integration-test:
	cargo run --color=always -- -e /proc/ -e /sys/ -e /dev/ -e /does-not-exist -d / | tee /dev/tty | grep "/home/circleci/project/test/.venv/lib/python3.6/site-packages/Pipfile.lock"
