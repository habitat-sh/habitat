# libarchive-rust

[![Build Status](https://travis-ci.org/reset/libarchive-rust.svg?branch=master)](https://travis-ci.org/reset/libarchive-rust)
[![crates.io](https://meritbadge.herokuapp.com/gpgme)](https://crates.io/crates/libarchive)

A Rust crate for interacting with archives using [libarchive](http://www.libarchive.org)

[Documentation](http://reset.github.io/libarchive-rust)

## Requirements

Version 3 of libarchive is required to use this library.

The required libraries and binaries can be installed by running:

#### Debian / Ubuntu
```shell
$ sudo apt-get install libarchive13
```

#### Mac OS X
```shell
$ brew install libarchive
```

## Usage

Put this in your `Cargo.toml`:

```toml
[dependencies]
libarchive = "*"
```

And this in your crate root:

```rust
extern crate libarchive;
```
