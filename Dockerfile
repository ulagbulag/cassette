# Copyright (c) 2024 Ho Kim (ho.kim@ulagbulag.io). All rights reserved.
# Use of this source code is governed by a GPL-3-style license that can be
# found in the LICENSE file.

# Configure environment variables
ARG DEBIAN_VERSION="bookworm"
ARG PACKAGE="cassette"

# Be ready for serving
FROM docker.io/library/debian:${DEBIAN_VERSION} as server

# Server Configuration
EXPOSE 6080/tcp
WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/bin/env" ]
CMD [ "trunk", "serve" ]

# Be ready for building
FROM docker.io/library/rust:1-${DEBIAN_VERSION} as builder

# Load source files
ADD . /src
WORKDIR /src

# Install dependencies
RUN \
    # Cache build outputs
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    # Create an output directory
    mkdir /out \
    # Build
    && cargo install trunk --root /usr/local \
    && cargo install wasm-bindgen-cli --root /usr/local

# Build it!
RUN \
    # Cache build outputs
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    # Build
    && cargo build --all --workspace --release \
    && find ./target/release/ -maxdepth 1 -type f -perm -a=x -print0 | xargs -0 -I {} mv {} /out \
    && cp ./Trunk.toml /out \
    && mv ./LICENSE /LICENSE

# Copy executable files
FROM server
ARG PACKAGE
COPY --from=builder /out/* /usr/local/bin/
COPY --from=builder /usr/local/bin/* /usr/local/bin/
COPY --from=builder /LICENSE /usr/share/licenses/${PACKAGE}/LICENSE
