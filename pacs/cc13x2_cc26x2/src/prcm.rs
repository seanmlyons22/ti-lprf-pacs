#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Infrastructure Clock Division Factor For Run Mode"]
    pub infrclkdivr: INFRCLKDIVR,
    #[doc = "0x04 - Infrastructure Clock Division Factor For Sleep Mode"]
    pub infrclkdivs: INFRCLKDIVS,
    #[doc = "0x08 - Infrastructure Clock Division Factor For DeepSleep Mode"]
    pub infrclkdivds: INFRCLKDIVDS,
    #[doc = "0x0c - MCU Voltage Domain Control"]
    pub vdctl: VDCTL,
    _reserved4: [u8; 0x18],
    #[doc = "0x28 - Load PRCM Settings To CLKCTRL Power Domain"]
    pub clkloadctl: CLKLOADCTL,
    #[doc = "0x2c - RFC Clock Gate"]
    pub rfcclkg: RFCCLKG,
    #[doc = "0x30 - VIMS Clock Gate"]
    pub vimsclkg: VIMSCLKG,
    _reserved7: [u8; 0x08],
    #[doc = "0x3c - SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
    pub secdmaclkgr: SECDMACLKGR,
    #[doc = "0x40 - SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
    pub secdmaclkgs: SECDMACLKGS,
    #[doc = "0x44 - SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
    pub secdmaclkgds: SECDMACLKGDS,
    #[doc = "0x48 - GPIO Clock Gate For Run And All Modes"]
    pub gpioclkgr: GPIOCLKGR,
    #[doc = "0x4c - GPIO Clock Gate For Sleep Mode"]
    pub gpioclkgs: GPIOCLKGS,
    #[doc = "0x50 - GPIO Clock Gate For Deep Sleep Mode"]
    pub gpioclkgds: GPIOCLKGDS,
    #[doc = "0x54 - GPT Clock Gate For Run And All Modes"]
    pub gptclkgr: GPTCLKGR,
    #[doc = "0x58 - GPT Clock Gate For Sleep Mode"]
    pub gptclkgs: GPTCLKGS,
    #[doc = "0x5c - GPT Clock Gate For Deep Sleep Mode"]
    pub gptclkgds: GPTCLKGDS,
    #[doc = "0x60 - I2C Clock Gate For Run And All Modes"]
    pub i2cclkgr: I2CCLKGR,
    #[doc = "0x64 - I2C Clock Gate For Sleep Mode"]
    pub i2cclkgs: I2CCLKGS,
    #[doc = "0x68 - I2C Clock Gate For Deep Sleep Mode"]
    pub i2cclkgds: I2CCLKGDS,
    #[doc = "0x6c - UART Clock Gate For Run And All Modes"]
    pub uartclkgr: UARTCLKGR,
    #[doc = "0x70 - UART Clock Gate For Sleep Mode"]
    pub uartclkgs: UARTCLKGS,
    #[doc = "0x74 - UART Clock Gate For Deep Sleep Mode"]
    pub uartclkgds: UARTCLKGDS,
    #[doc = "0x78 - SSI Clock Gate For Run And All Modes"]
    pub ssiclkgr: SSICLKGR,
    #[doc = "0x7c - SSI Clock Gate For Sleep Mode"]
    pub ssiclkgs: SSICLKGS,
    #[doc = "0x80 - SSI Clock Gate For Deep Sleep Mode"]
    pub ssiclkgds: SSICLKGDS,
    #[doc = "0x84 - I2S Clock Gate For Run And All Modes"]
    pub i2sclkgr: I2SCLKGR,
    #[doc = "0x88 - I2S Clock Gate For Sleep Mode"]
    pub i2sclkgs: I2SCLKGS,
    #[doc = "0x8c - I2S Clock Gate For Deep Sleep Mode"]
    pub i2sclkgds: I2SCLKGDS,
    _reserved28: [u8; 0x24],
    #[doc = "0xb4 - Internal. Only to be used through TI provided API."]
    pub sysbusclkdiv: SYSBUSCLKDIV,
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    pub cpuclkdiv: CPUCLKDIV,
    #[doc = "0xbc - Internal. Only to be used through TI provided API."]
    pub perbuscpuclkdiv: PERBUSCPUCLKDIV,
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    pub perbusdmaclkdiv: PERBUSDMACLKDIV,
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    pub perdmaclkdiv: PERDMACLKDIV,
    #[doc = "0xc8 - I2S Clock Control"]
    pub i2sbclksel: I2SBCLKSEL,
    #[doc = "0xcc - GPT Scalar"]
    pub gptclkdiv: GPTCLKDIV,
    #[doc = "0xd0 - I2S Clock Control"]
    pub i2sclkctl: I2SCLKCTL,
    #[doc = "0xd4 - MCLK Division Ratio"]
    pub i2smclkdiv: I2SMCLKDIV,
    #[doc = "0xd8 - BCLK Division Ratio"]
    pub i2sbclkdiv: I2SBCLKDIV,
    #[doc = "0xdc - WCLK Division Ratio"]
    pub i2swclkdiv: I2SWCLKDIV,
    _reserved39: [u8; 0x10],
    #[doc = "0xf0 - RESET For SEC (PKA And TRNG And CRYPTO) And UDMA"]
    pub resetsecdma: RESETSECDMA,
    #[doc = "0xf4 - RESET For GPIO IPs"]
    pub resetgpio: RESETGPIO,
    #[doc = "0xf8 - RESET For GPT Ips"]
    pub resetgpt: RESETGPT,
    #[doc = "0xfc - RESET For I2C IPs"]
    pub reseti2c: RESETI2C,
    #[doc = "0x100 - RESET For UART IPs"]
    pub resetuart: RESETUART,
    #[doc = "0x104 - RESET For SSI IPs"]
    pub resetssi: RESETSSI,
    #[doc = "0x108 - RESET For I2S IP"]
    pub reseti2s: RESETI2S,
    _reserved46: [u8; 0x20],
    #[doc = "0x12c - Power Domain Control"]
    pub pdctl0: PDCTL0,
    #[doc = "0x130 - RFC Power Domain Control"]
    pub pdctl0rfc: PDCTL0RFC,
    #[doc = "0x134 - SERIAL Power Domain Control"]
    pub pdctl0serial: PDCTL0SERIAL,
    #[doc = "0x138 - PERIPH Power Domain Control"]
    pub pdctl0periph: PDCTL0PERIPH,
    _reserved50: [u8; 0x04],
    #[doc = "0x140 - Power Domain Status"]
    pub pdstat0: PDSTAT0,
    #[doc = "0x144 - RFC Power Domain Status"]
    pub pdstat0rfc: PDSTAT0RFC,
    #[doc = "0x148 - SERIAL Power Domain Status"]
    pub pdstat0serial: PDSTAT0SERIAL,
    #[doc = "0x14c - PERIPH Power Domain Status"]
    pub pdstat0periph: PDSTAT0PERIPH,
    _reserved54: [u8; 0x2c],
    #[doc = "0x17c - Power Domain Control"]
    pub pdctl1: PDCTL1,
    _reserved55: [u8; 0x04],
    #[doc = "0x184 - CPU Power Domain Direct Control"]
    pub pdctl1cpu: PDCTL1CPU,
    #[doc = "0x188 - RFC Power Domain Direct Control"]
    pub pdctl1rfc: PDCTL1RFC,
    #[doc = "0x18c - VIMS Mode Direct Control"]
    pub pdctl1vims: PDCTL1VIMS,
    _reserved58: [u8; 0x04],
    #[doc = "0x194 - Power Manager Status"]
    pub pdstat1: PDSTAT1,
    #[doc = "0x198 - BUS Power Domain Direct Read Status"]
    pub pdstat1bus: PDSTAT1BUS,
    #[doc = "0x19c - RFC Power Domain Direct Read Status"]
    pub pdstat1rfc: PDSTAT1RFC,
    #[doc = "0x1a0 - CPU Power Domain Direct Read Status"]
    pub pdstat1cpu: PDSTAT1CPU,
    #[doc = "0x1a4 - VIMS Mode Direct Read Status"]
    pub pdstat1vims: PDSTAT1VIMS,
    _reserved63: [u8; 0x24],
    #[doc = "0x1cc - Control To RFC"]
    pub rfcbits: RFCBITS,
    #[doc = "0x1d0 - Selected RFC Mode"]
    pub rfcmodesel: RFCMODESEL,
    #[doc = "0x1d4 - Allowed RFC Modes"]
    pub rfcmodehwopt: RFCMODEHWOPT,
    _reserved66: [u8; 0x08],
    #[doc = "0x1e0 - Power Profiler Register"]
    pub pwrprofstat: PWRPROFSTAT,
    _reserved67: [u8; 0x38],
    #[doc = "0x21c - MCU SRAM configuration"]
    pub mcusramcfg: MCUSRAMCFG,
    _reserved68: [u8; 0x04],
    #[doc = "0x224 - Memory Retention Control"]
    pub ramreten: RAMRETEN,
    _reserved69: [u8; 0x68],
    #[doc = "0x290 - Oscillator Interrupt Mask Control"]
    pub oscimsc: OSCIMSC,
    #[doc = "0x294 - Oscillator Raw Interrupt Status"]
    pub oscris: OSCRIS,
    #[doc = "0x298 - Oscillator Raw Interrupt Clear"]
    pub oscicr: OSCICR,
}
#[doc = "INFRCLKDIVR (rw) register accessor: an alias for `Reg<INFRCLKDIVR_SPEC>`"]
pub type INFRCLKDIVR = crate::Reg<infrclkdivr::INFRCLKDIVR_SPEC>;
#[doc = "Infrastructure Clock Division Factor For Run Mode"]
pub mod infrclkdivr;
#[doc = "INFRCLKDIVS (rw) register accessor: an alias for `Reg<INFRCLKDIVS_SPEC>`"]
pub type INFRCLKDIVS = crate::Reg<infrclkdivs::INFRCLKDIVS_SPEC>;
#[doc = "Infrastructure Clock Division Factor For Sleep Mode"]
pub mod infrclkdivs;
#[doc = "INFRCLKDIVDS (rw) register accessor: an alias for `Reg<INFRCLKDIVDS_SPEC>`"]
pub type INFRCLKDIVDS = crate::Reg<infrclkdivds::INFRCLKDIVDS_SPEC>;
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode"]
pub mod infrclkdivds;
#[doc = "VDCTL (rw) register accessor: an alias for `Reg<VDCTL_SPEC>`"]
pub type VDCTL = crate::Reg<vdctl::VDCTL_SPEC>;
#[doc = "MCU Voltage Domain Control"]
pub mod vdctl;
#[doc = "CLKLOADCTL (rw) register accessor: an alias for `Reg<CLKLOADCTL_SPEC>`"]
pub type CLKLOADCTL = crate::Reg<clkloadctl::CLKLOADCTL_SPEC>;
#[doc = "Load PRCM Settings To CLKCTRL Power Domain"]
pub mod clkloadctl;
#[doc = "RFCCLKG (rw) register accessor: an alias for `Reg<RFCCLKG_SPEC>`"]
pub type RFCCLKG = crate::Reg<rfcclkg::RFCCLKG_SPEC>;
#[doc = "RFC Clock Gate"]
pub mod rfcclkg;
#[doc = "VIMSCLKG (rw) register accessor: an alias for `Reg<VIMSCLKG_SPEC>`"]
pub type VIMSCLKG = crate::Reg<vimsclkg::VIMSCLKG_SPEC>;
#[doc = "VIMS Clock Gate"]
pub mod vimsclkg;
#[doc = "SECDMACLKGR (rw) register accessor: an alias for `Reg<SECDMACLKGR_SPEC>`"]
pub type SECDMACLKGR = crate::Reg<secdmaclkgr::SECDMACLKGR_SPEC>;
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
pub mod secdmaclkgr;
#[doc = "SECDMACLKGS (rw) register accessor: an alias for `Reg<SECDMACLKGS_SPEC>`"]
pub type SECDMACLKGS = crate::Reg<secdmaclkgs::SECDMACLKGS_SPEC>;
#[doc = "SEC (PKA And TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
pub mod secdmaclkgs;
#[doc = "SECDMACLKGDS (rw) register accessor: an alias for `Reg<SECDMACLKGDS_SPEC>`"]
pub type SECDMACLKGDS = crate::Reg<secdmaclkgds::SECDMACLKGDS_SPEC>;
#[doc = "SEC (PKA And TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
pub mod secdmaclkgds;
#[doc = "GPIOCLKGR (rw) register accessor: an alias for `Reg<GPIOCLKGR_SPEC>`"]
pub type GPIOCLKGR = crate::Reg<gpioclkgr::GPIOCLKGR_SPEC>;
#[doc = "GPIO Clock Gate For Run And All Modes"]
pub mod gpioclkgr;
#[doc = "GPIOCLKGS (rw) register accessor: an alias for `Reg<GPIOCLKGS_SPEC>`"]
pub type GPIOCLKGS = crate::Reg<gpioclkgs::GPIOCLKGS_SPEC>;
#[doc = "GPIO Clock Gate For Sleep Mode"]
pub mod gpioclkgs;
#[doc = "GPIOCLKGDS (rw) register accessor: an alias for `Reg<GPIOCLKGDS_SPEC>`"]
pub type GPIOCLKGDS = crate::Reg<gpioclkgds::GPIOCLKGDS_SPEC>;
#[doc = "GPIO Clock Gate For Deep Sleep Mode"]
pub mod gpioclkgds;
#[doc = "GPTCLKGR (rw) register accessor: an alias for `Reg<GPTCLKGR_SPEC>`"]
pub type GPTCLKGR = crate::Reg<gptclkgr::GPTCLKGR_SPEC>;
#[doc = "GPT Clock Gate For Run And All Modes"]
pub mod gptclkgr;
#[doc = "GPTCLKGS (rw) register accessor: an alias for `Reg<GPTCLKGS_SPEC>`"]
pub type GPTCLKGS = crate::Reg<gptclkgs::GPTCLKGS_SPEC>;
#[doc = "GPT Clock Gate For Sleep Mode"]
pub mod gptclkgs;
#[doc = "GPTCLKGDS (rw) register accessor: an alias for `Reg<GPTCLKGDS_SPEC>`"]
pub type GPTCLKGDS = crate::Reg<gptclkgds::GPTCLKGDS_SPEC>;
#[doc = "GPT Clock Gate For Deep Sleep Mode"]
pub mod gptclkgds;
#[doc = "I2CCLKGR (rw) register accessor: an alias for `Reg<I2CCLKGR_SPEC>`"]
pub type I2CCLKGR = crate::Reg<i2cclkgr::I2CCLKGR_SPEC>;
#[doc = "I2C Clock Gate For Run And All Modes"]
pub mod i2cclkgr;
#[doc = "I2CCLKGS (rw) register accessor: an alias for `Reg<I2CCLKGS_SPEC>`"]
pub type I2CCLKGS = crate::Reg<i2cclkgs::I2CCLKGS_SPEC>;
#[doc = "I2C Clock Gate For Sleep Mode"]
pub mod i2cclkgs;
#[doc = "I2CCLKGDS (rw) register accessor: an alias for `Reg<I2CCLKGDS_SPEC>`"]
pub type I2CCLKGDS = crate::Reg<i2cclkgds::I2CCLKGDS_SPEC>;
#[doc = "I2C Clock Gate For Deep Sleep Mode"]
pub mod i2cclkgds;
#[doc = "UARTCLKGR (rw) register accessor: an alias for `Reg<UARTCLKGR_SPEC>`"]
pub type UARTCLKGR = crate::Reg<uartclkgr::UARTCLKGR_SPEC>;
#[doc = "UART Clock Gate For Run And All Modes"]
pub mod uartclkgr;
#[doc = "UARTCLKGS (rw) register accessor: an alias for `Reg<UARTCLKGS_SPEC>`"]
pub type UARTCLKGS = crate::Reg<uartclkgs::UARTCLKGS_SPEC>;
#[doc = "UART Clock Gate For Sleep Mode"]
pub mod uartclkgs;
#[doc = "UARTCLKGDS (rw) register accessor: an alias for `Reg<UARTCLKGDS_SPEC>`"]
pub type UARTCLKGDS = crate::Reg<uartclkgds::UARTCLKGDS_SPEC>;
#[doc = "UART Clock Gate For Deep Sleep Mode"]
pub mod uartclkgds;
#[doc = "SSICLKGR (rw) register accessor: an alias for `Reg<SSICLKGR_SPEC>`"]
pub type SSICLKGR = crate::Reg<ssiclkgr::SSICLKGR_SPEC>;
#[doc = "SSI Clock Gate For Run And All Modes"]
pub mod ssiclkgr;
#[doc = "SSICLKGS (rw) register accessor: an alias for `Reg<SSICLKGS_SPEC>`"]
pub type SSICLKGS = crate::Reg<ssiclkgs::SSICLKGS_SPEC>;
#[doc = "SSI Clock Gate For Sleep Mode"]
pub mod ssiclkgs;
#[doc = "SSICLKGDS (rw) register accessor: an alias for `Reg<SSICLKGDS_SPEC>`"]
pub type SSICLKGDS = crate::Reg<ssiclkgds::SSICLKGDS_SPEC>;
#[doc = "SSI Clock Gate For Deep Sleep Mode"]
pub mod ssiclkgds;
#[doc = "I2SCLKGR (rw) register accessor: an alias for `Reg<I2SCLKGR_SPEC>`"]
pub type I2SCLKGR = crate::Reg<i2sclkgr::I2SCLKGR_SPEC>;
#[doc = "I2S Clock Gate For Run And All Modes"]
pub mod i2sclkgr;
#[doc = "I2SCLKGS (rw) register accessor: an alias for `Reg<I2SCLKGS_SPEC>`"]
pub type I2SCLKGS = crate::Reg<i2sclkgs::I2SCLKGS_SPEC>;
#[doc = "I2S Clock Gate For Sleep Mode"]
pub mod i2sclkgs;
#[doc = "I2SCLKGDS (rw) register accessor: an alias for `Reg<I2SCLKGDS_SPEC>`"]
pub type I2SCLKGDS = crate::Reg<i2sclkgds::I2SCLKGDS_SPEC>;
#[doc = "I2S Clock Gate For Deep Sleep Mode"]
pub mod i2sclkgds;
#[doc = "SYSBUSCLKDIV (rw) register accessor: an alias for `Reg<SYSBUSCLKDIV_SPEC>`"]
pub type SYSBUSCLKDIV = crate::Reg<sysbusclkdiv::SYSBUSCLKDIV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sysbusclkdiv;
#[doc = "CPUCLKDIV (rw) register accessor: an alias for `Reg<CPUCLKDIV_SPEC>`"]
pub type CPUCLKDIV = crate::Reg<cpuclkdiv::CPUCLKDIV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpuclkdiv;
#[doc = "PERBUSCPUCLKDIV (rw) register accessor: an alias for `Reg<PERBUSCPUCLKDIV_SPEC>`"]
pub type PERBUSCPUCLKDIV = crate::Reg<perbuscpuclkdiv::PERBUSCPUCLKDIV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbuscpuclkdiv;
#[doc = "PERBUSDMACLKDIV (rw) register accessor: an alias for `Reg<PERBUSDMACLKDIV_SPEC>`"]
pub type PERBUSDMACLKDIV = crate::Reg<perbusdmaclkdiv::PERBUSDMACLKDIV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbusdmaclkdiv;
#[doc = "PERDMACLKDIV (rw) register accessor: an alias for `Reg<PERDMACLKDIV_SPEC>`"]
pub type PERDMACLKDIV = crate::Reg<perdmaclkdiv::PERDMACLKDIV_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perdmaclkdiv;
#[doc = "I2SBCLKSEL (rw) register accessor: an alias for `Reg<I2SBCLKSEL_SPEC>`"]
pub type I2SBCLKSEL = crate::Reg<i2sbclksel::I2SBCLKSEL_SPEC>;
#[doc = "I2S Clock Control"]
pub mod i2sbclksel;
#[doc = "GPTCLKDIV (rw) register accessor: an alias for `Reg<GPTCLKDIV_SPEC>`"]
pub type GPTCLKDIV = crate::Reg<gptclkdiv::GPTCLKDIV_SPEC>;
#[doc = "GPT Scalar"]
pub mod gptclkdiv;
#[doc = "I2SCLKCTL (rw) register accessor: an alias for `Reg<I2SCLKCTL_SPEC>`"]
pub type I2SCLKCTL = crate::Reg<i2sclkctl::I2SCLKCTL_SPEC>;
#[doc = "I2S Clock Control"]
pub mod i2sclkctl;
#[doc = "I2SMCLKDIV (rw) register accessor: an alias for `Reg<I2SMCLKDIV_SPEC>`"]
pub type I2SMCLKDIV = crate::Reg<i2smclkdiv::I2SMCLKDIV_SPEC>;
#[doc = "MCLK Division Ratio"]
pub mod i2smclkdiv;
#[doc = "I2SBCLKDIV (rw) register accessor: an alias for `Reg<I2SBCLKDIV_SPEC>`"]
pub type I2SBCLKDIV = crate::Reg<i2sbclkdiv::I2SBCLKDIV_SPEC>;
#[doc = "BCLK Division Ratio"]
pub mod i2sbclkdiv;
#[doc = "I2SWCLKDIV (rw) register accessor: an alias for `Reg<I2SWCLKDIV_SPEC>`"]
pub type I2SWCLKDIV = crate::Reg<i2swclkdiv::I2SWCLKDIV_SPEC>;
#[doc = "WCLK Division Ratio"]
pub mod i2swclkdiv;
#[doc = "RESETSECDMA (rw) register accessor: an alias for `Reg<RESETSECDMA_SPEC>`"]
pub type RESETSECDMA = crate::Reg<resetsecdma::RESETSECDMA_SPEC>;
#[doc = "RESET For SEC (PKA And TRNG And CRYPTO) And UDMA"]
pub mod resetsecdma;
#[doc = "RESETGPIO (rw) register accessor: an alias for `Reg<RESETGPIO_SPEC>`"]
pub type RESETGPIO = crate::Reg<resetgpio::RESETGPIO_SPEC>;
#[doc = "RESET For GPIO IPs"]
pub mod resetgpio;
#[doc = "RESETGPT (rw) register accessor: an alias for `Reg<RESETGPT_SPEC>`"]
pub type RESETGPT = crate::Reg<resetgpt::RESETGPT_SPEC>;
#[doc = "RESET For GPT Ips"]
pub mod resetgpt;
#[doc = "RESETI2C (rw) register accessor: an alias for `Reg<RESETI2C_SPEC>`"]
pub type RESETI2C = crate::Reg<reseti2c::RESETI2C_SPEC>;
#[doc = "RESET For I2C IPs"]
pub mod reseti2c;
#[doc = "RESETUART (rw) register accessor: an alias for `Reg<RESETUART_SPEC>`"]
pub type RESETUART = crate::Reg<resetuart::RESETUART_SPEC>;
#[doc = "RESET For UART IPs"]
pub mod resetuart;
#[doc = "RESETSSI (rw) register accessor: an alias for `Reg<RESETSSI_SPEC>`"]
pub type RESETSSI = crate::Reg<resetssi::RESETSSI_SPEC>;
#[doc = "RESET For SSI IPs"]
pub mod resetssi;
#[doc = "RESETI2S (rw) register accessor: an alias for `Reg<RESETI2S_SPEC>`"]
pub type RESETI2S = crate::Reg<reseti2s::RESETI2S_SPEC>;
#[doc = "RESET For I2S IP"]
pub mod reseti2s;
#[doc = "PDCTL0 (rw) register accessor: an alias for `Reg<PDCTL0_SPEC>`"]
pub type PDCTL0 = crate::Reg<pdctl0::PDCTL0_SPEC>;
#[doc = "Power Domain Control"]
pub mod pdctl0;
#[doc = "PDCTL0RFC (rw) register accessor: an alias for `Reg<PDCTL0RFC_SPEC>`"]
pub type PDCTL0RFC = crate::Reg<pdctl0rfc::PDCTL0RFC_SPEC>;
#[doc = "RFC Power Domain Control"]
pub mod pdctl0rfc;
#[doc = "PDCTL0SERIAL (rw) register accessor: an alias for `Reg<PDCTL0SERIAL_SPEC>`"]
pub type PDCTL0SERIAL = crate::Reg<pdctl0serial::PDCTL0SERIAL_SPEC>;
#[doc = "SERIAL Power Domain Control"]
pub mod pdctl0serial;
#[doc = "PDCTL0PERIPH (rw) register accessor: an alias for `Reg<PDCTL0PERIPH_SPEC>`"]
pub type PDCTL0PERIPH = crate::Reg<pdctl0periph::PDCTL0PERIPH_SPEC>;
#[doc = "PERIPH Power Domain Control"]
pub mod pdctl0periph;
#[doc = "PDSTAT0 (rw) register accessor: an alias for `Reg<PDSTAT0_SPEC>`"]
pub type PDSTAT0 = crate::Reg<pdstat0::PDSTAT0_SPEC>;
#[doc = "Power Domain Status"]
pub mod pdstat0;
#[doc = "PDSTAT0RFC (rw) register accessor: an alias for `Reg<PDSTAT0RFC_SPEC>`"]
pub type PDSTAT0RFC = crate::Reg<pdstat0rfc::PDSTAT0RFC_SPEC>;
#[doc = "RFC Power Domain Status"]
pub mod pdstat0rfc;
#[doc = "PDSTAT0SERIAL (rw) register accessor: an alias for `Reg<PDSTAT0SERIAL_SPEC>`"]
pub type PDSTAT0SERIAL = crate::Reg<pdstat0serial::PDSTAT0SERIAL_SPEC>;
#[doc = "SERIAL Power Domain Status"]
pub mod pdstat0serial;
#[doc = "PDSTAT0PERIPH (rw) register accessor: an alias for `Reg<PDSTAT0PERIPH_SPEC>`"]
pub type PDSTAT0PERIPH = crate::Reg<pdstat0periph::PDSTAT0PERIPH_SPEC>;
#[doc = "PERIPH Power Domain Status"]
pub mod pdstat0periph;
#[doc = "PDCTL1 (rw) register accessor: an alias for `Reg<PDCTL1_SPEC>`"]
pub type PDCTL1 = crate::Reg<pdctl1::PDCTL1_SPEC>;
#[doc = "Power Domain Control"]
pub mod pdctl1;
#[doc = "PDCTL1CPU (rw) register accessor: an alias for `Reg<PDCTL1CPU_SPEC>`"]
pub type PDCTL1CPU = crate::Reg<pdctl1cpu::PDCTL1CPU_SPEC>;
#[doc = "CPU Power Domain Direct Control"]
pub mod pdctl1cpu;
#[doc = "PDCTL1RFC (rw) register accessor: an alias for `Reg<PDCTL1RFC_SPEC>`"]
pub type PDCTL1RFC = crate::Reg<pdctl1rfc::PDCTL1RFC_SPEC>;
#[doc = "RFC Power Domain Direct Control"]
pub mod pdctl1rfc;
#[doc = "PDCTL1VIMS (rw) register accessor: an alias for `Reg<PDCTL1VIMS_SPEC>`"]
pub type PDCTL1VIMS = crate::Reg<pdctl1vims::PDCTL1VIMS_SPEC>;
#[doc = "VIMS Mode Direct Control"]
pub mod pdctl1vims;
#[doc = "PDSTAT1 (rw) register accessor: an alias for `Reg<PDSTAT1_SPEC>`"]
pub type PDSTAT1 = crate::Reg<pdstat1::PDSTAT1_SPEC>;
#[doc = "Power Manager Status"]
pub mod pdstat1;
#[doc = "PDSTAT1BUS (rw) register accessor: an alias for `Reg<PDSTAT1BUS_SPEC>`"]
pub type PDSTAT1BUS = crate::Reg<pdstat1bus::PDSTAT1BUS_SPEC>;
#[doc = "BUS Power Domain Direct Read Status"]
pub mod pdstat1bus;
#[doc = "PDSTAT1RFC (rw) register accessor: an alias for `Reg<PDSTAT1RFC_SPEC>`"]
pub type PDSTAT1RFC = crate::Reg<pdstat1rfc::PDSTAT1RFC_SPEC>;
#[doc = "RFC Power Domain Direct Read Status"]
pub mod pdstat1rfc;
#[doc = "PDSTAT1CPU (rw) register accessor: an alias for `Reg<PDSTAT1CPU_SPEC>`"]
pub type PDSTAT1CPU = crate::Reg<pdstat1cpu::PDSTAT1CPU_SPEC>;
#[doc = "CPU Power Domain Direct Read Status"]
pub mod pdstat1cpu;
#[doc = "PDSTAT1VIMS (rw) register accessor: an alias for `Reg<PDSTAT1VIMS_SPEC>`"]
pub type PDSTAT1VIMS = crate::Reg<pdstat1vims::PDSTAT1VIMS_SPEC>;
#[doc = "VIMS Mode Direct Read Status"]
pub mod pdstat1vims;
#[doc = "RFCBITS (rw) register accessor: an alias for `Reg<RFCBITS_SPEC>`"]
pub type RFCBITS = crate::Reg<rfcbits::RFCBITS_SPEC>;
#[doc = "Control To RFC"]
pub mod rfcbits;
#[doc = "RFCMODESEL (rw) register accessor: an alias for `Reg<RFCMODESEL_SPEC>`"]
pub type RFCMODESEL = crate::Reg<rfcmodesel::RFCMODESEL_SPEC>;
#[doc = "Selected RFC Mode"]
pub mod rfcmodesel;
#[doc = "RFCMODEHWOPT (rw) register accessor: an alias for `Reg<RFCMODEHWOPT_SPEC>`"]
pub type RFCMODEHWOPT = crate::Reg<rfcmodehwopt::RFCMODEHWOPT_SPEC>;
#[doc = "Allowed RFC Modes"]
pub mod rfcmodehwopt;
#[doc = "PWRPROFSTAT (rw) register accessor: an alias for `Reg<PWRPROFSTAT_SPEC>`"]
pub type PWRPROFSTAT = crate::Reg<pwrprofstat::PWRPROFSTAT_SPEC>;
#[doc = "Power Profiler Register"]
pub mod pwrprofstat;
#[doc = "MCUSRAMCFG (rw) register accessor: an alias for `Reg<MCUSRAMCFG_SPEC>`"]
pub type MCUSRAMCFG = crate::Reg<mcusramcfg::MCUSRAMCFG_SPEC>;
#[doc = "MCU SRAM configuration"]
pub mod mcusramcfg;
#[doc = "RAMRETEN (rw) register accessor: an alias for `Reg<RAMRETEN_SPEC>`"]
pub type RAMRETEN = crate::Reg<ramreten::RAMRETEN_SPEC>;
#[doc = "Memory Retention Control"]
pub mod ramreten;
#[doc = "OSCIMSC (rw) register accessor: an alias for `Reg<OSCIMSC_SPEC>`"]
pub type OSCIMSC = crate::Reg<oscimsc::OSCIMSC_SPEC>;
#[doc = "Oscillator Interrupt Mask Control"]
pub mod oscimsc;
#[doc = "OSCRIS (rw) register accessor: an alias for `Reg<OSCRIS_SPEC>`"]
pub type OSCRIS = crate::Reg<oscris::OSCRIS_SPEC>;
#[doc = "Oscillator Raw Interrupt Status"]
pub mod oscris;
#[doc = "OSCICR (rw) register accessor: an alias for `Reg<OSCICR_SPEC>`"]
pub type OSCICR = crate::Reg<oscicr::OSCICR_SPEC>;
#[doc = "Oscillator Raw Interrupt Clear"]
pub mod oscicr;
