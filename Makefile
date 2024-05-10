# dev tooling and versions
RUN_THAT_APP_VERSION = 0.6.0

build:  # performs a test build
	cargo check

docs:  # shows the RustDoc in a browser
	cargo doc --open

fix: tools/rta@${RUN_THAT_APP_VERSION}  # auto-corrects all issues
	tools/rta dprint fmt
	cargo +nightly fmt
	cargo +nightly fix --allow-dirty

help:   # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # compiles and installs the binary on this computer
	cargo install --path .

setup:  # prepares this codebase for development
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	yarn install

test: tools/rta@${RUN_THAT_APP_VERSION}  # runs all automated tests
	cargo build
	cargo clippy --all-targets --all-features -- -D warnings
	cargo test
	cargo fmt -- --check
	tools/rta dprint check
# ${CURDIR}/node_modules/.bin/text-run

unit:  # runs the unit tests
	cargo test

update: tools/rta@${RUN_THAT_APP_VERSION}  # updates the dependencies
	cargo install cargo-edit
	cargo upgrade
	tools/rta --update

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/rta@${RUN_THAT_APP_VERSION}:
	@rm -f tools/rta* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/rta tools/rta@${RUN_THAT_APP_VERSION}
	@ln -s rta@${RUN_THAT_APP_VERSION} tools/rta

.SILENT:
.DEFAULT_GOAL := help
