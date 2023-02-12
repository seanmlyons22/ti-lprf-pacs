# PACs for Texas Instruments Low Power RF microcontrollers

![CI](https://github.com/seanmlyons22/ti-lprf-pacs/actions/workflows/ci.yml/badge.svg)

This repository contains Peripheral Access Crates (PACs) for TI's Low Power RF
(LPRF) series of Cortex-M microcontrollers.

All these crates are automatically generated using [svd2rust] and [tixml2svd].

## Crates

Every device family within the LPRF chip series has its own PAC.
A device family is how TI groups similar devices that share the same peripherals.
In this naming scheme `x` is a wildcard. i.e. a CC2652 and CC1312 both belong
to the cc13x2_cc26x2 family.

To prevent lots of repetition in the crates, there is one PAC per device family,
not per device.

The PACs are listed below:

| PAC                 | Supported Devices                                                                      | TI SDK Link                                                                        | target                       |
|---------------------|----------------------------------------------------------------------------------------|------------------------------------------------------------------------------------|------------------------------|
| `cc13x0`            | CC1310, CC1350                                                                         | https://www.ti.com/tool/download/SIMPLELINK-CC13X0-SDK                             | `thumbv7m-none-eabi`         |
| `cc13x1_cc26x1`     | CC1311P3, CC1311R3, CC2651P3, CC2651R3, CC2651R3SIPA                                   | https://github.com/TexasInstruments/cc13xx_cc26xx_sdk                              | `thumbv7em-none-eabi`        |
| `cc13x2_cc26x2`     | CC2642R, CC2652R, CC2652RB, CC1312R, CC1352R, CC1352P, CC2652P, CC2652PSIP, CC2652RSIP | https://github.com/TexasInstruments/cc13xx_cc26xx_sdk                              | `thumbv7em-none-eabihf`      |
| `cc13x2x7_cc26x2x7` | CC1312R7, CC1352P7, CC2652P7, CC2652R7                                                 | https://github.com/TexasInstruments/cc13xx_cc26xx_sdk                              | `thumbv7em-none-eabihf`      |
| `cc13x4_cc26x4`     | CC1314R, CC1354R, CC1354P, CC2674R, CC2674P                                            | https://github.com/TexasInstruments/cc13xx_cc26xx_sdk                              | `thumbv8m.main-none-eabihf`  |
| `cc2640r2`          | CC2640R2                                                                               | https://www.ti.com/tool/download/SIMPLELINK-CC2640R2-SDK                           | `thumbv7m-none-eabi`         |
| `cc26x0`            | CC2640, CC2650                                                                         | http://downloads.ti.com/dsps/dsps_public_sw/sdo_sb/targetcontent/tirtos/index.html | `thumbv7m-none-eabi`         |

## How it works

This repo tries to automate as much as possible using [tixml2svd] and [svd2rust].
This section seeks to describe how this repo is built.

To see how the code is organized, try running the command below

```
$ tree -L 2
```
This should give an output similar to below:

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── input
│   ├── Modules
│   └── devices
├── pacs
│   ├── cc13x0
│   ├── cc13x1_cc26x1
│   ├── cc13x2_cc26x2
│   ├── cc13x2x7_cc26x2x7
│   ├── cc13x4_cc26x4
│   ├── cc2640r2
│   └── cc26x0
├── svds
│   ├── cc13x0.svd
│   ├── cc13x1_cc26x1.svd
│   ├── cc13x2_cc26x2.svd
│   ├── cc13x2x7_cc26x2x7.svd
│   ├── cc13x4_cc26x4.svd
│   ├── cc2640r2.svd
│   └── cc26x0.svd
└── tools
    └── generate_pacs.py

14 directories, 11 files
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

The TI XML source doesn't contain interrupt defintions. So there are also hand
maintained rust files that contain the needed code to define the interrupt
vector table in Rust. These files are used to patch the generated
`lib.rs` file. Ideally the SVD files would also be patched, but this is a bit
more complex, so we use a simpler solution, for now.

### Adding or Updating Devices

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
1. Run `python3 tools/generate_pacs.py -osvds svds/ -opacs pacs/ input/devices`
   from the root of this repo.
   - If on Windows `python3` may be called `python`
1. Update `device.x` based on the vector table you created in the last step

[svd2rust]: https://github.com/rust-embedded/svd2rust
[tixml2svd]: https://github.com/dhoove/tixml2svd
