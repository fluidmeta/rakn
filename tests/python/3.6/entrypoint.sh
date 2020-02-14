#!/bin/bash

rakn -e /proc/ -e /sys/ -e /dev/ -e /does-not-exist -o vulsio -p -d / | tee /dev/tty | grep "/root/.venv/lib/python3.6/site-packages/Pipfile.lock"
