#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sspsr: Sspsr,
    cspsr: Cspsr,
    _reserved2: [u8; 0x08],
    acpr: Acpr,
    _reserved3: [u8; 0xdc],
    sppr: Sppr,
    _reserved4: [u8; 0x020c],
    ffsr: Ffsr,
    ffcr: Ffcr,
    pscr: Pscr,
    _reserved7: [u8; 0x0c94],
    _reserved_7_claimset: [u8; 0x04],
    _reserved_8_claimclr: [u8; 0x04],
    _reserved9: [u8; 0x20],
    devid: Devid,
    devtype: Devtype,
}
impl RegisterBlock {
    #[doc = "0x00 - Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
    #[inline(always)]
    pub const fn sspsr(&self) -> &Sspsr {
        &self.sspsr
    }
    #[doc = "0x04 - Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
    #[inline(always)]
    pub const fn cspsr(&self) -> &Cspsr {
        &self.cspsr
    }
    #[doc = "0x10 - Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
    #[inline(always)]
    pub const fn acpr(&self) -> &Acpr {
        &self.acpr
    }
    #[doc = "0xf0 - Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
    #[inline(always)]
    pub const fn sppr(&self) -> &Sppr {
        &self.sppr
    }
    #[doc = "0x300 - Formatter and Flush Status"]
    #[inline(always)]
    pub const fn ffsr(&self) -> &Ffsr {
        &self.ffsr
    }
    #[doc = "0x304 - Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
    #[inline(always)]
    pub const fn ffcr(&self) -> &Ffcr {
        &self.ffcr
    }
    #[doc = "0x308 - Formatter Synchronization Counter"]
    #[inline(always)]
    pub const fn pscr(&self) -> &Pscr {
        &self.pscr
    }
    #[doc = "0xfa0 - Claim Tag Set"]
    #[inline(always)]
    pub const fn claimset(&self) -> &Claimset {
        unsafe { &*(self as *const Self).cast::<u8>().add(4000).cast() }
    }
    #[doc = "0xfa0 - Claim Tag Mask"]
    #[inline(always)]
    pub const fn claimmask(&self) -> &Claimmask {
        unsafe { &*(self as *const Self).cast::<u8>().add(4000).cast() }
    }
    #[doc = "0xfa4 - Claim Tag Clear"]
    #[inline(always)]
    pub const fn claimclr(&self) -> &Claimclr {
        unsafe { &*(self as *const Self).cast::<u8>().add(4004).cast() }
    }
    #[doc = "0xfa4 - Current Claim Tag"]
    #[inline(always)]
    pub const fn claimtag(&self) -> &Claimtag {
        unsafe { &*(self as *const Self).cast::<u8>().add(4004).cast() }
    }
    #[doc = "0xfc8 - Device ID"]
    #[inline(always)]
    pub const fn devid(&self) -> &Devid {
        &self.devid
    }
    #[doc = "0xfcc - The Device Type Identification"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
}
#[doc = "SSPSR (rw) register accessor: Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sspsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sspsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sspsr`]
module"]
#[doc(alias = "SSPSR")]
pub type Sspsr = crate::Reg<sspsr::SspsrSpec>;
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
pub mod sspsr;
#[doc = "CSPSR (rw) register accessor: Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cspsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cspsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cspsr`]
module"]
#[doc(alias = "CSPSR")]
pub type Cspsr = crate::Reg<cspsr::CspsrSpec>;
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
pub mod cspsr;
#[doc = "ACPR (rw) register accessor: Async Clock Prescaler This register scales the baud rate of the asynchronous output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acpr`]
module"]
#[doc(alias = "ACPR")]
pub type Acpr = crate::Reg<acpr::AcprSpec>;
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
pub mod acpr;
#[doc = "SPPR (rw) register accessor: Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sppr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sppr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sppr`]
module"]
#[doc(alias = "SPPR")]
pub type Sppr = crate::Reg<sppr::SpprSpec>;
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
pub mod sppr;
#[doc = "FFSR (rw) register accessor: Formatter and Flush Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffsr`]
module"]
#[doc(alias = "FFSR")]
pub type Ffsr = crate::Reg<ffsr::FfsrSpec>;
#[doc = "Formatter and Flush Status"]
pub mod ffsr;
#[doc = "FFCR (rw) register accessor: Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ffcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ffcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ffcr`]
module"]
#[doc(alias = "FFCR")]
pub type Ffcr = crate::Reg<ffcr::FfcrSpec>;
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
pub mod ffcr;
#[doc = "PSCR (rw) register accessor: Formatter Synchronization Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pscr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pscr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pscr`]
module"]
#[doc(alias = "PSCR")]
pub type Pscr = crate::Reg<pscr::PscrSpec>;
#[doc = "Formatter Synchronization Counter"]
pub mod pscr;
#[doc = "CLAIMMASK (rw) register accessor: Claim Tag Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimmask`]
module"]
#[doc(alias = "CLAIMMASK")]
pub type Claimmask = crate::Reg<claimmask::ClaimmaskSpec>;
#[doc = "Claim Tag Mask"]
pub mod claimmask;
#[doc = "CLAIMSET (rw) register accessor: Claim Tag Set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimset`]
module"]
#[doc(alias = "CLAIMSET")]
pub type Claimset = crate::Reg<claimset::ClaimsetSpec>;
#[doc = "Claim Tag Set"]
pub mod claimset;
#[doc = "CLAIMTAG (rw) register accessor: Current Claim Tag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimtag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimtag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimtag`]
module"]
#[doc(alias = "CLAIMTAG")]
pub type Claimtag = crate::Reg<claimtag::ClaimtagSpec>;
#[doc = "Current Claim Tag"]
pub mod claimtag;
#[doc = "CLAIMCLR (rw) register accessor: Claim Tag Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`claimclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`claimclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@claimclr`]
module"]
#[doc(alias = "CLAIMCLR")]
pub type Claimclr = crate::Reg<claimclr::ClaimclrSpec>;
#[doc = "Claim Tag Clear"]
pub mod claimclr;
#[doc = "DEVID (rw) register accessor: Device ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devid`]
module"]
#[doc(alias = "DEVID")]
pub type Devid = crate::Reg<devid::DevidSpec>;
#[doc = "Device ID"]
pub mod devid;
#[doc = "DEVTYPE (rw) register accessor: The Device Type Identification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`]
module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "The Device Type Identification"]
pub mod devtype;
