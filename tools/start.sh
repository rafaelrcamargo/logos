#!/bin/bash

# This script is used to start the application.

# Will receive two arguments:
# 1. The type of the application:
#   1.1. -c: client
#   1.2. -s: service
# 2. The name of the application.

if [ $1 = '-c' ]; then
    echo 'Starting client...'
    cd clients/$2
    yarn dev
elif [ $1 = '-s' ]; then
    echo 'Starting service...'
    cd services/$2
    cargo watch -q -c -x run
else
    echo 'Invalid type of application.'
fi
