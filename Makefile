NUM_CORES := $(shell nproc)

CRATES := $(shell find . -name Cargo.toml | xargs -n 1 dirname)
.PHONY: $(CRATES)
$(CRATES):
	cargo test --manifest-path $@/Cargo.toml

test:
	$(MAKE) -j$(NUM_CORES) $(CRATES)
