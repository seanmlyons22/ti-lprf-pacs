#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    remap: Remap,
    comp0: Comp0,
    comp1: Comp1,
    comp2: Comp2,
    comp3: Comp3,
    comp4: Comp4,
    comp5: Comp5,
    comp6: Comp6,
    comp7: Comp7,
}
impl RegisterBlock {
    #[doc = "0x00 - Control This register is used to enable the flash patch block."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
    #[inline(always)]
    pub const fn remap(&self) -> &Remap {
        &self.remap
    }
    #[doc = "0x08 - Comparator 0"]
    #[inline(always)]
    pub const fn comp0(&self) -> &Comp0 {
        &self.comp0
    }
    #[doc = "0x0c - Comparator 1"]
    #[inline(always)]
    pub const fn comp1(&self) -> &Comp1 {
        &self.comp1
    }
    #[doc = "0x10 - Comparator 2"]
    #[inline(always)]
    pub const fn comp2(&self) -> &Comp2 {
        &self.comp2
    }
    #[doc = "0x14 - Comparator 3"]
    #[inline(always)]
    pub const fn comp3(&self) -> &Comp3 {
        &self.comp3
    }
    #[doc = "0x18 - Comparator 4"]
    #[inline(always)]
    pub const fn comp4(&self) -> &Comp4 {
        &self.comp4
    }
    #[doc = "0x1c - Comparator 5"]
    #[inline(always)]
    pub const fn comp5(&self) -> &Comp5 {
        &self.comp5
    }
    #[doc = "0x20 - Comparator 6"]
    #[inline(always)]
    pub const fn comp6(&self) -> &Comp6 {
        &self.comp6
    }
    #[doc = "0x24 - Comparator 7"]
    #[inline(always)]
    pub const fn comp7(&self) -> &Comp7 {
        &self.comp7
    }
}
#[doc = "CTRL (rw) register accessor: Control This register is used to enable the flash patch block.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control This register is used to enable the flash patch block."]
pub mod ctrl;
#[doc = "REMAP (rw) register accessor: Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap`]
module"]
#[doc(alias = "REMAP")]
pub type Remap = crate::Reg<remap::RemapSpec>;
#[doc = "Remap This register provides the remap base address location where a matched addresses are remapped. The three most significant bits and the five least significant bits of the remap base address are hard-coded to 3'b001 and 5'b00000 respectively. The remap base address must be in system space and is it required to be 8-word aligned, with one word allocated to each of the eight FPB comparators."]
pub mod remap;
#[doc = "COMP0 (rw) register accessor: Comparator 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0`]
module"]
#[doc(alias = "COMP0")]
pub type Comp0 = crate::Reg<comp0::Comp0Spec>;
#[doc = "Comparator 0"]
pub mod comp0;
#[doc = "COMP1 (rw) register accessor: Comparator 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp1`]
module"]
#[doc(alias = "COMP1")]
pub type Comp1 = crate::Reg<comp1::Comp1Spec>;
#[doc = "Comparator 1"]
pub mod comp1;
#[doc = "COMP2 (rw) register accessor: Comparator 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2`]
module"]
#[doc(alias = "COMP2")]
pub type Comp2 = crate::Reg<comp2::Comp2Spec>;
#[doc = "Comparator 2"]
pub mod comp2;
#[doc = "COMP3 (rw) register accessor: Comparator 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp3`]
module"]
#[doc(alias = "COMP3")]
pub type Comp3 = crate::Reg<comp3::Comp3Spec>;
#[doc = "Comparator 3"]
pub mod comp3;
#[doc = "COMP4 (rw) register accessor: Comparator 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp4`]
module"]
#[doc(alias = "COMP4")]
pub type Comp4 = crate::Reg<comp4::Comp4Spec>;
#[doc = "Comparator 4"]
pub mod comp4;
#[doc = "COMP5 (rw) register accessor: Comparator 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp5`]
module"]
#[doc(alias = "COMP5")]
pub type Comp5 = crate::Reg<comp5::Comp5Spec>;
#[doc = "Comparator 5"]
pub mod comp5;
#[doc = "COMP6 (rw) register accessor: Comparator 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp6`]
module"]
#[doc(alias = "COMP6")]
pub type Comp6 = crate::Reg<comp6::Comp6Spec>;
#[doc = "Comparator 6"]
pub mod comp6;
#[doc = "COMP7 (rw) register accessor: Comparator 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp7`]
module"]
#[doc(alias = "COMP7")]
pub type Comp7 = crate::Reg<comp7::Comp7Spec>;
#[doc = "Comparator 7"]
pub mod comp7;
