#!/usr/bin/env just --justfile

default:
    @just --list

# For 'esp-idf-sys' builds, 'nightly' is a must. 'rust-toolchain.toml' adds it to us, and may enforce a specific version.
# This is only for commentary purposes.
nightly := ""

#
# NOTE: 'esp-idf-svc' logging (that we use) DOES NOT SUPPORT MODULE PATHS ("info,sled=debug"). This is only evident
#   by looking at its source code (it falls back to "info" - WITHOUT PROVIDING AN ERROR OR WARNING). It should warn.
#
#   tbd. Find a way to steer the larger stack and our code (sled,c,t,...) separately.
#
#export RUST_LOG := "debug"
export RUST_LOG := "info"
#export RUST_LOG := "info,sled,c"  # what we'd like: info for the rest, "debug" for us

alias b:= build

#alias ba:= build-a
#alias bb:= build-b
alias bc:= build-c
alias bt:= build-t

#alias t:=test

#alias ra:= run-a
#alias rb:= run-b
alias rc:= run-c
alias rt:= run-t

build:
	cargo {{nightly}} build --release -vv \
		--jobs 1

#build-a:
#	cargo {{nightly}} build --release -vv --bin a --features log
#
#run-a:
#	cargo {{nightly}} run --release -vv --bin a --features log
#
#build-b:
#	cargo {{nightly}} build --release -vv --bin b --features log
#
#run-b:
#	cargo {{nightly}} run --release -vv --bin b --features log

build-c:
	cargo {{nightly}} build --release -vv --example c

run-c:
	cargo {{nightly}} run --release -vv --example c

build-t:
	RUST_LOG=info \
	  cargo {{nightly}} build --release -vv --example t

run-t:
	RUST_LOG=info \
	  cargo {{nightly}} run --release -vv --example t
