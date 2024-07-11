# Copyright (c) 2024 Ho Kim (ho.kim@ulagbulag.io). All rights reserved.
# Use of this source code is governed by a GPL-3-style license that can be
# found in the LICENSE file.

# Load environment variables
set dotenv-load

# Configure environment variables
export DEBIAN_IMAGE := env_var_or_default('DEBIAN_IMAGE', 'docker.io/library/debian')
export DEBIAN_VERSION := env_var_or_default('DEBIAN_VERSION', 'bookworm')
export NGINX_IMAGE := env_var_or_default('NGINX_VERSION', "docker.io/library/nginx")
export NGINX_VERSION := env_var_or_default('NGINX_VERSION', "stable")
export NODE_IMAGE := env_var_or_default('NGINX_VERSION', "docker.io/library/node")
export NODE_VERSION := env_var_or_default('NGINX_VERSION', "22")
export RUST_IMAGE := env_var_or_default('RUST_IMAGE', "docker.io/library/rust")
export RUST_VERSION := env_var_or_default('RUST_VERSION', "1")

export OCI_BUILD_LOG_DIR := env_var_or_default('OCI_BUILD_LOG_DIR', './logs/')
export OCI_IMAGE := env_var_or_default('OCI_IMAGE', 'quay.io/ulagbulag/cassette')
export OCI_IMAGE_VERSION := env_var_or_default('OCI_IMAGE_VERSION', 'latest')
export OCI_PLATFORMS := env_var_or_default('OCI_PLATFORMS', 'linux/arm64,linux/amd64')

default: run

fmt:
  cargo fmt --all

clippy: fmt
  cargo clippy --all --workspace -- -D warnings

test: clippy
  cargo test --all --workspace

init:
  @# Node
  @test -d node_modules || npm clean-install

  @# Rust
  @rustup target list | grep wasm32-unknown-unknown | grep -q '(installed)' || rustup target add wasm32-unknown-unknown

  @# Rust Deny
  @which cargo-deny >/dev/null || cargo install cargo-deny

  @# Rust Trunk
  @which trunk >/dev/null || cargo install trunk
  @which wasm-bindgen >/dev/null || cargo install wasm-bindgen-cli

_trunk command *ARGS: init
  trunk "{{ command }}" {{ ARGS }}

check: init
  cargo deny check --show-stats

build *ARGS: ( _trunk "build" ARGS )

run *ARGS: ( _trunk "serve" ARGS )

run-gateway *ARGS:
  cargo run --package 'cassette-gateway' --release

run-operator *ARGS:
  cargo run --package 'cassette-operator' --release

_oci-build file oci_suffix *ARGS:
  mkdir -p "${OCI_BUILD_LOG_DIR}"
  docker buildx build \
    --file "{{ file }}" \
    --tag "${OCI_IMAGE}{{ oci_suffix }}:${OCI_IMAGE_VERSION}" \
    --build-arg DEBIAN_IMAGE="${DEBIAN_IMAGE}" \
    --build-arg DEBIAN_VERSION="${DEBIAN_VERSION}" \
    --build-arg NGINX_IMAGE="${NGINX_IMAGE}" \
    --build-arg NGINX_VERSION="${NGINX_VERSION}" \
    --build-arg NODE_IMAGE="${NODE_IMAGE}" \
    --build-arg NODE_VERSION="${NODE_VERSION}" \
    --build-arg RUST_IMAGE="${RUST_IMAGE}" \
    --build-arg RUST_VERSION="${RUST_VERSION}" \
    --platform "${OCI_PLATFORMS}" \
    --pull \
    {{ ARGS }} \
    . 2>&1 | tee "${OCI_BUILD_LOG_DIR}/build-base-$( date -u +%s ).log"

oci-build: (_oci-build './Dockerfile' '')

oci-build-server: (_oci-build '-server' './Dockerfile.server')

oci-push: (_oci-build './Dockerfile' '' "--push")

oci-push-server: (_oci-build './Dockerfile.server' '-server' "--push")
