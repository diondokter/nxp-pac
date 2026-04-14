#!/bin/bash
## on push branch=main
## priority -100
## dedup dequeue
## cooldown 15m

set -euxo pipefail

export RUSTUP_HOME=/ci/cache/rustup
export CARGO_HOME=/ci/cache/cargo
export CARGO_TARGET_DIR=/ci/cache/target
export BUILDER_THREADS=4
export BUILDER_COMPRESS=true
export RUST_TOOLCHAIN=nightly-2025-09-26

# SUBMODULES!!!
git submodule update --init --recursive --checkout

rustup toolchain install ${RUST_TOOLCHAIN}

cd nxp-pac
docserver build -i . -o webroot/crates/nxp-pac/git.zup

export KUBECONFIG=/ci/secrets/kubeconfig.yml
POD=$(kubectl -n embassy get po -l app=docserver -o jsonpath={.items[0].metadata.name})
kubectl cp webroot/crates $POD:/data
