#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    status: Status,
    cfg: Cfg,
    ctrl: Ctrl,
    altctrl: Altctrl,
    waitonreq: Waitonreq,
    softreq: Softreq,
    setburst: Setburst,
    clearburst: Clearburst,
    setreqmask: Setreqmask,
    clearreqmask: Clearreqmask,
    setchannelen: Setchannelen,
    clearchannelen: Clearchannelen,
    setchnlprialt: Setchnlprialt,
    clearchnlprialt: Clearchnlprialt,
    setchnlpriority: Setchnlpriority,
    clearchnlpriority: Clearchnlpriority,
    _reserved16: [u8; 0x0c],
    error: Error,
    _reserved17: [u8; 0x04b4],
    reqdone: Reqdone,
    _reserved18: [u8; 0x18],
    donemask: Donemask,
}
impl RegisterBlock {
    #[doc = "0x00 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x04 - Configuration"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Channel Control Data Base Pointer"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer"]
    #[inline(always)]
    pub const fn altctrl(&self) -> &Altctrl {
        &self.altctrl
    }
    #[doc = "0x10 - Channel Wait On Request Status"]
    #[inline(always)]
    pub const fn waitonreq(&self) -> &Waitonreq {
        &self.waitonreq
    }
    #[doc = "0x14 - Channel Software Request"]
    #[inline(always)]
    pub const fn softreq(&self) -> &Softreq {
        &self.softreq
    }
    #[doc = "0x18 - Channel Set UseBurst"]
    #[inline(always)]
    pub const fn setburst(&self) -> &Setburst {
        &self.setburst
    }
    #[doc = "0x1c - Channel Clear UseBurst"]
    #[inline(always)]
    pub const fn clearburst(&self) -> &Clearburst {
        &self.clearburst
    }
    #[doc = "0x20 - Channel Set Request Mask"]
    #[inline(always)]
    pub const fn setreqmask(&self) -> &Setreqmask {
        &self.setreqmask
    }
    #[doc = "0x24 - Clear Channel Request Mask"]
    #[inline(always)]
    pub const fn clearreqmask(&self) -> &Clearreqmask {
        &self.clearreqmask
    }
    #[doc = "0x28 - Set Channel Enable"]
    #[inline(always)]
    pub const fn setchannelen(&self) -> &Setchannelen {
        &self.setchannelen
    }
    #[doc = "0x2c - Clear Channel Enable"]
    #[inline(always)]
    pub const fn clearchannelen(&self) -> &Clearchannelen {
        &self.clearchannelen
    }
    #[doc = "0x30 - Channel Set Primary-Alternate"]
    #[inline(always)]
    pub const fn setchnlprialt(&self) -> &Setchnlprialt {
        &self.setchnlprialt
    }
    #[doc = "0x34 - Channel Clear Primary-Alternate"]
    #[inline(always)]
    pub const fn clearchnlprialt(&self) -> &Clearchnlprialt {
        &self.clearchnlprialt
    }
    #[doc = "0x38 - Set Channel Priority"]
    #[inline(always)]
    pub const fn setchnlpriority(&self) -> &Setchnlpriority {
        &self.setchnlpriority
    }
    #[doc = "0x3c - Clear Channel Priority"]
    #[inline(always)]
    pub const fn clearchnlpriority(&self) -> &Clearchnlpriority {
        &self.clearchnlpriority
    }
    #[doc = "0x4c - Error Status and Clear"]
    #[inline(always)]
    pub const fn error(&self) -> &Error {
        &self.error
    }
    #[doc = "0x504 - Channel Request Done"]
    #[inline(always)]
    pub const fn reqdone(&self) -> &Reqdone {
        &self.reqdone
    }
    #[doc = "0x520 - Channel Request Done Mask"]
    #[inline(always)]
    pub const fn donemask(&self) -> &Donemask {
        &self.donemask
    }
}
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "CFG (rw) register accessor: Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Configuration"]
pub mod cfg;
#[doc = "CTRL (rw) register accessor: Channel Control Data Base Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Channel Control Data Base Pointer"]
pub mod ctrl;
#[doc = "ALTCTRL (rw) register accessor: Channel Alternate Control Data Base Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`altctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`altctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@altctrl`]
module"]
#[doc(alias = "ALTCTRL")]
pub type Altctrl = crate::Reg<altctrl::AltctrlSpec>;
#[doc = "Channel Alternate Control Data Base Pointer"]
pub mod altctrl;
#[doc = "WAITONREQ (rw) register accessor: Channel Wait On Request Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`waitonreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`waitonreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@waitonreq`]
module"]
#[doc(alias = "WAITONREQ")]
pub type Waitonreq = crate::Reg<waitonreq::WaitonreqSpec>;
#[doc = "Channel Wait On Request Status"]
pub mod waitonreq;
#[doc = "SOFTREQ (rw) register accessor: Channel Software Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softreq`]
module"]
#[doc(alias = "SOFTREQ")]
pub type Softreq = crate::Reg<softreq::SoftreqSpec>;
#[doc = "Channel Software Request"]
pub mod softreq;
#[doc = "SETBURST (rw) register accessor: Channel Set UseBurst\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setburst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setburst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setburst`]
module"]
#[doc(alias = "SETBURST")]
pub type Setburst = crate::Reg<setburst::SetburstSpec>;
#[doc = "Channel Set UseBurst"]
pub mod setburst;
#[doc = "CLEARBURST (rw) register accessor: Channel Clear UseBurst\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearburst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearburst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearburst`]
module"]
#[doc(alias = "CLEARBURST")]
pub type Clearburst = crate::Reg<clearburst::ClearburstSpec>;
#[doc = "Channel Clear UseBurst"]
pub mod clearburst;
#[doc = "SETREQMASK (rw) register accessor: Channel Set Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setreqmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setreqmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setreqmask`]
module"]
#[doc(alias = "SETREQMASK")]
pub type Setreqmask = crate::Reg<setreqmask::SetreqmaskSpec>;
#[doc = "Channel Set Request Mask"]
pub mod setreqmask;
#[doc = "CLEARREQMASK (rw) register accessor: Clear Channel Request Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearreqmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearreqmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearreqmask`]
module"]
#[doc(alias = "CLEARREQMASK")]
pub type Clearreqmask = crate::Reg<clearreqmask::ClearreqmaskSpec>;
#[doc = "Clear Channel Request Mask"]
pub mod clearreqmask;
#[doc = "SETCHANNELEN (rw) register accessor: Set Channel Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setchannelen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setchannelen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setchannelen`]
module"]
#[doc(alias = "SETCHANNELEN")]
pub type Setchannelen = crate::Reg<setchannelen::SetchannelenSpec>;
#[doc = "Set Channel Enable"]
pub mod setchannelen;
#[doc = "CLEARCHANNELEN (rw) register accessor: Clear Channel Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearchannelen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearchannelen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearchannelen`]
module"]
#[doc(alias = "CLEARCHANNELEN")]
pub type Clearchannelen = crate::Reg<clearchannelen::ClearchannelenSpec>;
#[doc = "Clear Channel Enable"]
pub mod clearchannelen;
#[doc = "SETCHNLPRIALT (rw) register accessor: Channel Set Primary-Alternate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setchnlprialt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setchnlprialt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setchnlprialt`]
module"]
#[doc(alias = "SETCHNLPRIALT")]
pub type Setchnlprialt = crate::Reg<setchnlprialt::SetchnlprialtSpec>;
#[doc = "Channel Set Primary-Alternate"]
pub mod setchnlprialt;
#[doc = "CLEARCHNLPRIALT (rw) register accessor: Channel Clear Primary-Alternate\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearchnlprialt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearchnlprialt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearchnlprialt`]
module"]
#[doc(alias = "CLEARCHNLPRIALT")]
pub type Clearchnlprialt = crate::Reg<clearchnlprialt::ClearchnlprialtSpec>;
#[doc = "Channel Clear Primary-Alternate"]
pub mod clearchnlprialt;
#[doc = "SETCHNLPRIORITY (rw) register accessor: Set Channel Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`setchnlpriority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`setchnlpriority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@setchnlpriority`]
module"]
#[doc(alias = "SETCHNLPRIORITY")]
pub type Setchnlpriority = crate::Reg<setchnlpriority::SetchnlprioritySpec>;
#[doc = "Set Channel Priority"]
pub mod setchnlpriority;
#[doc = "CLEARCHNLPRIORITY (rw) register accessor: Clear Channel Priority\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clearchnlpriority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clearchnlpriority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clearchnlpriority`]
module"]
#[doc(alias = "CLEARCHNLPRIORITY")]
pub type Clearchnlpriority = crate::Reg<clearchnlpriority::ClearchnlprioritySpec>;
#[doc = "Clear Channel Priority"]
pub mod clearchnlpriority;
#[doc = "ERROR (rw) register accessor: Error Status and Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@error`]
module"]
#[doc(alias = "ERROR")]
pub type Error = crate::Reg<error::ErrorSpec>;
#[doc = "Error Status and Clear"]
pub mod error;
#[doc = "REQDONE (rw) register accessor: Channel Request Done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reqdone::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reqdone::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reqdone`]
module"]
#[doc(alias = "REQDONE")]
pub type Reqdone = crate::Reg<reqdone::ReqdoneSpec>;
#[doc = "Channel Request Done"]
pub mod reqdone;
#[doc = "DONEMASK (rw) register accessor: Channel Request Done Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`donemask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`donemask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@donemask`]
module"]
#[doc(alias = "DONEMASK")]
pub type Donemask = crate::Reg<donemask::DonemaskSpec>;
#[doc = "Channel Request Done Mask"]
pub mod donemask;
