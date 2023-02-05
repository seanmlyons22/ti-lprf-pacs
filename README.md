<div align="center">

<picture>
  <source media="(prefers-color-scheme: dark)" srcset="https://www.ti.com/content/dam/ticom/images/identities/ti-brand/ti-logo-hz-1c-white.svg" width="300">
  <img alt="Texas Instruments Logo" src="https://www.ti.com/content/dam/ticom/images/identities/ti-brand/ti-hz-2c-pos-rgb.svg" width="300">
</picture>

# <TI SimpleLink PACs>
This repository contains TI's `TI SimpleLink PACs` Software Development Kit (SDK).

[Summary](#summary) | [Setup Instructions](#setup-instructions) | [Build Instructions](#build-instructions) | [Supported Devices](#supported-devices) | [Licensing](#licensing) | [Contributions](#contributions) | [How It Works](#how-it-works) | [Developer Resources](#developer-resources)
</div>

## Summary

This repository contains Peripheral Access Crates (PACs) for TI's SimpleLink
series of microcontrollers. These provide a register access layer to the device
for use with the Rust programming lanuage.

All these crates are automatically generated using [svd2rust] and [tixml2svd].

## Supported Devices

Every device family within the chip series has its own PAC.
A device family is how TI groups similar devices that share the same peripherals.
In this naming scheme `x` is a wildcard.

To prevent lots of repetition in the crates, there is one PAC per device family,
not per device.

The PACs are listed below:

| PAC                 | Supported Devices                                                                      | TI SDK Link                                                                        | target                       |
|---------------------|----------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|------------------------------|
| `cc23x0r5`          | CC2340R5                                                                               | https://github.com/TexasInstruments/simplelink-lowpower-f3-sdk                     | `thumbv6m-none-eabi`         |

## Setup Instructions

### Installing Dependencies

In order to generate the PACs you will need some dependencies to be installed
and on your PATH. Install them by running the following:

```bash
cargo xtask install-deps
```

This will install tixml2svd, svd2rust, and form which are used to generate the
PACS.

### Updating PACs

Note: This section is only needed if you want to regenerate the PAC and SVD file.
This should not be necessary unless you are pulling new metadata from CCS or
trying to add a new device PAC.

To update the source files, copy the `Modules` folder from
`<INSTALL_LOC>/ti/ccs<CCS_VER>/ccs/ccs_base/common/targetdb/`
into `input/Modules` folder within this repository. `<INSTALL_LOC>` is the
location you installed CCS. `<CCS_VER>` is the version of CCS you installed
without any `.` or other delimiter so 12.2.0 becomes 1220.

1. Copy the files from CCS as described above
1. If adding a new device family, create a device family XML descriptor in
   `input/devices/`.
    - Tip: These can be copied from CCS
      `<INSTALL_LOC>/ti/ccs<CCS_VER>/ccs/ccs_base/common/targetdb/devices`.
    - Pick one device from your wanted family and copy it into the tree and
      rename it
1. If adding a new device family, create `device_family_ints.rs` where swapping
   `device_family` with the device family name.
   - For a hint, use the C file created by TI. It can be found in
     `<TI_SDK>/source/ti/devices/<device_family>/inc/hw_ints.h`.
1. Run `cargo xtask generate`
1. Update `device.x` based on the vector table you created in the last step

## Build Instructions

To build the pac itself:

```bash
cd pacs/cc23x0r5
cargo build --target thumbv6m-none-eabi
```

To build and run the blinky example:

```bash
cd tests/cc23x0r5
cargo build
cargo embed
```

To run the tests:

```bash
cd tests/cc23x0r5
cargo test
```

## Licensing

See the [License](LICENSE.txt).

## Contributions

This repository is not currently accepting community contributions.

## How it works

This repo tries to automate as much as possible using [tixml2svd] and [svd2rust].
This section seeks to describe how this repo is built.

To see how the code is organized, try running the command below

```
$ tree -L 2
```
This should give an output similar to below:

```txt
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE.txt
├── README.md
├── input
│   ├── Modules
│   └── devices
├── pacs
│   └── cc23x0r5
├── svds
│   └── cc23x0r5.svd
├── target
│   ├── CACHEDIR.TAG
│   ├── debug
│   └── tmp
├── tests
│   └── cc23x0r5
├── tools
│   └── generate_pacs.py
└── xtask
    ├── Cargo.toml
    └── src
```

The creation of pacs from the input XML files is automated using
`tools/generate_pacs.py`.

### Input files

The `input` directory contains the device definition files from TI. They can be
copied from a
[Code Composer Studio](https://www.ti.com/tool/CCSTUDIO#downloads) (CCS)
installation.

The files within the `Modules` folder define the peripheral set of the devices
in a TI XML format.

These files are used to generate `svd` files using [tixml2svd].

The XML files within `input/devices` are hand maintained. They are based upon
device families.  One XML file within `input/devices` will produce one PAC.

### Patches

The TI XML source doesn't contain interrupt definitions. So there are also hand
maintained rust files that contain the needed code to define the interrupt
vector table in Rust. These files are used to patch the generated
`lib.rs` file. Ideally the SVD files would also be patched, but this is a bit
more complex, so we use a simpler solution, for now.

This is also true for CCFG, HAPI (ROM API). So these files are hand maintained
in the `input/devices` folder.

---

## Developer Resources

[TI E2E™ design support forums](https://e2e.ti.com) | [Learn about software development at TI](https://www.ti.com/design-development/software-development.html) | [Training Academies](https://www.ti.com/design-development/ti-developer-zone.html#ti-developer-zone-tab-1) | [TI Developer Zone](https://dev.ti.com/)

[svd2rust]: https://github.com/rust-embedded/svd2rust
[tixml2svd]: https://github.com/dhoove/tixml2svd