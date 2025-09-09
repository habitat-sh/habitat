# GNU Make specific features enabled
MAKEFLAGS += --warn-undefined-variables
SHELL := bash
.SHELLFLAGS := -eu -o pipefail -c
.DELETE_ON_ERROR:
.SUFFIXES:

# Configuration variables
HAB_SUP_TEST_NETWORK_LOG_SERVICE ?= rando
HAB_SUP_TEST_NETWORK_SIZE ?= 3
HAB_SUP_TEST_NETWORK_PEER_NAME ?= rando_1
DOCKER_IMAGE_NAME := hab-sup-network
NETWORK_NAME := hab-sup-network
PROJECT_ROOT := $(shell cd ../.. && pwd)

# Phony targets
.PHONY: help up down start stop restart logs console kill ip-for ip-addresses status clean build rebuild all

# Default target
.DEFAULT_GOAL := help

# Colors for output (GNU Make specific)
GREEN := $(shell tput -Txterm setaf 2)
YELLOW := $(shell tput -Txterm setaf 3)
RED := $(shell tput -Txterm setaf 1)
RESET := $(shell tput -Txterm sgr0)

help: ## Show this help message
	@echo "$(GREEN)Available targets:$(RESET)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-15s$(RESET) %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# File targets
CTL_SECRET:
	@echo "$(GREEN)Generating CTL_SECRET...$(RESET)"
	hab sup secret generate > $@

# Build targets
build: ## Build Habitat binaries
	@echo "$(GREEN)Building Habitat components...$(RESET)"
	cd $(PROJECT_ROOT) && cargo build -p habitat-launcher
	cd $(PROJECT_ROOT) && cargo build -p habitat_sup
	cd $(PROJECT_ROOT) && cargo build -p hab
	@echo "$(GREEN)Build complete$(RESET)"

# Docker targets
up: build CTL_SECRET ## Build and start the network
	@echo "$(GREEN)Building Docker image...$(RESET)"
	docker build -t $(DOCKER_IMAGE_NAME) .
	@echo "$(GREEN)Starting network with $(HAB_SUP_TEST_NETWORK_SIZE) rando instances...$(RESET)"
	docker-compose up -d --scale rando=$(HAB_SUP_TEST_NETWORK_SIZE)

down: ## Stop and remove containers, networks, and secrets
	@echo "$(YELLOW)Stopping network...$(RESET)"
	docker-compose down --volumes --remove-orphans || true
	-rm -f CTL_SECRET
	@echo "$(GREEN)Network stopped and cleaned up$(RESET)"

start: ## Start existing containers
	docker-compose start

stop: ## Stop running containers without removing them
	docker-compose stop

restart: stop start ## Restart all containers

# Utility targets
logs-for: ## Follow logs for specified service (default: rando)
	docker-compose logs -f $(HAB_SUP_TEST_NETWORK_LOG_SERVICE)

console: ## Open bash shell in specified container
	@CONTAINER_ID=$$(docker ps -qf name=$(HAB_SUP_TEST_NETWORK_PEER_NAME)); \
	if [ -z "$$CONTAINER_ID" ]; then \
		echo "$(RED)Error: Container $(HAB_SUP_TEST_NETWORK_PEER_NAME) not found or not running$(RESET)"; \
		exit 1; \
	fi; \
	docker exec -it $$CONTAINER_ID /bin/bash

kill: ## Stop specified container to simulate departure
	@CONTAINER_ID=$$(docker ps -qf name=$(HAB_SUP_TEST_NETWORK_PEER_NAME)); \
	if [ -z "$$CONTAINER_ID" ]; then \
		echo "$(RED)Error: Container $(HAB_SUP_TEST_NETWORK_PEER_NAME) not found or not running$(RESET)"; \
		exit 1; \
	fi; \
	docker stop $$CONTAINER_ID; \
	echo "$(GREEN)Stopped container: $(HAB_SUP_TEST_NETWORK_PEER_NAME)$(RESET)"

ip-of: ## Get IP address of specified container (pass via HAB_SUP_TEST_NETWORK_PEER_NAME)
	@docker network inspect $(NETWORK_NAME) 2>/dev/null | \
		jq -r ".[].Containers | to_entries[] | select(.value.Name | contains(\"$(HAB_SUP_TEST_NETWORK_PEER_NAME)\")) | .value.IPv4Address" | \
		cut -d '/' -f 1 | head -1 || \
		echo "$(RED)Error: Could not find IP for $(HAB_SUP_TEST_NETWORK_PEER_NAME)$(RESET)"

ip-addresses: ## Get IP addresses of all containers
	@docker network inspect hab-sup-network | \
	jq -r '[ .[].Containers.[] | {Name: .Name, IPv4: .IPv4Address} ] | sort_by(.Name) | .[] | [.Name, .IPv4] | @tsv' | \
	column -t --table-columns Name,"IP Address"

status: ## Show status of all containers
	docker-compose ps

# Maintenance targets
clean: down ## Clean up everything including volumes and images
	@echo "$(YELLOW)Cleaning up volumes and images...$(RESET)"
	-docker volume rm hab-cache hab-pkgs 2>/dev/null
	-docker image rm $(DOCKER_IMAGE_NAME) 2>/dev/null
	@echo "$(GREEN)Cleanup complete$(RESET)"

rebuild: clean up ## Clean rebuild and restart

all: build up ## Build everything and start

# Development helpers
logs: ## Follow logs for all services
	docker-compose logs -f

shell: ## Open shell in bastion container
	@CONTAINER_ID=$$(docker ps -qf name=sup-network-bastion-1); \
	if [ -z "$$CONTAINER_ID" ]; then \
		echo "$(RED)Error: bastion container not found$(RESET)"; \
		exit 1; \
	fi; \
	docker exec -it $$CONTAINER_ID /bin/bash

scale: ## Scale rando service (use: make scale SIZE=5)
	docker-compose up -d --scale rando=$(if $(SIZE),$(SIZE),$(HAB_SUP_TEST_NETWORK_SIZE))

# Debug target
debug: ## Show configuration variables
	@echo "$(GREEN)Configuration:$(RESET)"
	@echo "  LOG_SERVICE: $(HAB_SUP_TEST_NETWORK_LOG_SERVICE)"
	@echo "  NETWORK_SIZE: $(HAB_SUP_TEST_NETWORK_SIZE)"
	@echo "  PEER_NAME: $(HAB_SUP_TEST_NETWORK_PEER_NAME)"
	@echo "  IMAGE_NAME: $(DOCKER_IMAGE_NAME)"
	@echo "  PROJECT_ROOT: $(PROJECT_ROOT)"
