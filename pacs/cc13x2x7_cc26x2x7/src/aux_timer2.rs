#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timer Control"]
    pub ctl: CTL,
    #[doc = "0x04 - Target User defined counter target."]
    pub target: TARGET,
    #[doc = "0x08 - Shadow Target"]
    pub shdwtarget: SHDWTARGET,
    #[doc = "0x0c - Counter"]
    pub cntr: CNTR,
    #[doc = "0x10 - Clock Prescaler Configuration"]
    pub precfg: PRECFG,
    #[doc = "0x14 - Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
    pub evctl: EVCTL,
    #[doc = "0x18 - Pulse Trigger"]
    pub pulsetrig: PULSETRIG,
    _reserved7: [u8; 0x64],
    #[doc = "0x80 - Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch0evcfg: CH0EVCFG,
    #[doc = "0x84 - Channel 0 Capture Configuration"]
    pub ch0ccfg: CH0CCFG,
    #[doc = "0x88 - Channel 0 Pipeline Capture Compare"]
    pub ch0pcc: CH0PCC,
    #[doc = "0x8c - Channel 0 Capture Compare"]
    pub ch0cc: CH0CC,
    #[doc = "0x90 - Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch1evcfg: CH1EVCFG,
    #[doc = "0x94 - Channel 1 Capture Configuration"]
    pub ch1ccfg: CH1CCFG,
    #[doc = "0x98 - Channel 1 Pipeline Capture Compare"]
    pub ch1pcc: CH1PCC,
    #[doc = "0x9c - Channel 1 Capture Compare"]
    pub ch1cc: CH1CC,
    #[doc = "0xa0 - Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch2evcfg: CH2EVCFG,
    #[doc = "0xa4 - Channel 2 Capture Configuration"]
    pub ch2ccfg: CH2CCFG,
    #[doc = "0xa8 - Channel 2 Pipeline Capture Compare"]
    pub ch2pcc: CH2PCC,
    #[doc = "0xac - Channel 2 Capture Compare"]
    pub ch2cc: CH2CC,
    #[doc = "0xb0 - Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
    pub ch3evcfg: CH3EVCFG,
    #[doc = "0xb4 - Channel 3 Capture Configuration"]
    pub ch3ccfg: CH3CCFG,
    #[doc = "0xb8 - Channel 3 Pipeline Capture Compare"]
    pub ch3pcc: CH3PCC,
    #[doc = "0xbc - Channel 3 Capture Compare"]
    pub ch3cc: CH3CC,
}
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Timer Control"]
pub mod ctl;
#[doc = "TARGET (rw) register accessor: an alias for `Reg<TARGET_SPEC>`"]
pub type TARGET = crate::Reg<target::TARGET_SPEC>;
#[doc = "Target User defined counter target."]
pub mod target;
#[doc = "SHDWTARGET (rw) register accessor: an alias for `Reg<SHDWTARGET_SPEC>`"]
pub type SHDWTARGET = crate::Reg<shdwtarget::SHDWTARGET_SPEC>;
#[doc = "Shadow Target"]
pub mod shdwtarget;
#[doc = "CNTR (rw) register accessor: an alias for `Reg<CNTR_SPEC>`"]
pub type CNTR = crate::Reg<cntr::CNTR_SPEC>;
#[doc = "Counter"]
pub mod cntr;
#[doc = "PRECFG (rw) register accessor: an alias for `Reg<PRECFG_SPEC>`"]
pub type PRECFG = crate::Reg<precfg::PRECFG_SPEC>;
#[doc = "Clock Prescaler Configuration"]
pub mod precfg;
#[doc = "EVCTL (rw) register accessor: an alias for `Reg<EVCTL_SPEC>`"]
pub type EVCTL = crate::Reg<evctl::EVCTL_SPEC>;
#[doc = "Event Control Set and clear individual events manually. Manual update of an event takes priority over automatic channel updates to the same event. You cannot set and clear an event at the same time, such requests will be neglected. An event can be automatically cleared, set, toggled, or pulsed by each channel, listed in decreasing order of priority. The action with highest priority happens when multiple channels want to update an event at the same time. The four events connect to the asynchronous AUX event bus: - Event 0 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV0. - Event 1 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV1. - Event 2 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV2. - Event 3 connects to AUX_EVCTL:EVSTAT3.AUX_TIMER2_EV3."]
pub mod evctl;
#[doc = "PULSETRIG (rw) register accessor: an alias for `Reg<PULSETRIG_SPEC>`"]
pub type PULSETRIG = crate::Reg<pulsetrig::PULSETRIG_SPEC>;
#[doc = "Pulse Trigger"]
pub mod pulsetrig;
#[doc = "CH0EVCFG (rw) register accessor: an alias for `Reg<CH0EVCFG_SPEC>`"]
pub type CH0EVCFG = crate::Reg<ch0evcfg::CH0EVCFG_SPEC>;
#[doc = "Channel 0 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch0evcfg;
#[doc = "CH0CCFG (rw) register accessor: an alias for `Reg<CH0CCFG_SPEC>`"]
pub type CH0CCFG = crate::Reg<ch0ccfg::CH0CCFG_SPEC>;
#[doc = "Channel 0 Capture Configuration"]
pub mod ch0ccfg;
#[doc = "CH0PCC (rw) register accessor: an alias for `Reg<CH0PCC_SPEC>`"]
pub type CH0PCC = crate::Reg<ch0pcc::CH0PCC_SPEC>;
#[doc = "Channel 0 Pipeline Capture Compare"]
pub mod ch0pcc;
#[doc = "CH0CC (rw) register accessor: an alias for `Reg<CH0CC_SPEC>`"]
pub type CH0CC = crate::Reg<ch0cc::CH0CC_SPEC>;
#[doc = "Channel 0 Capture Compare"]
pub mod ch0cc;
#[doc = "CH1EVCFG (rw) register accessor: an alias for `Reg<CH1EVCFG_SPEC>`"]
pub type CH1EVCFG = crate::Reg<ch1evcfg::CH1EVCFG_SPEC>;
#[doc = "Channel 1 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch1evcfg;
#[doc = "CH1CCFG (rw) register accessor: an alias for `Reg<CH1CCFG_SPEC>`"]
pub type CH1CCFG = crate::Reg<ch1ccfg::CH1CCFG_SPEC>;
#[doc = "Channel 1 Capture Configuration"]
pub mod ch1ccfg;
#[doc = "CH1PCC (rw) register accessor: an alias for `Reg<CH1PCC_SPEC>`"]
pub type CH1PCC = crate::Reg<ch1pcc::CH1PCC_SPEC>;
#[doc = "Channel 1 Pipeline Capture Compare"]
pub mod ch1pcc;
#[doc = "CH1CC (rw) register accessor: an alias for `Reg<CH1CC_SPEC>`"]
pub type CH1CC = crate::Reg<ch1cc::CH1CC_SPEC>;
#[doc = "Channel 1 Capture Compare"]
pub mod ch1cc;
#[doc = "CH2EVCFG (rw) register accessor: an alias for `Reg<CH2EVCFG_SPEC>`"]
pub type CH2EVCFG = crate::Reg<ch2evcfg::CH2EVCFG_SPEC>;
#[doc = "Channel 2 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch2evcfg;
#[doc = "CH2CCFG (rw) register accessor: an alias for `Reg<CH2CCFG_SPEC>`"]
pub type CH2CCFG = crate::Reg<ch2ccfg::CH2CCFG_SPEC>;
#[doc = "Channel 2 Capture Configuration"]
pub mod ch2ccfg;
#[doc = "CH2PCC (rw) register accessor: an alias for `Reg<CH2PCC_SPEC>`"]
pub type CH2PCC = crate::Reg<ch2pcc::CH2PCC_SPEC>;
#[doc = "Channel 2 Pipeline Capture Compare"]
pub mod ch2pcc;
#[doc = "CH2CC (rw) register accessor: an alias for `Reg<CH2CC_SPEC>`"]
pub type CH2CC = crate::Reg<ch2cc::CH2CC_SPEC>;
#[doc = "Channel 2 Capture Compare"]
pub mod ch2cc;
#[doc = "CH3EVCFG (rw) register accessor: an alias for `Reg<CH3EVCFG_SPEC>`"]
pub type CH3EVCFG = crate::Reg<ch3evcfg::CH3EVCFG_SPEC>;
#[doc = "Channel 3 Event Configuration This register configures channel function and enables event outputs. Each channel has an edge-detection circuit with memory. The circuit is: - enabled while CCACT selects a capture function and CTL.MODE is different from DIS. - flushed while CCACT selects a capture function and you change CTL.MODE from DIS to another mode. The flush action uses two AUX_SYSIF:TIMER2CLKCTL.SRC clock periods. It prevents capture events caused by expired signal values stored in the edge-detection circuit."]
pub mod ch3evcfg;
#[doc = "CH3CCFG (rw) register accessor: an alias for `Reg<CH3CCFG_SPEC>`"]
pub type CH3CCFG = crate::Reg<ch3ccfg::CH3CCFG_SPEC>;
#[doc = "Channel 3 Capture Configuration"]
pub mod ch3ccfg;
#[doc = "CH3PCC (rw) register accessor: an alias for `Reg<CH3PCC_SPEC>`"]
pub type CH3PCC = crate::Reg<ch3pcc::CH3PCC_SPEC>;
#[doc = "Channel 3 Pipeline Capture Compare"]
pub mod ch3pcc;
#[doc = "CH3CC (rw) register accessor: an alias for `Reg<CH3CC_SPEC>`"]
pub type CH3CC = crate::Reg<ch3cc::CH3CC_SPEC>;
#[doc = "Channel 3 Capture Compare"]
pub mod ch3cc;
