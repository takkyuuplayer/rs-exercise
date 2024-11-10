NUM_CORES := $(shell nproc)

CRATES := $(shell find . -name Cargo.toml | xargs -n 1 dirname)
.PHONY: $(CRATES)

test:
	$(MAKE) -j$(NUM_CORES) $(CRATES)/test

$(CRATES)/test:
	cargo test --manifest-path $@/Cargo.toml

lint:
	$(MAKE) -j$(NUM_CORES) $(CRATES)/lint

$(CRATES)/lint:
	cargo fmt --manifest-path $@/Cargo.toml # -- --check
	#cargo clippy --manifest-path $@/Cargo.toml --all-targets --all-features -- -D warnings
