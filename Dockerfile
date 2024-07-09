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
ENTRYPOINT [ "/usr/bin/env" ]
CMD [ "trunk", "serve" ]

# Be ready for building
FROM docker.io/library/rust:1-${DEBIAN_VERSION} as builder-dep

# Load source files
ADD . /src
WORKDIR /src

# Install dependencies
RUN true \
    # Enable wasm32 target
    && rustup target add wasm32-unknown-unknown \
    # Build
    && cargo install --root /usr/local \
    trunk \
    wasm-bindgen-cli

# Copy executable dependency files
FROM server
COPY --from=builder /usr/local/bin/* /usr/local/bin/

# Build it!
FROM builder-dep as builder
RUN \
    # Cache build outputs
    --mount=type=cache,target=/src/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    # Create an output directory
    mkdir /out \
    # Exclude non-client packages
    && find ./ -type f -name Cargo.toml -exec sed -i 's/^\( *\)\(.*\# *exclude *( *client *)\)$/\1# \2/g' {} + \
    && find ./ -type f -name Cargo.toml -exec sed -i 's/^\( *\)\# *\(.*\# *include *( *client *)\)$/\1\2/g' {} + \
    # Build
    && trunk build './crates/cassette/index.html' --release \
    && find ./target/release/ -maxdepth 1 -type f -perm -a=x -print0 | xargs -0 -I {} mv {} /out \
    && cp ./Trunk.toml /out \
    && mv ./LICENSE /LICENSE

# Copy executable files
FROM server
ARG PACKAGE
COPY --from=builder /usr/local/bin/* /usr/local/bin/
COPY --from=builder /out/* /usr/local/bin/
COPY --from=builder /LICENSE /usr/share/licenses/${PACKAGE}/LICENSE
