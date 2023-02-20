#!/bin/bash

# This script is used to start the application.

# Will receive two arguments:
## The first is the type of the application (client or service)
## The second is the name of the application.

if [ $1 = '-c' ]; then
    echo 'Starting client...'
    cd clients/$2
    yarn start
elif [ $1 = '-s' ]; then
    echo 'Starting service...'
    cd services/$2
    cargo watch -q -c -x run
else
    echo 'Invalid type of application.'
fi
