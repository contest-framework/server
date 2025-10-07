# dev tooling and versions
RUN_THAT_APP_VERSION = 0.20.0

build:  # performs a test build
	cargo check

cuke:  # runs the end-to-end tests
	cargo test --test cucumber

cukethis:  # runs only end-to-end tests with a @this tag
	cargo test --test cucumber -- -t @this

doc: node_modules  # verifies the documentation
	tools/rta --optional node node_modules/.bin/text-runner

docs:  # shows the RustDoc in a browser
	cargo doc --open

fix: tools/rta@${RUN_THAT_APP_VERSION}  # auto-corrects all issues
	tools/rta dprint fmt
	cargo +nightly fmt
	cargo +nightly fix --allow-dirty
	tools/rta ghokin fmt replace features/

help:   # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # compiles and installs the binary on this computer
	cargo install --locked --path .

lint: tools/rta@${RUN_THAT_APP_VERSION}  # runs all linters
	cargo clippy --all-targets --all-features
	tools/rta actionlint

setup:  # prepares this codebase for development
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	npm install

test: tools/rta@${RUN_THAT_APP_VERSION}  # runs all automated tests
	cargo build
	make --no-print-dir lint
	cargo test
	make --no-print-dir cuke
	cargo +nightly fmt -- --check
	tools/rta dprint fmt
	make --no-print-dir doc

unit:  # runs the unit tests
	cargo test

update: tools/rta@${RUN_THAT_APP_VERSION}  # updates the dependencies
	cargo install cargo-edit
	cargo upgrade
	tools/rta --update
	npm exec -- npm-check-updates -u && npm install

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

tools/rta@${RUN_THAT_APP_VERSION}:
	@rm -f tools/rta* tools/rta
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh)
	@mv tools/rta tools/rta@${RUN_THAT_APP_VERSION}
	@ln -s rta@${RUN_THAT_APP_VERSION} tools/rta

node_modules: package-lock.json tools/rta@${RUN_THAT_APP_VERSION}
	tools/rta npm ci
	touch node_modules  # update timestamp of the node_modules folder so that Make doesn't re-install it on every command

.SILENT:
.DEFAULT_GOAL := help
