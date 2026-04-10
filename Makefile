CARGO ?= cargo
PYTHON ?= python3
UV ?= uv
ZENSICAL ?= zensical
PYTHONPYCACHEPREFIX ?= $(CURDIR)/.pycache

LOCALSTACK_IMAGE ?= localstack/localstack:3
LOCALSTACK_CONTAINER ?= s3q-test-localstack
LOCALSTACK_PORT ?= 4566
LOCALSTACK_REGION ?= us-east-1
S3Q_S3_TEST_BUCKET ?= s3q-test-bucket

.PHONY: fmt
fmt:
	$(CARGO) fmt --all

.PHONY: check
check:
	$(MAKE) check-rust
	$(MAKE) check-py

.PHONY: check-rust
check-rust:
	$(CARGO) fmt --all -- --check
	$(CARGO) check

.PHONY: check-py
check-py:
	$(CARGO) fmt --manifest-path python/Cargo.toml --all -- --check
	$(CARGO) check --manifest-path python/Cargo.toml
	PYTHONPYCACHEPREFIX="$(PYTHONPYCACHEPREFIX)" $(PYTHON) -m compileall python/s3q python/tests

.PHONY: test
test:
	$(MAKE) test-rust
	$(MAKE) test-py

.PHONY: test-rust
test-rust:
	$(CARGO) test

.PHONY: start-localstack
start-localstack:
ifdef CI_LOCALSTACK_RUNNING
	@echo "Skipping LocalStack container start (CI_LOCALSTACK_RUNNING=true)"
else
	docker rm -f $(LOCALSTACK_CONTAINER) || true
	docker run -d --name $(LOCALSTACK_CONTAINER) \
		-p $(LOCALSTACK_PORT):4566 \
		-e SERVICES=s3 \
		-e AWS_DEFAULT_REGION=$(LOCALSTACK_REGION) \
		-e AWS_ACCESS_KEY_ID=test \
		-e AWS_SECRET_ACCESS_KEY=test \
		$(LOCALSTACK_IMAGE)
	@echo "Waiting for LocalStack S3 to be ready..."
	@until curl -fsS "http://localhost:$(LOCALSTACK_PORT)/_localstack/health" | grep -Eq '"s3"[[:space:]]*:[[:space:]]*"(running|available)"'; do sleep 1; done
	@docker exec $(LOCALSTACK_CONTAINER) awslocal s3api create-bucket --bucket $(S3Q_S3_TEST_BUCKET) >/dev/null 2>&1 || true
endif

.PHONY: stop-localstack
stop-localstack:
ifdef CI_LOCALSTACK_RUNNING
	@echo "Skipping LocalStack container stop (CI_LOCALSTACK_RUNNING=true)"
else
	docker rm -f $(LOCALSTACK_CONTAINER) || true
endif

.PHONY: test-localstack
test-localstack: start-localstack
ifdef CI_LOCALSTACK_RUNNING
	@echo "Running S3 integration tests with CI LocalStack"
	AWS_ENDPOINT_URL="$${AWS_ENDPOINT_URL:-http://localhost:4566}" \
	AWS_REGION="$${AWS_REGION:-$(LOCALSTACK_REGION)}" \
	PGQRS_S3_BUCKET="$${PGQRS_S3_BUCKET:-$(S3Q_S3_TEST_BUCKET)}" \
	AWS_ACCESS_KEY_ID="$${AWS_ACCESS_KEY_ID:-test}" \
	AWS_SECRET_ACCESS_KEY="$${AWS_SECRET_ACCESS_KEY:-test}" \
	$(CARGO) test --test s3_integration -- --ignored
else
	@echo "Running S3 integration tests with local LocalStack"
	@AWS_ENDPOINT_URL="http://localhost:$(LOCALSTACK_PORT)" \
	AWS_REGION="$(LOCALSTACK_REGION)" \
	PGQRS_S3_BUCKET="$(S3Q_S3_TEST_BUCKET)" \
	AWS_ACCESS_KEY_ID="test" \
	AWS_SECRET_ACCESS_KEY="test" \
	$(CARGO) test --test s3_integration -- --ignored; \
	test_status=$$?; \
	$(MAKE) stop-localstack; \
	exit $$test_status
endif

.PHONY: test-s3
test-s3: test-localstack

.PHONY: test-py
test-py:
	PYTHONPYCACHEPREFIX="$(PYTHONPYCACHEPREFIX)" $(PYTHON) -m compileall python/s3q python/tests
	PYTHONPATH=python $(PYTHON) -m unittest discover -s python/tests -p 'test_*.py'

.PHONY: build-py
build-py:
	$(UV) run --with maturin maturin develop --manifest-path python/Cargo.toml

.PHONY: docs-build
docs-build:
	$(UV) run --with zensical $(ZENSICAL) build

.PHONY: docs-serve
docs-serve:
	$(UV) run --with zensical $(ZENSICAL) serve

.PHONY: docs
docs: docs-serve
