SCRIPT_PATH = scripts/generate_types.rs

run:
	rust-script $(SCRIPT_PATH)


chmod:
	chmod +x $(SCRIPT_PATH)


clean:
	rm -rf src/models/mod.rs

.PHONY: run chmod clean
