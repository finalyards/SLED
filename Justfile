#!/usr/bin/env just --justfile

default:
    @just --list

# For 'esp-idf-sys' builds, 'nightly' is a must. 'rust-toolchain.toml' adds it to us, and may enforce a specific version.
# This is only for commentary purposes.
nightly := ""

alias b:= build
alias ba:= build-a

alias r:= run-a
alias ra:= run-a

build:
	cargo {{nightly}} build --release -vv \
		--jobs 1

build-a:
	cargo {{nightly}} build --release -vv --example a --features log

run-a:
	cargo {{nightly}} run --release -vv --example a --features log
