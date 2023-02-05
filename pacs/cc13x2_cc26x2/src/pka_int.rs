#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved_0: RESERVED_0,
    _reserved1: [u8; 0x0ff4],
    #[doc = "0xff8 - PKA Options register"]
    pub options: OPTIONS,
    #[doc = "0xffc - PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
    pub revision: REVISION,
}
#[doc = "RESERVED_0 (rw) register accessor: an alias for `Reg<RESERVED_0_SPEC>`"]
pub type RESERVED_0 = crate::Reg<reserved_0::RESERVED_0_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved_0;
#[doc = "OPTIONS (rw) register accessor: an alias for `Reg<OPTIONS_SPEC>`"]
pub type OPTIONS = crate::Reg<options::OPTIONS_SPEC>;
#[doc = "PKA Options register"]
pub mod options;
#[doc = "REVISION (rw) register accessor: an alias for `Reg<REVISION_SPEC>`"]
pub type REVISION = crate::Reg<revision::REVISION_SPEC>;
#[doc = "PKA hardware revision register This register allows the host access to the hardware revision number of the PKA engine for software driver matching and diagnostic purposes. It is always located at the highest address in the access space of the module and contains an encoding of the EIP number (with its complement as signature) for recognition of the hardware module."]
pub mod revision;
