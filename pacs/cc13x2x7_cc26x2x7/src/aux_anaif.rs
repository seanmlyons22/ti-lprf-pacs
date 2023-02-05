#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    #[doc = "0x10 - ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
    pub adcctl: ADCCTL,
    #[doc = "0x14 - ADC FIFO Status FIFO can hold up to four ADC samples."]
    pub adcfifostat: ADCFIFOSTAT,
    #[doc = "0x18 - ADC FIFO"]
    pub adcfifo: ADCFIFO,
    #[doc = "0x1c - ADC Trigger"]
    pub adctrig: ADCTRIG,
    #[doc = "0x20 - Current Source Control"]
    pub isrcctl: ISRCCTL,
    _reserved5: [u8; 0x0c],
    #[doc = "0x30 - DAC Control This register controls the analog part of the DAC."]
    pub dacctl: DACCTL,
    #[doc = "0x34 - Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
    pub lpmbiasctl: LPMBIASCTL,
    #[doc = "0x38 - DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
    pub dacsmplctl: DACSMPLCTL,
    #[doc = "0x3c - DAC Sample Configuration 0"]
    pub dacsmplcfg0: DACSMPLCFG0,
    #[doc = "0x40 - DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
    pub dacsmplcfg1: DACSMPLCFG1,
    #[doc = "0x44 - DAC Value"]
    pub dacvalue: DACVALUE,
    #[doc = "0x48 - DAC Status"]
    pub dacstat: DACSTAT,
}
#[doc = "ADCCTL (rw) register accessor: an alias for `Reg<ADCCTL_SPEC>`"]
pub type ADCCTL = crate::Reg<adcctl::ADCCTL_SPEC>;
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
pub mod adcctl;
#[doc = "ADCFIFOSTAT (rw) register accessor: an alias for `Reg<ADCFIFOSTAT_SPEC>`"]
pub type ADCFIFOSTAT = crate::Reg<adcfifostat::ADCFIFOSTAT_SPEC>;
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
pub mod adcfifostat;
#[doc = "ADCFIFO (rw) register accessor: an alias for `Reg<ADCFIFO_SPEC>`"]
pub type ADCFIFO = crate::Reg<adcfifo::ADCFIFO_SPEC>;
#[doc = "ADC FIFO"]
pub mod adcfifo;
#[doc = "ADCTRIG (rw) register accessor: an alias for `Reg<ADCTRIG_SPEC>`"]
pub type ADCTRIG = crate::Reg<adctrig::ADCTRIG_SPEC>;
#[doc = "ADC Trigger"]
pub mod adctrig;
#[doc = "ISRCCTL (rw) register accessor: an alias for `Reg<ISRCCTL_SPEC>`"]
pub type ISRCCTL = crate::Reg<isrcctl::ISRCCTL_SPEC>;
#[doc = "Current Source Control"]
pub mod isrcctl;
#[doc = "DACCTL (rw) register accessor: an alias for `Reg<DACCTL_SPEC>`"]
pub type DACCTL = crate::Reg<dacctl::DACCTL_SPEC>;
#[doc = "DAC Control This register controls the analog part of the DAC."]
pub mod dacctl;
#[doc = "LPMBIASCTL (rw) register accessor: an alias for `Reg<LPMBIASCTL_SPEC>`"]
pub type LPMBIASCTL = crate::Reg<lpmbiasctl::LPMBIASCTL_SPEC>;
#[doc = "Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
pub mod lpmbiasctl;
#[doc = "DACSMPLCTL (rw) register accessor: an alias for `Reg<DACSMPLCTL_SPEC>`"]
pub type DACSMPLCTL = crate::Reg<dacsmplctl::DACSMPLCTL_SPEC>;
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
pub mod dacsmplctl;
#[doc = "DACSMPLCFG0 (rw) register accessor: an alias for `Reg<DACSMPLCFG0_SPEC>`"]
pub type DACSMPLCFG0 = crate::Reg<dacsmplcfg0::DACSMPLCFG0_SPEC>;
#[doc = "DAC Sample Configuration 0"]
pub mod dacsmplcfg0;
#[doc = "DACSMPLCFG1 (rw) register accessor: an alias for `Reg<DACSMPLCFG1_SPEC>`"]
pub type DACSMPLCFG1 = crate::Reg<dacsmplcfg1::DACSMPLCFG1_SPEC>;
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) < 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
pub mod dacsmplcfg1;
#[doc = "DACVALUE (rw) register accessor: an alias for `Reg<DACVALUE_SPEC>`"]
pub type DACVALUE = crate::Reg<dacvalue::DACVALUE_SPEC>;
#[doc = "DAC Value"]
pub mod dacvalue;
#[doc = "DACSTAT (rw) register accessor: an alias for `Reg<DACSTAT_SPEC>`"]
pub type DACSTAT = crate::Reg<dacstat::DACSTAT_SPEC>;
#[doc = "DAC Status"]
pub mod dacstat;
