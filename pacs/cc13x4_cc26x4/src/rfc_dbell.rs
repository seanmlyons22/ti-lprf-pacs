#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Doorbell Command Register"]
    pub cmdr: CMDR,
    #[doc = "0x04 - Doorbell Command Status Register"]
    pub cmdsta: CMDSTA,
    #[doc = "0x08 - Interrupt Flags From RF Hardware Modules"]
    pub rfhwifg: RFHWIFG,
    #[doc = "0x0c - Interrupt Enable For RF Hardware Modules"]
    pub rfhwien: RFHWIEN,
    #[doc = "0x10 - Interrupt Flags For Command and Packet Engine Generated Interrupts"]
    pub rfcpeifg: RFCPEIFG,
    #[doc = "0x14 - Interrupt Enable For Command and Packet Engine Generated Interrupts"]
    pub rfcpeien: RFCPEIEN,
    #[doc = "0x18 - Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
    pub rfcpeisl: RFCPEISL,
    #[doc = "0x1c - Doorbell Command Acknowledgement Interrupt Flag"]
    pub rfackifg: RFACKIFG,
    #[doc = "0x20 - RF Core General Purpose Output Control"]
    pub sysgpoctl: SYSGPOCTL,
}
#[doc = "CMDR (rw) register accessor: an alias for `Reg<CMDR_SPEC>`"]
pub type CMDR = crate::Reg<cmdr::CMDR_SPEC>;
#[doc = "Doorbell Command Register"]
pub mod cmdr;
#[doc = "CMDSTA (rw) register accessor: an alias for `Reg<CMDSTA_SPEC>`"]
pub type CMDSTA = crate::Reg<cmdsta::CMDSTA_SPEC>;
#[doc = "Doorbell Command Status Register"]
pub mod cmdsta;
#[doc = "RFHWIFG (rw) register accessor: an alias for `Reg<RFHWIFG_SPEC>`"]
pub type RFHWIFG = crate::Reg<rfhwifg::RFHWIFG_SPEC>;
#[doc = "Interrupt Flags From RF Hardware Modules"]
pub mod rfhwifg;
#[doc = "RFHWIEN (rw) register accessor: an alias for `Reg<RFHWIEN_SPEC>`"]
pub type RFHWIEN = crate::Reg<rfhwien::RFHWIEN_SPEC>;
#[doc = "Interrupt Enable For RF Hardware Modules"]
pub mod rfhwien;
#[doc = "RFCPEIFG (rw) register accessor: an alias for `Reg<RFCPEIFG_SPEC>`"]
pub type RFCPEIFG = crate::Reg<rfcpeifg::RFCPEIFG_SPEC>;
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeifg;
#[doc = "RFCPEIEN (rw) register accessor: an alias for `Reg<RFCPEIEN_SPEC>`"]
pub type RFCPEIEN = crate::Reg<rfcpeien::RFCPEIEN_SPEC>;
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeien;
#[doc = "RFCPEISL (rw) register accessor: an alias for `Reg<RFCPEISL_SPEC>`"]
pub type RFCPEISL = crate::Reg<rfcpeisl::RFCPEISL_SPEC>;
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeisl;
#[doc = "RFACKIFG (rw) register accessor: an alias for `Reg<RFACKIFG_SPEC>`"]
pub type RFACKIFG = crate::Reg<rfackifg::RFACKIFG_SPEC>;
#[doc = "Doorbell Command Acknowledgement Interrupt Flag"]
pub mod rfackifg;
#[doc = "SYSGPOCTL (rw) register accessor: an alias for `Reg<SYSGPOCTL_SPEC>`"]
pub type SYSGPOCTL = crate::Reg<sysgpoctl::SYSGPOCTL_SPEC>;
#[doc = "RF Core General Purpose Output Control"]
pub mod sysgpoctl;
