NUM_CORES := $(shell nproc)

CRATES := $(shell find . -name Cargo.toml | xargs -n 1 dirname)
.PHONY: $(CRATES)

test:
	$(MAKE) -j$(NUM_CORES) $(addsuffix /test, $(CRATES))

$(CRATES:%=%/test):
	cargo test --manifest-path $(@D)/Cargo.toml

lint:
	$(MAKE) -j$(NUM_CORES) $(addsuffix /lint, $(CRATES))

$(CRATES:%=%/lint):
	cargo fmt --manifest-path $(@D)/Cargo.toml -- --check
	cargo clippy --manifest-path $(@D)/Cargo.toml --all-targets --all-features -- -D warnings
