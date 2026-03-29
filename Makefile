PYTHON ?= python3
CARGO ?= cargo

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: check
check:
	$(CARGO) fmt --all -- --check
	$(PYTHON) -m compileall python

.PHONY: test
test:
	$(CARGO) test
	$(PYTHON) -m compileall python

.PHONY: docs-build
docs-build:
	mkdocs build

.PHONY: docs-serve
docs-serve:
	mkdocs serve
