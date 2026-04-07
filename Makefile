CARGO ?= cargo
PYTHON ?= python3
UV ?= uv
ZENSICAL ?= zensical

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: check
check:
	$(CARGO) fmt --all -- --check
	$(CARGO) check
	$(PYTHON) -m compileall python

.PHONY: check-rust
check-rust:
	$(CARGO) fmt --all -- --check
	$(CARGO) check

.PHONY: check-py
check-py:
	$(PYTHON) -m compileall python

.PHONY: test
test:
	$(MAKE) test-rust
	$(MAKE) test-py

.PHONY: test-rust
test-rust:
	$(CARGO) test

.PHONY: test-py
test-py:
	$(PYTHON) -m compileall python

.PHONY: docs-build
docs-build:
	$(UV) run --with zensical $(ZENSICAL) build

.PHONY: docs
docs: docs-build

.PHONY: docs-serve
docs-serve:
	$(UV) run --with zensical $(ZENSICAL) serve
