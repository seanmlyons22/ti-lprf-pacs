# PAC Test

This an example binary for the cc13x2_cc26x2 PAC. It blinks the LEDs on the LaunchPad. This is done to ensure that the PAC is working correctly. It is based on the [cortex-m template](https://github.com/rust-embedded/cortex-m-quickstart).

This test is not meant to be extensive, but is a good benchmark to see if the PAC is functioning correctly because it checks:

1. Interrupt Vectors are provided
1. IOC can be configured

## Debug

The LPRF series of microcontrollers isn't supported by probe-rs.
Instead oopenocd and the Coretex Debug plugin is used. See the `.vscode` folder for more details.
