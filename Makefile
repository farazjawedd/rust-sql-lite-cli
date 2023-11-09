# Targets
.PHONY: all build test format lint clean

all: build test format lint clean

build:
	cargo build --release --manifest-path ./cli_tool/Cargo.toml

copy:
	cp cli_tool/target/release/cli_tool ./"Binary_Executable"

test:
	cargo test --quiet --manifest-path ./cli_tool/Cargo.toml

format:
	cargo fmt --manifest-path ./cli_tool/Cargo.toml

lint:
	cargo clippy --quiet --manifest-path ./cli_tool/Cargo.toml

clean:
	cargo clean --manifest-path ./cli_tool/Cargo.toml

# Generate and push changes to GitHub
generate_and_push:
	@if [ -n "$$(git status --porcelain)" ]; then \
		git config --local user.email "action@github.com"; \
		git config --local user.name "GitHub Action"; \
		git add .; \
		git commit -m "Add query log"; \
		git push; \
	else \
		echo "No changes to commit. Skipping commit and push."; \
	fi