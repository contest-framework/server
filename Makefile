# dev tooling and versions
RUN_THAT_APP_VERSION = 0.11.0

build:  # performs a test build
	cargo check

cuke:  # runs the end-to-end tests
	cargo test --test cucumber

cukethis:  # runs only end-to-end tests with a @this tag
	cargo test --test cucumber -- -t @this

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

lint: tools/rta@${RUN_THAT_APP_VERSION}  # runs all linters
	cargo clippy --all-targets --all-features
	tools/rta actionlint

setup:  # prepares this codebase for development
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	npm install

test: tools/rta@${RUN_THAT_APP_VERSION}  # runs all automated tests
	cargo build
	make --no-print-dir lint
	cargo test
	make --no-print-dir cuke
	cargo +nightly fmt -- --check
	tools/rta dprint check
	npm exec -- text-runner

unit:  # runs the unit tests
	cargo test

update: tools/rta@${RUN_THAT_APP_VERSION}  # updates the dependencies
	cargo install cargo-edit
	cargo upgrade
	tools/rta --update
	(cd tools && npx npm-check-updates -u && npm install)

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/rta@${RUN_THAT_APP_VERSION}:
	@rm -f tools/rta* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/rta tools/rta@${RUN_THAT_APP_VERSION}
	@ln -s rta@${RUN_THAT_APP_VERSION} tools/rta

.SILENT:
.DEFAULT_GOAL := help
