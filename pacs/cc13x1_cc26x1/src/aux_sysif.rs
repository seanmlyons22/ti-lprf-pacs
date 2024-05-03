#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opmodereq: Opmodereq,
    opmodeack: Opmodeack,
    _reserved2: [u8; 0x40],
    evsyncrate: Evsyncrate,
    peroprate: Peroprate,
    adcclkctl: Adcclkctl,
    tdcclkctl: Tdcclkctl,
    tdcrefclkctl: Tdcrefclkctl,
    _reserved7: [u8; 0x20],
    rtcsubsecinc0: Rtcsubsecinc0,
    rtcsubsecinc1: Rtcsubsecinc1,
    rtcsubsecincctl: Rtcsubsecincctl,
    _reserved10: [u8; 0x08],
    rtcevclr: Rtcevclr,
    _reserved11: [u8; 0x0c],
    timerhalt: Timerhalt,
    _reserved12: [u8; 0x10],
    swpwrprof: Swpwrprof,
}
impl RegisterBlock {
    #[doc = "0x00 - Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
    #[inline(always)]
    pub const fn opmodereq(&self) -> &Opmodereq {
        &self.opmodereq
    }
    #[doc = "0x04 - Operational Mode Acknowledgement User must assume that the current operational mode is the one acknowledged."]
    #[inline(always)]
    pub const fn opmodeack(&self) -> &Opmodeack {
        &self.opmodeack
    }
    #[doc = "0x48 - Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. Select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
    #[inline(always)]
    pub const fn evsyncrate(&self) -> &Evsyncrate {
        &self.evsyncrate
    }
    #[doc = "0x4c - Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. Select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
    #[inline(always)]
    pub const fn peroprate(&self) -> &Peroprate {
        &self.peroprate
    }
    #[doc = "0x50 - ADC Clock Control"]
    #[inline(always)]
    pub const fn adcclkctl(&self) -> &Adcclkctl {
        &self.adcclkctl
    }
    #[doc = "0x54 - TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
    #[inline(always)]
    pub const fn tdcclkctl(&self) -> &Tdcclkctl {
        &self.tdcclkctl
    }
    #[doc = "0x58 - TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
    #[inline(always)]
    pub const fn tdcrefclkctl(&self) -> &Tdcrefclkctl {
        &self.tdcrefclkctl
    }
    #[doc = "0x7c - Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    #[inline(always)]
    pub const fn rtcsubsecinc0(&self) -> &Rtcsubsecinc0 {
        &self.rtcsubsecinc0
    }
    #[doc = "0x80 - Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
    #[inline(always)]
    pub const fn rtcsubsecinc1(&self) -> &Rtcsubsecinc1 {
        &self.rtcsubsecinc1
    }
    #[doc = "0x84 - Real Time Counter Sub Second Increment Control"]
    #[inline(always)]
    pub const fn rtcsubsecincctl(&self) -> &Rtcsubsecincctl {
        &self.rtcsubsecincctl
    }
    #[doc = "0x90 - AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
    #[inline(always)]
    pub const fn rtcevclr(&self) -> &Rtcevclr {
        &self.rtcevclr
    }
    #[doc = "0xa0 - Timer Halt Debug register"]
    #[inline(always)]
    pub const fn timerhalt(&self) -> &Timerhalt {
        &self.timerhalt
    }
    #[doc = "0xb4 - Software Power Profiler"]
    #[inline(always)]
    pub const fn swpwrprof(&self) -> &Swpwrprof {
        &self.swpwrprof
    }
}
#[doc = "OPMODEREQ (rw) register accessor: Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opmodereq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opmodereq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opmodereq`]
module"]
#[doc(alias = "OPMODEREQ")]
pub type Opmodereq = crate::Reg<opmodereq::OpmodereqSpec>;
#[doc = "Operational Mode Request AUX can operate in three operational modes. Each mode is associated with: - a SCE clock source or rate, given by AON_PMCTL:AUXSCECLK. This rate is termed SCE_RATE. - a system power supply state request. AUX can request powerdown (uLDO) or active (GLDO or DCDC) system power supply state. Follow these rules: - It is not allowed to change a request until it has been acknowledged through OPMODEACK. - A change in mode request must happen stepwise along this sequence, the direction is irrelevant: PDA - A - LP - PDLP. Failure to follow these rules might result in unexpected behavior and must be avoided."]
pub mod opmodereq;
#[doc = "OPMODEACK (rw) register accessor: Operational Mode Acknowledgement User must assume that the current operational mode is the one acknowledged.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opmodeack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opmodeack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opmodeack`]
module"]
#[doc(alias = "OPMODEACK")]
pub type Opmodeack = crate::Reg<opmodeack::OpmodeackSpec>;
#[doc = "Operational Mode Acknowledgement User must assume that the current operational mode is the one acknowledged."]
pub mod opmodeack;
#[doc = "EVSYNCRATE (rw) register accessor: Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. Select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evsyncrate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evsyncrate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evsyncrate`]
module"]
#[doc(alias = "EVSYNCRATE")]
pub type Evsyncrate = crate::Reg<evsyncrate::EvsyncrateSpec>;
#[doc = "Event Synchronization Rate Configure synchronization rate for certain events to the synchronous AUX event bus. Select AUX bus rate when system CPU uses the event. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by two when MCU domain is active."]
pub mod evsyncrate;
#[doc = "PEROPRATE (rw) register accessor: Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. Select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peroprate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peroprate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peroprate`]
module"]
#[doc(alias = "PEROPRATE")]
pub type Peroprate = crate::Reg<peroprate::PeroprateSpec>;
#[doc = "Peripheral Operational Rate Some AUX peripherals are operated at either SCE or at AUX bus rate. Select AUX bus rate when system CPU uses such peripheral. SCE rate equals rate configured in AON_PMCTL:AUXSCECLK. AUX bus rate equals SCE rate, or SCLK_HF divided by 2 when MCU domain is active."]
pub mod peroprate;
#[doc = "ADCCLKCTL (rw) register accessor: ADC Clock Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclkctl`]
module"]
#[doc(alias = "ADCCLKCTL")]
pub type Adcclkctl = crate::Reg<adcclkctl::AdcclkctlSpec>;
#[doc = "ADC Clock Control"]
pub mod adcclkctl;
#[doc = "TDCCLKCTL (rw) register accessor: TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcclkctl`]
module"]
#[doc(alias = "TDCCLKCTL")]
pub type Tdcclkctl = crate::Reg<tdcclkctl::TdcclkctlSpec>;
#[doc = "TDC Counter Clock Control Controls if the AUX_TDC counter clock source is enabled. These are the recommended steps to configure and request the counter clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_TDC_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the counter clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
pub mod tdcclkctl;
#[doc = "TDCREFCLKCTL (rw) register accessor: TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcrefclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcrefclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcrefclkctl`]
module"]
#[doc(alias = "TDCREFCLKCTL")]
pub type Tdcrefclkctl = crate::Reg<tdcrefclkctl::TdcrefclkctlSpec>;
#[doc = "TDC Reference Clock Control Controls if the AUX_TDC reference clock source is enabled. These are the recommended steps to configure and request the reference clock: - Ensure that REQ=0 and ACK=0. - Configure clock source in DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL. - Read DDI_0_OSC:CTL0 to avoid a race condition between previous step and next step. - Set REQ=1 to request the clock. - Wait until ACK=1. After these steps ACK stays high until REQ=0. It is hence not recommended to reconfigure DDI_0_OSC:CTL0.ACLK_REF_SRC_SEL when ACK=1. In this case, there will be no indication of when the new clock source selection is ready. These are the recommended steps to stop the reference clock: - Ensure that REQ=1 and ACK=1. - Set REQ=0 to stop the clock. - Wait until ACK=0."]
pub mod tdcrefclkctl;
#[doc = "RTCSUBSECINC0 (rw) register accessor: Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecinc0`]
module"]
#[doc(alias = "RTCSUBSECINC0")]
pub type Rtcsubsecinc0 = crate::Reg<rtcsubsecinc0::Rtcsubsecinc0Spec>;
#[doc = "Real Time Counter Sub Second Increment 0 INC15_0 will replace bits 15:0 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc0;
#[doc = "RTCSUBSECINC1 (rw) register accessor: Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecinc1`]
module"]
#[doc(alias = "RTCSUBSECINC1")]
pub type Rtcsubsecinc1 = crate::Reg<rtcsubsecinc1::Rtcsubsecinc1Spec>;
#[doc = "Real Time Counter Sub Second Increment 1 INC23_16 will replace bits 23:16 in AON_RTC:SUBSECINC when RTCSUBSECINCCTL.UPD_REQ is set."]
pub mod rtcsubsecinc1;
#[doc = "RTCSUBSECINCCTL (rw) register accessor: Real Time Counter Sub Second Increment Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecincctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecincctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecincctl`]
module"]
#[doc(alias = "RTCSUBSECINCCTL")]
pub type Rtcsubsecincctl = crate::Reg<rtcsubsecincctl::RtcsubsecincctlSpec>;
#[doc = "Real Time Counter Sub Second Increment Control"]
pub mod rtcsubsecincctl;
#[doc = "RTCEVCLR (rw) register accessor: AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcevclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcevclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcevclr`]
module"]
#[doc(alias = "RTCEVCLR")]
pub type Rtcevclr = crate::Reg<rtcevclr::RtcevclrSpec>;
#[doc = "AON_RTC Event Clear Request to clear events: - AON_RTC:EVFLAGS.CH2. - AON_RTC:EVFLAGS.CH2 delayed version. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2. - AUX_EVCTL:EVSTAT2.AON_RTC_CH2_DLY."]
pub mod rtcevclr;
#[doc = "TIMERHALT (rw) register accessor: Timer Halt Debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timerhalt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timerhalt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timerhalt`]
module"]
#[doc(alias = "TIMERHALT")]
pub type Timerhalt = crate::Reg<timerhalt::TimerhaltSpec>;
#[doc = "Timer Halt Debug register"]
pub mod timerhalt;
#[doc = "SWPWRPROF (rw) register accessor: Software Power Profiler\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swpwrprof::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpwrprof::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpwrprof`]
module"]
#[doc(alias = "SWPWRPROF")]
pub type Swpwrprof = crate::Reg<swpwrprof::SwpwrprofSpec>;
#[doc = "Software Power Profiler"]
pub mod swpwrprof;
