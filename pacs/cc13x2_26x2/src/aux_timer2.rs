#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    target: Target,
    shdwtarget: Shdwtarget,
    cntr: Cntr,
    precfg: Precfg,
    evctl: Evctl,
    pulsetrig: Pulsetrig,
    _reserved7: [u8; 0x64],
    ch0evcfg: Ch0evcfg,
    ch0ccfg: Ch0ccfg,
    ch0pcc: Ch0pcc,
    ch0cc: Ch0cc,
    ch1evcfg: Ch1evcfg,
    ch1ccfg: Ch1ccfg,
    ch1pcc: Ch1pcc,
    ch1cc: Ch1cc,
    ch2evcfg: Ch2evcfg,
    ch2ccfg: Ch2ccfg,
    ch2pcc: Ch2pcc,
    ch2cc: Ch2cc,
    ch3evcfg: Ch3evcfg,
    ch3ccfg: Ch3ccfg,
    ch3pcc: Ch3pcc,
    ch3cc: Ch3cc,
}
impl RegisterBlock {
    #[doc = "0x00 - Timer Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Target User defined counter target."]
    #[inline(always)]
    pub const fn target(&self) -> &Target {
        &self.target
    }
    #[doc = "0x08 - Shadow Target"]
    #[inline(always)]
    pub const fn shdwtarget(&self) -> &Shdwtarget {
        &self.shdwtarget
    }
    #[doc = "0x0c - Counter"]
    #[inline(always)]
    pub const fn cntr(&self) -> &Cntr {
        &self.cntr
    }
    #[doc = "0x10 - Clock Prescaler Configuration"]
    #[inline(always)]
    pub const fn precfg(&self) -> &Precfg {
        &self.precfg
    }
    #[doc = "0x14 - Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    #[inline(always)]
    pub const fn evctl(&self) -> &Evctl {
        &self.evctl
    }
    #[doc = "0x18 - Pulse Trigger"]
    #[inline(always)]
    pub const fn pulsetrig(&self) -> &Pulsetrig {
        &self.pulsetrig
    }
    #[doc = "0x80 - Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    #[inline(always)]
    pub const fn ch0evcfg(&self) -> &Ch0evcfg {
        &self.ch0evcfg
    }
    #[doc = "0x84 - Channel 0 Capture Configuration"]
    #[inline(always)]
    pub const fn ch0ccfg(&self) -> &Ch0ccfg {
        &self.ch0ccfg
    }
    #[doc = "0x88 - Channel 0 Pipeline Capture Compare"]
    #[inline(always)]
    pub const fn ch0pcc(&self) -> &Ch0pcc {
        &self.ch0pcc
    }
    #[doc = "0x8c - Channel 0 Capture Compare"]
    #[inline(always)]
    pub const fn ch0cc(&self) -> &Ch0cc {
        &self.ch0cc
    }
    #[doc = "0x90 - Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    #[inline(always)]
    pub const fn ch1evcfg(&self) -> &Ch1evcfg {
        &self.ch1evcfg
    }
    #[doc = "0x94 - Channel 1 Capture Configuration"]
    #[inline(always)]
    pub const fn ch1ccfg(&self) -> &Ch1ccfg {
        &self.ch1ccfg
    }
    #[doc = "0x98 - Channel 1 Pipeline Capture Compare"]
    #[inline(always)]
    pub const fn ch1pcc(&self) -> &Ch1pcc {
        &self.ch1pcc
    }
    #[doc = "0x9c - Channel 1 Capture Compare"]
    #[inline(always)]
    pub const fn ch1cc(&self) -> &Ch1cc {
        &self.ch1cc
    }
    #[doc = "0xa0 - Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    #[inline(always)]
    pub const fn ch2evcfg(&self) -> &Ch2evcfg {
        &self.ch2evcfg
    }
    #[doc = "0xa4 - Channel 2 Capture Configuration"]
    #[inline(always)]
    pub const fn ch2ccfg(&self) -> &Ch2ccfg {
        &self.ch2ccfg
    }
    #[doc = "0xa8 - Channel 2 Pipeline Capture Compare"]
    #[inline(always)]
    pub const fn ch2pcc(&self) -> &Ch2pcc {
        &self.ch2pcc
    }
    #[doc = "0xac - Channel 2 Capture Compare"]
    #[inline(always)]
    pub const fn ch2cc(&self) -> &Ch2cc {
        &self.ch2cc
    }
    #[doc = "0xb0 - Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    #[inline(always)]
    pub const fn ch3evcfg(&self) -> &Ch3evcfg {
        &self.ch3evcfg
    }
    #[doc = "0xb4 - Channel 3 Capture Configuration"]
    #[inline(always)]
    pub const fn ch3ccfg(&self) -> &Ch3ccfg {
        &self.ch3ccfg
    }
    #[doc = "0xb8 - Channel 3 Pipeline Capture Compare"]
    #[inline(always)]
    pub const fn ch3pcc(&self) -> &Ch3pcc {
        &self.ch3pcc
    }
    #[doc = "0xbc - Channel 3 Capture Compare"]
    #[inline(always)]
    pub const fn ch3cc(&self) -> &Ch3cc {
        &self.ch3cc
    }
}
#[doc = "CTL (rw) register accessor: Timer Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Timer Control"]
pub mod ctl;
#[doc = "TARGET (rw) register accessor: Target User defined counter target.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`target::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`target::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@target`]
module"]
#[doc(alias = "TARGET")]
pub type Target = crate::Reg<target::TargetSpec>;
#[doc = "Target User defined counter target."]
pub mod target;
#[doc = "SHDWTARGET (rw) register accessor: Shadow Target\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdwtarget::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdwtarget::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shdwtarget`]
module"]
#[doc(alias = "SHDWTARGET")]
pub type Shdwtarget = crate::Reg<shdwtarget::ShdwtargetSpec>;
#[doc = "Shadow Target"]
pub mod shdwtarget;
#[doc = "CNTR (rw) register accessor: Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cntr`]
module"]
#[doc(alias = "CNTR")]
pub type Cntr = crate::Reg<cntr::CntrSpec>;
#[doc = "Counter"]
pub mod cntr;
#[doc = "PRECFG (rw) register accessor: Clock Prescaler Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`precfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`precfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@precfg`]
module"]
#[doc(alias = "PRECFG")]
pub type Precfg = crate::Reg<precfg::PrecfgSpec>;
#[doc = "Clock Prescaler Configuration"]
pub mod precfg;
#[doc = "EVCTL (rw) register accessor: Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctl`]
module"]
#[doc(alias = "EVCTL")]
pub type Evctl = crate::Reg<evctl::EvctlSpec>;
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
pub mod evctl;
#[doc = "PULSETRIG (rw) register accessor: Pulse Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pulsetrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pulsetrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pulsetrig`]
module"]
#[doc(alias = "PULSETRIG")]
pub type Pulsetrig = crate::Reg<pulsetrig::PulsetrigSpec>;
#[doc = "Pulse Trigger"]
pub mod pulsetrig;
#[doc = "CH0EVCFG (rw) register accessor: Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0evcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0evcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0evcfg`]
module"]
#[doc(alias = "CH0EVCFG")]
pub type Ch0evcfg = crate::Reg<ch0evcfg::Ch0evcfgSpec>;
#[doc = "Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch0evcfg;
#[doc = "CH0CCFG (rw) register accessor: Channel 0 Capture Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0ccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0ccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0ccfg`]
module"]
#[doc(alias = "CH0CCFG")]
pub type Ch0ccfg = crate::Reg<ch0ccfg::Ch0ccfgSpec>;
#[doc = "Channel 0 Capture Configuration"]
pub mod ch0ccfg;
#[doc = "CH0PCC (rw) register accessor: Channel 0 Pipeline Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0pcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0pcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0pcc`]
module"]
#[doc(alias = "CH0PCC")]
pub type Ch0pcc = crate::Reg<ch0pcc::Ch0pccSpec>;
#[doc = "Channel 0 Pipeline Capture Compare"]
pub mod ch0pcc;
#[doc = "CH0CC (rw) register accessor: Channel 0 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch0cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch0cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0cc`]
module"]
#[doc(alias = "CH0CC")]
pub type Ch0cc = crate::Reg<ch0cc::Ch0ccSpec>;
#[doc = "Channel 0 Capture Compare"]
pub mod ch0cc;
#[doc = "CH1EVCFG (rw) register accessor: Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1evcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1evcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1evcfg`]
module"]
#[doc(alias = "CH1EVCFG")]
pub type Ch1evcfg = crate::Reg<ch1evcfg::Ch1evcfgSpec>;
#[doc = "Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch1evcfg;
#[doc = "CH1CCFG (rw) register accessor: Channel 1 Capture Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1ccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1ccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1ccfg`]
module"]
#[doc(alias = "CH1CCFG")]
pub type Ch1ccfg = crate::Reg<ch1ccfg::Ch1ccfgSpec>;
#[doc = "Channel 1 Capture Configuration"]
pub mod ch1ccfg;
#[doc = "CH1PCC (rw) register accessor: Channel 1 Pipeline Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1pcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1pcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1pcc`]
module"]
#[doc(alias = "CH1PCC")]
pub type Ch1pcc = crate::Reg<ch1pcc::Ch1pccSpec>;
#[doc = "Channel 1 Pipeline Capture Compare"]
pub mod ch1pcc;
#[doc = "CH1CC (rw) register accessor: Channel 1 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1cc`]
module"]
#[doc(alias = "CH1CC")]
pub type Ch1cc = crate::Reg<ch1cc::Ch1ccSpec>;
#[doc = "Channel 1 Capture Compare"]
pub mod ch1cc;
#[doc = "CH2EVCFG (rw) register accessor: Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2evcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2evcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2evcfg`]
module"]
#[doc(alias = "CH2EVCFG")]
pub type Ch2evcfg = crate::Reg<ch2evcfg::Ch2evcfgSpec>;
#[doc = "Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch2evcfg;
#[doc = "CH2CCFG (rw) register accessor: Channel 2 Capture Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2ccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2ccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2ccfg`]
module"]
#[doc(alias = "CH2CCFG")]
pub type Ch2ccfg = crate::Reg<ch2ccfg::Ch2ccfgSpec>;
#[doc = "Channel 2 Capture Configuration"]
pub mod ch2ccfg;
#[doc = "CH2PCC (rw) register accessor: Channel 2 Pipeline Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2pcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2pcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2pcc`]
module"]
#[doc(alias = "CH2PCC")]
pub type Ch2pcc = crate::Reg<ch2pcc::Ch2pccSpec>;
#[doc = "Channel 2 Pipeline Capture Compare"]
pub mod ch2pcc;
#[doc = "CH2CC (rw) register accessor: Channel 2 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2cc`]
module"]
#[doc(alias = "CH2CC")]
pub type Ch2cc = crate::Reg<ch2cc::Ch2ccSpec>;
#[doc = "Channel 2 Capture Compare"]
pub mod ch2cc;
#[doc = "CH3EVCFG (rw) register accessor: Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3evcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3evcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3evcfg`]
module"]
#[doc(alias = "CH3EVCFG")]
pub type Ch3evcfg = crate::Reg<ch3evcfg::Ch3evcfgSpec>;
#[doc = "Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch3evcfg;
#[doc = "CH3CCFG (rw) register accessor: Channel 3 Capture Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3ccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3ccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3ccfg`]
module"]
#[doc(alias = "CH3CCFG")]
pub type Ch3ccfg = crate::Reg<ch3ccfg::Ch3ccfgSpec>;
#[doc = "Channel 3 Capture Configuration"]
pub mod ch3ccfg;
#[doc = "CH3PCC (rw) register accessor: Channel 3 Pipeline Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3pcc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3pcc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3pcc`]
module"]
#[doc(alias = "CH3PCC")]
pub type Ch3pcc = crate::Reg<ch3pcc::Ch3pccSpec>;
#[doc = "Channel 3 Pipeline Capture Compare"]
pub mod ch3pcc;
#[doc = "CH3CC (rw) register accessor: Channel 3 Capture Compare\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch3cc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch3cc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3cc`]
module"]
#[doc(alias = "CH3CC")]
pub type Ch3cc = crate::Reg<ch3cc::Ch3ccSpec>;
#[doc = "Channel 3 Capture Compare"]
pub mod ch3cc;
