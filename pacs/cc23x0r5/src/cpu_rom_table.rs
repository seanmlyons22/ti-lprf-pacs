#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    entry_scs: EntryScs,
    entry_dwt: EntryDwt,
    entry_bpu: EntryBpu,
    entry_end: EntryEnd,
    _reserved4: [u8; 0x0fbc],
    memtype: Memtype,
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
    #[doc = "0x00 - Points to the System Control Space (SCS) at 0xE000E000. This includes core debug control registers."]
    #[inline(always)]
    pub const fn entry_scs(&self) -> &EntryScs {
        &self.entry_scs
    }
    #[doc = "0x04 - Points to the DW unit at 0xE0001000."]
    #[inline(always)]
    pub const fn entry_dwt(&self) -> &EntryDwt {
        &self.entry_dwt
    }
    #[doc = "0x08 - Points to the BPU at 0xE0002000."]
    #[inline(always)]
    pub const fn entry_bpu(&self) -> &EntryBpu {
        &self.entry_bpu
    }
    #[doc = "0x0c - Marks of end of table."]
    #[inline(always)]
    pub const fn entry_end(&self) -> &EntryEnd {
        &self.entry_end
    }
    #[doc = "0xfcc - Identifies the type of memory present on the bus that connects the DAP to the ROM Table."]
    #[inline(always)]
    pub const fn memtype(&self) -> &Memtype {
        &self.memtype
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
#[doc = "EntrySCS (rw) register accessor: Points to the System Control Space (SCS) at 0xE000E000. This includes core debug control registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_scs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_scs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry_scs`]
module"]
#[doc(alias = "EntrySCS")]
pub type EntryScs = crate::Reg<entry_scs::EntryScsSpec>;
#[doc = "Points to the System Control Space (SCS) at 0xE000E000. This includes core debug control registers."]
pub mod entry_scs;
#[doc = "EntryDWT (rw) register accessor: Points to the DW unit at 0xE0001000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_dwt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_dwt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry_dwt`]
module"]
#[doc(alias = "EntryDWT")]
pub type EntryDwt = crate::Reg<entry_dwt::EntryDwtSpec>;
#[doc = "Points to the DW unit at 0xE0001000."]
pub mod entry_dwt;
#[doc = "EntryBPU (rw) register accessor: Points to the BPU at 0xE0002000.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_bpu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_bpu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry_bpu`]
module"]
#[doc(alias = "EntryBPU")]
pub type EntryBpu = crate::Reg<entry_bpu::EntryBpuSpec>;
#[doc = "Points to the BPU at 0xE0002000."]
pub mod entry_bpu;
#[doc = "EntryEnd (rw) register accessor: Marks of end of table.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`entry_end::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`entry_end::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry_end`]
module"]
pub type EntryEnd = crate::Reg<entry_end::EntryEndSpec>;
#[doc = "Marks of end of table."]
pub mod entry_end;
#[doc = "MEMTYPE (rw) register accessor: Identifies the type of memory present on the bus that connects the DAP to the ROM Table.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`memtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`memtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@memtype`]
module"]
#[doc(alias = "MEMTYPE")]
pub type Memtype = crate::Reg<memtype::MemtypeSpec>;
#[doc = "Identifies the type of memory present on the bus that connects the DAP to the ROM Table."]
pub mod memtype;
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
