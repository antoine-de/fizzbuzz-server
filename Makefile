.DEFAULT_GOAL := help
.PHONY: build test run help

build: ## build the code
	cargo build --release

test: ## run tests
	cargo test

run: ## run the server
	cargo run --release

help: ## Print this help message
	@grep -E '^[a-zA-Z_-]+:.*## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
