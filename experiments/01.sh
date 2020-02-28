#!/usr/bin/env bash

for i in {1..5}; do cargo run --release -- -a 10000 -d sars; done

for i in {1..5}; do cargo run --release -- -a 100000 -d sars; done

for i in {1..5}; do cargo run --release -- -a 10000 -d covid_19; done

for i in {1..5}; do cargo run --release -- -a 100000 -d covid_19; done
