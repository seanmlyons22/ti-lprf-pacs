#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    evflags: Evflags,
    sec: Sec,
    subsec: Subsec,
    subsecinc: Subsecinc,
    chctl: Chctl,
    ch0cmp: Ch0cmp,
    ch1cmp: Ch1cmp,
    ch2cmp: Ch2cmp,
    ch2cmpinc: Ch2cmpinc,
    ch1capt: Ch1capt,
    sync: Sync,
    time: Time,
    synclf: Synclf,
}
impl RegisterBlock {
    #[doc = "0x00 - Control This register contains various bitfields for configuration of RTC RTL Name = CONFIG"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
    #[inline(always)]
    pub const fn evflags(&self) -> &Evflags {
        &self.evflags
    }
    #[doc = "0x08 - Second Counter Value, Integer Part"]
    #[inline(always)]
    pub const fn sec(&self) -> &Sec {
        &self.sec
    }
    #[doc = "0x0c - Second Counter Value, Fractional Part"]
    #[inline(always)]
    pub const fn subsec(&self) -> &Subsec {
        &self.subsec
    }
    #[doc = "0x10 - Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
    #[inline(always)]
    pub const fn subsecinc(&self) -> &Subsecinc {
        &self.subsecinc
    }
    #[doc = "0x14 - Channel Configuration"]
    #[inline(always)]
    pub const fn chctl(&self) -> &Chctl {
        &self.chctl
    }
    #[doc = "0x18 - Channel 0 Compare Value"]
    #[inline(always)]
    pub const fn ch0cmp(&self) -> &Ch0cmp {
        &self.ch0cmp
    }
    #[doc = "0x1c - Channel 1 Compare Value"]
    #[inline(always)]
    pub const fn ch1cmp(&self) -> &Ch1cmp {
        &self.ch1cmp
    }
    #[doc = "0x20 - Channel 2 Compare Value"]
    #[inline(always)]
    pub const fn ch2cmp(&self) -> &Ch2cmp {
        &self.ch2cmp
    }
    #[doc = "0x24 - Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event."]
    #[inline(always)]
    pub const fn ch2cmpinc(&self) -> &Ch2cmpinc {
        &self.ch2cmpinc
    }
    #[doc = "0x28 - Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
    #[inline(always)]
    pub const fn ch1capt(&self) -> &Ch1capt {
        &self.ch1capt
    }
    #[doc = "0x2c - AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x30 - Current Counter Value"]
    #[inline(always)]
    pub const fn time(&self) -> &Time {
        &self.time
    }
    #[doc = "0x34 - Synchronization to SCLK_LF This register is used for synchronizing MCU to positive or negative edge of SCLK_LF."]
    #[inline(always)]
    pub const fn synclf(&self) -> &Synclf {
        &self.synclf
    }
}
#[doc = "CTL (rw) register accessor: Control This register contains various bitfields for configuration of RTC RTL Name = CONFIG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control This register contains various bitfields for configuration of RTC RTL Name = CONFIG"]
pub mod ctl;
#[doc = "EVFLAGS (rw) register accessor: Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evflags`]
module"]
#[doc(alias = "EVFLAGS")]
pub type Evflags = crate::Reg<evflags::EvflagsSpec>;
#[doc = "Event Flags, RTC Status This register contains event flags from the 3 RTC channels. Each flag will be cleared when writing a '1' to the corresponding bitfield."]
pub mod evflags;
#[doc = "SEC (rw) register accessor: Second Counter Value, Integer Part\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
#[doc(alias = "SEC")]
pub type Sec = crate::Reg<sec::SecSpec>;
#[doc = "Second Counter Value, Integer Part"]
pub mod sec;
#[doc = "SUBSEC (rw) register accessor: Second Counter Value, Fractional Part\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subsec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsec`]
module"]
#[doc(alias = "SUBSEC")]
pub type Subsec = crate::Reg<subsec::SubsecSpec>;
#[doc = "Second Counter Value, Fractional Part"]
pub mod subsec;
#[doc = "SUBSECINC (rw) register accessor: Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`subsecinc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`subsecinc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@subsecinc`]
module"]
#[doc(alias = "SUBSECINC")]
pub type Subsecinc = crate::Reg<subsecinc::SubsecincSpec>;
#[doc = "Subseconds Increment Value added to SUBSEC.VALUE on every SCLK_LFclock cycle."]
pub mod subsecinc;
#[doc = "CHCTL (rw) register accessor: Channel Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chctl`]
module"]
#[doc(alias = "CHCTL")]
pub type Chctl = crate::Reg<chctl::ChctlSpec>;
#[doc = "Channel Configuration"]
pub mod chctl;
#[doc = "CH0CMP (rw) register accessor: Channel 0 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cmp`]
module"]
#[doc(alias = "CH0CMP")]
pub type Ch0cmp = crate::Reg<ch0cmp::Ch0cmpSpec>;
#[doc = "Channel 0 Compare Value"]
pub mod ch0cmp;
#[doc = "CH1CMP (rw) register accessor: Channel 1 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cmp`]
module"]
#[doc(alias = "CH1CMP")]
pub type Ch1cmp = crate::Reg<ch1cmp::Ch1cmpSpec>;
#[doc = "Channel 1 Compare Value"]
pub mod ch1cmp;
#[doc = "CH2CMP (rw) register accessor: Channel 2 Compare Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cmp`]
module"]
#[doc(alias = "CH2CMP")]
pub type Ch2cmp = crate::Reg<ch2cmp::Ch2cmpSpec>;
#[doc = "Channel 2 Compare Value"]
pub mod ch2cmp;
#[doc = "CH2CMPINC (rw) register accessor: Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cmpinc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cmpinc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cmpinc`]
module"]
#[doc(alias = "CH2CMPINC")]
pub type Ch2cmpinc = crate::Reg<ch2cmpinc::Ch2cmpincSpec>;
#[doc = "Channel 2 Compare Value Auto-increment This register is primarily used to generate periodical wake-up for the AUX_SCE module, through the \\[AUX_EVCTL.EVSTAT0.AON_RTC\\]
event."]
pub mod ch2cmpinc;
#[doc = "CH1CAPT (rw) register accessor: Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1capt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1capt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1capt`]
module"]
#[doc(alias = "CH1CAPT")]
pub type Ch1capt = crate::Reg<ch1capt::Ch1captSpec>;
#[doc = "Channel 1 Capture Value If CHCTL.CH1_EN = 1and CHCTL.CH1_CAPT_EN = 1, capture occurs on each rising edge of the event selected in AON_EVENT:RTCSEL."]
pub mod ch1capt;
#[doc = "SYNC (rw) register accessor: AON Synchronization This register is used for synchronizing between MCU and entire AON domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "AON Synchronization This register is used for synchronizing between MCU and entire AON domain."]
pub mod sync;
#[doc = "TIME (rw) register accessor: Current Counter Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`time::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`time::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@time`]
module"]
#[doc(alias = "TIME")]
pub type Time = crate::Reg<time::TimeSpec>;
#[doc = "Current Counter Value"]
pub mod time;
#[doc = "SYNCLF (rw) register accessor: Synchronization to SCLK_LF This register is used for synchronizing MCU to positive or negative edge of SCLK_LF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`synclf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`synclf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@synclf`]
module"]
#[doc(alias = "SYNCLF")]
pub type Synclf = crate::Reg<synclf::SynclfSpec>;
#[doc = "Synchronization to SCLK_LF This register is used for synchronizing MCU to positive or negative edge of SCLK_LF."]
pub mod synclf;
