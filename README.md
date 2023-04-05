# Strand
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Strand is a minimal flexible proxy server which exposes the ability to piggy back the requests and proxy them with another hop.

## Tech
- [tokio](https://tokio.rs/)
- [tower](https://docs.rs/tokio-tower/latest/tokio_tower/)
- [hyper](https://hyper.rs/)
- [reqwest](https://docs.rs/reqwest/latest/reqwest/)

## Running & Installation
```
$ cargo run
$ curl --proxy http://127.0.0.1:3000 <TARGET_URL>
```
