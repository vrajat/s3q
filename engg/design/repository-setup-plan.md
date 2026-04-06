# Repository Setup Plan

**Status:** Proposed  
**Scope:** Bring `s3q` repository setup up to the same baseline quality and operational shape as `pgqrs`, with `zensical` replacing MkDocs.

## 1. Goal

Before implementing the queue product, the repository should have a stable developer and CI foundation.

The target is not to copy `pgqrs` mechanically. The target is to match its level of maturity in:

- toolchain management
- CI structure
- docs hosting setup
- engineering documentation layout
- local developer workflows

## 2. Current Gaps

Compared to `pgqrs`, the current `s3q` repository is still missing or underdeveloped in several areas:

- no `mise.toml`
- no Buildkite bootstrap scripts
- no GitHub Actions workflows
- Buildkite pipeline is only a smoke scaffold
- docs are wired to MkDocs instead of `zensical`
- README still reflects the old queue-plus-workflow direction
- `engg/` is much thinner than the `pgqrs` structure
- Makefile is too small to serve as the main local/CI task entrypoint

## 3. Guiding Decisions

### 3.1 Follow `pgqrs` Repo Shape

When there is no strong reason to differ, `s3q` should follow `pgqrs` conventions for:

- CI layout
- repo task structure
- engineering-doc organization
- toolchain bootstrapping

### 3.2 Use `zensical` Instead of MkDocs

This is an intentional project-level deviation from `pgqrs`.

Implications:

- remove `mkdocs.yml`
- replace docs build and serve commands
- update CI to validate `zensical` instead of MkDocs

### 3.3 Use `mise` for Tool Versions, Keep `Makefile` for Tasks

`mise` should be introduced for version and environment management.

`Makefile` should remain the main task runner for now.

Reason:

- this matches the practical `pgqrs` model
- it keeps local workflows simple
- it avoids overcommitting to `mise` tasks while worktree ergonomics remain a concern

This means the intended pattern is:

- `mise.toml` defines tools
- Buildkite runs `mise install`
- Buildkite and local dev execute `mise exec -- make ...`

## 4. Target Repository Baseline

After this setup work, `s3q` should have:

- `mise.toml`
- a more complete `Makefile`
- `.buildkite/pipeline.yml`
- `.buildkite/scripts/bootstrap.sh`
- `.github/workflows/ci.yml`
- `zensical` docs configuration
- queue-aligned README and docs
- expanded `engg/` structure

Release automation is explicitly out of scope for this phase.

## 5. Workstreams

### Workstream A: Tooling Foundation

Add the basic toolchain and local task structure.

Deliverables:

- `mise.toml`
- expanded `Makefile`
- task naming aligned with likely CI steps

Planned commands:

- `make check`
- `make test`
- `make test-rust`
- `make test-py`
- `make docs-build`
- `make docs-serve`

Notes:

- `Makefile` should stay thin and explicit
- avoid hiding complex logic in shell one-liners
- keep commands usable both directly and through `mise exec`

### Workstream B: Buildkite Parity

Upgrade Buildkite from a smoke pipeline to a real repository CI surface.

Deliverables:

- new `.buildkite/pipeline.yml`
- `.buildkite/scripts/bootstrap.sh`
- optional helper scripts only if needed

Initial Buildkite steps:

- format/check
- Rust tests
- Python compile/test
- docs build

Design rule:

- Buildkite should be the primary CI path
- cached toolchain and cargo artifacts should follow the same broad pattern as `pgqrs`

### Workstream C: GitHub Actions Baseline

Add a minimal `ci.yml` for manual/reference use.

Deliverables:

- `.github/workflows/ci.yml`

Purpose:

- serve as a portable CI definition
- allow manual runs outside Buildkite
- document expected checks for contributors

Non-goal:

- full release automation in this phase

### Workstream D: Docs Migration to `zensical`

Replace MkDocs plumbing with `zensical`.

Deliverables:

- remove `mkdocs.yml`
- add `zensical` config
- wire docs build/serve into `Makefile`
- update CI docs step
- create the initial docs skeleton for the queue-only product

Notes:

- keep the `docs/` content directory
- migrate infra first, but establish the docs skeleton during setup rather than deferring docs until the end

### Workstream E: Repository Content Cleanup

Bring repo-level docs in line with the queue-only PRD.

Deliverables:

- updated `README.md`
- updated docs home page
- removal of stale workflow language
- local command examples updated for `mise` and `zensical`
- initial placeholder pages for:
  - queue concepts
  - inspection and metrics
  - Rust quickstart
  - Python quickstart
  - CLI reference
  - development/setup

### Workstream F: `engg/` Structure Parity

Expand `engg/` to match the organizational quality of `pgqrs`.

Deliverables:

- `engg/design/README.md`
- `engg/adr/template.md`
- `engg/processes/README.md`
- `engg/reviews/README.md`

Potential later additions:

- `engg/PHILOSOPHY.md`
- process docs for CI, release, and local development

## 6. Explicit Non-Goals

This phase should not include:

- queue API implementation
- Rust/Python binding implementation
- release publishing workflows
- package publishing automation
- workflow-engine remnants or redesign work

## 7. Recommended Execution Order

### Phase 1: Toolchain and CI Skeleton

1. Add `mise.toml`
2. Expand `Makefile`
3. Add Buildkite bootstrap script
4. Replace Buildkite pipeline

This establishes the operational baseline first.

### Phase 2: GitHub and Docs Plumbing

1. Add GitHub Actions `ci.yml`
2. Migrate docs from MkDocs to `zensical`
3. Wire docs build into CI
4. Create the queue-only docs skeleton

This makes the repo easier to validate outside Buildkite.

### Phase 3: Documentation and Engineering Structure Cleanup

1. Rewrite `README.md`
2. Align docs home and development docs with queue-only scope
3. Expand `engg/` structure

This ensures the repo tells the right story before implementation begins.

## 8. Risks and Tradeoffs

### `mise` vs `Makefile`

Risk:

- trying to move all task orchestration into `mise` too early creates friction, especially with worktrees

Decision:

- use `mise` for tool versions
- keep `Makefile` for repo tasks

### Docs Migration Cost

Risk:

- `zensical` migration may require some docs layout adjustments beyond a config swap

Decision:

- accept this early while the docs corpus is still small

### CI Overdesign

Risk:

- copying too much of `pgqrs` CI before `s3q` has a real feature matrix creates churn

Decision:

- copy the structure, not the entire matrix
- keep the first CI version intentionally small but real

## 9. Success Criteria

This setup phase is complete when:

- contributors can run repo tasks through `mise exec -- make ...`
- Buildkite runs meaningful checks, not just smoke commands
- GitHub Actions has a usable reference CI workflow
- docs build through `zensical`
- README and docs reflect the queue-only product direction
- the docs site has an initial queue-only skeleton ready before product implementation begins
- `engg/` has a clear place for PRD, design, ADRs, reviews, and processes

## 10. Next Step After This Plan

Once the repository baseline is in place, the next design document should cover:

- queue architecture on top of `pgqrs::store::s3::S3Store`
- Rust API design aligned with `pgmq` where in doubt
- Python SDK and CLI shape
- inspection and metrics implementation plan
