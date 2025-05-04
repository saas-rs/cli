.PHONY: build
build:
	cargo fmt
	cargo build


.PHONY: check
check:
	cargo clippy --all-targets -- --no-deps
	cargo machete
	cargo test --all-targets -- --show-output


.PHONY: fmt
fmt:
	cargo fmt --all


.PHONY: clean
clean:
	-rm -r protocol/src/generated/*
	cargo clean


.PHONY: outdated
outdated:
	cargo install --locked cargo-outdated
	cargo outdated > /dev/null
	cargo outdated -R
