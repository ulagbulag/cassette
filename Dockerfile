# Copyright (c) 2024 Ho Kim (ho.kim@ulagbulag.io). All rights reserved.
# Use of this source code is governed by a GPL-3-style license that can be
# found in the LICENSE file.

# Configure environment variables
ARG PACKAGE="cassette"

ARG DEBIAN_VERSION="bookworm"
ARG NGINX_IMAGE="docker.io/library/nginx"
ARG NGINX_VERSION="stable"
ARG NODE_IMAGE="docker.io/library/node"
ARG NODE_VERSION="22"
ARG RUST_IMAGE="docker.io/library/rust"
ARG RUST_VERSION="1"

ARG _OS_VERSION="${DEBIAN_VERSION}"

# Be ready for serving
FROM "${NGINX_IMAGE}:${NGINX_VERSION}-${_OS_VERSION}-otel" AS server

# Server Configuration
EXPOSE 6080/tcp
WORKDIR /usr/local/bin

# Copy static files
ARG PACKAGE
ADD ./LICENSE /usr/share/licenses/${PACKAGE}/LICENSE
# ADD ./favicon.ico /usr/share/nginx/html/favicon.ico
ADD ./nginx.conf /etc/nginx/conf.d/default.conf
ADD ./assets/ /usr/share/nginx/html/assets/

# Be ready for building npm
FROM "${NODE_IMAGE}:${NODE_VERSION}-${_OS_VERSION}" AS node_builder

# Load package metadata files
ADD ./package.json /src/
ADD ./package-lock.json /src/
WORKDIR /src

# Install node dependencies
RUN npm clean-install

# Be ready for building
FROM "${RUST_IMAGE}:${RUST_VERSION}-${_OS_VERSION}" AS builder

# Install dependencies
RUN true \
    # Enable wasm32 target
    && rustup target add wasm32-unknown-unknown \
    # Build
    && cargo install --root /usr/local \
    trunk \
    wasm-bindgen-cli

# Load node dependencies
COPY --from=node_builder /src/node_modules/ /src/node_modules/

# Load source files
ADD ./LICENSE /src/LICENSE
ADD ./assets/images/icons /src/assets/images/icons
ADD ./Cargo.toml /src/
ADD ./crates /src/crates

# Load static files
ADD ./index.html /src/
ADD ./static /src/static
ADD ./Trunk.toml /src/
WORKDIR /src

# Build it!
RUN \
    # Cache build outputs
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    # Create an output directory
    mkdir /out \
    # Disable terminal hooks
    && sed -i '/\[\[hooks\]\]/,$d' 'Trunk.toml' \
    # Build
    && trunk build --dist '/out' --release

# Copy executable files
FROM server
COPY --from=builder /out /usr/share/nginx/html
