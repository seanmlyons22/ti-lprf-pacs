#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control This register contains various bitfields for configuration of RTC RTL Name = CONFIG"]
    pub ctl: CTL,
    #[doc = "0x04 - Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
    pub evflags: EVFLAGS,
    #[doc = "0x08 - Second Counter Value, Integer Part"]
    pub sec: SEC,
    #[doc = "0x0c - Second Counter Value, Fractional Part"]
    pub subsec: SUBSEC,
    #[doc = "0x10 - Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
    pub subsecinc: SUBSECINC,
    #[doc = "0x14 - Channel Configuration"]
    pub chctl: CHCTL,
    #[doc = "0x18 - Channel 0 Compare Value"]
    pub ch0cmp: CH0CMP,
    #[doc = "0x1c - Channel 1 Compare Value"]
    pub ch1cmp: CH1CMP,
    #[doc = "0x20 - Channel 2 Compare Value"]
    pub ch2cmp: CH2CMP,
    #[doc = "0x24 - Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event."]
    pub ch2cmpinc: CH2CMPINC,
    #[doc = "0x28 - Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
    pub ch1capt: CH1CAPT,
    #[doc = "0x2c - AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
    pub sync: SYNC,
    #[doc = "0x30 - Current Counter Value"]
    pub time: TIME,
    #[doc = "0x34 - Synchronization to SCLK_LF This register is used for synchronizing MCU to positive or negative edge of SCLK_LF."]
    pub synclf: SYNCLF,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control This register contains various bitfields for configuration of RTC RTL Name = CONFIG"]
pub mod ctl;
#[doc = "EVFLAGS (rw) register accessor: an alias for `Reg<EVFLAGS_SPEC>`"]
pub type EVFLAGS = crate::Reg<evflags::EVFLAGS_SPEC>;
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
pub mod evflags;
#[doc = "SEC (rw) register accessor: an alias for `Reg<SEC_SPEC>`"]
pub type SEC = crate::Reg<sec::SEC_SPEC>;
#[doc = "Second Counter Value, Integer Part"]
pub mod sec;
#[doc = "SUBSEC (rw) register accessor: an alias for `Reg<SUBSEC_SPEC>`"]
pub type SUBSEC = crate::Reg<subsec::SUBSEC_SPEC>;
#[doc = "Second Counter Value, Fractional Part"]
pub mod subsec;
#[doc = "SUBSECINC (rw) register accessor: an alias for `Reg<SUBSECINC_SPEC>`"]
pub type SUBSECINC = crate::Reg<subsecinc::SUBSECINC_SPEC>;
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
pub mod subsecinc;
#[doc = "CHCTL (rw) register accessor: an alias for `Reg<CHCTL_SPEC>`"]
pub type CHCTL = crate::Reg<chctl::CHCTL_SPEC>;
#[doc = "Channel Configuration"]
pub mod chctl;
#[doc = "CH0CMP (rw) register accessor: an alias for `Reg<CH0CMP_SPEC>`"]
pub type CH0CMP = crate::Reg<ch0cmp::CH0CMP_SPEC>;
#[doc = "Channel 0 Compare Value"]
pub mod ch0cmp;
#[doc = "CH1CMP (rw) register accessor: an alias for `Reg<CH1CMP_SPEC>`"]
pub type CH1CMP = crate::Reg<ch1cmp::CH1CMP_SPEC>;
#[doc = "Channel 1 Compare Value"]
pub mod ch1cmp;
#[doc = "CH2CMP (rw) register accessor: an alias for `Reg<CH2CMP_SPEC>`"]
pub type CH2CMP = crate::Reg<ch2cmp::CH2CMP_SPEC>;
#[doc = "Channel 2 Compare Value"]
pub mod ch2cmp;
#[doc = "CH2CMPINC (rw) register accessor: an alias for `Reg<CH2CMPINC_SPEC>`"]
pub type CH2CMPINC = crate::Reg<ch2cmpinc::CH2CMPINC_SPEC>;
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event."]
pub mod ch2cmpinc;
#[doc = "CH1CAPT (rw) register accessor: an alias for `Reg<CH1CAPT_SPEC>`"]
pub type CH1CAPT = crate::Reg<ch1capt::CH1CAPT_SPEC>;
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
pub mod ch1capt;
#[doc = "SYNC (rw) register accessor: an alias for `Reg<SYNC_SPEC>`"]
pub type SYNC = crate::Reg<sync::SYNC_SPEC>;
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
pub mod sync;
#[doc = "TIME (rw) register accessor: an alias for `Reg<TIME_SPEC>`"]
pub type TIME = crate::Reg<time::TIME_SPEC>;
#[doc = "Current Counter Value"]
pub mod time;
#[doc = "SYNCLF (rw) register accessor: an alias for `Reg<SYNCLF_SPEC>`"]
pub type SYNCLF = crate::Reg<synclf::SYNCLF_SPEC>;
#[doc = "Synchronization to SCLK_LF This register is used for synchronizing MCU to positive or negative edge of SCLK_LF."]
pub mod synclf;
