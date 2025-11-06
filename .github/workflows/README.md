# GitHub Actions CI Workflows

This directory contains GitHub Actions workflows for the Ceridwen project.

## Workflows

### ci-setup.yml

A reusable workflow that sets up Rust toolchain and caches Cargo dependencies. This workflow can be called from other workflows to ensure consistent setup and efficient caching.

**Features:**
- Installs Rust toolchain (configurable version)
- Installs Rust components (rustfmt, clippy by default)
- Caches Cargo registry index
- Caches Cargo registry cache
- Caches Cargo git dependencies
- Caches build artifacts (target directory)
- Verifies installation

**Inputs:**
- `rust-toolchain`: Rust toolchain version (default: 'stable')
- `rust-components`: Comma-separated list of components (default: 'rustfmt,clippy')

**Usage Example:**
```yaml
jobs:
  setup:
    uses: ./.github/workflows/ci-setup.yml
    with:
      rust-toolchain: stable
      rust-components: rustfmt,clippy
```

### ci.yml

Main CI workflow that runs on pushes and pull requests to `main` and `develop` branches.

**Workflow steps:**
1. Checkout code
2. Install Rust toolchain with rustfmt and clippy
3. Cache Cargo dependencies and build artifacts
4. Check code formatting with `cargo fmt`
5. Run linter with `cargo clippy`
6. Build the project
7. Run tests

**Caching Strategy:**

The workflow uses multiple cache layers to optimize build times:

1. **Cargo Registry Index** (`~/.cargo/registry/index`):
   - Contains metadata about available crates
   - Key: OS + hash of Cargo.lock
   
2. **Cargo Registry Cache** (`~/.cargo/registry/cache`):
   - Downloaded crate archives
   - Key: OS + hash of Cargo.lock

3. **Cargo Git Dependencies** (`~/.cargo/git`):
   - Git-based dependencies
   - Key: OS + hash of Cargo.lock

4. **Build Artifacts** (`target/`):
   - Compiled dependencies and project binaries
   - Key: OS + toolchain + hash of Cargo.lock
   - Restore keys allow partial matches for faster rebuilds

**Cache Invalidation:**

Caches are automatically invalidated when:
- `Cargo.lock` changes (new dependencies, version updates)
- Rust toolchain version changes (for build artifacts cache)

## Local Testing

To run the same checks locally before pushing:

```bash
# Format check
cargo fmt --all -- --check

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Build
cargo build --verbose

# Run tests
cargo test --verbose
```

## Benefits of Caching

- **Faster CI runs**: Cached dependencies significantly reduce build times
- **Reduced network usage**: Dependencies are downloaded once per lock file change
- **Reliable builds**: Consistent environment across runs
- **Cost savings**: Fewer compute minutes used
