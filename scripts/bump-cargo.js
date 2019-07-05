const fs = require('fs')
const path = require('path')
const toml = require('@iarna/toml')

/**
 * Read version key from package.json
 */
const version = JSON.parse(
  fs.readFileSync(
    path.join(__dirname, '..', 'package.json'),
    'utf-8'
  )
).version;

/**
 * Parse Cargo.toml
 */
const cargo = toml.parse(
  fs.readFileSync(
    path.join(__dirname, '..', 'Cargo.toml'),
    'utf-8'
  )
)

/**
 * Copy the new bumped version of the package from
 * package.json to Cargo.toml
 */
cargo.package.version = version

/**
 * Write the changes to Cargo.toml
 */
fs.writeFileSync(
  path.join(__dirname, '..', 'Cargo.toml'),
  toml.stringify(cargo)
)
