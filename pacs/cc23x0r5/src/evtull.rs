#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    _reserved2: [u8; 0x5c],
    dtb: Dtb,
    _reserved3: [u8; 0x0398],
    nmisel: Nmisel,
    rtccptsel: Rtccptsel,
    _reserved5: [u8; 0x03f8],
    wkupmask: Wkupmask,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Extended Description Register. This register provides configuration details of the IP to software drivers and end users."]
    #[inline(always)]
    pub const fn descex(&self) -> &Descex {
        &self.descex
    }
    #[doc = "0x64 - Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
    #[inline(always)]
    pub const fn dtb(&self) -> &Dtb {
        &self.dtb
    }
    #[doc = "0x400 - Output Selection for CPU NMI Interrupt"]
    #[inline(always)]
    pub const fn nmisel(&self) -> &Nmisel {
        &self.nmisel
    }
    #[doc = "0x404 - Output Selection for CPU Interrupt RTCCPT"]
    #[inline(always)]
    pub const fn rtccptsel(&self) -> &Rtccptsel {
        &self.rtccptsel
    }
    #[doc = "0x800 - WAKEUP Mask"]
    #[inline(always)]
    pub const fn wkupmask(&self) -> &Wkupmask {
        &self.wkupmask
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "DESCEX (rw) register accessor: Extended Description Register. This register provides configuration details of the IP to software drivers and end users.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex`]
module"]
#[doc(alias = "DESCEX")]
pub type Descex = crate::Reg<descex::DescexSpec>;
#[doc = "Extended Description Register. This register provides configuration details of the IP to software drivers and end users."]
pub mod descex;
#[doc = "DTB (rw) register accessor: Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb`]
module"]
#[doc(alias = "DTB")]
pub type Dtb = crate::Reg<dtb::DtbSpec>;
#[doc = "Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
pub mod dtb;
#[doc = "NMISEL (rw) register accessor: Output Selection for CPU NMI Interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nmisel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nmisel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nmisel`]
module"]
#[doc(alias = "NMISEL")]
pub type Nmisel = crate::Reg<nmisel::NmiselSpec>;
#[doc = "Output Selection for CPU NMI Interrupt"]
pub mod nmisel;
#[doc = "RTCCPTSEL (rw) register accessor: Output Selection for CPU Interrupt RTCCPT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtccptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtccptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtccptsel`]
module"]
#[doc(alias = "RTCCPTSEL")]
pub type Rtccptsel = crate::Reg<rtccptsel::RtccptselSpec>;
#[doc = "Output Selection for CPU Interrupt RTCCPT"]
pub mod rtccptsel;
#[doc = "WKUPMASK (rw) register accessor: WAKEUP Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkupmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkupmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkupmask`]
module"]
#[doc(alias = "WKUPMASK")]
pub type Wkupmask = crate::Reg<wkupmask::WkupmaskSpec>;
#[doc = "WAKEUP Mask"]
pub mod wkupmask;
