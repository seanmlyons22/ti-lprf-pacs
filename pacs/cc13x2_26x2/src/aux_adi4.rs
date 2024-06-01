#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mux0: Mux0,
    mux1: Mux1,
    mux2: Mux2,
    mux3: Mux3,
    isrc: Isrc,
    comp: Comp,
    _reserved6: [u8; 0x01],
    mux4: Mux4,
    adc0: Adc0,
    adc1: Adc1,
    adcref0: Adcref0,
    adcref1: Adcref1,
    _reserved11: [u8; 0x02],
    lpmbias: Lpmbias,
    stat: Stat,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn mux0(&self) -> &Mux0 {
        &self.mux0
    }
    #[doc = "0x01 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn mux1(&self) -> &Mux1 {
        &self.mux1
    }
    #[doc = "0x02 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn mux2(&self) -> &Mux2 {
        &self.mux2
    }
    #[doc = "0x03 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn mux3(&self) -> &Mux3 {
        &self.mux3
    }
    #[doc = "0x04 - Current Source Strength and trim control for current source. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn isrc(&self) -> &Isrc {
        &self.isrc
    }
    #[doc = "0x05 - Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn comp(&self) -> &Comp {
        &self.comp
    }
    #[doc = "0x07 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn mux4(&self) -> &Mux4 {
        &self.mux4
    }
    #[doc = "0x08 - ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn adc0(&self) -> &Adc0 {
        &self.adc0
    }
    #[doc = "0x09 - ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn adc1(&self) -> &Adc1 {
        &self.adc1
    }
    #[doc = "0x0a - ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn adcref0(&self) -> &Adcref0 {
        &self.adcref0
    }
    #[doc = "0x0b - ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn adcref1(&self) -> &Adcref1 {
        &self.adcref1
    }
    #[doc = "0x0e - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn lpmbias(&self) -> &Lpmbias {
        &self.lpmbias
    }
    #[doc = "0x0f - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
}
#[doc = "MUX0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux0`]
module"]
#[doc(alias = "MUX0")]
pub type Mux0 = crate::Reg<mux0::Mux0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux0;
#[doc = "MUX1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux1`]
module"]
#[doc(alias = "MUX1")]
pub type Mux1 = crate::Reg<mux1::Mux1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux1;
#[doc = "MUX2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux2`]
module"]
#[doc(alias = "MUX2")]
pub type Mux2 = crate::Reg<mux2::Mux2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux2;
#[doc = "MUX3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux3`]
module"]
#[doc(alias = "MUX3")]
pub type Mux3 = crate::Reg<mux3::Mux3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux3;
#[doc = "ISRC (rw) register accessor: Current Source Strength and trim control for current source. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isrc`]
module"]
#[doc(alias = "ISRC")]
pub type Isrc = crate::Reg<isrc::IsrcSpec>;
#[doc = "Current Source Strength and trim control for current source. Only to be used through TI provided API."]
pub mod isrc;
#[doc = "COMP (rw) register accessor: Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp`]
module"]
#[doc(alias = "COMP")]
pub type Comp = crate::Reg<comp::CompSpec>;
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API."]
pub mod comp;
#[doc = "MUX4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mux4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mux4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mux4`]
module"]
#[doc(alias = "MUX4")]
pub type Mux4 = crate::Reg<mux4::Mux4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod mux4;
#[doc = "ADC0 (rw) register accessor: ADC Control 0 ADC Sample Control. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc0`]
module"]
#[doc(alias = "ADC0")]
pub type Adc0 = crate::Reg<adc0::Adc0Spec>;
#[doc = "ADC Control 0 ADC Sample Control. Only to be used through TI provided API."]
pub mod adc0;
#[doc = "ADC1 (rw) register accessor: ADC Control 1 ADC Comparator Control. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1`]
module"]
#[doc(alias = "ADC1")]
pub type Adc1 = crate::Reg<adc1::Adc1Spec>;
#[doc = "ADC Control 1 ADC Comparator Control. Only to be used through TI provided API."]
pub mod adc1;
#[doc = "ADCREF0 (rw) register accessor: ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcref0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcref0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcref0`]
module"]
#[doc(alias = "ADCREF0")]
pub type Adcref0 = crate::Reg<adcref0::Adcref0Spec>;
#[doc = "ADC Reference 0 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref0;
#[doc = "ADCREF1 (rw) register accessor: ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcref1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcref1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcref1`]
module"]
#[doc(alias = "ADCREF1")]
pub type Adcref1 = crate::Reg<adcref1::Adcref1Spec>;
#[doc = "ADC Reference 1 Control reference used by the ADC. Only to be used through TI provided API."]
pub mod adcref1;
#[doc = "LPMBIAS (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmbias::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmbias::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmbias`]
module"]
#[doc(alias = "LPMBIAS")]
pub type Lpmbias = crate::Reg<lpmbias::LpmbiasSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod lpmbias;
#[doc = "STAT (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod stat;
