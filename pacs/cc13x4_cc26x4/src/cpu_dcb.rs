#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - Controls halting debug"]
    pub dhcsr: DHCSR,
    #[doc = "0x14 - With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer"]
    pub dcrsr: DCRSR,
    #[doc = "0x18 - With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE"]
    pub dcrdr: DCRDR,
    #[doc = "0x1c - Manages vector catch behavior and DebugMonitor handling when debugging"]
    pub demcr: DEMCR,
    _reserved4: [u8; 0x04],
    #[doc = "0x24 - This register allows the external authentication interface to be overridden from software."]
    pub dauthctrl: DAUTHCTRL,
    #[doc = "0x28 - Provides control and status information for Secure debug"]
    pub dscsr: DSCSR,
}
#[doc = "DHCSR (rw) register accessor: an alias for `Reg<DHCSR_SPEC>`"]
pub type DHCSR = crate::Reg<dhcsr::DHCSR_SPEC>;
#[doc = "Controls halting debug"]
pub mod dhcsr;
#[doc = "DCRSR (rw) register accessor: an alias for `Reg<DCRSR_SPEC>`"]
pub type DCRSR = crate::Reg<dcrsr::DCRSR_SPEC>;
#[doc = "With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer"]
pub mod dcrsr;
#[doc = "DCRDR (rw) register accessor: an alias for `Reg<DCRDR_SPEC>`"]
pub type DCRDR = crate::Reg<dcrdr::DCRDR_SPEC>;
#[doc = "With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE"]
pub mod dcrdr;
#[doc = "DEMCR (rw) register accessor: an alias for `Reg<DEMCR_SPEC>`"]
pub type DEMCR = crate::Reg<demcr::DEMCR_SPEC>;
#[doc = "Manages vector catch behavior and DebugMonitor handling when debugging"]
pub mod demcr;
#[doc = "DAUTHCTRL (rw) register accessor: an alias for `Reg<DAUTHCTRL_SPEC>`"]
pub type DAUTHCTRL = crate::Reg<dauthctrl::DAUTHCTRL_SPEC>;
#[doc = "This register allows the external authentication interface to be overridden from software."]
pub mod dauthctrl;
#[doc = "DSCSR (rw) register accessor: an alias for `Reg<DSCSR_SPEC>`"]
pub type DSCSR = crate::Reg<dscsr::DSCSR_SPEC>;
#[doc = "Provides control and status information for Secure debug"]
pub mod dscsr;
