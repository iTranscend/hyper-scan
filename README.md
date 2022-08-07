# hyper-scan

A multi-threaded port scanner


## Demo


## Installation

- Install [Rust](https://rustup.rs/) (Installs `cargo`)
- Install hyper-scan via cargo
  ```bash
    cargo install hyper-scan
  ```

## Usage
```bash
hs [host] [ports...] [options...]
```

See full help information with the `--help` flag.

## Examples

- Scan all ports on your localhost `127.0.0.1`:
  ```console
  hs
  ```

- Scan all open ports on `128.2.3.5`:
  ```console
  hs 128.2.3.5
  ```

- Scan all ports on `128.2.3.5` between `200` and `50300`:
  ```console
  hs 128.2.3.5 -s 200 -e 50300
  ```

- Scan all ports on `128.2.3.5` between `200` and `50300` using `8` threads:
  ```console
  hs 128.2.3.5 -s 200 -e 50300 -j 8
  ```


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as below, without any additional terms or conditions.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
