.PHONY: fact
fact: target/release/dog
	./$<

.INTERMEDIATE: target/release/dog
.PHONY: target/release/dog
target/release/dog:
	cargo build --release >/dev/null 2>/dev/null

.PHONY: lint
lint:
	cargo fmt -- --check
