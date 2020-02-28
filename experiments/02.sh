#!/usr/bin/env bash

for i in {1..2}; do cargo run --release -- -a 1000000 -d sars; done
