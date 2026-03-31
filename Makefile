.PHONY: help dev dev-build dev-down test-unit test-sync test-forge fmt fmt-check clippy build build-release build-contracts key-gen clean docker-clean

# Default target: show help
help:
	@echo "Nightfall 4 CE - Development Commands"
	@echo ""
	@echo "  make dev              Start the full development stack"
	@echo "  make dev-build        Build and start the development stack"
	@echo "  make dev-down         Stop all development services"
	@echo ""
	@echo "  make build            Build all Rust crates"
	@echo "  make build-release    Build all Rust crates (release mode)"
	@echo "  make build-contracts  Build Solidity contracts with Foundry"
	@echo ""
	@echo "  make test-unit        Run Rust unit tests"
	@echo "  make test-sync        Run synchronization tests via Docker"
	@echo "  make test-forge       Run Solidity contract tests"
	@echo ""
	@echo "  make fmt              Format Rust code"
	@echo "  make fmt-check        Check Rust code formatting"
	@echo "  make clippy           Run clippy linter"
	@echo ""
	@echo "  make key-gen          Generate ZK proving keys (heavy)"
	@echo "  make clean            Clean Rust and Solidity build artifacts"
	@echo "  make docker-clean     Remove Docker containers, volumes, and images"

# Start the full development stack
dev:
	docker compose --profile development --env-file .env up

# Build and start the development stack
dev-build:
	docker compose --profile development --env-file .env up --build

# Stop all development services
dev-down:
	docker compose --profile development down

# Run Rust unit tests
test-unit:
	cargo test

# Run synchronization tests via Docker
test-sync:
	docker compose --profile sync_test --env-file .env up

# Run Solidity contract tests
test-forge:
	forge test

# Format Rust code (requires nightly)
fmt:
	cargo +nightly fmt

# Check Rust code formatting
fmt-check:
	cargo +nightly fmt -- --check

# Run clippy linter (strict mode)
clippy:
	cargo clippy --all-targets -- -D warnings

# Build all Rust crates
build:
	cargo build

# Build all Rust crates in release mode
build-release:
	cargo build --release

# Build Solidity contracts with Foundry
build-contracts:
	forge clean && forge build

# Generate ZK proving keys (resource-intensive)
key-gen:
	NF4_MOCK_PROVER=false cargo run --release --bin key_generation

# Clean Rust and Solidity build artifacts
clean:
	cargo clean && forge clean

# Remove Docker containers, volumes, and locally-built images
docker-clean:
	docker compose down -v --rmi local
