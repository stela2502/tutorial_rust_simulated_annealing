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


# Makefile for deploying Bookdown project

# Variables
BOOK_DIR = _book
MAIN_PAGE = using-rust-to-implement-the-simulated-annealing-algorithm.html
INDEX_PAGE = index.html
REPO_URL = git@github.com:stela2502/tutorial_rust_simulated_annealing.git
BRANCH = gh-pages


# Render the book
render_book:
	Rscript -e 'bookdown::render_book("index.Rmd", "bookdown::gitbook")'

# Copy main page as index.html
copy_index:
	cp $(BOOK_DIR)/$(MAIN_PAGE) $(BOOK_DIR)/$(INDEX_PAGE)

# Commit and push to GitHub Pages
deploy_book: render_book copy_index
	cd $(BOOK_DIR) && \
	git init && \
	git remote add origin $(REPO_URL) && \
	git checkout -b $(BRANCH) && \
	git add . && \
	git commit -m "Deploy book" && \
	git push -u origin $(BRANCH) --force

# Clean up
clean_book:
	rm -rf $(BOOK_DIR)

deploy: clean_book deploy_book 

.PHONY: all render_book copy_index deploy_book clean_book

