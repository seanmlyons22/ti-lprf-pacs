#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Enables or reads the enabled state of each group of 32 interrupts"]
    pub iser0: ISER0,
    #[doc = "0x04 - Enables or reads the enabled state of each group of 32 interrupts"]
    pub iser1: ISER1,
    _reserved2: [u8; 0x78],
    #[doc = "0x80 - Clears or reads the enabled state of each group of 32 interrupts"]
    pub icer0: ICER0,
    #[doc = "0x84 - Clears or reads the enabled state of each group of 32 interrupts"]
    pub icer1: ICER1,
    _reserved4: [u8; 0x78],
    #[doc = "0x100 - Enables or reads the pending state of each group of 32 interrupts"]
    pub ispr0: ISPR0,
    #[doc = "0x104 - Enables or reads the pending state of each group of 32 interrupts"]
    pub ispr1: ISPR1,
    _reserved6: [u8; 0x78],
    #[doc = "0x180 - Clears or reads the pending state of each group of 32 interrupts"]
    pub icpr0: ICPR0,
    #[doc = "0x184 - Clears or reads the pending state of each group of 32 interrupts"]
    pub icpr1: ICPR1,
    _reserved8: [u8; 0x78],
    #[doc = "0x200 - For each group of 32 interrupts, shows the active state of each interrupt"]
    pub iabr0: IABR0,
    #[doc = "0x204 - For each group of 32 interrupts, shows the active state of each interrupt"]
    pub iabr1: IABR1,
    _reserved10: [u8; 0x78],
    #[doc = "0x280 - For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
    pub itns0: ITNS0,
    #[doc = "0x284 - For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
    pub itns1: ITNS1,
    _reserved12: [u8; 0x78],
    #[doc = "0x300 - Sets or reads interrupt priorities"]
    pub ipr0: IPR0,
    #[doc = "0x304 - Sets or reads interrupt priorities"]
    pub ipr1: IPR1,
    #[doc = "0x308 - Sets or reads interrupt priorities"]
    pub ipr2: IPR2,
    #[doc = "0x30c - Sets or reads interrupt priorities"]
    pub ipr3: IPR3,
    #[doc = "0x310 - Sets or reads interrupt priorities"]
    pub ipr4: IPR4,
    #[doc = "0x314 - Sets or reads interrupt priorities"]
    pub ipr5: IPR5,
    #[doc = "0x318 - Sets or reads interrupt priorities"]
    pub ipr6: IPR6,
    #[doc = "0x31c - Sets or reads interrupt priorities"]
    pub ipr7: IPR7,
    #[doc = "0x320 - Sets or reads interrupt priorities"]
    pub ipr8: IPR8,
    #[doc = "0x324 - Sets or reads interrupt priorities"]
    pub ipr9: IPR9,
    #[doc = "0x328 - Sets or reads interrupt priorities"]
    pub ipr10: IPR10,
    #[doc = "0x32c - Sets or reads interrupt priorities"]
    pub ipr11: IPR11,
}
#[doc = "ISER0 (rw) register accessor: an alias for `Reg<ISER0_SPEC>`"]
pub type ISER0 = crate::Reg<iser0::ISER0_SPEC>;
#[doc = "Enables or reads the enabled state of each group of 32 interrupts"]
pub mod iser0;
#[doc = "ISER1 (rw) register accessor: an alias for `Reg<ISER1_SPEC>`"]
pub type ISER1 = crate::Reg<iser1::ISER1_SPEC>;
#[doc = "Enables or reads the enabled state of each group of 32 interrupts"]
pub mod iser1;
#[doc = "ICER0 (rw) register accessor: an alias for `Reg<ICER0_SPEC>`"]
pub type ICER0 = crate::Reg<icer0::ICER0_SPEC>;
#[doc = "Clears or reads the enabled state of each group of 32 interrupts"]
pub mod icer0;
#[doc = "ICER1 (rw) register accessor: an alias for `Reg<ICER1_SPEC>`"]
pub type ICER1 = crate::Reg<icer1::ICER1_SPEC>;
#[doc = "Clears or reads the enabled state of each group of 32 interrupts"]
pub mod icer1;
#[doc = "ISPR0 (rw) register accessor: an alias for `Reg<ISPR0_SPEC>`"]
pub type ISPR0 = crate::Reg<ispr0::ISPR0_SPEC>;
#[doc = "Enables or reads the pending state of each group of 32 interrupts"]
pub mod ispr0;
#[doc = "ISPR1 (rw) register accessor: an alias for `Reg<ISPR1_SPEC>`"]
pub type ISPR1 = crate::Reg<ispr1::ISPR1_SPEC>;
#[doc = "Enables or reads the pending state of each group of 32 interrupts"]
pub mod ispr1;
#[doc = "ICPR0 (rw) register accessor: an alias for `Reg<ICPR0_SPEC>`"]
pub type ICPR0 = crate::Reg<icpr0::ICPR0_SPEC>;
#[doc = "Clears or reads the pending state of each group of 32 interrupts"]
pub mod icpr0;
#[doc = "ICPR1 (rw) register accessor: an alias for `Reg<ICPR1_SPEC>`"]
pub type ICPR1 = crate::Reg<icpr1::ICPR1_SPEC>;
#[doc = "Clears or reads the pending state of each group of 32 interrupts"]
pub mod icpr1;
#[doc = "IABR0 (rw) register accessor: an alias for `Reg<IABR0_SPEC>`"]
pub type IABR0 = crate::Reg<iabr0::IABR0_SPEC>;
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt"]
pub mod iabr0;
#[doc = "IABR1 (rw) register accessor: an alias for `Reg<IABR1_SPEC>`"]
pub type IABR1 = crate::Reg<iabr1::IABR1_SPEC>;
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt"]
pub mod iabr1;
#[doc = "ITNS0 (rw) register accessor: an alias for `Reg<ITNS0_SPEC>`"]
pub type ITNS0 = crate::Reg<itns0::ITNS0_SPEC>;
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
pub mod itns0;
#[doc = "ITNS1 (rw) register accessor: an alias for `Reg<ITNS1_SPEC>`"]
pub type ITNS1 = crate::Reg<itns1::ITNS1_SPEC>;
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
pub mod itns1;
#[doc = "IPR0 (rw) register accessor: an alias for `Reg<IPR0_SPEC>`"]
pub type IPR0 = crate::Reg<ipr0::IPR0_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr0;
#[doc = "IPR1 (rw) register accessor: an alias for `Reg<IPR1_SPEC>`"]
pub type IPR1 = crate::Reg<ipr1::IPR1_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr1;
#[doc = "IPR2 (rw) register accessor: an alias for `Reg<IPR2_SPEC>`"]
pub type IPR2 = crate::Reg<ipr2::IPR2_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr2;
#[doc = "IPR3 (rw) register accessor: an alias for `Reg<IPR3_SPEC>`"]
pub type IPR3 = crate::Reg<ipr3::IPR3_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr3;
#[doc = "IPR4 (rw) register accessor: an alias for `Reg<IPR4_SPEC>`"]
pub type IPR4 = crate::Reg<ipr4::IPR4_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr4;
#[doc = "IPR5 (rw) register accessor: an alias for `Reg<IPR5_SPEC>`"]
pub type IPR5 = crate::Reg<ipr5::IPR5_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr5;
#[doc = "IPR6 (rw) register accessor: an alias for `Reg<IPR6_SPEC>`"]
pub type IPR6 = crate::Reg<ipr6::IPR6_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr6;
#[doc = "IPR7 (rw) register accessor: an alias for `Reg<IPR7_SPEC>`"]
pub type IPR7 = crate::Reg<ipr7::IPR7_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr7;
#[doc = "IPR8 (rw) register accessor: an alias for `Reg<IPR8_SPEC>`"]
pub type IPR8 = crate::Reg<ipr8::IPR8_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr8;
#[doc = "IPR9 (rw) register accessor: an alias for `Reg<IPR9_SPEC>`"]
pub type IPR9 = crate::Reg<ipr9::IPR9_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr9;
#[doc = "IPR10 (rw) register accessor: an alias for `Reg<IPR10_SPEC>`"]
pub type IPR10 = crate::Reg<ipr10::IPR10_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr10;
#[doc = "IPR11 (rw) register accessor: an alias for `Reg<IPR11_SPEC>`"]
pub type IPR11 = crate::Reg<ipr11::IPR11_SPEC>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr11;
