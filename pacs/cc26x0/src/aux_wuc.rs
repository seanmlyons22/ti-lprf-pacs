#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
    pub modclken0: MODCLKEN0,
    #[doc = "0x04 - Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
    pub pwroffreq: PWROFFREQ,
    #[doc = "0x08 - Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC"]
    pub pwrdwnreq: PWRDWNREQ,
    #[doc = "0x0c - Power Down Acknowledgment"]
    pub pwrdwnack: PWRDWNACK,
    #[doc = "0x10 - Low Frequency Clock Request"]
    pub clklfreq: CLKLFREQ,
    #[doc = "0x14 - Low Frequency Clock Acknowledgment"]
    pub clklfack: CLKLFACK,
    _reserved6: [u8; 0x10],
    #[doc = "0x28 - Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR"]
    pub wuevflags: WUEVFLAGS,
    #[doc = "0x2c - Wake-up Event Clear Clears wake-up events from the AON domain"]
    pub wuevclr: WUEVCLR,
    #[doc = "0x30 - ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set"]
    pub adcclkctl: ADCCLKCTL,
    #[doc = "0x34 - TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
    pub tdcclkctl: TDCCLKCTL,
    #[doc = "0x38 - Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
    pub refclkctl: REFCLKCTL,
    #[doc = "0x3c - Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    pub rtcsubsecinc0: RTCSUBSECINC0,
    #[doc = "0x40 - Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
    pub rtcsubsecinc1: RTCSUBSECINC1,
    #[doc = "0x44 - Real Time Counter Sub Second Increment Control"]
    pub rtcsubsecincctl: RTCSUBSECINCCTL,
    #[doc = "0x48 - MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
    pub mcubusctl: MCUBUSCTL,
    #[doc = "0x4c - MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
    pub mcubusstat: MCUBUSSTAT,
    #[doc = "0x50 - AON Domain Control Status Status of AUX domain control from AON_WUC."]
    pub aonctlstat: AONCTLSTAT,
    #[doc = "0x54 - AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
    pub auxiolatch: AUXIOLATCH,
    _reserved18: [u8; 0x04],
    #[doc = "0x5c - Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
    pub modclken1: MODCLKEN1,
}
#[doc = "MODCLKEN0 (rw) register accessor: an alias for `Reg<MODCLKEN0_SPEC>`"]
pub type MODCLKEN0 = crate::Reg<modclken0::MODCLKEN0_SPEC>;
#[doc = "Module Clock Enable Clock enable for each module in the AUX domain For use by the system CPU The settings in this register are OR'ed with the corresponding settings in MODCLKEN1. This allows the system CPU and AUX_SCE to request clocks independently. Settings take effect immediately."]
pub mod modclken0;
#[doc = "PWROFFREQ (rw) register accessor: an alias for `Reg<PWROFFREQ_SPEC>`"]
pub type PWROFFREQ = crate::Reg<pwroffreq::PWROFFREQ_SPEC>;
#[doc = "Power Off Request Requests power off request for the AUX domain. When powered off, the power supply and clock is disabled. This may only be used when taking the entire device into shutdown mode (i.e. with full device reset when resuming operation). Power off is prevented if AON_WUC:AUXCTL.AUX_FORCE_ON has been set, or if MCUBUSCTL.DISCONNECT_REQ has been cleared."]
pub mod pwroffreq;
#[doc = "PWRDWNREQ (rw) register accessor: an alias for `Reg<PWRDWNREQ_SPEC>`"]
pub type PWRDWNREQ = crate::Reg<pwrdwnreq::PWRDWNREQ_SPEC>;
#[doc = "Power Down Request Request from AUX for system to enter power down. When system is in power down there is limited current supply available and the clock source is set by AON_WUC:AUXCLK.PWR_DWN_SRC"]
pub mod pwrdwnreq;
#[doc = "PWRDWNACK (rw) register accessor: an alias for `Reg<PWRDWNACK_SPEC>`"]
pub type PWRDWNACK = crate::Reg<pwrdwnack::PWRDWNACK_SPEC>;
#[doc = "Power Down Acknowledgment"]
pub mod pwrdwnack;
#[doc = "CLKLFREQ (rw) register accessor: an alias for `Reg<CLKLFREQ_SPEC>`"]
pub type CLKLFREQ = crate::Reg<clklfreq::CLKLFREQ_SPEC>;
#[doc = "Low Frequency Clock Request"]
pub mod clklfreq;
#[doc = "CLKLFACK (rw) register accessor: an alias for `Reg<CLKLFACK_SPEC>`"]
pub type CLKLFACK = crate::Reg<clklfack::CLKLFACK_SPEC>;
#[doc = "Low Frequency Clock Acknowledgment"]
pub mod clklfack;
#[doc = "WUEVFLAGS (rw) register accessor: an alias for `Reg<WUEVFLAGS_SPEC>`"]
pub type WUEVFLAGS = crate::Reg<wuevflags::WUEVFLAGS_SPEC>;
#[doc = "Wake-up Event Flags Status of wake-up events from the AON domain The event flags are cleared by setting the corresponding bits in WUEVCLR"]
pub mod wuevflags;
#[doc = "WUEVCLR (rw) register accessor: an alias for `Reg<WUEVCLR_SPEC>`"]
pub type WUEVCLR = crate::Reg<wuevclr::WUEVCLR_SPEC>;
#[doc = "Wake-up Event Clear Clears wake-up events from the AON domain"]
pub mod wuevclr;
#[doc = "ADCCLKCTL (rw) register accessor: an alias for `Reg<ADCCLKCTL_SPEC>`"]
pub type ADCCLKCTL = crate::Reg<adcclkctl::ADCCLKCTL_SPEC>;
#[doc = "ADC Clock Control Controls the ADC internal clock Note that the ADC command and data interface requires MODCLKEN0.ANAIF or MODCLKEN1.ANAIF also to be set"]
pub mod adcclkctl;
#[doc = "TDCCLKCTL (rw) register accessor: an alias for `Reg<TDCCLKCTL_SPEC>`"]
pub type TDCCLKCTL = crate::Reg<tdcclkctl::TDCCLKCTL_SPEC>;
#[doc = "TDC Clock Control Controls the TDC counter clock source, which steps the TDC counter value The source of this clock is controlled by OSC_DIG:CTL0.ACLK_TDC_SRC_SEL."]
pub mod tdcclkctl;
#[doc = "REFCLKCTL (rw) register accessor: an alias for `Reg<REFCLKCTL_SPEC>`"]
pub type REFCLKCTL = crate::Reg<refclkctl::REFCLKCTL_SPEC>;
#[doc = "Reference Clock Control Controls the TDC reference clock source, which is to be compared against the TDC counter clock. The source of this clock is controlled by OSC_DIG:CTL0.ACLK_REF_SRC_SEL."]
pub mod refclkctl;
#[doc = "RTCSUBSECINC0 (rw) register accessor: an alias for `Reg<RTCSUBSECINC0_SPEC>`"]
pub type RTCSUBSECINC0 = crate::Reg<rtcsubsecinc0::RTCSUBSECINC0_SPEC>;
#[doc = "Real Time Counter Sub Second Increment 0 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 15:0. After setting INC15_0 and RTCSUBSECINC1.INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
pub mod rtcsubsecinc0;
#[doc = "RTCSUBSECINC1 (rw) register accessor: an alias for `Reg<RTCSUBSECINC1_SPEC>`"]
pub type RTCSUBSECINC1 = crate::Reg<rtcsubsecinc1::RTCSUBSECINC1_SPEC>;
#[doc = "Real Time Counter Sub Second Increment 1 New value for the real-time counter (AON_RTC) sub-second increment value, part corresponding to AON_RTC:SUBSECINC bits 23:16. After setting RTCSUBSECINC0.INC15_0 and INC23_16, the value is loaded into AON_RTC:SUBSECINC.VALUEINC by setting RTCSUBSECINCCTL.UPD_REQ."]
pub mod rtcsubsecinc1;
#[doc = "RTCSUBSECINCCTL (rw) register accessor: an alias for `Reg<RTCSUBSECINCCTL_SPEC>`"]
pub type RTCSUBSECINCCTL = crate::Reg<rtcsubsecincctl::RTCSUBSECINCCTL_SPEC>;
#[doc = "Real Time Counter Sub Second Increment Control"]
pub mod rtcsubsecincctl;
#[doc = "MCUBUSCTL (rw) register accessor: an alias for `Reg<MCUBUSCTL_SPEC>`"]
pub type MCUBUSCTL = crate::Reg<mcubusctl::MCUBUSCTL_SPEC>;
#[doc = "MCU Bus Control Controls the connection between the AUX domain bus and the MCU domain bus. The buses must be disconnected to allow power-down or power-off of the AUX domain."]
pub mod mcubusctl;
#[doc = "MCUBUSSTAT (rw) register accessor: an alias for `Reg<MCUBUSSTAT_SPEC>`"]
pub type MCUBUSSTAT = crate::Reg<mcubusstat::MCUBUSSTAT_SPEC>;
#[doc = "MCU Bus Status Indicates the connection state of the AUX domain and MCU domain buses. Note that this register cannot be read from the MCU domain while disconnected, and is therefore only useful for the AUX_SCE."]
pub mod mcubusstat;
#[doc = "AONCTLSTAT (rw) register accessor: an alias for `Reg<AONCTLSTAT_SPEC>`"]
pub type AONCTLSTAT = crate::Reg<aonctlstat::AONCTLSTAT_SPEC>;
#[doc = "AON Domain Control Status Status of AUX domain control from AON_WUC."]
pub mod aonctlstat;
#[doc = "AUXIOLATCH (rw) register accessor: an alias for `Reg<AUXIOLATCH_SPEC>`"]
pub type AUXIOLATCH = crate::Reg<auxiolatch::AUXIOLATCH_SPEC>;
#[doc = "AUX Input Output Latch Controls latching of signals between AUX_AIODIO0/AUX_AIODIO1 and AON_IOC."]
pub mod auxiolatch;
#[doc = "MODCLKEN1 (rw) register accessor: an alias for `Reg<MODCLKEN1_SPEC>`"]
pub type MODCLKEN1 = crate::Reg<modclken1::MODCLKEN1_SPEC>;
#[doc = "Module Clock Enable 1 Clock enable for each module in the AUX domain, for use by the AUX_SCE. Settings take effect immediately. The settings in this register are OR'ed with the corresponding settings in MODCLKEN0. This allows system CPU and AUX_SCE to request clocks independently."]
pub mod modclken1;
