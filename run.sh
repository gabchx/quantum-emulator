#!/bin/bash
set -e

if [[ "$1" = "run" || -z $1 ]];then
    cd $(dirname $0)
    # Run rust bin in background
    ./target/release/quantum_emulator &
    # Run python main.py in background
    POETRY_VENV_PATH=$(~/.local/bin/poetry env info -p)
    ~/.local/bin/poetry  run app &
    sleep 5
    echo ""
    echo "Both rust and python are running at http://127.0.0.1:8000/home"
    echo "Ctrl+C to stop"
    wait

else

    echo "Wrong argument : $1"

fi