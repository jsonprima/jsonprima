# JSONPrima

RFC 8259 compliant JSON validator in Rust.

Documentation:
  - [API reference (docs.rs)](https://docs.rs/jsonprima)

## Code Status
[![Build Status](https://travis-ci.org/jsonprima/jsonprima.svg?branch=master)](https://travis-ci.org/jsonprima/jsonprima) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

## Usage
Use this library in your project. Add this to your Cargo.toml:

```toml
[dependencies]
jsonprima = "0.7"
```

## CLI
You can grab the [latest release](https://github.com/jsonprima/jsonprima/releases/latest) of the binary.

### Use inline code
Validate inline code using  the `-t` option.

```bash
jsonprima -t "[true, false]"
```

### Use text file
Validate a file using the `-f` option.

```bash
jsonprima -f /path/to/text/file
```

## License
JSONPrima is primarily distributed under the terms of the MIT license.

See [LICENSE.md](LICENSE.md) for details.
