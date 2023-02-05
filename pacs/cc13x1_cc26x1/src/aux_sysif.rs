#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
    pub opmodereq: OPMODEREQ,
    #[doc = "0x04 - Operational Mode Acknowledgement User must assume that the current operational mode is the one acknowledged."]
    pub opmodeack: OPMODEACK,
    _reserved2: [u8; 0x40],
    #[doc = "0x48 - Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. Select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
    pub evsyncrate: EVSYNCRATE,
    #[doc = "0x4c - Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. Select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
    pub peroprate: PEROPRATE,
    #[doc = "0x50 - ADC Clock Control"]
    pub adcclkctl: ADCCLKCTL,
    #[doc = "0x54 - TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
    pub tdcclkctl: TDCCLKCTL,
    #[doc = "0x58 - TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
    pub tdcrefclkctl: TDCREFCLKCTL,
    _reserved7: [u8; 0x20],
    #[doc = "0x7c - Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    pub rtcsubsecinc0: RTCSUBSECINC0,
    #[doc = "0x80 - Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    pub rtcsubsecinc1: RTCSUBSECINC1,
    #[doc = "0x84 - Real Time Counter Sub Second Increment Control"]
    pub rtcsubsecincctl: RTCSUBSECINCCTL,
    _reserved10: [u8; 0x08],
    #[doc = "0x90 - AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
    pub rtcevclr: RTCEVCLR,
    _reserved11: [u8; 0x0c],
    #[doc = "0xa0 - Timer Halt Debug register"]
    pub timerhalt: TIMERHALT,
    _reserved12: [u8; 0x10],
    #[doc = "0xb4 - Software Power Profiler"]
    pub swpwrprof: SWPWRPROF,
}
#[doc = "OPMODEREQ (rw) register accessor: an alias for `Reg<OPMODEREQ_SPEC>`"]
pub type OPMODEREQ = crate::Reg<opmodereq::OPMODEREQ_SPEC>;
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
pub mod opmodereq;
#[doc = "OPMODEACK (rw) register accessor: an alias for `Reg<OPMODEACK_SPEC>`"]
pub type OPMODEACK = crate::Reg<opmodeack::OPMODEACK_SPEC>;
#[doc = "Operational Mode Acknowledgement User must assume that the current operational mode is the one acknowledged."]
pub mod opmodeack;
#[doc = "EVSYNCRATE (rw) register accessor: an alias for `Reg<EVSYNCRATE_SPEC>`"]
pub type EVSYNCRATE = crate::Reg<evsyncrate::EVSYNCRATE_SPEC>;
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. Select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
pub mod evsyncrate;
#[doc = "PEROPRATE (rw) register accessor: an alias for `Reg<PEROPRATE_SPEC>`"]
pub type PEROPRATE = crate::Reg<peroprate::PEROPRATE_SPEC>;
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. Select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
pub mod peroprate;
#[doc = "ADCCLKCTL (rw) register accessor: an alias for `Reg<ADCCLKCTL_SPEC>`"]
pub type ADCCLKCTL = crate::Reg<adcclkctl::ADCCLKCTL_SPEC>;
#[doc = "ADC Clock Control"]
pub mod adcclkctl;
#[doc = "TDCCLKCTL (rw) register accessor: an alias for `Reg<TDCCLKCTL_SPEC>`"]
pub type TDCCLKCTL = crate::Reg<tdcclkctl::TDCCLKCTL_SPEC>;
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
pub mod tdcclkctl;
#[doc = "TDCREFCLKCTL (rw) register accessor: an alias for `Reg<TDCREFCLKCTL_SPEC>`"]
pub type TDCREFCLKCTL = crate::Reg<tdcrefclkctl::TDCREFCLKCTL_SPEC>;
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
pub mod tdcrefclkctl;
#[doc = "RTCSUBSECINC0 (rw) register accessor: an alias for `Reg<RTCSUBSECINC0_SPEC>`"]
pub type RTCSUBSECINC0 = crate::Reg<rtcsubsecinc0::RTCSUBSECINC0_SPEC>;
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc0;
#[doc = "RTCSUBSECINC1 (rw) register accessor: an alias for `Reg<RTCSUBSECINC1_SPEC>`"]
pub type RTCSUBSECINC1 = crate::Reg<rtcsubsecinc1::RTCSUBSECINC1_SPEC>;
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc1;
#[doc = "RTCSUBSECINCCTL (rw) register accessor: an alias for `Reg<RTCSUBSECINCCTL_SPEC>`"]
pub type RTCSUBSECINCCTL = crate::Reg<rtcsubsecincctl::RTCSUBSECINCCTL_SPEC>;
#[doc = "Real Time Counter Sub Second Increment Control"]
pub mod rtcsubsecincctl;
#[doc = "RTCEVCLR (rw) register accessor: an alias for `Reg<RTCEVCLR_SPEC>`"]
pub type RTCEVCLR = crate::Reg<rtcevclr::RTCEVCLR_SPEC>;
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
pub mod rtcevclr;
#[doc = "TIMERHALT (rw) register accessor: an alias for `Reg<TIMERHALT_SPEC>`"]
pub type TIMERHALT = crate::Reg<timerhalt::TIMERHALT_SPEC>;
#[doc = "Timer Halt Debug register"]
pub mod timerhalt;
#[doc = "SWPWRPROF (rw) register accessor: an alias for `Reg<SWPWRPROF_SPEC>`"]
pub type SWPWRPROF = crate::Reg<swpwrprof::SWPWRPROF_SPEC>;
#[doc = "Software Power Profiler"]
pub mod swpwrprof;
