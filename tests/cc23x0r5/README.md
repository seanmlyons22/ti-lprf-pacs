# PAC Blinky

This an example binary for the cc23x0r5 PAC. It blinks the LEDs on the
LaunchPad.
This is done to ensure that the PAC is working correctly.
It is based on the [cortex-m template](https://github.com/rust-embedded/cortex-m-quickstart).

This test is not meant to be extensive, but is a good benchmark to see if the
PAC is functioning correctly because it checks:

1. Interrupt Vectors are provided
1. IOC can be configured

## Debug

Probe-rs is needed for flashing the device and also running the tests
Install it using

```bash
cargo install probe-rs-tools@0.28.0 --locked
```

## Testing

There are a couple of tests written using
[Embedded-test](https://crates.io/crates/embedded-test).

This means that `cargo test` will execute these tests.
Testing is kept limited to only check that the PAC is working.
