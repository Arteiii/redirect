# redirect

This repository contains a simple Rust-based web server application
that handles HTTP redirects based on configurations provided in `redirect.conf`.  
It is designed to be run inside a Docker container for easy deployment and management.

syntax `redirect.conf` file:

```toml
[redirects]
repo = "https://github.com/arteiii/redirect"
license = "https://github.com/arteiii/redirect?tab=License-1-ov-file#readme"
```


## Podman / Docker

build the container using:

```shell
podman compose build
```

and start using:

```shell
podman compose up
```

> [!NOTE]
> I prefer podman, but it should work with docker as well

## Cargo

```shell
cargo run --bin redirect --release
```

the redirect.conf needs to be available at the root of your project directory

## Logging / Tracing

tracing level can be set using `RUST_LOG` environment  
[Read More](https://docs.rs/env_logger/latest/env_logger/#enabling-logging)

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE-MIT) file for details.