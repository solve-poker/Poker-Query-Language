# List available recipes
default:
    @just --list

nightly := "cargo +nightly"
lcov_dir := "./target/coverage"

# Build the project
build:
    cargo build --all-features

# Run tests via nextest (e.g. `just test my_test`)
test *args:
    {{ nightly }} nextest run {{ args }}

# Run clippy lints
lint:
    {{ nightly }} clippy --all-targets --tests --benches --all-features

# Fix and format code
fmt:
    {{ nightly }} clippy --fix --tests --allow-dirty --all-features
    {{ nightly }} fmt

# Build and serve documentation
book:
    mdbook build docs

book-serve:
    mdbook serve docs

doc:
    cargo doc --no-deps

# Generate coverage report
coverage:
    mkdir -p {{ lcov_dir }}
    {{ nightly }} llvm-cov nextest --no-report
    {{ nightly }} llvm-cov report --lcov --output-path {{ lcov_dir }}/lcov.raw
    lcov -a {{ lcov_dir }}/lcov.raw \
        --filter region \
        --rc "c_file_extensions=c|h|cpp|cc|cxx|rs" \
        --ignore-errors inconsistent,corrupt,unsupported \
        --output-file {{ lcov_dir }}/lcov
    genhtml \
        --filter region,branch_region \
        --rc "c_file_extensions=c|h|cpp|cc|cxx|rs" \
        --title "open-pql Coverage" \
        --sort --flat \
        --output-directory {{ lcov_dir }}/ \
        {{ lcov_dir }}/lcov
    @echo "Coverage report: {{ lcov_dir }}/index.html"

# Install required toolchain components and cargo tools
install-tools:
    #!/usr/bin/env bash
    set -euo pipefail
    for tc in stable nightly; do
        rustup toolchain list | grep -q "^$tc" || rustup toolchain install "$tc"
    done
    rustup component add rust-analyzer rust-src llvm-tools
    rustup component add llvm-tools-preview --toolchain nightly
    cargo binstall --no-confirm cargo-edit cargo-shear cargo-llvm-cov cargo-nextest
