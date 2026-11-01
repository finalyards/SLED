#!/usr/bin/env just --justfile

default:
    @just --list

# For 'esp-idf-sys' builds, 'nightly' is a must. 'rust-toolchain.toml' adds it to us, and may enforce a specific version.
# This is only for commentary purposes.
nightly := ""

alias b:= build
alias ba:= build-a
alias bb:= build-b
alias bc:= build-c

#alias r:= run-a
alias ra:= run-a
alias rb:= run-b
alias rc:= run-c

build:
	cargo {{nightly}} build --release -vv \
		--jobs 1

build-a:
	cargo {{nightly}} build --release -vv --bin a --features log

run-a:
	cargo {{nightly}} run --release -vv --bin a --features log

build-b:
	cargo {{nightly}} build --release -vv --bin b --features log

run-b:
	cargo {{nightly}} run --release -vv --bin b --features log

build-c:
	cargo {{nightly}} build --release -vv --example c

run-c:
	cargo {{nightly}} run --release -vv --example c
