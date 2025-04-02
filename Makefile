# The root directory of your project
ROOT_DIR := $(shell pwd)

# Define the steps for the tutorial
STAGES := step1 step2 step3 step4 step5

# Build each step's cargo project
all: $(STAGES)

# Loop through each stage and run cargo build
$(STAGES):
	@echo "Building $(ROOT_DIR)/rust_stages/$@..."
	@cd $(ROOT_DIR)/rust_stages/$@ && cargo test -r 2>&1 | tee $(ROOT_DIR)/$@_build.log || true
	@echo "Finished building $(ROOT_DIR)/rust_stages/$@. Logs are in $@_build.log"

# Optional: Clean up each stage's cargo build
clean: $(STAGES)
	@for stage in $(STAGES); do \
		echo "Cleaning $(ROOT_DIR)/rust_stages/$$stage..."; \
		cd $(ROOT_DIR)/rust_stages/$$stage && cargo clean; \
		echo "Finished cleaning $$stage"; \
	done
