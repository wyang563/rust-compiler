#!/usr/bin/env bash

for (( i=1; i<=$#; i++ ))
do
    if [[ ${!i} == "--output" ]]; then 
        ((i++))
        OUTPUT=$i;
    elif [[ ${!i} == "-o" ]]; then
        ((i++))
        OUTPUT=$i;
    fi
done

if [ -z $OUTPUT ]; then
    target/debug/decaf-skeleton-rust "$@"
else
    target/debug/decaf-skeleton-rust "$@" > ${!OUTPUT}
fi