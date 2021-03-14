.SILENT:

build:  # performs a test build
	cargo check

docs:  # shows the RustDoc in a browser
	cargo doc --open

fix:  # auto-corrects all formatting issues
	${CURDIR}/node_modules/.bin/prettier --write .

help:   # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # compiles and installs the binary on this computer
	cargo install --path .

setup:  # prepares this codebase for development
	cargo check
	yarn install

test:  # runs all automated tests
	cargo clippy
	cargo test
	${CURDIR}/node_modules/.bin/prettier -l .
