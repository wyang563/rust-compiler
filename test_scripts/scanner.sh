#!/bin/bash

SCANNER="target/debug/rust-compiler"
ROOT="src/private-tests-main/scanner"
TIMEOUT=5
COUNT=0
TOTAL=0

cargo build

for filename in "$ROOT"/input/*.dcf; do
    touch tmp.out
    echo "Testing: $filename"
    outname="$(basename "${filename%.dcf}.out")"
    timeout $TIMEOUT $SCANNER --target scan $filename --output tmp.out
    if [[ $filename == *invalid* ]]; then
        if grep -q "Error" tmp.out; then
            echo "Pass (invalid): $(basename "$filename")"
            COUNT=$((COUNT+1))
        else
            echo "Fail (invalid): $(basename "$filename")"
        fi
    else
        if diff tmp.out $ROOT/output/$outname; then
            echo "Pass: $(basename $filename)"
            COUNT=$((COUNT+1))
        else
            echo "Fail: $(basename $filename)"
        fi
    fi
    TOTAL=$((TOTAL+1))
    rm -f tmp.out
    
done

echo "Passed $COUNT out of $TOTAL scanner tests"