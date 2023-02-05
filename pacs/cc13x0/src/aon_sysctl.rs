#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    pub pwrctl: PWRCTL,
    #[doc = "0x04 - Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
    pub resetctl: RESETCTL,
    #[doc = "0x08 - Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    pub sleepctl: SLEEPCTL,
}
#[doc = "PWRCTL (rw) register accessor: an alias for `Reg<PWRCTL_SPEC>`"]
pub type PWRCTL = crate::Reg<pwrctl::PWRCTL_SPEC>;
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "RESETCTL (rw) register accessor: an alias for `Reg<RESETCTL_SPEC>`"]
pub type RESETCTL = crate::Reg<resetctl::RESETCTL_SPEC>;
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "SLEEPCTL (rw) register accessor: an alias for `Reg<SLEEPCTL_SPEC>`"]
pub type SLEEPCTL = crate::Reg<sleepctl::SLEEPCTL_SPEC>;
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;
