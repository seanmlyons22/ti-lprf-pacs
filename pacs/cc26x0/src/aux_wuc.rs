#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    modclken0: Modclken0,
    pwroffreq: Pwroffreq,
    pwrdwnreq: Pwrdwnreq,
    pwrdwnack: Pwrdwnack,
    clklfreq: Clklfreq,
    clklfack: Clklfack,
    _reserved6: [u8; 0x10],
    wuevflags: Wuevflags,
    wuevclr: Wuevclr,
    adcclkctl: Adcclkctl,
    tdcclkctl: Tdcclkctl,
    refclkctl: Refclkctl,
    rtcsubsecinc0: Rtcsubsecinc0,
    rtcsubsecinc1: Rtcsubsecinc1,
    rtcsubsecincctl: Rtcsubsecincctl,
    mcubusctl: Mcubusctl,
    mcubusstat: Mcubusstat,
    aonctlstat: Aonctlstat,
    auxiolatch: Auxiolatch,
    _reserved18: [u8; 0x04],
    modclken1: Modclken1,
}
impl RegisterBlock {
    #[doc = "0x00 - Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
    #[inline(always)]
    pub const fn modclken0(&self) -> &Modclken0 {
        &self.modclken0
    }
    #[doc = "0x04 - Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
    #[inline(always)]
    pub const fn pwroffreq(&self) -> &Pwroffreq {
        &self.pwroffreq
    }
    #[doc = "0x08 - Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC"]
    #[inline(always)]
    pub const fn pwrdwnreq(&self) -> &Pwrdwnreq {
        &self.pwrdwnreq
    }
    #[doc = "0x0c - Power Down Acknowledgment"]
    #[inline(always)]
    pub const fn pwrdwnack(&self) -> &Pwrdwnack {
        &self.pwrdwnack
    }
    #[doc = "0x10 - Low Frequency Clock Request"]
    #[inline(always)]
    pub const fn clklfreq(&self) -> &Clklfreq {
        &self.clklfreq
    }
    #[doc = "0x14 - Low Frequency Clock Acknowledgment"]
    #[inline(always)]
    pub const fn clklfack(&self) -> &Clklfack {
        &self.clklfack
    }
    #[doc = "0x28 - Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR"]
    #[inline(always)]
    pub const fn wuevflags(&self) -> &Wuevflags {
        &self.wuevflags
    }
    #[doc = "0x2c - Wake-up Event Clear Clears wake-up events from the AON domain"]
    #[inline(always)]
    pub const fn wuevclr(&self) -> &Wuevclr {
        &self.wuevclr
    }
    #[doc = "0x30 - ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set"]
    #[inline(always)]
    pub const fn adcclkctl(&self) -> &Adcclkctl {
        &self.adcclkctl
    }
    #[doc = "0x34 - TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
    #[inline(always)]
    pub const fn tdcclkctl(&self) -> &Tdcclkctl {
        &self.tdcclkctl
    }
    #[doc = "0x38 - Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
    #[inline(always)]
    pub const fn refclkctl(&self) -> &Refclkctl {
        &self.refclkctl
    }
    #[doc = "0x3c - Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    #[inline(always)]
    pub const fn rtcsubsecinc0(&self) -> &Rtcsubsecinc0 {
        &self.rtcsubsecinc0
    }
    #[doc = "0x40 - Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    #[inline(always)]
    pub const fn rtcsubsecinc1(&self) -> &Rtcsubsecinc1 {
        &self.rtcsubsecinc1
    }
    #[doc = "0x44 - Real Time Counter Sub Second Increment Control"]
    #[inline(always)]
    pub const fn rtcsubsecincctl(&self) -> &Rtcsubsecincctl {
        &self.rtcsubsecincctl
    }
    #[doc = "0x48 - MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
    #[inline(always)]
    pub const fn mcubusctl(&self) -> &Mcubusctl {
        &self.mcubusctl
    }
    #[doc = "0x4c - MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
    #[inline(always)]
    pub const fn mcubusstat(&self) -> &Mcubusstat {
        &self.mcubusstat
    }
    #[doc = "0x50 - AON Domain Control Status Status of AUX domain control from AON_WUC."]
    #[inline(always)]
    pub const fn aonctlstat(&self) -> &Aonctlstat {
        &self.aonctlstat
    }
    #[doc = "0x54 - AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
    #[inline(always)]
    pub const fn auxiolatch(&self) -> &Auxiolatch {
        &self.auxiolatch
    }
    #[doc = "0x5c - Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
    #[inline(always)]
    pub const fn modclken1(&self) -> &Modclken1 {
        &self.modclken1
    }
}
#[doc = "MODCLKEN0 (rw) register accessor: Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modclken0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modclken0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modclken0`]
module"]
#[doc(alias = "MODCLKEN0")]
pub type Modclken0 = crate::Reg<modclken0::Modclken0Spec>;
#[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
pub mod modclken0;
#[doc = "PWROFFREQ (rw) register accessor: Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwroffreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwroffreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwroffreq`]
module"]
#[doc(alias = "PWROFFREQ")]
pub type Pwroffreq = crate::Reg<pwroffreq::PwroffreqSpec>;
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
pub mod pwroffreq;
#[doc = "PWRDWNREQ (rw) register accessor: Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdwnreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdwnreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdwnreq`]
module"]
#[doc(alias = "PWRDWNREQ")]
pub type Pwrdwnreq = crate::Reg<pwrdwnreq::PwrdwnreqSpec>;
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC"]
pub mod pwrdwnreq;
#[doc = "PWRDWNACK (rw) register accessor: Power Down Acknowledgment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrdwnack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrdwnack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrdwnack`]
module"]
#[doc(alias = "PWRDWNACK")]
pub type Pwrdwnack = crate::Reg<pwrdwnack::PwrdwnackSpec>;
#[doc = "Power Down Acknowledgment"]
pub mod pwrdwnack;
#[doc = "CLKLFREQ (rw) register accessor: Low Frequency Clock Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clklfreq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clklfreq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clklfreq`]
module"]
#[doc(alias = "CLKLFREQ")]
pub type Clklfreq = crate::Reg<clklfreq::ClklfreqSpec>;
#[doc = "Low Frequency Clock Request"]
pub mod clklfreq;
#[doc = "CLKLFACK (rw) register accessor: Low Frequency Clock Acknowledgment\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clklfack::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clklfack::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clklfack`]
module"]
#[doc(alias = "CLKLFACK")]
pub type Clklfack = crate::Reg<clklfack::ClklfackSpec>;
#[doc = "Low Frequency Clock Acknowledgment"]
pub mod clklfack;
#[doc = "WUEVFLAGS (rw) register accessor: Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuevflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuevflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuevflags`]
module"]
#[doc(alias = "WUEVFLAGS")]
pub type Wuevflags = crate::Reg<wuevflags::WuevflagsSpec>;
#[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR"]
pub mod wuevflags;
#[doc = "WUEVCLR (rw) register accessor: Wake-up Event Clear Clears wake-up events from the AON domain\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wuevclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wuevclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wuevclr`]
module"]
#[doc(alias = "WUEVCLR")]
pub type Wuevclr = crate::Reg<wuevclr::WuevclrSpec>;
#[doc = "Wake-up Event Clear Clears wake-up events from the AON domain"]
pub mod wuevclr;
#[doc = "ADCCLKCTL (rw) register accessor: ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclkctl`]
module"]
#[doc(alias = "ADCCLKCTL")]
pub type Adcclkctl = crate::Reg<adcclkctl::AdcclkctlSpec>;
#[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set"]
pub mod adcclkctl;
#[doc = "TDCCLKCTL (rw) register accessor: TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdcclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdcclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdcclkctl`]
module"]
#[doc(alias = "TDCCLKCTL")]
pub type Tdcclkctl = crate::Reg<tdcclkctl::TdcclkctlSpec>;
#[doc = "TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
pub mod tdcclkctl;
#[doc = "REFCLKCTL (rw) register accessor: Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refclkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refclkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@refclkctl`]
module"]
#[doc(alias = "REFCLKCTL")]
pub type Refclkctl = crate::Reg<refclkctl::RefclkctlSpec>;
#[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
pub mod refclkctl;
#[doc = "RTCSUBSECINC0 (rw) register accessor: Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecinc0`]
module"]
#[doc(alias = "RTCSUBSECINC0")]
pub type Rtcsubsecinc0 = crate::Reg<rtcsubsecinc0::Rtcsubsecinc0Spec>;
#[doc = "Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
pub mod rtcsubsecinc0;
#[doc = "RTCSUBSECINC1 (rw) register accessor: Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecinc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecinc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecinc1`]
module"]
#[doc(alias = "RTCSUBSECINC1")]
pub type Rtcsubsecinc1 = crate::Reg<rtcsubsecinc1::Rtcsubsecinc1Spec>;
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
pub mod rtcsubsecinc1;
#[doc = "RTCSUBSECINCCTL (rw) register accessor: Real Time Counter Sub Second Increment Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsubsecincctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsubsecincctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsubsecincctl`]
module"]
#[doc(alias = "RTCSUBSECINCCTL")]
pub type Rtcsubsecincctl = crate::Reg<rtcsubsecincctl::RtcsubsecincctlSpec>;
#[doc = "Real Time Counter Sub Second Increment Control"]
pub mod rtcsubsecincctl;
#[doc = "MCUBUSCTL (rw) register accessor: MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcubusctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcubusctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcubusctl`]
module"]
#[doc(alias = "MCUBUSCTL")]
pub type Mcubusctl = crate::Reg<mcubusctl::McubusctlSpec>;
#[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
pub mod mcubusctl;
#[doc = "MCUBUSSTAT (rw) register accessor: MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcubusstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcubusstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcubusstat`]
module"]
#[doc(alias = "MCUBUSSTAT")]
pub type Mcubusstat = crate::Reg<mcubusstat::McubusstatSpec>;
#[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
pub mod mcubusstat;
#[doc = "AONCTLSTAT (rw) register accessor: AON Domain Control Status Status of AUX domain control from AON_WUC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonctlstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonctlstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aonctlstat`]
module"]
#[doc(alias = "AONCTLSTAT")]
pub type Aonctlstat = crate::Reg<aonctlstat::AonctlstatSpec>;
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC."]
pub mod aonctlstat;
#[doc = "AUXIOLATCH (rw) register accessor: AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxiolatch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxiolatch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxiolatch`]
module"]
#[doc(alias = "AUXIOLATCH")]
pub type Auxiolatch = crate::Reg<auxiolatch::AuxiolatchSpec>;
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
pub mod auxiolatch;
#[doc = "MODCLKEN1 (rw) register accessor: Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`modclken1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modclken1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modclken1`]
module"]
#[doc(alias = "MODCLKEN1")]
pub type Modclken1 = crate::Reg<modclken1::Modclken1Spec>;
#[doc = "Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
pub mod modclken1;
