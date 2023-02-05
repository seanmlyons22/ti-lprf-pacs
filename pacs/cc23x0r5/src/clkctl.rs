#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex0: Descex0,
    descex1: Descex1,
    clkcfg0: Clkcfg0,
    clkcfg1: Clkcfg1,
    clkenset0: Clkenset0,
    clkenset1: Clkenset1,
    _reserved7: [u8; 0x04],
    clkenclr0: Clkenclr0,
    clkenclr1: Clkenclr1,
    _reserved9: [u8; 0x14],
    stbyptr: Stbyptr,
    _reserved10: [u8; 0x08],
    idlecfg: Idlecfg,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Extended Description Register 0. This register shows SVT IP availability, HW features and memory size configuration."]
    #[inline(always)]
    pub const fn descex0(&self) -> &Descex0 {
        &self.descex0
    }
    #[doc = "0x08 - Extended Description Register 1. This register shows SVT IP availability, HW features and memory size configuration."]
    #[inline(always)]
    pub const fn descex1(&self) -> &Descex1 {
        &self.descex1
    }
    #[doc = "0x0c - Clock Configuration Register 0. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET0 and CLKENCLR0."]
    #[inline(always)]
    pub const fn clkcfg0(&self) -> &Clkcfg0 {
        &self.clkcfg0
    }
    #[doc = "0x10 - Clock Configuration Register 1. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET1 and CLKENCLR1."]
    #[inline(always)]
    pub const fn clkcfg1(&self) -> &Clkcfg1 {
        &self.clkcfg1
    }
    #[doc = "0x14 - Clock Enable Set Register 0. This register enables IP clocks in the system. Used to set the corresponding fields in CLKCFG0 to 1."]
    #[inline(always)]
    pub const fn clkenset0(&self) -> &Clkenset0 {
        &self.clkenset0
    }
    #[doc = "0x18 - Clock Enable Set Register 1. This register enables IP clocks in the system. Used to set the corresponding fields in CLKCFG1 to 1."]
    #[inline(always)]
    pub const fn clkenset1(&self) -> &Clkenset1 {
        &self.clkenset1
    }
    #[doc = "0x20 - Clock Enable Clear Register 0. This register disables IP clocks in the system. Used to clear the corresponding fields in CLKCFG0 to 0."]
    #[inline(always)]
    pub const fn clkenclr0(&self) -> &Clkenclr0 {
        &self.clkenclr0
    }
    #[doc = "0x24 - Clock Enable Clear Register 1. This register disables IP clocks in the system. Used to clear the corresponding fields in CLKCFG1 to 0."]
    #[inline(always)]
    pub const fn clkenclr1(&self) -> &Clkenclr1 {
        &self.clkenclr1
    }
    #[doc = "0x3c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn stbyptr(&self) -> &Stbyptr {
        &self.stbyptr
    }
    #[doc = "0x48 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn idlecfg(&self) -> &Idlecfg {
        &self.idlecfg
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "DESCEX0 (rw) register accessor: Extended Description Register 0. This register shows SVT IP availability, HW features and memory size configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex0`]
module"]
#[doc(alias = "DESCEX0")]
pub type Descex0 = crate::Reg<descex0::Descex0Spec>;
#[doc = "Extended Description Register 0. This register shows SVT IP availability, HW features and memory size configuration."]
pub mod descex0;
#[doc = "DESCEX1 (rw) register accessor: Extended Description Register 1. This register shows SVT IP availability, HW features and memory size configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex1`]
module"]
#[doc(alias = "DESCEX1")]
pub type Descex1 = crate::Reg<descex1::Descex1Spec>;
#[doc = "Extended Description Register 1. This register shows SVT IP availability, HW features and memory size configuration."]
pub mod descex1;
#[doc = "CLKCFG0 (rw) register accessor: Clock Configuration Register 0. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET0 and CLKENCLR0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg0`]
module"]
#[doc(alias = "CLKCFG0")]
pub type Clkcfg0 = crate::Reg<clkcfg0::Clkcfg0Spec>;
#[doc = "Clock Configuration Register 0. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET0 and CLKENCLR0."]
pub mod clkcfg0;
#[doc = "CLKCFG1 (rw) register accessor: Clock Configuration Register 1. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET1 and CLKENCLR1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg1`]
module"]
#[doc(alias = "CLKCFG1")]
pub type Clkcfg1 = crate::Reg<clkcfg1::Clkcfg1Spec>;
#[doc = "Clock Configuration Register 1. This register shows the IP clock configuration for the system. The configuration is updated through CLKENSET1 and CLKENCLR1."]
pub mod clkcfg1;
#[doc = "CLKENSET0 (rw) register accessor: Clock Enable Set Register 0. This register enables IP clocks in the system. Used to set the corresponding fields in CLKCFG0 to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkenset0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkenset0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkenset0`]
module"]
#[doc(alias = "CLKENSET0")]
pub type Clkenset0 = crate::Reg<clkenset0::Clkenset0Spec>;
#[doc = "Clock Enable Set Register 0. This register enables IP clocks in the system. Used to set the corresponding fields in CLKCFG0 to 1."]
pub mod clkenset0;
#[doc = "CLKENSET1 (rw) register accessor: Clock Enable Set Register 1. This register enables IP clocks in the system. Used to set the corresponding fields in CLKCFG1 to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkenset1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkenset1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkenset1`]
module"]
#[doc(alias = "CLKENSET1")]
pub type Clkenset1 = crate::Reg<clkenset1::Clkenset1Spec>;
#[doc = "Clock Enable Set Register 1. This register enables IP clocks in the system. Used to set the corresponding fields in CLKCFG1 to 1."]
pub mod clkenset1;
#[doc = "CLKENCLR0 (rw) register accessor: Clock Enable Clear Register 0. This register disables IP clocks in the system. Used to clear the corresponding fields in CLKCFG0 to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkenclr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkenclr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkenclr0`]
module"]
#[doc(alias = "CLKENCLR0")]
pub type Clkenclr0 = crate::Reg<clkenclr0::Clkenclr0Spec>;
#[doc = "Clock Enable Clear Register 0. This register disables IP clocks in the system. Used to clear the corresponding fields in CLKCFG0 to 0."]
pub mod clkenclr0;
#[doc = "CLKENCLR1 (rw) register accessor: Clock Enable Clear Register 1. This register disables IP clocks in the system. Used to clear the corresponding fields in CLKCFG1 to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkenclr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkenclr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkenclr1`]
module"]
#[doc(alias = "CLKENCLR1")]
pub type Clkenclr1 = crate::Reg<clkenclr1::Clkenclr1Spec>;
#[doc = "Clock Enable Clear Register 1. This register disables IP clocks in the system. Used to clear the corresponding fields in CLKCFG1 to 0."]
pub mod clkenclr1;
#[doc = "STBYPTR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stbyptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stbyptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stbyptr`]
module"]
#[doc(alias = "STBYPTR")]
pub type Stbyptr = crate::Reg<stbyptr::StbyptrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod stbyptr;
#[doc = "IDLECFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idlecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idlecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idlecfg`]
module"]
#[doc(alias = "IDLECFG")]
pub type Idlecfg = crate::Reg<idlecfg::IdlecfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod idlecfg;
