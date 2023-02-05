#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer 0 Configuration"]
    pub t0cfg: T0CFG,
    #[doc = "0x04 - Timer 1 Configuration"]
    pub t1cfg: T1CFG,
    #[doc = "0x08 - Timer 0 Control"]
    pub t0ctl: T0CTL,
    #[doc = "0x0c - Timer 0 Target"]
    pub t0target: T0TARGET,
    #[doc = "0x10 - Timer 1 Target Timer 1 counter target value"]
    pub t1target: T1TARGET,
    #[doc = "0x14 - Timer 1 Control"]
    pub t1ctl: T1CTL,
}
#[doc = "T0CFG (rw) register accessor: an alias for `Reg<T0CFG_SPEC>`"]
pub type T0CFG = crate::Reg<t0cfg::T0CFG_SPEC>;
#[doc = "Timer 0 Configuration"]
pub mod t0cfg;
#[doc = "T1CFG (rw) register accessor: an alias for `Reg<T1CFG_SPEC>`"]
pub type T1CFG = crate::Reg<t1cfg::T1CFG_SPEC>;
#[doc = "Timer 1 Configuration"]
pub mod t1cfg;
#[doc = "T0CTL (rw) register accessor: an alias for `Reg<T0CTL_SPEC>`"]
pub type T0CTL = crate::Reg<t0ctl::T0CTL_SPEC>;
#[doc = "Timer 0 Control"]
pub mod t0ctl;
#[doc = "T0TARGET (rw) register accessor: an alias for `Reg<T0TARGET_SPEC>`"]
pub type T0TARGET = crate::Reg<t0target::T0TARGET_SPEC>;
#[doc = "Timer 0 Target"]
pub mod t0target;
#[doc = "T1TARGET (rw) register accessor: an alias for `Reg<T1TARGET_SPEC>`"]
pub type T1TARGET = crate::Reg<t1target::T1TARGET_SPEC>;
#[doc = "Timer 1 Target Timer 1 counter target value"]
pub mod t1target;
#[doc = "T1CTL (rw) register accessor: an alias for `Reg<T1CTL_SPEC>`"]
pub type T1CTL = crate::Reg<t1ctl::T1CTL_SPEC>;
#[doc = "Timer 1 Control"]
pub mod t1ctl;
