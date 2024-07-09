# Copyright (c) 2024 Ho Kim (ho.kim@ulagbulag.io). All rights reserved.
# Use of this source code is governed by a GPL-3-style license that can be
# found in the LICENSE file.

# Configure environment variables
ARG DEBIAN_VERSION="bookworm"
ARG NGINX_VERSION="stable-${DEBIAN_VERSION}-otel"
ARG PACKAGE="cassette"

# Be ready for serving
FROM docker.io/library/nginx:${NGINX_VERSION} as server

# Server Configuration
EXPOSE 6080/tcp
WORKDIR /usr/local/bin

# Copy static files
ARG PACKAGE
ADD ./LICENSE /usr/share/licenses/${PACKAGE}/LICENSE
# ADD ./favicon.ico /usr/share/nginx/html/favicon.ico
ADD ./nginx.conf /etc/nginx/conf.d/default.conf
ADD ./assets/ /usr/share/nginx/html/assets/

# Be ready for building
FROM docker.io/library/rust:1-${DEBIAN_VERSION} as builder

# Install dependencies
RUN true \
    # Enable wasm32 target
    && rustup target add wasm32-unknown-unknown \
    # Build
    && cargo install --root /usr/local \
    trunk \
    wasm-bindgen-cli

# Load source files
ADD ./Cargo.toml /src/
ADD ./crates /src/crates
WORKDIR /src

# Build it!
RUN \
    # Cache build outputs
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    # Create an output directory
    mkdir /out \
    # Create an empty assets directory
    && mkdir assets \
    # Build
    && trunk build './crates/cassette/index.html' --dist '/out' --release

# Copy executable files
FROM server
COPY --from=builder /out/* /usr/share/nginx/html/
