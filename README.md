# JSONPrima

RFC 8259 compliant JSON validator in Rust.

Documentation:
  - [API reference (docs.rs)](https://docs.rs/jsonprima)

## Code Status
[![Build Status](https://travis-ci.org/jsonprima/jsonprima.svg?branch=master)](https://travis-ci.org/jsonprima/jsonprima) [![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

## Library

This package can be used as library in Rust projects. See the [API reference (docs.rs)](https://docs.rs/jsonprima) for more info.

## CLI
You can grab the [latest release](https://github.com/jsonprima/jsonprima/releases/latest) of the binary on GitHub.

Pass the JSON document to validate as argument using  the `-i` option.

```bash
$ jsonprima -i "[true, false]"
[]
```

The returned value is an JSON array with the returned errors as described bellow:

```ts
interface Error {
  code: string,
  description: string,
  index_start: number,
  index_end: number
}
```

In the above example the JSON document is valid, so the array does not contain any errors.

Here is an example of a wrong JSON document:

```bash
$ jsonprima -i "trua"
"[{\"code\": \"E106\", \"description\": \"Invalid literal.\", \"index_end\": 2, \"index_start\": 1}]"
```

**Note:** This is a non-tolerant parser, expect that there will be at most one error in the returned array.

## License
JSONPrima is primarily distributed under the terms of the MIT license.

See [LICENSE.md](LICENSE.md) for details.
