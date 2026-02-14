# Contributing to idsmith

## Repo Structure

```
idsmith/
├── src/                   # Rust core library + CLI
├── tests/                 # Integration tests
├── bindings/
│   ├── python/            # PyO3 + maturin → PyPI
│   └── node/              # napi-rs → npm
└── .github/workflows/     # CI per package, with path filters
```

This is a Cargo workspace. The root crate (`idsmith`) publishes to crates.io independently. Binding crates (`idsmith-python`, `idsmith-node`) are `publish = false`.

## Quick Reference

### Core Rust crate

```bash
cargo check -p idsmith
cargo test -p idsmith
cargo fmt -p idsmith -- --check
cargo clippy -p idsmith -- -D warnings
cargo run -p idsmith -- iban DE 5
```

### Python bindings

```bash
cd bindings/python
python -m venv .venv && source .venv/bin/activate
pip install maturin[patchelf] pytest
maturin develop
pytest tests/ -v
```

### Node.js bindings

```bash
cd bindings/node
npm install
npm run build
npm test
```

### Full workspace

```bash
# Check everything compiles (requires no extra toolchains)
cargo check --workspace

# Format all Rust code
cargo fmt --workspace

# Lint all Rust code
cargo clippy --workspace -- -D warnings
```

## CI Path Filters

Each workflow runs only when relevant files change:

| Workflow | Triggers on |
|---|---|
| `ci.yml` | `src/**`, `tests/**`, `Cargo.toml` |
| `python-ci.yml` | `bindings/python/**`, `src/**`, `Cargo.toml` |
| `node-ci.yml` | `bindings/node/**`, `src/**`, `Cargo.toml` |
| `release.yml` | `Cargo.toml` (version change detection) |

This means changing only Python bindings won't trigger Rust core CI, and vice versa.

## Shared Cargo.lock

The workspace shares a single `Cargo.lock`. This is committed to git and ensures reproducible builds. When updating dependencies, the lock file changes will show binding dependencies (pyo3, napi) even if you only touched the core crate — this is expected.

## Release Process

All three packages publish from a single version bump:

1. Bump `version` in root `Cargo.toml`, `bindings/python/pyproject.toml`, and `bindings/node/package.json`
2. Push to `main` → `release.yml` detects version change, builds binaries, creates GitHub release
3. The GitHub release triggers all three publish workflows:
   - `publish.yml` → crates.io
   - `python-publish.yml` → PyPI (builds wheels for Linux/macOS/Windows)
   - `node-publish.yml` → npm (builds native modules for all platforms)

### Required secrets
- `CARGO_REGISTRY_TOKEN` — crates.io API token
- `NPM_TOKEN` — npm publish token
- PyPI uses OIDC trusted publishing (no secret needed, configure in PyPI settings)
