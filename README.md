# :snake: rsnake

![rsnake](example.png)

A Snake clone in Rust with an autopilot.

## :hammer_and_wrench: Setup

Just call `cargo`, (you can get it at, eg., [rustup.rs](https://rustup.rs/)):

```bash
cargo run --release
```

## :whale: Docker

If you are running Linux and have an X server installed (if you do not know what
this means, it is probably true; XWayland does also work), you can also use the provided docker container:

```bash
docker-compose build
# this is needed to allow access to your X-server from within the Docker container
xhost +local:
docker-compose up
```
