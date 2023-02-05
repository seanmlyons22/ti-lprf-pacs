#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    pub mux0: MUX0,
    #[doc = "0x01 - Internal. Only to be used through TI provided API."]
    pub mux1: MUX1,
    #[doc = "0x02 - Internal. Only to be used through TI provided API."]
    pub mux2: MUX2,
    #[doc = "0x03 - Internal. Only to be used through TI provided API."]
    pub mux3: MUX3,
    #[doc = "0x04 - Current Source Strength and trim control for current source. Only to be used through TI provided API."]
    pub isrc: ISRC,
    #[doc = "0x05 - Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
    pub comp: COMP,
    _reserved6: [u8; 0x01],
    #[doc = "0x07 - Internal. Only to be used through TI provided API."]
    pub mux4: MUX4,
    #[doc = "0x08 - ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
    pub adc0: ADC0,
    #[doc = "0x09 - ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
    pub adc1: ADC1,
    #[doc = "0x0a - ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
    pub adcref0: ADCREF0,
    #[doc = "0x0b - ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
    pub adcref1: ADCREF1,
    _reserved11: [u8; 0x02],
    #[doc = "0x0e - Internal. Only to be used through TI provided API."]
    pub lpmbias: LPMBIAS,
    #[doc = "0x0f - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub stat: STAT,
}
#[doc = "MUX0 (rw) register accessor: an alias for `Reg<MUX0_SPEC>`"]
pub type MUX0 = crate::Reg<mux0::MUX0_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux0;
#[doc = "MUX1 (rw) register accessor: an alias for `Reg<MUX1_SPEC>`"]
pub type MUX1 = crate::Reg<mux1::MUX1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux1;
#[doc = "MUX2 (rw) register accessor: an alias for `Reg<MUX2_SPEC>`"]
pub type MUX2 = crate::Reg<mux2::MUX2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux2;
#[doc = "MUX3 (rw) register accessor: an alias for `Reg<MUX3_SPEC>`"]
pub type MUX3 = crate::Reg<mux3::MUX3_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux3;
#[doc = "ISRC (rw) register accessor: an alias for `Reg<ISRC_SPEC>`"]
pub type ISRC = crate::Reg<isrc::ISRC_SPEC>;
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API."]
pub mod isrc;
#[doc = "COMP (rw) register accessor: an alias for `Reg<COMP_SPEC>`"]
pub type COMP = crate::Reg<comp::COMP_SPEC>;
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
pub mod comp;
#[doc = "MUX4 (rw) register accessor: an alias for `Reg<MUX4_SPEC>`"]
pub type MUX4 = crate::Reg<mux4::MUX4_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux4;
#[doc = "ADC0 (rw) register accessor: an alias for `Reg<ADC0_SPEC>`"]
pub type ADC0 = crate::Reg<adc0::ADC0_SPEC>;
#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
pub mod adc0;
#[doc = "ADC1 (rw) register accessor: an alias for `Reg<ADC1_SPEC>`"]
pub type ADC1 = crate::Reg<adc1::ADC1_SPEC>;
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
pub mod adc1;
#[doc = "ADCREF0 (rw) register accessor: an alias for `Reg<ADCREF0_SPEC>`"]
pub type ADCREF0 = crate::Reg<adcref0::ADCREF0_SPEC>;
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref0;
#[doc = "ADCREF1 (rw) register accessor: an alias for `Reg<ADCREF1_SPEC>`"]
pub type ADCREF1 = crate::Reg<adcref1::ADCREF1_SPEC>;
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref1;
#[doc = "LPMBIAS (rw) register accessor: an alias for `Reg<LPMBIAS_SPEC>`"]
pub type LPMBIAS = crate::Reg<lpmbias::LPMBIAS_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod lpmbias;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod stat;
