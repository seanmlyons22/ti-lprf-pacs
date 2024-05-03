#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    infrclkdivr: Infrclkdivr,
    infrclkdivs: Infrclkdivs,
    infrclkdivds: Infrclkdivds,
    vdctl: Vdctl,
    _reserved4: [u8; 0x18],
    clkloadctl: Clkloadctl,
    rfcclkg: Rfcclkg,
    vimsclkg: Vimsclkg,
    _reserved7: [u8; 0x08],
    secdmaclkgr: Secdmaclkgr,
    secdmaclkgs: Secdmaclkgs,
    secdmaclkgds: Secdmaclkgds,
    gpioclkgr: Gpioclkgr,
    gpioclkgs: Gpioclkgs,
    gpioclkgds: Gpioclkgds,
    gptclkgr: Gptclkgr,
    gptclkgs: Gptclkgs,
    gptclkgds: Gptclkgds,
    i2cclkgr: I2cclkgr,
    i2cclkgs: I2cclkgs,
    i2cclkgds: I2cclkgds,
    uartclkgr: Uartclkgr,
    uartclkgs: Uartclkgs,
    uartclkgds: Uartclkgds,
    ssiclkgr: Ssiclkgr,
    ssiclkgs: Ssiclkgs,
    ssiclkgds: Ssiclkgds,
    i2sclkgr: I2sclkgr,
    i2sclkgs: I2sclkgs,
    i2sclkgds: I2sclkgds,
    _reserved28: [u8; 0x24],
    sysbusclkdiv: Sysbusclkdiv,
    cpuclkdiv: Cpuclkdiv,
    perbuscpuclkdiv: Perbuscpuclkdiv,
    perbusdmaclkdiv: Perbusdmaclkdiv,
    perdmaclkdiv: Perdmaclkdiv,
    i2sbclksel: I2sbclksel,
    gptclkdiv: Gptclkdiv,
    i2sclkctl: I2sclkctl,
    i2smclkdiv: I2smclkdiv,
    i2sbclkdiv: I2sbclkdiv,
    i2swclkdiv: I2swclkdiv,
    _reserved39: [u8; 0x10],
    resetsecdma: Resetsecdma,
    resetgpio: Resetgpio,
    resetgpt: Resetgpt,
    reseti2c: Reseti2c,
    resetuart: Resetuart,
    resetssi: Resetssi,
    reseti2s: Reseti2s,
    _reserved46: [u8; 0x20],
    pdctl0: Pdctl0,
    pdctl0rfc: Pdctl0rfc,
    pdctl0serial: Pdctl0serial,
    pdctl0periph: Pdctl0periph,
    _reserved50: [u8; 0x04],
    pdstat0: Pdstat0,
    pdstat0rfc: Pdstat0rfc,
    pdstat0serial: Pdstat0serial,
    pdstat0periph: Pdstat0periph,
    _reserved54: [u8; 0x2c],
    pdctl1: Pdctl1,
    _reserved55: [u8; 0x04],
    pdctl1cpu: Pdctl1cpu,
    pdctl1rfc: Pdctl1rfc,
    pdctl1vims: Pdctl1vims,
    _reserved58: [u8; 0x04],
    pdstat1: Pdstat1,
    pdstat1bus: Pdstat1bus,
    pdstat1rfc: Pdstat1rfc,
    pdstat1cpu: Pdstat1cpu,
    pdstat1vims: Pdstat1vims,
    _reserved63: [u8; 0x24],
    rfcbits: Rfcbits,
    rfcmodesel: Rfcmodesel,
    rfcmodehwopt: Rfcmodehwopt,
    _reserved66: [u8; 0x08],
    pwrprofstat: Pwrprofstat,
    _reserved67: [u8; 0x40],
    ramreten: Ramreten,
    _reserved68: [u8; 0x68],
    oscimsc: Oscimsc,
    oscris: Oscris,
    oscicr: Oscicr,
}
impl RegisterBlock {
    #[doc = "0x00 - Infrastructure Clock Division Factor For Run Mode"]
    #[inline(always)]
    pub const fn infrclkdivr(&self) -> &Infrclkdivr {
        &self.infrclkdivr
    }
    #[doc = "0x04 - Infrastructure Clock Division Factor For Sleep Mode"]
    #[inline(always)]
    pub const fn infrclkdivs(&self) -> &Infrclkdivs {
        &self.infrclkdivs
    }
    #[doc = "0x08 - Infrastructure Clock Division Factor For DeepSleep Mode"]
    #[inline(always)]
    pub const fn infrclkdivds(&self) -> &Infrclkdivds {
        &self.infrclkdivds
    }
    #[doc = "0x0c - MCU Voltage Domain Control"]
    #[inline(always)]
    pub const fn vdctl(&self) -> &Vdctl {
        &self.vdctl
    }
    #[doc = "0x28 - Load PRCM Settings To CLKCTRL Power Domain"]
    #[inline(always)]
    pub const fn clkloadctl(&self) -> &Clkloadctl {
        &self.clkloadctl
    }
    #[doc = "0x2c - RFC Clock Gate"]
    #[inline(always)]
    pub const fn rfcclkg(&self) -> &Rfcclkg {
        &self.rfcclkg
    }
    #[doc = "0x30 - VIMS Clock Gate"]
    #[inline(always)]
    pub const fn vimsclkg(&self) -> &Vimsclkg {
        &self.vimsclkg
    }
    #[doc = "0x3c - SEC (TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
    #[inline(always)]
    pub const fn secdmaclkgr(&self) -> &Secdmaclkgr {
        &self.secdmaclkgr
    }
    #[doc = "0x40 - SEC (TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
    #[inline(always)]
    pub const fn secdmaclkgs(&self) -> &Secdmaclkgs {
        &self.secdmaclkgs
    }
    #[doc = "0x44 - SEC (TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
    #[inline(always)]
    pub const fn secdmaclkgds(&self) -> &Secdmaclkgds {
        &self.secdmaclkgds
    }
    #[doc = "0x48 - GPIO Clock Gate For Run And All Modes"]
    #[inline(always)]
    pub const fn gpioclkgr(&self) -> &Gpioclkgr {
        &self.gpioclkgr
    }
    #[doc = "0x4c - GPIO Clock Gate For Sleep Mode"]
    #[inline(always)]
    pub const fn gpioclkgs(&self) -> &Gpioclkgs {
        &self.gpioclkgs
    }
    #[doc = "0x50 - GPIO Clock Gate For Deep Sleep Mode"]
    #[inline(always)]
    pub const fn gpioclkgds(&self) -> &Gpioclkgds {
        &self.gpioclkgds
    }
    #[doc = "0x54 - GPT Clock Gate For Run And All Modes"]
    #[inline(always)]
    pub const fn gptclkgr(&self) -> &Gptclkgr {
        &self.gptclkgr
    }
    #[doc = "0x58 - GPT Clock Gate For Sleep Mode"]
    #[inline(always)]
    pub const fn gptclkgs(&self) -> &Gptclkgs {
        &self.gptclkgs
    }
    #[doc = "0x5c - GPT Clock Gate For Deep Sleep Mode"]
    #[inline(always)]
    pub const fn gptclkgds(&self) -> &Gptclkgds {
        &self.gptclkgds
    }
    #[doc = "0x60 - I2C Clock Gate For Run And All Modes"]
    #[inline(always)]
    pub const fn i2cclkgr(&self) -> &I2cclkgr {
        &self.i2cclkgr
    }
    #[doc = "0x64 - I2C Clock Gate For Sleep Mode"]
    #[inline(always)]
    pub const fn i2cclkgs(&self) -> &I2cclkgs {
        &self.i2cclkgs
    }
    #[doc = "0x68 - I2C Clock Gate For Deep Sleep Mode"]
    #[inline(always)]
    pub const fn i2cclkgds(&self) -> &I2cclkgds {
        &self.i2cclkgds
    }
    #[doc = "0x6c - UART Clock Gate For Run And All Modes"]
    #[inline(always)]
    pub const fn uartclkgr(&self) -> &Uartclkgr {
        &self.uartclkgr
    }
    #[doc = "0x70 - UART Clock Gate For Sleep Mode"]
    #[inline(always)]
    pub const fn uartclkgs(&self) -> &Uartclkgs {
        &self.uartclkgs
    }
    #[doc = "0x74 - UART Clock Gate For Deep Sleep Mode"]
    #[inline(always)]
    pub const fn uartclkgds(&self) -> &Uartclkgds {
        &self.uartclkgds
    }
    #[doc = "0x78 - SSI Clock Gate For Run And All Modes"]
    #[inline(always)]
    pub const fn ssiclkgr(&self) -> &Ssiclkgr {
        &self.ssiclkgr
    }
    #[doc = "0x7c - SSI Clock Gate For Sleep Mode"]
    #[inline(always)]
    pub const fn ssiclkgs(&self) -> &Ssiclkgs {
        &self.ssiclkgs
    }
    #[doc = "0x80 - SSI Clock Gate For Deep Sleep Mode"]
    #[inline(always)]
    pub const fn ssiclkgds(&self) -> &Ssiclkgds {
        &self.ssiclkgds
    }
    #[doc = "0x84 - I2S Clock Gate For Run And All Modes"]
    #[inline(always)]
    pub const fn i2sclkgr(&self) -> &I2sclkgr {
        &self.i2sclkgr
    }
    #[doc = "0x88 - I2S Clock Gate For Sleep Mode"]
    #[inline(always)]
    pub const fn i2sclkgs(&self) -> &I2sclkgs {
        &self.i2sclkgs
    }
    #[doc = "0x8c - I2S Clock Gate For Deep Sleep Mode"]
    #[inline(always)]
    pub const fn i2sclkgds(&self) -> &I2sclkgds {
        &self.i2sclkgds
    }
    #[doc = "0xb4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn sysbusclkdiv(&self) -> &Sysbusclkdiv {
        &self.sysbusclkdiv
    }
    #[doc = "0xb8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn cpuclkdiv(&self) -> &Cpuclkdiv {
        &self.cpuclkdiv
    }
    #[doc = "0xbc - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn perbuscpuclkdiv(&self) -> &Perbuscpuclkdiv {
        &self.perbuscpuclkdiv
    }
    #[doc = "0xc0 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn perbusdmaclkdiv(&self) -> &Perbusdmaclkdiv {
        &self.perbusdmaclkdiv
    }
    #[doc = "0xc4 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn perdmaclkdiv(&self) -> &Perdmaclkdiv {
        &self.perdmaclkdiv
    }
    #[doc = "0xc8 - I2S Clock Control"]
    #[inline(always)]
    pub const fn i2sbclksel(&self) -> &I2sbclksel {
        &self.i2sbclksel
    }
    #[doc = "0xcc - GPT Scalar"]
    #[inline(always)]
    pub const fn gptclkdiv(&self) -> &Gptclkdiv {
        &self.gptclkdiv
    }
    #[doc = "0xd0 - I2S Clock Control"]
    #[inline(always)]
    pub const fn i2sclkctl(&self) -> &I2sclkctl {
        &self.i2sclkctl
    }
    #[doc = "0xd4 - MCLK Division Ratio"]
    #[inline(always)]
    pub const fn i2smclkdiv(&self) -> &I2smclkdiv {
        &self.i2smclkdiv
    }
    #[doc = "0xd8 - BCLK Division Ratio"]
    #[inline(always)]
    pub const fn i2sbclkdiv(&self) -> &I2sbclkdiv {
        &self.i2sbclkdiv
    }
    #[doc = "0xdc - WCLK Division Ratio"]
    #[inline(always)]
    pub const fn i2swclkdiv(&self) -> &I2swclkdiv {
        &self.i2swclkdiv
    }
    #[doc = "0xf0 - RESET For SEC (TRNG And CRYPTO) And UDMA"]
    #[inline(always)]
    pub const fn resetsecdma(&self) -> &Resetsecdma {
        &self.resetsecdma
    }
    #[doc = "0xf4 - RESET For GPIO IPs"]
    #[inline(always)]
    pub const fn resetgpio(&self) -> &Resetgpio {
        &self.resetgpio
    }
    #[doc = "0xf8 - RESET For GPT Ips"]
    #[inline(always)]
    pub const fn resetgpt(&self) -> &Resetgpt {
        &self.resetgpt
    }
    #[doc = "0xfc - RESET For I2C IPs"]
    #[inline(always)]
    pub const fn reseti2c(&self) -> &Reseti2c {
        &self.reseti2c
    }
    #[doc = "0x100 - RESET For UART IPs"]
    #[inline(always)]
    pub const fn resetuart(&self) -> &Resetuart {
        &self.resetuart
    }
    #[doc = "0x104 - RESET For SSI IPs"]
    #[inline(always)]
    pub const fn resetssi(&self) -> &Resetssi {
        &self.resetssi
    }
    #[doc = "0x108 - RESET For I2S IP"]
    #[inline(always)]
    pub const fn reseti2s(&self) -> &Reseti2s {
        &self.reseti2s
    }
    #[doc = "0x12c - Power Domain Control"]
    #[inline(always)]
    pub const fn pdctl0(&self) -> &Pdctl0 {
        &self.pdctl0
    }
    #[doc = "0x130 - RFC Power Domain Control"]
    #[inline(always)]
    pub const fn pdctl0rfc(&self) -> &Pdctl0rfc {
        &self.pdctl0rfc
    }
    #[doc = "0x134 - SERIAL Power Domain Control"]
    #[inline(always)]
    pub const fn pdctl0serial(&self) -> &Pdctl0serial {
        &self.pdctl0serial
    }
    #[doc = "0x138 - PERIPH Power Domain Control"]
    #[inline(always)]
    pub const fn pdctl0periph(&self) -> &Pdctl0periph {
        &self.pdctl0periph
    }
    #[doc = "0x140 - Power Domain Status"]
    #[inline(always)]
    pub const fn pdstat0(&self) -> &Pdstat0 {
        &self.pdstat0
    }
    #[doc = "0x144 - RFC Power Domain Status"]
    #[inline(always)]
    pub const fn pdstat0rfc(&self) -> &Pdstat0rfc {
        &self.pdstat0rfc
    }
    #[doc = "0x148 - SERIAL Power Domain Status"]
    #[inline(always)]
    pub const fn pdstat0serial(&self) -> &Pdstat0serial {
        &self.pdstat0serial
    }
    #[doc = "0x14c - PERIPH Power Domain Status"]
    #[inline(always)]
    pub const fn pdstat0periph(&self) -> &Pdstat0periph {
        &self.pdstat0periph
    }
    #[doc = "0x17c - Power Domain Control"]
    #[inline(always)]
    pub const fn pdctl1(&self) -> &Pdctl1 {
        &self.pdctl1
    }
    #[doc = "0x184 - CPU Power Domain Direct Control"]
    #[inline(always)]
    pub const fn pdctl1cpu(&self) -> &Pdctl1cpu {
        &self.pdctl1cpu
    }
    #[doc = "0x188 - RFC Power Domain Direct Control"]
    #[inline(always)]
    pub const fn pdctl1rfc(&self) -> &Pdctl1rfc {
        &self.pdctl1rfc
    }
    #[doc = "0x18c - VIMS Mode Direct Control"]
    #[inline(always)]
    pub const fn pdctl1vims(&self) -> &Pdctl1vims {
        &self.pdctl1vims
    }
    #[doc = "0x194 - Power Manager Status"]
    #[inline(always)]
    pub const fn pdstat1(&self) -> &Pdstat1 {
        &self.pdstat1
    }
    #[doc = "0x198 - BUS Power Domain Direct Read Status"]
    #[inline(always)]
    pub const fn pdstat1bus(&self) -> &Pdstat1bus {
        &self.pdstat1bus
    }
    #[doc = "0x19c - RFC Power Domain Direct Read Status"]
    #[inline(always)]
    pub const fn pdstat1rfc(&self) -> &Pdstat1rfc {
        &self.pdstat1rfc
    }
    #[doc = "0x1a0 - CPU Power Domain Direct Read Status"]
    #[inline(always)]
    pub const fn pdstat1cpu(&self) -> &Pdstat1cpu {
        &self.pdstat1cpu
    }
    #[doc = "0x1a4 - VIMS Mode Direct Read Status"]
    #[inline(always)]
    pub const fn pdstat1vims(&self) -> &Pdstat1vims {
        &self.pdstat1vims
    }
    #[doc = "0x1cc - Control To RFC"]
    #[inline(always)]
    pub const fn rfcbits(&self) -> &Rfcbits {
        &self.rfcbits
    }
    #[doc = "0x1d0 - Selected RFC Mode"]
    #[inline(always)]
    pub const fn rfcmodesel(&self) -> &Rfcmodesel {
        &self.rfcmodesel
    }
    #[doc = "0x1d4 - Allowed RFC Modes"]
    #[inline(always)]
    pub const fn rfcmodehwopt(&self) -> &Rfcmodehwopt {
        &self.rfcmodehwopt
    }
    #[doc = "0x1e0 - Power Profiler Register"]
    #[inline(always)]
    pub const fn pwrprofstat(&self) -> &Pwrprofstat {
        &self.pwrprofstat
    }
    #[doc = "0x224 - Memory Retention Control"]
    #[inline(always)]
    pub const fn ramreten(&self) -> &Ramreten {
        &self.ramreten
    }
    #[doc = "0x290 - Oscillator Interrupt Mask Control"]
    #[inline(always)]
    pub const fn oscimsc(&self) -> &Oscimsc {
        &self.oscimsc
    }
    #[doc = "0x294 - Oscillator Raw Interrupt Status"]
    #[inline(always)]
    pub const fn oscris(&self) -> &Oscris {
        &self.oscris
    }
    #[doc = "0x298 - Oscillator Raw Interrupt Clear"]
    #[inline(always)]
    pub const fn oscicr(&self) -> &Oscicr {
        &self.oscicr
    }
}
#[doc = "INFRCLKDIVR (rw) register accessor: Infrastructure Clock Division Factor For Run Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infrclkdivr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`infrclkdivr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infrclkdivr`]
module"]
#[doc(alias = "INFRCLKDIVR")]
pub type Infrclkdivr = crate::Reg<infrclkdivr::InfrclkdivrSpec>;
#[doc = "Infrastructure Clock Division Factor For Run Mode"]
pub mod infrclkdivr;
#[doc = "INFRCLKDIVS (rw) register accessor: Infrastructure Clock Division Factor For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infrclkdivs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`infrclkdivs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infrclkdivs`]
module"]
#[doc(alias = "INFRCLKDIVS")]
pub type Infrclkdivs = crate::Reg<infrclkdivs::InfrclkdivsSpec>;
#[doc = "Infrastructure Clock Division Factor For Sleep Mode"]
pub mod infrclkdivs;
#[doc = "INFRCLKDIVDS (rw) register accessor: Infrastructure Clock Division Factor For DeepSleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`infrclkdivds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`infrclkdivds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@infrclkdivds`]
module"]
#[doc(alias = "INFRCLKDIVDS")]
pub type Infrclkdivds = crate::Reg<infrclkdivds::InfrclkdivdsSpec>;
#[doc = "Infrastructure Clock Division Factor For DeepSleep Mode"]
pub mod infrclkdivds;
#[doc = "VDCTL (rw) register accessor: MCU Voltage Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vdctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vdctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdctl`]
module"]
#[doc(alias = "VDCTL")]
pub type Vdctl = crate::Reg<vdctl::VdctlSpec>;
#[doc = "MCU Voltage Domain Control"]
pub mod vdctl;
#[doc = "CLKLOADCTL (rw) register accessor: Load PRCM Settings To CLKCTRL Power Domain\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkloadctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkloadctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkloadctl`]
module"]
#[doc(alias = "CLKLOADCTL")]
pub type Clkloadctl = crate::Reg<clkloadctl::ClkloadctlSpec>;
#[doc = "Load PRCM Settings To CLKCTRL Power Domain"]
pub mod clkloadctl;
#[doc = "RFCCLKG (rw) register accessor: RFC Clock Gate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcclkg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcclkg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcclkg`]
module"]
#[doc(alias = "RFCCLKG")]
pub type Rfcclkg = crate::Reg<rfcclkg::RfcclkgSpec>;
#[doc = "RFC Clock Gate"]
pub mod rfcclkg;
#[doc = "VIMSCLKG (rw) register accessor: VIMS Clock Gate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vimsclkg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vimsclkg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vimsclkg`]
module"]
#[doc(alias = "VIMSCLKG")]
pub type Vimsclkg = crate::Reg<vimsclkg::VimsclkgSpec>;
#[doc = "VIMS Clock Gate"]
pub mod vimsclkg;
#[doc = "SECDMACLKGR (rw) register accessor: SEC (TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secdmaclkgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secdmaclkgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secdmaclkgr`]
module"]
#[doc(alias = "SECDMACLKGR")]
pub type Secdmaclkgr = crate::Reg<secdmaclkgr::SecdmaclkgrSpec>;
#[doc = "SEC (TRNG And CRYPTO) And UDMA Clock Gate For Run And All Modes"]
pub mod secdmaclkgr;
#[doc = "SECDMACLKGS (rw) register accessor: SEC (TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secdmaclkgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secdmaclkgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secdmaclkgs`]
module"]
#[doc(alias = "SECDMACLKGS")]
pub type Secdmaclkgs = crate::Reg<secdmaclkgs::SecdmaclkgsSpec>;
#[doc = "SEC (TRNG And CRYPTO) And UDMA Clock Gate For Sleep Mode"]
pub mod secdmaclkgs;
#[doc = "SECDMACLKGDS (rw) register accessor: SEC (TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`secdmaclkgds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`secdmaclkgds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@secdmaclkgds`]
module"]
#[doc(alias = "SECDMACLKGDS")]
pub type Secdmaclkgds = crate::Reg<secdmaclkgds::SecdmaclkgdsSpec>;
#[doc = "SEC (TRNG and CRYPTO) And UDMA Clock Gate For Deep Sleep Mode"]
pub mod secdmaclkgds;
#[doc = "GPIOCLKGR (rw) register accessor: GPIO Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioclkgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioclkgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioclkgr`]
module"]
#[doc(alias = "GPIOCLKGR")]
pub type Gpioclkgr = crate::Reg<gpioclkgr::GpioclkgrSpec>;
#[doc = "GPIO Clock Gate For Run And All Modes"]
pub mod gpioclkgr;
#[doc = "GPIOCLKGS (rw) register accessor: GPIO Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioclkgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioclkgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioclkgs`]
module"]
#[doc(alias = "GPIOCLKGS")]
pub type Gpioclkgs = crate::Reg<gpioclkgs::GpioclkgsSpec>;
#[doc = "GPIO Clock Gate For Sleep Mode"]
pub mod gpioclkgs;
#[doc = "GPIOCLKGDS (rw) register accessor: GPIO Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioclkgds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioclkgds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpioclkgds`]
module"]
#[doc(alias = "GPIOCLKGDS")]
pub type Gpioclkgds = crate::Reg<gpioclkgds::GpioclkgdsSpec>;
#[doc = "GPIO Clock Gate For Deep Sleep Mode"]
pub mod gpioclkgds;
#[doc = "GPTCLKGR (rw) register accessor: GPT Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptclkgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptclkgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptclkgr`]
module"]
#[doc(alias = "GPTCLKGR")]
pub type Gptclkgr = crate::Reg<gptclkgr::GptclkgrSpec>;
#[doc = "GPT Clock Gate For Run And All Modes"]
pub mod gptclkgr;
#[doc = "GPTCLKGS (rw) register accessor: GPT Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptclkgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptclkgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptclkgs`]
module"]
#[doc(alias = "GPTCLKGS")]
pub type Gptclkgs = crate::Reg<gptclkgs::GptclkgsSpec>;
#[doc = "GPT Clock Gate For Sleep Mode"]
pub mod gptclkgs;
#[doc = "GPTCLKGDS (rw) register accessor: GPT Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptclkgds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptclkgds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptclkgds`]
module"]
#[doc(alias = "GPTCLKGDS")]
pub type Gptclkgds = crate::Reg<gptclkgds::GptclkgdsSpec>;
#[doc = "GPT Clock Gate For Deep Sleep Mode"]
pub mod gptclkgds;
#[doc = "I2CCLKGR (rw) register accessor: I2C Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cclkgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cclkgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cclkgr`]
module"]
#[doc(alias = "I2CCLKGR")]
pub type I2cclkgr = crate::Reg<i2cclkgr::I2cclkgrSpec>;
#[doc = "I2C Clock Gate For Run And All Modes"]
pub mod i2cclkgr;
#[doc = "I2CCLKGS (rw) register accessor: I2C Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cclkgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cclkgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cclkgs`]
module"]
#[doc(alias = "I2CCLKGS")]
pub type I2cclkgs = crate::Reg<i2cclkgs::I2cclkgsSpec>;
#[doc = "I2C Clock Gate For Sleep Mode"]
pub mod i2cclkgs;
#[doc = "I2CCLKGDS (rw) register accessor: I2C Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cclkgds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cclkgds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cclkgds`]
module"]
#[doc(alias = "I2CCLKGDS")]
pub type I2cclkgds = crate::Reg<i2cclkgds::I2cclkgdsSpec>;
#[doc = "I2C Clock Gate For Deep Sleep Mode"]
pub mod i2cclkgds;
#[doc = "UARTCLKGR (rw) register accessor: UART Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartclkgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartclkgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartclkgr`]
module"]
#[doc(alias = "UARTCLKGR")]
pub type Uartclkgr = crate::Reg<uartclkgr::UartclkgrSpec>;
#[doc = "UART Clock Gate For Run And All Modes"]
pub mod uartclkgr;
#[doc = "UARTCLKGS (rw) register accessor: UART Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartclkgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartclkgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartclkgs`]
module"]
#[doc(alias = "UARTCLKGS")]
pub type Uartclkgs = crate::Reg<uartclkgs::UartclkgsSpec>;
#[doc = "UART Clock Gate For Sleep Mode"]
pub mod uartclkgs;
#[doc = "UARTCLKGDS (rw) register accessor: UART Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uartclkgds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uartclkgds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uartclkgds`]
module"]
#[doc(alias = "UARTCLKGDS")]
pub type Uartclkgds = crate::Reg<uartclkgds::UartclkgdsSpec>;
#[doc = "UART Clock Gate For Deep Sleep Mode"]
pub mod uartclkgds;
#[doc = "SSICLKGR (rw) register accessor: SSI Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssiclkgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssiclkgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssiclkgr`]
module"]
#[doc(alias = "SSICLKGR")]
pub type Ssiclkgr = crate::Reg<ssiclkgr::SsiclkgrSpec>;
#[doc = "SSI Clock Gate For Run And All Modes"]
pub mod ssiclkgr;
#[doc = "SSICLKGS (rw) register accessor: SSI Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssiclkgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssiclkgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssiclkgs`]
module"]
#[doc(alias = "SSICLKGS")]
pub type Ssiclkgs = crate::Reg<ssiclkgs::SsiclkgsSpec>;
#[doc = "SSI Clock Gate For Sleep Mode"]
pub mod ssiclkgs;
#[doc = "SSICLKGDS (rw) register accessor: SSI Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssiclkgds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssiclkgds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssiclkgds`]
module"]
#[doc(alias = "SSICLKGDS")]
pub type Ssiclkgds = crate::Reg<ssiclkgds::SsiclkgdsSpec>;
#[doc = "SSI Clock Gate For Deep Sleep Mode"]
pub mod ssiclkgds;
#[doc = "I2SCLKGR (rw) register accessor: I2S Clock Gate For Run And All Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclkgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclkgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclkgr`]
module"]
#[doc(alias = "I2SCLKGR")]
pub type I2sclkgr = crate::Reg<i2sclkgr::I2sclkgrSpec>;
#[doc = "I2S Clock Gate For Run And All Modes"]
pub mod i2sclkgr;
#[doc = "I2SCLKGS (rw) register accessor: I2S Clock Gate For Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclkgs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclkgs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclkgs`]
module"]
#[doc(alias = "I2SCLKGS")]
pub type I2sclkgs = crate::Reg<i2sclkgs::I2sclkgsSpec>;
#[doc = "I2S Clock Gate For Sleep Mode"]
pub mod i2sclkgs;
#[doc = "I2SCLKGDS (rw) register accessor: I2S Clock Gate For Deep Sleep Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclkgds::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclkgds::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclkgds`]
module"]
#[doc(alias = "I2SCLKGDS")]
pub type I2sclkgds = crate::Reg<i2sclkgds::I2sclkgdsSpec>;
#[doc = "I2S Clock Gate For Deep Sleep Mode"]
pub mod i2sclkgds;
#[doc = "SYSBUSCLKDIV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysbusclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysbusclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysbusclkdiv`]
module"]
#[doc(alias = "SYSBUSCLKDIV")]
pub type Sysbusclkdiv = crate::Reg<sysbusclkdiv::SysbusclkdivSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sysbusclkdiv;
#[doc = "CPUCLKDIV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuclkdiv`]
module"]
#[doc(alias = "CPUCLKDIV")]
pub type Cpuclkdiv = crate::Reg<cpuclkdiv::CpuclkdivSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod cpuclkdiv;
#[doc = "PERBUSCPUCLKDIV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbuscpuclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbuscpuclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbuscpuclkdiv`]
module"]
#[doc(alias = "PERBUSCPUCLKDIV")]
pub type Perbuscpuclkdiv = crate::Reg<perbuscpuclkdiv::PerbuscpuclkdivSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbuscpuclkdiv;
#[doc = "PERBUSDMACLKDIV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perbusdmaclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perbusdmaclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perbusdmaclkdiv`]
module"]
#[doc(alias = "PERBUSDMACLKDIV")]
pub type Perbusdmaclkdiv = crate::Reg<perbusdmaclkdiv::PerbusdmaclkdivSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perbusdmaclkdiv;
#[doc = "PERDMACLKDIV (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perdmaclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perdmaclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@perdmaclkdiv`]
module"]
#[doc(alias = "PERDMACLKDIV")]
pub type Perdmaclkdiv = crate::Reg<perdmaclkdiv::PerdmaclkdivSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod perdmaclkdiv;
#[doc = "I2SBCLKSEL (rw) register accessor: I2S Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sbclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sbclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sbclksel`]
module"]
#[doc(alias = "I2SBCLKSEL")]
pub type I2sbclksel = crate::Reg<i2sbclksel::I2sbclkselSpec>;
#[doc = "I2S Clock Control"]
pub mod i2sbclksel;
#[doc = "GPTCLKDIV (rw) register accessor: GPT Scalar\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gptclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gptclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gptclkdiv`]
module"]
#[doc(alias = "GPTCLKDIV")]
pub type Gptclkdiv = crate::Reg<gptclkdiv::GptclkdivSpec>;
#[doc = "GPT Scalar"]
pub mod gptclkdiv;
#[doc = "I2SCLKCTL (rw) register accessor: I2S Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sclkctl`]
module"]
#[doc(alias = "I2SCLKCTL")]
pub type I2sclkctl = crate::Reg<i2sclkctl::I2sclkctlSpec>;
#[doc = "I2S Clock Control"]
pub mod i2sclkctl;
#[doc = "I2SMCLKDIV (rw) register accessor: MCLK Division Ratio\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2smclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2smclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2smclkdiv`]
module"]
#[doc(alias = "I2SMCLKDIV")]
pub type I2smclkdiv = crate::Reg<i2smclkdiv::I2smclkdivSpec>;
#[doc = "MCLK Division Ratio"]
pub mod i2smclkdiv;
#[doc = "I2SBCLKDIV (rw) register accessor: BCLK Division Ratio\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sbclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sbclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sbclkdiv`]
module"]
#[doc(alias = "I2SBCLKDIV")]
pub type I2sbclkdiv = crate::Reg<i2sbclkdiv::I2sbclkdivSpec>;
#[doc = "BCLK Division Ratio"]
pub mod i2sbclkdiv;
#[doc = "I2SWCLKDIV (rw) register accessor: WCLK Division Ratio\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2swclkdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2swclkdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2swclkdiv`]
module"]
#[doc(alias = "I2SWCLKDIV")]
pub type I2swclkdiv = crate::Reg<i2swclkdiv::I2swclkdivSpec>;
#[doc = "WCLK Division Ratio"]
pub mod i2swclkdiv;
#[doc = "RESETSECDMA (rw) register accessor: RESET For SEC (TRNG And CRYPTO) And UDMA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetsecdma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetsecdma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetsecdma`]
module"]
#[doc(alias = "RESETSECDMA")]
pub type Resetsecdma = crate::Reg<resetsecdma::ResetsecdmaSpec>;
#[doc = "RESET For SEC (TRNG And CRYPTO) And UDMA"]
pub mod resetsecdma;
#[doc = "RESETGPIO (rw) register accessor: RESET For GPIO IPs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetgpio`]
module"]
#[doc(alias = "RESETGPIO")]
pub type Resetgpio = crate::Reg<resetgpio::ResetgpioSpec>;
#[doc = "RESET For GPIO IPs"]
pub mod resetgpio;
#[doc = "RESETGPT (rw) register accessor: RESET For GPT Ips\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetgpt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetgpt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetgpt`]
module"]
#[doc(alias = "RESETGPT")]
pub type Resetgpt = crate::Reg<resetgpt::ResetgptSpec>;
#[doc = "RESET For GPT Ips"]
pub mod resetgpt;
#[doc = "RESETI2C (rw) register accessor: RESET For I2C IPs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reseti2c::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reseti2c::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reseti2c`]
module"]
#[doc(alias = "RESETI2C")]
pub type Reseti2c = crate::Reg<reseti2c::Reseti2cSpec>;
#[doc = "RESET For I2C IPs"]
pub mod reseti2c;
#[doc = "RESETUART (rw) register accessor: RESET For UART IPs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetuart::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetuart::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetuart`]
module"]
#[doc(alias = "RESETUART")]
pub type Resetuart = crate::Reg<resetuart::ResetuartSpec>;
#[doc = "RESET For UART IPs"]
pub mod resetuart;
#[doc = "RESETSSI (rw) register accessor: RESET For SSI IPs\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetssi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetssi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetssi`]
module"]
#[doc(alias = "RESETSSI")]
pub type Resetssi = crate::Reg<resetssi::ResetssiSpec>;
#[doc = "RESET For SSI IPs"]
pub mod resetssi;
#[doc = "RESETI2S (rw) register accessor: RESET For I2S IP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reseti2s::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reseti2s::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reseti2s`]
module"]
#[doc(alias = "RESETI2S")]
pub type Reseti2s = crate::Reg<reseti2s::Reseti2sSpec>;
#[doc = "RESET For I2S IP"]
pub mod reseti2s;
#[doc = "PDCTL0 (rw) register accessor: Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl0`]
module"]
#[doc(alias = "PDCTL0")]
pub type Pdctl0 = crate::Reg<pdctl0::Pdctl0Spec>;
#[doc = "Power Domain Control"]
pub mod pdctl0;
#[doc = "PDCTL0RFC (rw) register accessor: RFC Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl0rfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl0rfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl0rfc`]
module"]
#[doc(alias = "PDCTL0RFC")]
pub type Pdctl0rfc = crate::Reg<pdctl0rfc::Pdctl0rfcSpec>;
#[doc = "RFC Power Domain Control"]
pub mod pdctl0rfc;
#[doc = "PDCTL0SERIAL (rw) register accessor: SERIAL Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl0serial::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl0serial::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl0serial`]
module"]
#[doc(alias = "PDCTL0SERIAL")]
pub type Pdctl0serial = crate::Reg<pdctl0serial::Pdctl0serialSpec>;
#[doc = "SERIAL Power Domain Control"]
pub mod pdctl0serial;
#[doc = "PDCTL0PERIPH (rw) register accessor: PERIPH Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl0periph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl0periph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl0periph`]
module"]
#[doc(alias = "PDCTL0PERIPH")]
pub type Pdctl0periph = crate::Reg<pdctl0periph::Pdctl0periphSpec>;
#[doc = "PERIPH Power Domain Control"]
pub mod pdctl0periph;
#[doc = "PDSTAT0 (rw) register accessor: Power Domain Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat0`]
module"]
#[doc(alias = "PDSTAT0")]
pub type Pdstat0 = crate::Reg<pdstat0::Pdstat0Spec>;
#[doc = "Power Domain Status"]
pub mod pdstat0;
#[doc = "PDSTAT0RFC (rw) register accessor: RFC Power Domain Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat0rfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat0rfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat0rfc`]
module"]
#[doc(alias = "PDSTAT0RFC")]
pub type Pdstat0rfc = crate::Reg<pdstat0rfc::Pdstat0rfcSpec>;
#[doc = "RFC Power Domain Status"]
pub mod pdstat0rfc;
#[doc = "PDSTAT0SERIAL (rw) register accessor: SERIAL Power Domain Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat0serial::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat0serial::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat0serial`]
module"]
#[doc(alias = "PDSTAT0SERIAL")]
pub type Pdstat0serial = crate::Reg<pdstat0serial::Pdstat0serialSpec>;
#[doc = "SERIAL Power Domain Status"]
pub mod pdstat0serial;
#[doc = "PDSTAT0PERIPH (rw) register accessor: PERIPH Power Domain Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat0periph::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat0periph::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat0periph`]
module"]
#[doc(alias = "PDSTAT0PERIPH")]
pub type Pdstat0periph = crate::Reg<pdstat0periph::Pdstat0periphSpec>;
#[doc = "PERIPH Power Domain Status"]
pub mod pdstat0periph;
#[doc = "PDCTL1 (rw) register accessor: Power Domain Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl1`]
module"]
#[doc(alias = "PDCTL1")]
pub type Pdctl1 = crate::Reg<pdctl1::Pdctl1Spec>;
#[doc = "Power Domain Control"]
pub mod pdctl1;
#[doc = "PDCTL1CPU (rw) register accessor: CPU Power Domain Direct Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl1cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl1cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl1cpu`]
module"]
#[doc(alias = "PDCTL1CPU")]
pub type Pdctl1cpu = crate::Reg<pdctl1cpu::Pdctl1cpuSpec>;
#[doc = "CPU Power Domain Direct Control"]
pub mod pdctl1cpu;
#[doc = "PDCTL1RFC (rw) register accessor: RFC Power Domain Direct Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl1rfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl1rfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl1rfc`]
module"]
#[doc(alias = "PDCTL1RFC")]
pub type Pdctl1rfc = crate::Reg<pdctl1rfc::Pdctl1rfcSpec>;
#[doc = "RFC Power Domain Direct Control"]
pub mod pdctl1rfc;
#[doc = "PDCTL1VIMS (rw) register accessor: VIMS Mode Direct Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdctl1vims::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdctl1vims::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdctl1vims`]
module"]
#[doc(alias = "PDCTL1VIMS")]
pub type Pdctl1vims = crate::Reg<pdctl1vims::Pdctl1vimsSpec>;
#[doc = "VIMS Mode Direct Control"]
pub mod pdctl1vims;
#[doc = "PDSTAT1 (rw) register accessor: Power Manager Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat1`]
module"]
#[doc(alias = "PDSTAT1")]
pub type Pdstat1 = crate::Reg<pdstat1::Pdstat1Spec>;
#[doc = "Power Manager Status"]
pub mod pdstat1;
#[doc = "PDSTAT1BUS (rw) register accessor: BUS Power Domain Direct Read Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat1bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat1bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat1bus`]
module"]
#[doc(alias = "PDSTAT1BUS")]
pub type Pdstat1bus = crate::Reg<pdstat1bus::Pdstat1busSpec>;
#[doc = "BUS Power Domain Direct Read Status"]
pub mod pdstat1bus;
#[doc = "PDSTAT1RFC (rw) register accessor: RFC Power Domain Direct Read Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat1rfc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat1rfc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat1rfc`]
module"]
#[doc(alias = "PDSTAT1RFC")]
pub type Pdstat1rfc = crate::Reg<pdstat1rfc::Pdstat1rfcSpec>;
#[doc = "RFC Power Domain Direct Read Status"]
pub mod pdstat1rfc;
#[doc = "PDSTAT1CPU (rw) register accessor: CPU Power Domain Direct Read Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat1cpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat1cpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat1cpu`]
module"]
#[doc(alias = "PDSTAT1CPU")]
pub type Pdstat1cpu = crate::Reg<pdstat1cpu::Pdstat1cpuSpec>;
#[doc = "CPU Power Domain Direct Read Status"]
pub mod pdstat1cpu;
#[doc = "PDSTAT1VIMS (rw) register accessor: VIMS Mode Direct Read Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pdstat1vims::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pdstat1vims::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdstat1vims`]
module"]
#[doc(alias = "PDSTAT1VIMS")]
pub type Pdstat1vims = crate::Reg<pdstat1vims::Pdstat1vimsSpec>;
#[doc = "VIMS Mode Direct Read Status"]
pub mod pdstat1vims;
#[doc = "RFCBITS (rw) register accessor: Control To RFC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcbits::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcbits::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcbits`]
module"]
#[doc(alias = "RFCBITS")]
pub type Rfcbits = crate::Reg<rfcbits::RfcbitsSpec>;
#[doc = "Control To RFC"]
pub mod rfcbits;
#[doc = "RFCMODESEL (rw) register accessor: Selected RFC Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcmodesel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcmodesel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcmodesel`]
module"]
#[doc(alias = "RFCMODESEL")]
pub type Rfcmodesel = crate::Reg<rfcmodesel::RfcmodeselSpec>;
#[doc = "Selected RFC Mode"]
pub mod rfcmodesel;
#[doc = "RFCMODEHWOPT (rw) register accessor: Allowed RFC Modes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcmodehwopt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcmodehwopt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcmodehwopt`]
module"]
#[doc(alias = "RFCMODEHWOPT")]
pub type Rfcmodehwopt = crate::Reg<rfcmodehwopt::RfcmodehwoptSpec>;
#[doc = "Allowed RFC Modes"]
pub mod rfcmodehwopt;
#[doc = "PWRPROFSTAT (rw) register accessor: Power Profiler Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrprofstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrprofstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrprofstat`]
module"]
#[doc(alias = "PWRPROFSTAT")]
pub type Pwrprofstat = crate::Reg<pwrprofstat::PwrprofstatSpec>;
#[doc = "Power Profiler Register"]
pub mod pwrprofstat;
#[doc = "RAMRETEN (rw) register accessor: Memory Retention Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ramreten::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ramreten::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramreten`]
module"]
#[doc(alias = "RAMRETEN")]
pub type Ramreten = crate::Reg<ramreten::RamretenSpec>;
#[doc = "Memory Retention Control"]
pub mod ramreten;
#[doc = "OSCIMSC (rw) register accessor: Oscillator Interrupt Mask Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscimsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscimsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscimsc`]
module"]
#[doc(alias = "OSCIMSC")]
pub type Oscimsc = crate::Reg<oscimsc::OscimscSpec>;
#[doc = "Oscillator Interrupt Mask Control"]
pub mod oscimsc;
#[doc = "OSCRIS (rw) register accessor: Oscillator Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscris`]
module"]
#[doc(alias = "OSCRIS")]
pub type Oscris = crate::Reg<oscris::OscrisSpec>;
#[doc = "Oscillator Raw Interrupt Status"]
pub mod oscris;
#[doc = "OSCICR (rw) register accessor: Oscillator Raw Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oscicr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oscicr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oscicr`]
module"]
#[doc(alias = "OSCICR")]
pub type Oscicr = crate::Reg<oscicr::OscicrSpec>;
#[doc = "Oscillator Raw Interrupt Clear"]
pub mod oscicr;
