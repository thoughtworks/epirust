#!/usr/bin/env bash

for i in {1..3}; do cargo run --release -- -a 1000000 -d covid_19; done
