# ── Shell & environment ────────────────────────────────────────────────────────

set shell := ["bash", "-euo", "pipefail", "-c"]

nightly := "cargo +nightly"
lcov_dir := "target/coverage"
docs_dir := "docs"

# ── Info ──────────────────────────────────────────────────────────────────────

# List available recipes
default:
    @just --list

# Check all required tools are installed
checkhealth:
    #!/usr/bin/env bash
    ok=true; pass=0; fail=0

    tick()    { printf "  {{ GREEN }}✔{{ NORMAL }}  %-24s %s\n" "$1" "$2"; pass=$((pass + 1)); }
    cross()   { printf "  {{ RED }}✘{{ NORMAL }}  %-24s %s\n" "$1" "${2:-}"; ok=false; fail=$((fail + 1)); }
    section() { printf "\n  {{ BOLD }}{{ BLUE }}%s{{ NORMAL }}\n" "$1"; }

    check() {
        local name="${2:-$1}"
        if path=$(command -v "$1" 2>/dev/null); then tick "$name" "$path"; else cross "$name"; fi
    }

    printf "\n  {{ BOLD }}open-pql · environment check{{ NORMAL }}\n"

    section "Rust";              check rustup; check rustc; check cargo
    section "Cargo subcommands"; check cargo-nextest; check cargo-llvm-cov; check cargo-binstall
    section "Coverage";          check lcov; check genhtml
    section "Docs";              check mdbook; check wrangler
    section "Dev tools";         check just

    section "Toolchains"
    if nightly_ver=$(rustup toolchain list 2>/dev/null | grep '^nightly' | head -1 | awk '{print $1}') \
        && [[ -n "$nightly_ver" ]]; then
        tick "nightly" "$nightly_ver"
    else
        cross "nightly" "not installed — run: rustup toolchain install nightly"
    fi

    echo
    if $ok; then
        printf "  {{ BOLD }}{{ GREEN }}%d/%d checks passed{{ NORMAL }}\n\n" "$pass" "$((pass + fail))"
    else
        printf "  {{ BOLD }}{{ RED }}%d check(s) failed{{ NORMAL }} — run: just install-tools\n\n" "$fail"
        exit 1
    fi

# ── Setup ─────────────────────────────────────────────────────────────────────

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

# ── Rust ──────────────────────────────────────────────────────────────────────

# Check the workspace compiles
check:
    cargo check --all-features

# Build the workspace
build:
    cargo build --all-features

# Run tests via nextest (e.g. `just test my_test`)
test *args:
    {{ nightly }} nextest run {{ args }}

# Run Clippy lints
lint:
    {{ nightly }} clippy --all-targets --tests --benches --all-features

# Run clippy --fix then nightly fmt
fmt:
    {{ nightly }} clippy --fix --tests --allow-dirty --all-features
    {{ nightly }} fmt

# Generate rustdoc for all workspace members (no deps)
doc:
    cargo doc --workspace --no-deps

# ── Coverage ──────────────────────────────────────────────────────────────────

# Generate HTML + LCOV coverage reports into {{lcov_dir}}/
coverage:
    #!/usr/bin/env bash
    set -euo pipefail

    cov()   { {{ nightly }} llvm-cov "$@"; }
    rs_lcov="{{ lcov_dir }}/lcov.raw"
    out_lcov="{{ lcov_dir }}/lcov"

    echo "🧪 Running Rust tests with instrumentation..."
    cov nextest --no-report

    echo "📊 Generating coverage report..."
    mkdir -p {{ lcov_dir }}/
    cov report --lcov --output-path "$rs_lcov"

    workspace_root=$(dirname "$(cargo locate-project --workspace --message-format plain)")

    lcov -a "$rs_lcov" --filter region \
        --rc "c_file_extensions=c|h|cpp|cc|cxx|rs" \
        --ignore-errors inconsistent,corrupt,unsupported \
        --output-file "$out_lcov"

    genhtml --filter region,branch_region --ignore-errors category \
        --rc "c_file_extensions=c|h|cpp|cc|cxx|rs" \
        --title "open-pql Coverage" \
        --prefix "$workspace_root" \
        --legend --sort \
        --output-directory {{ lcov_dir }}/ "$out_lcov"

    echo "✅ Coverage report: {{ lcov_dir }}/index.html"

# ── Docs (mdbook) ─────────────────────────────────────────────────────────────

# Build the mdbook
book:
    mdbook build {{ docs_dir }}

# Serve the mdbook locally with live reload
book-serve:
    mdbook serve {{ docs_dir }}

# Build and deploy the book to Cloudflare Pages (project: openpql-docs)
book-deploy:
    mdbook build {{ docs_dir }}
    wrangler pages deploy {{ docs_dir }}/book --project-name=openpql-docs --branch=main

# ── CI ────────────────────────────────────────────────────────────────────────

# Parallel local-dev rebuild: test + fmt + coverage + book in parallel
dev-rebuild:
    #!/usr/bin/env bash
    set -euo pipefail
    commands=(
        'just test'
        'just fmt'
        'just coverage'
        'just book'
    )
    pids=()
    for cmd in "${commands[@]}"; do
        echo "🚀 Starting: $cmd"
        bash -c "$cmd" &
        pids+=($!)
    done
    exit_code=0
    for pid in "${pids[@]}"; do
        wait "$pid" || { echo "❌ Job (PID $pid) failed."; exit_code=1; }
    done
    if [[ $exit_code -eq 0 ]]; then
        echo "✅ All jobs succeeded."
    else
        echo "⚠️  At least one job failed."
    fi
    exit $exit_code
