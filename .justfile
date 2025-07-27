alias co := cov-open

default:
    echo 'default'

fmt:
    __CARGO_FIX_YOLO=1 cargo +nightly clippy --fix --lib -p rs-pql --tests -Z unstable-options && cargo +nightly fmt

cov:
    . ./scripts/cov.sh

cov-open:
    handlr open ./target/llvm-cov/html/index.html
