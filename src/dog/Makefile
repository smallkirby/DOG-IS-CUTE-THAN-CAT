.PHONY: fact
fact: vm/target/release/dog-vm dog.dog
	./$< < ./dog.dog

.INTERMEDIATE: vm/target/release/dog-vm
.PHONY: vm/target/release/dog-vm
vm/target/release/dog-vm:
	cd vm && cargo build --release >/dev/null 2>/dev/null

.PHONY: lint
lint:
	cd vm && cargo fmt -- --check
