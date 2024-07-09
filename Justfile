# Copyright (c) 2024 Ho Kim (ho.kim@ulagbulag.io). All rights reserved.
# Use of this source code is governed by a GPL-3-style license that can be
# found in the LICENSE file.

# Load environment variables
set dotenv-load

# Configure environment variables
export DEBIAN_VERSION := env_var_or_default('DEBIAN_VERSION', 'bookworm')
export NGINX_VERSION := env_var_or_default('NGINX_VERSION', "stable-bookworm-otel")
export OCI_BUILD_LOG_DIR := env_var_or_default('OCI_BUILD_LOG_DIR', './logs/')
export OCI_IMAGE := env_var_or_default('OCI_IMAGE', 'quay.io/ulagbulag/cassette')
export OCI_IMAGE_VERSION := env_var_or_default('OCI_IMAGE_VERSION', 'latest')
export OCI_PLATFORMS := env_var_or_default('OCI_PLATFORMS', 'linux/arm64,linux/amd64')

default: run

fmt:
  cargo fmt --all

clippy: fmt
  cargo clippy --all --workspace

test: clippy
  cargo test --all --workspace

init:
  @rustup target list | grep wasm32-unknown-unknown | grep -q '(installed)' || rustup target add wasm32-unknown-unknown
  @which trunk >/dev/null || cargo install trunk
  @which wasm-bindgen >/dev/null || cargo install wasm-bindgen-cli

_trunk command *ARGS: init
  trunk "{{ command }}" --dist './dist' './crates/cassette/index.html' {{ ARGS }}

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
    --build-arg DEBIAN_VERSION="${DEBIAN_VERSION}" \
    --build-arg NGINX_VERSION="${NGINX_VERSION}" \
    --platform "${OCI_PLATFORMS}" \
    --pull \
    {{ ARGS }} \
    . 2>&1 | tee "${OCI_BUILD_LOG_DIR}/build-base-$( date -u +%s ).log"

oci-build: (_oci-build './Dockerfile' '')

oci-build-server: (_oci-build '-server' './Dockerfile.server')

oci-push: (_oci-build './Dockerfile' '' "--push")

oci-push-server: (_oci-build './Dockerfile.server' '-server' "--push")
