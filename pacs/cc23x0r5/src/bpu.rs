#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bp_ctrl: BpCtrl,
    _reserved1: [u8; 0x04],
    bp_comp0: BpComp0,
    bp_comp1: BpComp1,
    bp_comp2: BpComp2,
    bp_comp3: BpComp3,
    _reserved5: [u8; 0x0fb8],
    pidr4: Pidr4,
    pidr5: Pidr5,
    pidr6: Pidr6,
    pidr7: Pidr7,
    pidr0: Pidr0,
    pidr1: Pidr1,
    pidr2: Pidr2,
    pidr3: Pidr3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Use the Breakpoint Control Register to enable the Breakpoint block"]
    #[inline(always)]
    pub const fn bp_ctrl(&self) -> &BpCtrl {
        &self.bp_ctrl
    }
    #[doc = "0x08 - Use the Breakpoint Comparator Registers to store the values to compare with the instruction address."]
    #[inline(always)]
    pub const fn bp_comp0(&self) -> &BpComp0 {
        &self.bp_comp0
    }
    #[doc = "0x0c - Use the Breakpoint Comparator Registers to store the values to compare with the instruction address."]
    #[inline(always)]
    pub const fn bp_comp1(&self) -> &BpComp1 {
        &self.bp_comp1
    }
    #[doc = "0x10 - Use the Breakpoint Comparator Registers to store the values to compare with the PC address."]
    #[inline(always)]
    pub const fn bp_comp2(&self) -> &BpComp2 {
        &self.bp_comp2
    }
    #[doc = "0x14 - Use the Breakpoint Comparator Registers to store the values to compare with the instruction address."]
    #[inline(always)]
    pub const fn bp_comp3(&self) -> &BpComp3 {
        &self.bp_comp3
    }
    #[doc = "0xfd0 - Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator."]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - Reserved"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - Reserved"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - Reserved"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
    }
    #[doc = "0xfe0 - Part of the set of Peripheral Identification registers. Contains part of the designer specific part number."]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity."]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision."]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields."]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - A component identification register, that indicates that the identification registers are present. This register also indicates the component class."]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - A component identification register, that indicates that the identification registers are present."]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "BP_CTRL (rw) register accessor: Use the Breakpoint Control Register to enable the Breakpoint block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_ctrl`]
module"]
#[doc(alias = "BP_CTRL")]
pub type BpCtrl = crate::Reg<bp_ctrl::BpCtrlSpec>;
#[doc = "Use the Breakpoint Control Register to enable the Breakpoint block"]
pub mod bp_ctrl;
#[doc = "BP_COMP0 (rw) register accessor: Use the Breakpoint Comparator Registers to store the values to compare with the instruction address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_comp0`]
module"]
#[doc(alias = "BP_COMP0")]
pub type BpComp0 = crate::Reg<bp_comp0::BpComp0Spec>;
#[doc = "Use the Breakpoint Comparator Registers to store the values to compare with the instruction address."]
pub mod bp_comp0;
#[doc = "BP_COMP1 (rw) register accessor: Use the Breakpoint Comparator Registers to store the values to compare with the instruction address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_comp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_comp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_comp1`]
module"]
#[doc(alias = "BP_COMP1")]
pub type BpComp1 = crate::Reg<bp_comp1::BpComp1Spec>;
#[doc = "Use the Breakpoint Comparator Registers to store the values to compare with the instruction address."]
pub mod bp_comp1;
#[doc = "BP_COMP2 (rw) register accessor: Use the Breakpoint Comparator Registers to store the values to compare with the PC address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_comp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_comp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_comp2`]
module"]
#[doc(alias = "BP_COMP2")]
pub type BpComp2 = crate::Reg<bp_comp2::BpComp2Spec>;
#[doc = "Use the Breakpoint Comparator Registers to store the values to compare with the PC address."]
pub mod bp_comp2;
#[doc = "BP_COMP3 (rw) register accessor: Use the Breakpoint Comparator Registers to store the values to compare with the instruction address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bp_comp3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bp_comp3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bp_comp3`]
module"]
#[doc(alias = "BP_COMP3")]
pub type BpComp3 = crate::Reg<bp_comp3::BpComp3Spec>;
#[doc = "Use the Breakpoint Comparator Registers to store the values to compare with the instruction address."]
pub mod bp_comp3;
#[doc = "PIDR4 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`]
module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the memory footprint indicator."]
pub mod pidr4;
#[doc = "PIDR5 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`]
module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "Reserved"]
pub mod pidr5;
#[doc = "PIDR6 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`]
module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "Reserved"]
pub mod pidr6;
#[doc = "PIDR7 (rw) register accessor: Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`]
module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "Reserved"]
pub mod pidr7;
#[doc = "PIDR0 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer specific part number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`]
module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number."]
pub mod pidr0;
#[doc = "PIDR1 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`]
module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer specific part number and part of the designer identity."]
pub mod pidr1;
#[doc = "PIDR2 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`]
module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains part of the designer identity and the product revision."]
pub mod pidr2;
#[doc = "PIDR3 (rw) register accessor: Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`]
module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "Part of the set of Peripheral Identification registers. Contains the RevAnd and Customer Modified fields."]
pub mod pidr3;
#[doc = "CIDR0 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`]
module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: A component identification register, that indicates that the identification registers are present. This register also indicates the component class.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`]
module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "A component identification register, that indicates that the identification registers are present. This register also indicates the component class."]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`]
module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: A component identification register, that indicates that the identification registers are present.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`]
module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "A component identification register, that indicates that the identification registers are present."]
pub mod cidr3;
