#!/bin/bash

INTERPRETER="target/debug/rust-compiler"
PUBLIC_ROOT="tests/public-tests-main/semantics"
PRIVATE_ROOT="tests/private-tests-main/semantics"
TIMEOUT=5
COUNT=0
TOTAL=0

cargo build

run_tests () {
    COUNT=0
    TOTAL=0
    for filename in "$1"/illegal/*.dcf; do
        touch tests/tmp.out
        echo "Testing: $filename"
        timeout $TIMEOUT $INTERPRETER --target inter $filename --output tests/tmp.out
        CODE=$?
        if [ $CODE -ne 1 ]; then
            echo "Fail (illegal): $(basename "$filename")"
            rm -f tests/tmp.out
        else 
            echo "Pass (illegal): $(basename "$filename")"
            COUNT=$((COUNT+1))
        fi
        TOTAL=$((TOTAL+1))
        # rm -f tests/tmp.out
    done

    for filename in "$1"/legal/*.dcf; do
        touch tests/tmp.out
        echo "Testing: $filename"
        timeout $TIMEOUT $INTERPRETER --target inter $filename --output tests/tmp.out
        CODE=$?
        if [ $CODE -ne 0 ]; then
            echo "Fail (legal): $(basename "$filename")"
            rm -f tests/tmp.out
        else 
            echo "Pass (legal): $(basename "$filename")"
            COUNT=$((COUNT+1))
        fi
        TOTAL=$((TOTAL+1))
        # rm -f tests/tmp.out
    done    

    echo "Passed $COUNT out of $TOTAL semantics tests"
}

if [[ $1 == "public" ]]; then
    echo "Running public tests"
    run_tests $PUBLIC_ROOT
else
    echo "Running public tests"
    run_tests $PUBLIC_ROOT
    echo "Running private tests"
    run_tests $PRIVATE_ROOT 
fi
