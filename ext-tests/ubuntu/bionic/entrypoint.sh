#!/bin/bash

rakn | tee /dev/tty | grep "git:1:2.17.1-1ubuntu0.5"
rakn | grep "Release: 18.04"
