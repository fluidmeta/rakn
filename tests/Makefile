venv:
	mkdir -p ./.venv
	python3 -m venv ./.venv
	. .venv/bin/activate; pip install -r requirements.txt

circle-ci-integration-test:
	cargo run --color=always -- -e /proc/ -e /sys/ -e /dev/ -e /does-not-exist -o vulsio -p -d / | tee /dev/tty | grep "/root/project/tests/.venv/lib/python3.5/site-packages/Pipfile.lock"
	cargo run --color=always -- -e /proc/ -e /sys/ -e /dev/ -e /does-not-exist -o vulsio -p -d / | tee /dev/tty | grep "\"git\": {"
