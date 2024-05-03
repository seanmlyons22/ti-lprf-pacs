#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    adcctl: Adcctl,
    adcfifostat: Adcfifostat,
    adcfifo: Adcfifo,
    adctrig: Adctrig,
    isrcctl: Isrcctl,
}
impl RegisterBlock {
    #[doc = "0x10 - ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
    #[inline(always)]
    pub const fn adcctl(&self) -> &Adcctl {
        &self.adcctl
    }
    #[doc = "0x14 - ADC FIFO Status FIFO can hold up to four ADC samples."]
    #[inline(always)]
    pub const fn adcfifostat(&self) -> &Adcfifostat {
        &self.adcfifostat
    }
    #[doc = "0x18 - ADC FIFO"]
    #[inline(always)]
    pub const fn adcfifo(&self) -> &Adcfifo {
        &self.adcfifo
    }
    #[doc = "0x1c - ADC Trigger"]
    #[inline(always)]
    pub const fn adctrig(&self) -> &Adctrig {
        &self.adctrig
    }
    #[doc = "0x20 - Current Source Control"]
    #[inline(always)]
    pub const fn isrcctl(&self) -> &Isrcctl {
        &self.isrcctl
    }
}
#[doc = "ADCCTL (rw) register accessor: ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcctl`]
module"]
#[doc(alias = "ADCCTL")]
pub type Adcctl = crate::Reg<adcctl::AdcctlSpec>;
#[doc = "ADC Control Configuration of ADI_4_AUX:ADC0.SMPL_MODE decides if the ADC trigger starts sampling or conversion."]
pub mod adcctl;
#[doc = "ADCFIFOSTAT (rw) register accessor: ADC FIFO Status FIFO can hold up to four ADC samples.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcfifostat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcfifostat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcfifostat`]
module"]
#[doc(alias = "ADCFIFOSTAT")]
pub type Adcfifostat = crate::Reg<adcfifostat::AdcfifostatSpec>;
#[doc = "ADC FIFO Status FIFO can hold up to four ADC samples."]
pub mod adcfifostat;
#[doc = "ADCFIFO (rw) register accessor: ADC FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcfifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcfifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcfifo`]
module"]
#[doc(alias = "ADCFIFO")]
pub type Adcfifo = crate::Reg<adcfifo::AdcfifoSpec>;
#[doc = "ADC FIFO"]
pub mod adcfifo;
#[doc = "ADCTRIG (rw) register accessor: ADC Trigger\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adctrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adctrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adctrig`]
module"]
#[doc(alias = "ADCTRIG")]
pub type Adctrig = crate::Reg<adctrig::AdctrigSpec>;
#[doc = "ADC Trigger"]
pub mod adctrig;
#[doc = "ISRCCTL (rw) register accessor: Current Source Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isrcctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isrcctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isrcctl`]
module"]
#[doc(alias = "ISRCCTL")]
pub type Isrcctl = crate::Reg<isrcctl::IsrcctlSpec>;
#[doc = "Current Source Control"]
pub mod isrcctl;
