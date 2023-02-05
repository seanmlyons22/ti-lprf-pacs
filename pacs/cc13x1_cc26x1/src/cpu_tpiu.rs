#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
    pub sspsr: SSPSR,
    #[doc = "0x04 - Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
    pub cspsr: CSPSR,
    _reserved2: [u8; 0x08],
    #[doc = "0x10 - Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
    pub acpr: ACPR,
    _reserved3: [u8; 0xdc],
    #[doc = "0xf0 - Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
    pub sppr: SPPR,
    _reserved4: [u8; 0x020c],
    #[doc = "0x300 - Formatter and Flush Status"]
    pub ffsr: FFSR,
    #[doc = "0x304 - Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
    pub ffcr: FFCR,
    #[doc = "0x308 - Formatter Synchronization Counter"]
    pub fscr: FSCR,
    _reserved7: [u8; 0x0c94],
    _reserved_7_claimset: [u8; 0x04],
    _reserved_8_claimclr: [u8; 0x04],
    _reserved9: [u8; 0x20],
    #[doc = "0xfc8 - Device ID"]
    pub devid: DEVID,
}
impl RegisterBlock {
    #[doc = "0xfa0 - Claim Tag Set"]
    #[inline(always)]
    pub const fn claimset(&self) -> &CLAIMSET {
        unsafe { &*(self as *const Self).cast::<u8>().add(4000usize).cast() }
    }
    #[doc = "0xfa0 - Claim Tag Mask"]
    #[inline(always)]
    pub const fn claimmask(&self) -> &CLAIMMASK {
        unsafe { &*(self as *const Self).cast::<u8>().add(4000usize).cast() }
    }
    #[doc = "0xfa4 - Claim Tag Clear"]
    #[inline(always)]
    pub const fn claimclr(&self) -> &CLAIMCLR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4004usize).cast() }
    }
    #[doc = "0xfa4 - Current Claim Tag"]
    #[inline(always)]
    pub const fn claimtag(&self) -> &CLAIMTAG {
        unsafe { &*(self as *const Self).cast::<u8>().add(4004usize).cast() }
    }
}
#[doc = "SSPSR (rw) register accessor: an alias for `Reg<SSPSR_SPEC>`"]
pub type SSPSR = crate::Reg<sspsr::SSPSR_SPEC>;
#[doc = "Supported Sync Port Sizes This register represents a single port size that is supported on the device, that is, 4, 2 or 1. This is to ensure that tools do not attempt to select a port width that an attached TPA cannot capture."]
pub mod sspsr;
#[doc = "CSPSR (rw) register accessor: an alias for `Reg<CSPSR_SPEC>`"]
pub type CSPSR = crate::Reg<cspsr::CSPSR_SPEC>;
#[doc = "Current Sync Port Size This register has the same format as SSPSR but only one bit can be set, and all others must be zero. Writing values with more than one bit set, or setting a bit that is not indicated as supported can cause Unpredictable behavior. On reset this defaults to the smallest possible port size, 1 bit."]
pub mod cspsr;
#[doc = "ACPR (rw) register accessor: an alias for `Reg<ACPR_SPEC>`"]
pub type ACPR = crate::Reg<acpr::ACPR_SPEC>;
#[doc = "Async Clock Prescaler This register scales the baud rate of the asynchronous output."]
pub mod acpr;
#[doc = "SPPR (rw) register accessor: an alias for `Reg<SPPR_SPEC>`"]
pub type SPPR = crate::Reg<sppr::SPPR_SPEC>;
#[doc = "Selected Pin Protocol This register selects the protocol to be used for trace output. Note: If this register is changed while trace data is being output, data corruption occurs."]
pub mod sppr;
#[doc = "FFSR (rw) register accessor: an alias for `Reg<FFSR_SPEC>`"]
pub type FFSR = crate::Reg<ffsr::FFSR_SPEC>;
#[doc = "Formatter and Flush Status"]
pub mod ffsr;
#[doc = "FFCR (rw) register accessor: an alias for `Reg<FFCR_SPEC>`"]
pub type FFCR = crate::Reg<ffcr::FFCR_SPEC>;
#[doc = "Formatter and Flush Control When one of the two single wire output (SWO) modes is selected, ENFCONT enables the formatter to be bypassed. If the formatter is bypassed, only the ITM/DWT trace source (ATDATA2) passes through. The TPIU accepts and discards data that is presented on the ETM port (ATDATA1). This function is intended to be used when it is necessary to connect a device containing an ETM to a trace capture device that is only able to capture Serial Wire Output (SWO) data. Enabling or disabling the formatter causes momentary data corruption. Note: If the selected pin protocol register (SPPR.PROTOCOL) is set to 0x00 (TracePort mode), this register always reads 0x102, because the formatter is automatically enabled. If one of the serial wire modes is then selected, the register reverts to its previously programmed value."]
pub mod ffcr;
#[doc = "FSCR (rw) register accessor: an alias for `Reg<FSCR_SPEC>`"]
pub type FSCR = crate::Reg<fscr::FSCR_SPEC>;
#[doc = "Formatter Synchronization Counter"]
pub mod fscr;
#[doc = "CLAIMMASK (rw) register accessor: an alias for `Reg<CLAIMMASK_SPEC>`"]
pub type CLAIMMASK = crate::Reg<claimmask::CLAIMMASK_SPEC>;
#[doc = "Claim Tag Mask"]
pub mod claimmask;
#[doc = "CLAIMSET (rw) register accessor: an alias for `Reg<CLAIMSET_SPEC>`"]
pub type CLAIMSET = crate::Reg<claimset::CLAIMSET_SPEC>;
#[doc = "Claim Tag Set"]
pub mod claimset;
#[doc = "CLAIMTAG (rw) register accessor: an alias for `Reg<CLAIMTAG_SPEC>`"]
pub type CLAIMTAG = crate::Reg<claimtag::CLAIMTAG_SPEC>;
#[doc = "Current Claim Tag"]
pub mod claimtag;
#[doc = "CLAIMCLR (rw) register accessor: an alias for `Reg<CLAIMCLR_SPEC>`"]
pub type CLAIMCLR = crate::Reg<claimclr::CLAIMCLR_SPEC>;
#[doc = "Claim Tag Clear"]
pub mod claimclr;
#[doc = "DEVID (rw) register accessor: an alias for `Reg<DEVID_SPEC>`"]
pub type DEVID = crate::Reg<devid::DEVID_SPEC>;
#[doc = "Device ID"]
pub mod devid;
