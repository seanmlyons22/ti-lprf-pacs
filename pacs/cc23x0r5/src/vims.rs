#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    flws1t: Flws1t,
    flws2t: Flws2t,
    _reserved4: [u8; 0x08],
    ptrmc0: Ptrmc0,
    b0trmc1: B0trmc1,
    b0trmc0: B0trmc0,
    _reserved7: [u8; 0xdc],
    flblck: Flblck,
    _reserved8: [u8; 0x02f8],
    cfg: Cfg,
    rdprmn: Rdprmn,
    rdprnmn: Rdprnmn,
    rdprtrm: Rdprtrm,
    rdpregr: Rdpregr,
    wepra: Wepra,
    weprb: Weprb,
    _reserved15: [u8; 0x04],
    wepraux: Wepraux,
    flbstat: Flbstat,
    cchctrl: Cchctrl,
    _reserved18: [u8; 0x03d8],
    dtb: Dtb,
}
impl RegisterBlock {
    #[doc = "0x00 - This register identifies the peripheral and its exact version."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - This register describes the configuration of VIMS."]
    #[inline(always)]
    pub const fn descex(&self) -> &Descex {
        &self.descex
    }
    #[doc = "0x08 - This register is used to specify the number of waitstates necessary for accessing the flash in 1T mode. This register is retained."]
    #[inline(always)]
    pub const fn flws1t(&self) -> &Flws1t {
        &self.flws1t
    }
    #[doc = "0x0c - This register is used to specify the number of waitstates necessary for accessing the flash in 2T mode. This register is retained."]
    #[inline(always)]
    pub const fn flws2t(&self) -> &Flws2t {
        &self.flws2t
    }
    #[doc = "0x18 - Stores FLASH Pump trim values. This register is retained."]
    #[inline(always)]
    pub const fn ptrmc0(&self) -> &Ptrmc0 {
        &self.ptrmc0
    }
    #[doc = "0x1c - This register is used to store flash bank 0 trim value 1. This register is retained."]
    #[inline(always)]
    pub const fn b0trmc1(&self) -> &B0trmc1 {
        &self.b0trmc1
    }
    #[doc = "0x20 - This register is used to store flash bank 0 trim value 0. This register is retained."]
    #[inline(always)]
    pub const fn b0trmc0(&self) -> &B0trmc0 {
        &self.b0trmc0
    }
    #[doc = "0x100 - This register is used to block user read, write and erase operation to flash. This register is sticky when written with value 1. This register is retained."]
    #[inline(always)]
    pub const fn flblck(&self) -> &Flblck {
        &self.flblck
    }
    #[doc = "0x3fc - This register is used for flash configuration. This register is retained."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x400 - Flash main region read protection register upto first 8KB. This register is sticky when written with value 0. This register is retained."]
    #[inline(always)]
    pub const fn rdprmn(&self) -> &Rdprmn {
        &self.rdprmn
    }
    #[doc = "0x404 - Flash non main region read protection register for last 512 B. This register is sticky when written with value 0. This register is retained."]
    #[inline(always)]
    pub const fn rdprnmn(&self) -> &Rdprnmn {
        &self.rdprnmn
    }
    #[doc = "0x408 - Flash trim region read protection register for last 512 B. This register is sticky when written with value 0. This register is retained."]
    #[inline(always)]
    pub const fn rdprtrm(&self) -> &Rdprtrm {
        &self.rdprtrm
    }
    #[doc = "0x40c - Flash engr region read protection register. This register is sticky when written with value 0. This register is retained"]
    #[inline(always)]
    pub const fn rdpregr(&self) -> &Rdpregr {
        &self.rdpregr
    }
    #[doc = "0x410 - Flash main region write/erase protection for first 32 sectors. Nth bit corresponds to the Nth sector. This register is sticky when written with value 0. This register is retained."]
    #[inline(always)]
    pub const fn wepra(&self) -> &Wepra {
        &self.wepra
    }
    #[doc = "0x414 - Flash main region write/erase protection for remaining sectors. Each bit corresponds to 8 sectors. Bit 0 corresponds to sector 32-39, bit 1 corresponds to sector 40-47 and so on. This register is sticky when written with value 0. This register is retained."]
    #[inline(always)]
    pub const fn weprb(&self) -> &Weprb {
        &self.weprb
    }
    #[doc = "0x41c - Flash Write/Erase protection for Non-Main, TRIM and ENGR Regions. This register is sticky when written with value 0. This register is retained."]
    #[inline(always)]
    pub const fn wepraux(&self) -> &Wepraux {
        &self.wepraux
    }
    #[doc = "0x420 - This register is used to indicate status of flash."]
    #[inline(always)]
    pub const fn flbstat(&self) -> &Flbstat {
        &self.flbstat
    }
    #[doc = "0x424 - This register is used for enabling cache, prefetch $amp;amp; micropredictor units. This register is retained"]
    #[inline(always)]
    pub const fn cchctrl(&self) -> &Cchctrl {
        &self.cchctrl
    }
    #[doc = "0x800 - Digital test bus mux selection"]
    #[inline(always)]
    pub const fn dtb(&self) -> &Dtb {
        &self.dtb
    }
}
#[doc = "DESC (rw) register accessor: This register identifies the peripheral and its exact version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "This register identifies the peripheral and its exact version."]
pub mod desc;
#[doc = "DESCEX (rw) register accessor: This register describes the configuration of VIMS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex`]
module"]
#[doc(alias = "DESCEX")]
pub type Descex = crate::Reg<descex::DescexSpec>;
#[doc = "This register describes the configuration of VIMS."]
pub mod descex;
#[doc = "FLWS1T (rw) register accessor: This register is used to specify the number of waitstates necessary for accessing the flash in 1T mode. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flws1t::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flws1t::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flws1t`]
module"]
#[doc(alias = "FLWS1T")]
pub type Flws1t = crate::Reg<flws1t::Flws1tSpec>;
#[doc = "This register is used to specify the number of waitstates necessary for accessing the flash in 1T mode. This register is retained."]
pub mod flws1t;
#[doc = "FLWS2T (rw) register accessor: This register is used to specify the number of waitstates necessary for accessing the flash in 2T mode. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flws2t::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flws2t::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flws2t`]
module"]
#[doc(alias = "FLWS2T")]
pub type Flws2t = crate::Reg<flws2t::Flws2tSpec>;
#[doc = "This register is used to specify the number of waitstates necessary for accessing the flash in 2T mode. This register is retained."]
pub mod flws2t;
#[doc = "PTRMC0 (rw) register accessor: Stores FLASH Pump trim values. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ptrmc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ptrmc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptrmc0`]
module"]
#[doc(alias = "PTRMC0")]
pub type Ptrmc0 = crate::Reg<ptrmc0::Ptrmc0Spec>;
#[doc = "Stores FLASH Pump trim values. This register is retained."]
pub mod ptrmc0;
#[doc = "B0TRMC1 (rw) register accessor: This register is used to store flash bank 0 trim value 1. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b0trmc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b0trmc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0trmc1`]
module"]
#[doc(alias = "B0TRMC1")]
pub type B0trmc1 = crate::Reg<b0trmc1::B0trmc1Spec>;
#[doc = "This register is used to store flash bank 0 trim value 1. This register is retained."]
pub mod b0trmc1;
#[doc = "B0TRMC0 (rw) register accessor: This register is used to store flash bank 0 trim value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`b0trmc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`b0trmc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@b0trmc0`]
module"]
#[doc(alias = "B0TRMC0")]
pub type B0trmc0 = crate::Reg<b0trmc0::B0trmc0Spec>;
#[doc = "This register is used to store flash bank 0 trim value 0. This register is retained."]
pub mod b0trmc0;
#[doc = "FLBLCK (rw) register accessor: This register is used to block user read, write and erase operation to flash. This register is sticky when written with value 1. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flblck::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flblck::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flblck`]
module"]
#[doc(alias = "FLBLCK")]
pub type Flblck = crate::Reg<flblck::FlblckSpec>;
#[doc = "This register is used to block user read, write and erase operation to flash. This register is sticky when written with value 1. This register is retained."]
pub mod flblck;
#[doc = "CFG (rw) register accessor: This register is used for flash configuration. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "This register is used for flash configuration. This register is retained."]
pub mod cfg;
#[doc = "RDPRMN (rw) register accessor: Flash main region read protection register upto first 8KB. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdprmn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdprmn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdprmn`]
module"]
#[doc(alias = "RDPRMN")]
pub type Rdprmn = crate::Reg<rdprmn::RdprmnSpec>;
#[doc = "Flash main region read protection register upto first 8KB. This register is sticky when written with value 0. This register is retained."]
pub mod rdprmn;
#[doc = "RDPRNMN (rw) register accessor: Flash non main region read protection register for last 512 B. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdprnmn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdprnmn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdprnmn`]
module"]
#[doc(alias = "RDPRNMN")]
pub type Rdprnmn = crate::Reg<rdprnmn::RdprnmnSpec>;
#[doc = "Flash non main region read protection register for last 512 B. This register is sticky when written with value 0. This register is retained."]
pub mod rdprnmn;
#[doc = "RDPRTRM (rw) register accessor: Flash trim region read protection register for last 512 B. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdprtrm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdprtrm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdprtrm`]
module"]
#[doc(alias = "RDPRTRM")]
pub type Rdprtrm = crate::Reg<rdprtrm::RdprtrmSpec>;
#[doc = "Flash trim region read protection register for last 512 B. This register is sticky when written with value 0. This register is retained."]
pub mod rdprtrm;
#[doc = "RDPREGR (rw) register accessor: Flash engr region read protection register. This register is sticky when written with value 0. This register is retained\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdpregr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdpregr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdpregr`]
module"]
#[doc(alias = "RDPREGR")]
pub type Rdpregr = crate::Reg<rdpregr::RdpregrSpec>;
#[doc = "Flash engr region read protection register. This register is sticky when written with value 0. This register is retained"]
pub mod rdpregr;
#[doc = "WEPRA (rw) register accessor: Flash main region write/erase protection for first 32 sectors. Nth bit corresponds to the Nth sector. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wepra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wepra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wepra`]
module"]
#[doc(alias = "WEPRA")]
pub type Wepra = crate::Reg<wepra::WepraSpec>;
#[doc = "Flash main region write/erase protection for first 32 sectors. Nth bit corresponds to the Nth sector. This register is sticky when written with value 0. This register is retained."]
pub mod wepra;
#[doc = "WEPRB (rw) register accessor: Flash main region write/erase protection for remaining sectors. Each bit corresponds to 8 sectors. Bit 0 corresponds to sector 32-39, bit 1 corresponds to sector 40-47 and so on. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`weprb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`weprb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@weprb`]
module"]
#[doc(alias = "WEPRB")]
pub type Weprb = crate::Reg<weprb::WeprbSpec>;
#[doc = "Flash main region write/erase protection for remaining sectors. Each bit corresponds to 8 sectors. Bit 0 corresponds to sector 32-39, bit 1 corresponds to sector 40-47 and so on. This register is sticky when written with value 0. This register is retained."]
pub mod weprb;
#[doc = "WEPRAUX (rw) register accessor: Flash Write/Erase protection for Non-Main, TRIM and ENGR Regions. This register is sticky when written with value 0. This register is retained.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wepraux::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wepraux::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wepraux`]
module"]
#[doc(alias = "WEPRAUX")]
pub type Wepraux = crate::Reg<wepraux::WeprauxSpec>;
#[doc = "Flash Write/Erase protection for Non-Main, TRIM and ENGR Regions. This register is sticky when written with value 0. This register is retained."]
pub mod wepraux;
#[doc = "FLBSTAT (rw) register accessor: This register is used to indicate status of flash.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flbstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flbstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flbstat`]
module"]
#[doc(alias = "FLBSTAT")]
pub type Flbstat = crate::Reg<flbstat::FlbstatSpec>;
#[doc = "This register is used to indicate status of flash."]
pub mod flbstat;
#[doc = "CCHCTRL (rw) register accessor: This register is used for enabling cache, prefetch $amp;amp; micropredictor units. This register is retained\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cchctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cchctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cchctrl`]
module"]
#[doc(alias = "CCHCTRL")]
pub type Cchctrl = crate::Reg<cchctrl::CchctrlSpec>;
#[doc = "This register is used for enabling cache, prefetch $amp;amp; micropredictor units. This register is retained"]
pub mod cchctrl;
#[doc = "DTB (rw) register accessor: Digital test bus mux selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb`]
module"]
#[doc(alias = "DTB")]
pub type Dtb = crate::Reg<dtb::DtbSpec>;
#[doc = "Digital test bus mux selection"]
pub mod dtb;
