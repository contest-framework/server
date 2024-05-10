.DEFAULT_GOAL := help
.SILENT:

build:  # performs a test build
	cargo check

docs:  # shows the RustDoc in a browser
	cargo doc --open

fix:  # auto-corrects all formatting issues
	dprint fmt

help:   # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # compiles and installs the binary on this computer
	cargo install --path .

setup:  # prepares this codebase for development
	yarn install
	echo
	echo "Please make sure you have dprint installed."
	echo "https://dprint.dev/install"

test:  # runs all automated tests
	cargo build
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	cargo fmt -- --check
	dprint check
# ${CURDIR}/node_modules/.bin/text-run

unit:  # runs the unit tests
	cargo test

update: tools/rta@${RUN_THAT_APP_VERSION}  # updates the dependencies
	cargo install cargo-edit
	cargo upgrade
	tools/rta --update
