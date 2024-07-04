# Rust embedded example for STM32 H743 microcontroller

Instructions

Probably need to install first
```
rustup target add thumbv7em-none-eabihf
```

```
cargo build --release

```

check compiled binary file size. It should be quite large, e.g.
```
ll target/thumbv7em-none-eabihf/release/app
-rwxrwxr-x 2 knoppix knoppix 1107216 Jul  4 22:25 target/thumbv7em-none-eabihf/release/app*
```

in one console window open openocd
```
openocd -f openocd.cfg
```
in second window upload code into microcontroller
```
arm-none-eabi-gdb -x openocd.gdb
```
