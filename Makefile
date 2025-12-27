.PHONY: build test clippy clippy-fix format format-check

build:
	cargo build --target-dir target/

test:
	$(info ****************** running tests ******************)
	cargo test

clippy:
	$(info ****************** running clippy in check mode ******************)
	cargo clippy

clippy-fix:
	$(info ****************** running clippy in fix mode ******************)
	cargo clippy --fix --bin "postgres-dv"

format:
	$(info ****************** running rustfmt in fix mode ******************)
	cargo fmt

format-check:
	$(info ****************** running rustfmt in check mode ******************)
	cargo fmt --check

version-bump:
	$(info ****************** bumping version in package.json and Cargo.toml ******************)
	python3 scripts/version_bump.py