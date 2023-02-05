#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Allows enabling of the Security Attribution Unit"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Indicates the number of regions implemented by the Security Attribution Unit"]
    pub type_: TYPE,
    #[doc = "0x08 - Selects the region currently accessed by SAU_RBAR and SAU_RLAR"]
    pub rnr: RNR,
    #[doc = "0x0c - Provides indirect read and write access to the base address of the currently selected SAU region"]
    pub rbar: RBAR,
    #[doc = "0x10 - Provides indirect read and write access to the limit address of the currently selected SAU region"]
    pub rlar: RLAR,
    #[doc = "0x14 - Provides information about any security related faults"]
    pub sfsr: SFSR,
    #[doc = "0x18 - Shows the address of the memory location that caused a Security violation"]
    pub sfar: SFAR,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Allows enabling of the Security Attribution Unit"]
pub mod ctrl;
#[doc = "TYPE (rw) register accessor: an alias for `Reg<TYPE_SPEC>`"]
pub type TYPE = crate::Reg<type_::TYPE_SPEC>;
#[doc = "Indicates the number of regions implemented by the Security Attribution Unit"]
pub mod type_;
#[doc = "RNR (rw) register accessor: an alias for `Reg<RNR_SPEC>`"]
pub type RNR = crate::Reg<rnr::RNR_SPEC>;
#[doc = "Selects the region currently accessed by SAU_RBAR and SAU_RLAR"]
pub mod rnr;
#[doc = "RBAR (rw) register accessor: an alias for `Reg<RBAR_SPEC>`"]
pub type RBAR = crate::Reg<rbar::RBAR_SPEC>;
#[doc = "Provides indirect read and write access to the base address of the currently selected SAU region"]
pub mod rbar;
#[doc = "RLAR (rw) register accessor: an alias for `Reg<RLAR_SPEC>`"]
pub type RLAR = crate::Reg<rlar::RLAR_SPEC>;
#[doc = "Provides indirect read and write access to the limit address of the currently selected SAU region"]
pub mod rlar;
#[doc = "SFSR (rw) register accessor: an alias for `Reg<SFSR_SPEC>`"]
pub type SFSR = crate::Reg<sfsr::SFSR_SPEC>;
#[doc = "Provides information about any security related faults"]
pub mod sfsr;
#[doc = "SFAR (rw) register accessor: an alias for `Reg<SFAR_SPEC>`"]
pub type SFAR = crate::Reg<sfar::SFAR_SPEC>;
#[doc = "Shows the address of the memory location that caused a Security violation"]
pub mod sfar;
