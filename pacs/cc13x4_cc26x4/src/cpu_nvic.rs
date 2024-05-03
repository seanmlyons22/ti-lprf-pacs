#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iser0: Iser0,
    iser1: Iser1,
    _reserved2: [u8; 0x78],
    icer0: Icer0,
    icer1: Icer1,
    _reserved4: [u8; 0x78],
    ispr0: Ispr0,
    ispr1: Ispr1,
    _reserved6: [u8; 0x78],
    icpr0: Icpr0,
    icpr1: Icpr1,
    _reserved8: [u8; 0x78],
    iabr0: Iabr0,
    iabr1: Iabr1,
    _reserved10: [u8; 0x78],
    itns0: Itns0,
    itns1: Itns1,
    _reserved12: [u8; 0x78],
    ipr0: Ipr0,
    ipr1: Ipr1,
    ipr2: Ipr2,
    ipr3: Ipr3,
    ipr4: Ipr4,
    ipr5: Ipr5,
    ipr6: Ipr6,
    ipr7: Ipr7,
    ipr8: Ipr8,
    ipr9: Ipr9,
    ipr10: Ipr10,
    ipr11: Ipr11,
}
impl RegisterBlock {
    #[doc = "0x00 - Enables or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn iser0(&self) -> &Iser0 {
        &self.iser0
    }
    #[doc = "0x04 - Enables or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn iser1(&self) -> &Iser1 {
        &self.iser1
    }
    #[doc = "0x80 - Clears or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn icer0(&self) -> &Icer0 {
        &self.icer0
    }
    #[doc = "0x84 - Clears or reads the enabled state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn icer1(&self) -> &Icer1 {
        &self.icer1
    }
    #[doc = "0x100 - Enables or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn ispr0(&self) -> &Ispr0 {
        &self.ispr0
    }
    #[doc = "0x104 - Enables or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn ispr1(&self) -> &Ispr1 {
        &self.ispr1
    }
    #[doc = "0x180 - Clears or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn icpr0(&self) -> &Icpr0 {
        &self.icpr0
    }
    #[doc = "0x184 - Clears or reads the pending state of each group of 32 interrupts"]
    #[inline(always)]
    pub const fn icpr1(&self) -> &Icpr1 {
        &self.icpr1
    }
    #[doc = "0x200 - For each group of 32 interrupts, shows the active state of each interrupt"]
    #[inline(always)]
    pub const fn iabr0(&self) -> &Iabr0 {
        &self.iabr0
    }
    #[doc = "0x204 - For each group of 32 interrupts, shows the active state of each interrupt"]
    #[inline(always)]
    pub const fn iabr1(&self) -> &Iabr1 {
        &self.iabr1
    }
    #[doc = "0x280 - For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
    #[inline(always)]
    pub const fn itns0(&self) -> &Itns0 {
        &self.itns0
    }
    #[doc = "0x284 - For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
    #[inline(always)]
    pub const fn itns1(&self) -> &Itns1 {
        &self.itns1
    }
    #[doc = "0x300 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr0(&self) -> &Ipr0 {
        &self.ipr0
    }
    #[doc = "0x304 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr1(&self) -> &Ipr1 {
        &self.ipr1
    }
    #[doc = "0x308 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr2(&self) -> &Ipr2 {
        &self.ipr2
    }
    #[doc = "0x30c - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr3(&self) -> &Ipr3 {
        &self.ipr3
    }
    #[doc = "0x310 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr4(&self) -> &Ipr4 {
        &self.ipr4
    }
    #[doc = "0x314 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr5(&self) -> &Ipr5 {
        &self.ipr5
    }
    #[doc = "0x318 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr6(&self) -> &Ipr6 {
        &self.ipr6
    }
    #[doc = "0x31c - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr7(&self) -> &Ipr7 {
        &self.ipr7
    }
    #[doc = "0x320 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr8(&self) -> &Ipr8 {
        &self.ipr8
    }
    #[doc = "0x324 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr9(&self) -> &Ipr9 {
        &self.ipr9
    }
    #[doc = "0x328 - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr10(&self) -> &Ipr10 {
        &self.ipr10
    }
    #[doc = "0x32c - Sets or reads interrupt priorities"]
    #[inline(always)]
    pub const fn ipr11(&self) -> &Ipr11 {
        &self.ipr11
    }
}
#[doc = "ISER0 (rw) register accessor: Enables or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iser0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iser0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iser0`]
module"]
#[doc(alias = "ISER0")]
pub type Iser0 = crate::Reg<iser0::Iser0Spec>;
#[doc = "Enables or reads the enabled state of each group of 32 interrupts"]
pub mod iser0;
#[doc = "ISER1 (rw) register accessor: Enables or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iser1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iser1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iser1`]
module"]
#[doc(alias = "ISER1")]
pub type Iser1 = crate::Reg<iser1::Iser1Spec>;
#[doc = "Enables or reads the enabled state of each group of 32 interrupts"]
pub mod iser1;
#[doc = "ICER0 (rw) register accessor: Clears or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icer0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icer0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icer0`]
module"]
#[doc(alias = "ICER0")]
pub type Icer0 = crate::Reg<icer0::Icer0Spec>;
#[doc = "Clears or reads the enabled state of each group of 32 interrupts"]
pub mod icer0;
#[doc = "ICER1 (rw) register accessor: Clears or reads the enabled state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icer1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icer1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icer1`]
module"]
#[doc(alias = "ICER1")]
pub type Icer1 = crate::Reg<icer1::Icer1Spec>;
#[doc = "Clears or reads the enabled state of each group of 32 interrupts"]
pub mod icer1;
#[doc = "ISPR0 (rw) register accessor: Enables or reads the pending state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ispr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ispr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ispr0`]
module"]
#[doc(alias = "ISPR0")]
pub type Ispr0 = crate::Reg<ispr0::Ispr0Spec>;
#[doc = "Enables or reads the pending state of each group of 32 interrupts"]
pub mod ispr0;
#[doc = "ISPR1 (rw) register accessor: Enables or reads the pending state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ispr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ispr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ispr1`]
module"]
#[doc(alias = "ISPR1")]
pub type Ispr1 = crate::Reg<ispr1::Ispr1Spec>;
#[doc = "Enables or reads the pending state of each group of 32 interrupts"]
pub mod ispr1;
#[doc = "ICPR0 (rw) register accessor: Clears or reads the pending state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icpr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icpr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpr0`]
module"]
#[doc(alias = "ICPR0")]
pub type Icpr0 = crate::Reg<icpr0::Icpr0Spec>;
#[doc = "Clears or reads the pending state of each group of 32 interrupts"]
pub mod icpr0;
#[doc = "ICPR1 (rw) register accessor: Clears or reads the pending state of each group of 32 interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icpr1`]
module"]
#[doc(alias = "ICPR1")]
pub type Icpr1 = crate::Reg<icpr1::Icpr1Spec>;
#[doc = "Clears or reads the pending state of each group of 32 interrupts"]
pub mod icpr1;
#[doc = "IABR0 (rw) register accessor: For each group of 32 interrupts, shows the active state of each interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iabr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iabr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iabr0`]
module"]
#[doc(alias = "IABR0")]
pub type Iabr0 = crate::Reg<iabr0::Iabr0Spec>;
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt"]
pub mod iabr0;
#[doc = "IABR1 (rw) register accessor: For each group of 32 interrupts, shows the active state of each interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iabr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iabr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iabr1`]
module"]
#[doc(alias = "IABR1")]
pub type Iabr1 = crate::Reg<iabr1::Iabr1Spec>;
#[doc = "For each group of 32 interrupts, shows the active state of each interrupt"]
pub mod iabr1;
#[doc = "ITNS0 (rw) register accessor: For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itns0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itns0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itns0`]
module"]
#[doc(alias = "ITNS0")]
pub type Itns0 = crate::Reg<itns0::Itns0Spec>;
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
pub mod itns0;
#[doc = "ITNS1 (rw) register accessor: For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itns1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itns1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itns1`]
module"]
#[doc(alias = "ITNS1")]
pub type Itns1 = crate::Reg<itns1::Itns1Spec>;
#[doc = "For each group of 32 interrupts, determines whether each interrupt targets Non-secure or Secure state"]
pub mod itns1;
#[doc = "IPR0 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr0`]
module"]
#[doc(alias = "IPR0")]
pub type Ipr0 = crate::Reg<ipr0::Ipr0Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr0;
#[doc = "IPR1 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr1`]
module"]
#[doc(alias = "IPR1")]
pub type Ipr1 = crate::Reg<ipr1::Ipr1Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr1;
#[doc = "IPR2 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr2`]
module"]
#[doc(alias = "IPR2")]
pub type Ipr2 = crate::Reg<ipr2::Ipr2Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr2;
#[doc = "IPR3 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr3`]
module"]
#[doc(alias = "IPR3")]
pub type Ipr3 = crate::Reg<ipr3::Ipr3Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr3;
#[doc = "IPR4 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr4`]
module"]
#[doc(alias = "IPR4")]
pub type Ipr4 = crate::Reg<ipr4::Ipr4Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr4;
#[doc = "IPR5 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr5`]
module"]
#[doc(alias = "IPR5")]
pub type Ipr5 = crate::Reg<ipr5::Ipr5Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr5;
#[doc = "IPR6 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr6`]
module"]
#[doc(alias = "IPR6")]
pub type Ipr6 = crate::Reg<ipr6::Ipr6Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr6;
#[doc = "IPR7 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr7`]
module"]
#[doc(alias = "IPR7")]
pub type Ipr7 = crate::Reg<ipr7::Ipr7Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr7;
#[doc = "IPR8 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr8`]
module"]
#[doc(alias = "IPR8")]
pub type Ipr8 = crate::Reg<ipr8::Ipr8Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr8;
#[doc = "IPR9 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr9`]
module"]
#[doc(alias = "IPR9")]
pub type Ipr9 = crate::Reg<ipr9::Ipr9Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr9;
#[doc = "IPR10 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr10`]
module"]
#[doc(alias = "IPR10")]
pub type Ipr10 = crate::Reg<ipr10::Ipr10Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr10;
#[doc = "IPR11 (rw) register accessor: Sets or reads interrupt priorities\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ipr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipr11`]
module"]
#[doc(alias = "IPR11")]
pub type Ipr11 = crate::Reg<ipr11::Ipr11Spec>;
#[doc = "Sets or reads interrupt priorities"]
pub mod ipr11;
