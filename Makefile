SCRIPT_PATH = scripts/generate_types.rs

generate:
	rust-script $(SCRIPT_PATH)


wallet_example:
		cargo run --example  wallet

account_example:
		cargo run --example account


chmod:
	chmod +x $(SCRIPT_PATH)


clean:
	rm -rf src/models/mod.rs


prepare:
	cargo install cargo-nextest


test:
	cargo nextest run


doc:
	cargo test --doc


.PHONY: generate example chmod clean prepare test doc
