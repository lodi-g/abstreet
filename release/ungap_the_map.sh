#!/bin/bash

cd game
echo See logs in output.txt
RUST_BACKTRACE=1 ./game --ungap 1> ../output.txt 2>&1
