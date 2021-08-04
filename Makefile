.PHONY: build-anchor
build-anchor:
	npx anchor build

.PHONY: check-advisories
check-advisories:
	cargo deny check advisories

.PHONY: check-clippy
check-clippy:
	cargo clippy --workspace --all-targets -- -D warnings

.PHONY: check-clippy-nightly
check-clippy-nightly:
	cargo +nightly clippy --workspace --all-targets -- -D warnings

.PHONY: check-fmt
check-fmt:
	cargo fmt --all -- --check

.PHONY: clean
clean:
	rm -rf \
		.anchor \
		node_modules \
		target \
		test-ledger

.PHONY: fmt
fmt:
	cargo fmt --all

.PHONY: test-bpf
test-bpf:
	cargo test-bpf -- --nocapture
