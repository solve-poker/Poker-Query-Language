# List available recipes
default:
    @just --list

# ── Build ──────────────────────────────────────────────

# Build in debug mode
build:
    cargo build --all-features

# Build in release mode
build-release:
    cargo build --release --all-features

# ── Quality ────────────────────────────────────────────

# Run tests
test:
    cargo nextest run

# Run clippy lints
lint:
    cargo clippy

# Run clippy --fix then nightly fmt
fmt:
    {{ nightly }} clippy --fix --tests --allow-dirty
    {{ nightly }} fmt

# ── Coverage ───────────────────────────────────────────

nightly := "cargo +nightly"
lcov_dir := "./target/coverage"

# Generate an LCOV coverage report
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
        --sort \
        --flat \
        --output-directory {{ lcov_dir }}/ \
        {{ lcov_dir }}/lcov
    echo "Coverage report: {{ lcov_dir }}/"

# ── Documentation & inspection ─────────────────────────

# Build rustdoc (no dependencies)
doc:
    cargo doc --no-deps

# Expand macros in openpql-runner
expand-macro:
    cd openpql-runner && cargo expand functions

# Run bench suite (compile-only)
bench:
    cargo bench --no-run

# ── WASM ───────────────────────────────────────────────

# Serve the WASM playground locally
wasm:
    trunk serve --config ./open-pql-wasm/Trunk.toml

# ── Setup ──────────────────────────────────────────────

# Install required cargo tools and components
install-tools:
    rustup component add rust-analyzer rust-src llvm-tools llvm-tools-preview
    cargo binstall cargo-edit cargo-shear cargo-llvm-cov cargo-nextest trunk --no-confirm

# Verify all required tools are installed
checkhealth:
    #!/usr/bin/env bash
    ok=true
    check() {
        if command -v "$1" &>/dev/null; then
            echo "  [ok] $1"
        else
            echo "  [missing] $1"
            ok=false
        fi
    }

    echo "--- required ---"
    check cargo
    check rustup
    check rustc

    echo "--- cargo subcommands ---"
    check cargo-nextest
    check cargo-llvm-cov
    check cargo-binstall

    echo "--- coverage ---"
    check lcov
    check genhtml

    echo "--- wasm ---"
    check trunk

    echo "--- nightly toolchain ---"
    if rustup toolchain list | grep -q nightly; then
        echo "  [ok] nightly"
    else
        echo "  [missing] nightly (run: rustup toolchain install nightly)"
        ok=false
    fi

    $ok && echo "" && echo "all checks passed" || { echo ""; echo "run: just install-tools"; exit 1; }
