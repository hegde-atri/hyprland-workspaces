default: help

dev: ## Build project and install it to your cargo path
	cargo install --path .
	@echo "Done."

offline: ## Build project and move executable to root of project.
	@cargo build
	@mv target/debug/repoman .

build: ## Build the release version
	@cargo build --release

help: ## Display this help screen
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n"} /^[$$()% a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)
