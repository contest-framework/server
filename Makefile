RUN_THAT_APP_VERSION = 0.37.0  # run-that-app version to use

RTA      = tools/rta@$(RUN_THAT_APP_VERSION)
ACTIONLINT = $(RTA) actionlint
CUCUMBER_SORT = $(RTA) cucumber-sort
DPRINT = $(RTA) dprint
GHOKIN   = $(RTA) ghokin
NODE = $(RTA) node
NPM = $(RTA) npm
TEXTRUNNER = $(RTA) --optional node node_modules/.bin/text-runner

build:  # performs a test build
	cargo check

cuke:  # runs the end-to-end tests
	cargo test --test cucumber

cukethis:  # runs only end-to-end tests with a @this tag
	cargo test --test cucumber -- -t @this

doc: node_modules  # verifies the documentation
	$(TEXTRUNNER)

docs:  # shows the RustDoc in a browser
	cargo doc --open

fix: ${RTA}  # auto-corrects all issues
	$(DPRINT) fmt
	cargo +nightly fmt
	cargo +nightly fix --allow-dirty
	$(GHOKIN) fmt replace features/
	$(CUCUMBER_SORT) format

help:   # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.PHONY' | grep -v '.SILENT' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # compiles and installs the binary on this computer
	cargo install --locked --path .

lint: ${RTA}  # runs all linters
	cargo clippy --all-targets --all-features
	$(ACTIONLINT)
	$(CUCUMBER_SORT) check

setup:  # prepares this codebase for development
	rustup component add clippy
	rustup toolchain add nightly
	rustup component add rustfmt --toolchain nightly
	$(NPM) install

test: ${RTA}  # runs all automated tests
	cargo build
	make --no-print-dir lint
	cargo test
	make --no-print-dir cuke
	cargo +nightly fmt -- --check
	$(DPRINT) fmt
	make --no-print-dir doc

unit:  # runs the unit tests
	cargo test

update: ${RTA}  # updates the dependencies
	cargo install cargo-edit
	cargo upgrade
	$(RTA) --update
	$(NPM) exec -- npm-check-updates -u && $(NPM) install

# --- HELPER TARGETS --------------------------------------------------------------------------------------------------------------------------------

${RTA}:
	@rm -f tools/rta*
	@(cd tools && curl https://raw.githubusercontent.com/kevgo/run-that-app/main/download.sh | sh -s -- --version ${RUN_THAT_APP_VERSION} --name rta@${RUN_THAT_APP_VERSION})

node_modules: package-lock.json ${RTA}
	$(NPM) ci
	touch node_modules  # update timestamp of the node_modules folder so that Make doesn't re-install it on every command

.SILENT:
.DEFAULT_GOAL := help
