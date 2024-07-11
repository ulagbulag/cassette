# 📼 Cassette

![Cassette logo](/assets/images/icons/logo.webp)

This is a Cloud-native Template-based dynamic declarative web UI framework built with [Yew].

## Tutorial

TBD

## Usage

TBD

## Building

### Dependencies

- node (npm)
- rustup

### Install dependencies

```bash
cargo install just  # for Justfile
```

### Build

```bash
just build
```

### Run a local server

```bash
just run  # or, just type "just"
```

### Test

```bash
just test
```

## Building Container Images

### Build

```bash
just oci-build
```

### Deploy

```bash
just oci-push
```

## LICENSE

It is licensed under [AGPL v3.0 OR LATER](LICENSE).

[Yew]: https://github.com/yewstack/yew
