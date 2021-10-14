.DEFAULT_GOAL := help
.PHONY: build test run help

build: ## build the code
	cargo build --release

build_docker: ## build a docker image
	docker build -t fizz_buzz_server .

test: ## run tests
	cargo test

run: ## run the server
	cargo run --release

run_docker_locally: build_docker ## run the server via docker
	docker run --rm -p 8080:8080 --name fizzbuzz fizz_buzz_server:latest

help: ## Print this help message
	@grep -E '^[a-zA-Z_-]+:.*## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'
