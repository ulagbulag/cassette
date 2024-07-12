# 📼 Cassette

![Cassette logo](/assets/images/icons/logo.webp)

This is a Cloud-native Template-based dynamic declarative web UI framework built with [Yew].

You can get started by experiencing the various features here: https://api.ulagbulag.io

You can also deploy your own example app using one-line simple command:

```bash
just run-examples  # trunk serve --features examples
```

## Tutorial

TBD

## Usage

TBD

## Building

### Cassette Player (App)

#### Dependencies

- node (npm)
- rustup

#### Install dependencies

```bash
cargo install just  # for Justfile
```

#### Build

```bash
just build
```

#### Run a local server

```bash
just run  # or, just type "just"
```

### Cassette Gateway

#### Dependencies

- rustup

#### Install dependencies

```bash
cargo install just  # for Justfile
```

#### Run a local server

```bash
just run-gateway
```

#### Test

```bash
just test
```

### Cassette K8S Operator

#### Dependencies

- rustup

#### Install dependencies

```bash
cargo install just  # for Justfile
```

#### Run a local server

```bash
just run-operator
```

#### Test

```bash
just test
```

## Building Container Images

### Build

```bash
just oci-build  # Player (App)
just oci-build-server  # Gateway, Operator
```

### Run a local server

```bash
# Gateway
docker run --name cassette --rm \
    -p 8080:8080 \
    "quay.io/ulagbulag/cassette-server:latest" \
    'cassette-gateway'

# Player (App)
docker run --name cassette --rm \
    -p 6080:6080 \
    "quay.io/ulagbulag/cassette:latest"
```

### Run a local K8S operator

```bash
docker run --name cassette --rm \
    -v ~/.kube:/root/.kube:ro \
    "quay.io/ulagbulag/cassette-server:latest" \
    'cassette-operator'
```

### Deploy

```bash
just oci-push  # Player (App)
just oci-push-server  # Gateway, Operator
```

## Deploy on K8S

Please check [sample kubernetes](/kubernetes) files.

## LICENSE

It is licensed under [AGPL v3.0 OR LATER](LICENSE).

[Yew]: https://github.com/yewstack/yew
