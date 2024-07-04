# Rust embedded example for STM32 H743 microcontroller

Instructions

Probably need to install first
```
rustup target add thumbv7em-none-eabihf
```

```
cargo build --release

```

chack compiled binary file size. It shouil be quite large, eg.
```
ll target/thumbv7em-none-eabihf/release/app
-rwxrwxr-x 2 knoppix knoppix 1107216 Jul  4 22:25 target/thumbv7em-none-eabihf/release/app*
```
