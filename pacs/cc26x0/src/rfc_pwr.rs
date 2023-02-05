#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RF Core Power Management and Clock Enable"]
    pub pwmclken: PWMCLKEN,
}
#[doc = "PWMCLKEN (rw) register accessor: an alias for `Reg<PWMCLKEN_SPEC>`"]
pub type PWMCLKEN = crate::Reg<pwmclken::PWMCLKEN_SPEC>;
#[doc = "RF Core Power Management and Clock Enable"]
pub mod pwmclken;
