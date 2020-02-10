circle-ci:
	mkdir -p ./.venv
	virtualenv ./.venv
	. .venv/bin/activate; pip install -r requirements.txt
	cargo run --color=always -- -d ${HOME} | tee /dev/tty | grep "/home/circleci/project/test/.venv/lib/python3.6/site-packages/Pipfile.lock"
