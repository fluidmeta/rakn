#!/bin/bash

rakn -e /proc/ -e /sys/ -e /dev/ -e /does-not-exist -o vulsio -p -d / | tee /dev/tty | grep "\"git\": {"
