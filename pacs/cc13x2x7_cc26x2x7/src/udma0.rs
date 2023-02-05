#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Status"]
    pub status: STATUS,
    #[doc = "0x04 - Configuration"]
    pub cfg: CFG,
    #[doc = "0x08 - Channel Control Data Base Pointer"]
    pub ctrl: CTRL,
    #[doc = "0x0c - Channel Alternate Control Data Base Pointer"]
    pub altctrl: ALTCTRL,
    #[doc = "0x10 - Channel Wait On Request Status"]
    pub waitonreq: WAITONREQ,
    #[doc = "0x14 - Channel Software Request"]
    pub softreq: SOFTREQ,
    #[doc = "0x18 - Channel Set UseBurst"]
    pub setburst: SETBURST,
    #[doc = "0x1c - Channel Clear UseBurst"]
    pub clearburst: CLEARBURST,
    #[doc = "0x20 - Channel Set Request Mask"]
    pub setreqmask: SETREQMASK,
    #[doc = "0x24 - Clear Channel Request Mask"]
    pub clearreqmask: CLEARREQMASK,
    #[doc = "0x28 - Set Channel Enable"]
    pub setchannelen: SETCHANNELEN,
    #[doc = "0x2c - Clear Channel Enable"]
    pub clearchannelen: CLEARCHANNELEN,
    #[doc = "0x30 - Channel Set Primary-Alternate"]
    pub setchnlprialt: SETCHNLPRIALT,
    #[doc = "0x34 - Channel Clear Primary-Alternate"]
    pub clearchnlprialt: CLEARCHNLPRIALT,
    #[doc = "0x38 - Set Channel Priority"]
    pub setchnlpriority: SETCHNLPRIORITY,
    #[doc = "0x3c - Clear Channel Priority"]
    pub clearchnlpriority: CLEARCHNLPRIORITY,
    _reserved16: [u8; 0x0c],
    #[doc = "0x4c - Error Status and Clear"]
    pub error: ERROR,
    _reserved17: [u8; 0x04b4],
    #[doc = "0x504 - Channel Request Done"]
    pub reqdone: REQDONE,
    _reserved18: [u8; 0x18],
    #[doc = "0x520 - Channel Request Done Mask"]
    pub donemask: DONEMASK,
}
#[doc = "STATUS (rw) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status"]
pub mod status;
#[doc = "CFG (rw) register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "Configuration"]
pub mod cfg;
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Channel Control Data Base Pointer"]
pub mod ctrl;
#[doc = "ALTCTRL (rw) register accessor: an alias for `Reg<ALTCTRL_SPEC>`"]
pub type ALTCTRL = crate::Reg<altctrl::ALTCTRL_SPEC>;
#[doc = "Channel Alternate Control Data Base Pointer"]
pub mod altctrl;
#[doc = "WAITONREQ (rw) register accessor: an alias for `Reg<WAITONREQ_SPEC>`"]
pub type WAITONREQ = crate::Reg<waitonreq::WAITONREQ_SPEC>;
#[doc = "Channel Wait On Request Status"]
pub mod waitonreq;
#[doc = "SOFTREQ (rw) register accessor: an alias for `Reg<SOFTREQ_SPEC>`"]
pub type SOFTREQ = crate::Reg<softreq::SOFTREQ_SPEC>;
#[doc = "Channel Software Request"]
pub mod softreq;
#[doc = "SETBURST (rw) register accessor: an alias for `Reg<SETBURST_SPEC>`"]
pub type SETBURST = crate::Reg<setburst::SETBURST_SPEC>;
#[doc = "Channel Set UseBurst"]
pub mod setburst;
#[doc = "CLEARBURST (rw) register accessor: an alias for `Reg<CLEARBURST_SPEC>`"]
pub type CLEARBURST = crate::Reg<clearburst::CLEARBURST_SPEC>;
#[doc = "Channel Clear UseBurst"]
pub mod clearburst;
#[doc = "SETREQMASK (rw) register accessor: an alias for `Reg<SETREQMASK_SPEC>`"]
pub type SETREQMASK = crate::Reg<setreqmask::SETREQMASK_SPEC>;
#[doc = "Channel Set Request Mask"]
pub mod setreqmask;
#[doc = "CLEARREQMASK (rw) register accessor: an alias for `Reg<CLEARREQMASK_SPEC>`"]
pub type CLEARREQMASK = crate::Reg<clearreqmask::CLEARREQMASK_SPEC>;
#[doc = "Clear Channel Request Mask"]
pub mod clearreqmask;
#[doc = "SETCHANNELEN (rw) register accessor: an alias for `Reg<SETCHANNELEN_SPEC>`"]
pub type SETCHANNELEN = crate::Reg<setchannelen::SETCHANNELEN_SPEC>;
#[doc = "Set Channel Enable"]
pub mod setchannelen;
#[doc = "CLEARCHANNELEN (rw) register accessor: an alias for `Reg<CLEARCHANNELEN_SPEC>`"]
pub type CLEARCHANNELEN = crate::Reg<clearchannelen::CLEARCHANNELEN_SPEC>;
#[doc = "Clear Channel Enable"]
pub mod clearchannelen;
#[doc = "SETCHNLPRIALT (rw) register accessor: an alias for `Reg<SETCHNLPRIALT_SPEC>`"]
pub type SETCHNLPRIALT = crate::Reg<setchnlprialt::SETCHNLPRIALT_SPEC>;
#[doc = "Channel Set Primary-Alternate"]
pub mod setchnlprialt;
#[doc = "CLEARCHNLPRIALT (rw) register accessor: an alias for `Reg<CLEARCHNLPRIALT_SPEC>`"]
pub type CLEARCHNLPRIALT = crate::Reg<clearchnlprialt::CLEARCHNLPRIALT_SPEC>;
#[doc = "Channel Clear Primary-Alternate"]
pub mod clearchnlprialt;
#[doc = "SETCHNLPRIORITY (rw) register accessor: an alias for `Reg<SETCHNLPRIORITY_SPEC>`"]
pub type SETCHNLPRIORITY = crate::Reg<setchnlpriority::SETCHNLPRIORITY_SPEC>;
#[doc = "Set Channel Priority"]
pub mod setchnlpriority;
#[doc = "CLEARCHNLPRIORITY (rw) register accessor: an alias for `Reg<CLEARCHNLPRIORITY_SPEC>`"]
pub type CLEARCHNLPRIORITY = crate::Reg<clearchnlpriority::CLEARCHNLPRIORITY_SPEC>;
#[doc = "Clear Channel Priority"]
pub mod clearchnlpriority;
#[doc = "ERROR (rw) register accessor: an alias for `Reg<ERROR_SPEC>`"]
pub type ERROR = crate::Reg<error::ERROR_SPEC>;
#[doc = "Error Status and Clear"]
pub mod error;
#[doc = "REQDONE (rw) register accessor: an alias for `Reg<REQDONE_SPEC>`"]
pub type REQDONE = crate::Reg<reqdone::REQDONE_SPEC>;
#[doc = "Channel Request Done"]
pub mod reqdone;
#[doc = "DONEMASK (rw) register accessor: an alias for `Reg<DONEMASK_SPEC>`"]
pub type DONEMASK = crate::Reg<donemask::DONEMASK_SPEC>;
#[doc = "Channel Request Done Mask"]
pub mod donemask;
