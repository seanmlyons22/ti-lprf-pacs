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
