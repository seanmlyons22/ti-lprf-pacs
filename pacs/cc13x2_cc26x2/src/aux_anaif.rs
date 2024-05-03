#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    adcctl: Adcctl,
    adcfifostat: Adcfifostat,
    adcfifo: Adcfifo,
    adctrig: Adctrig,
    isrcctl: Isrcctl,
    _reserved5: [u8; 0x0c],
    dacctl: Dacctl,
    lpmbiasctl: Lpmbiasctl,
    dacsmplctl: Dacsmplctl,
    dacsmplcfg0: Dacsmplcfg0,
    dacsmplcfg1: Dacsmplcfg1,
    dacvalue: Dacvalue,
    dacstat: Dacstat,
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
    #[doc = "0x30 - DAC Control This register controls the analog part of the DAC."]
    #[inline(always)]
    pub const fn dacctl(&self) -> &Dacctl {
        &self.dacctl
    }
    #[doc = "0x34 - Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
    #[inline(always)]
    pub const fn lpmbiasctl(&self) -> &Lpmbiasctl {
        &self.lpmbiasctl
    }
    #[doc = "0x38 - DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
    #[inline(always)]
    pub const fn dacsmplctl(&self) -> &Dacsmplctl {
        &self.dacsmplctl
    }
    #[doc = "0x3c - DAC Sample Configuration 0"]
    #[inline(always)]
    pub const fn dacsmplcfg0(&self) -> &Dacsmplcfg0 {
        &self.dacsmplcfg0
    }
    #[doc = "0x40 - DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) &lt; 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
    #[inline(always)]
    pub const fn dacsmplcfg1(&self) -> &Dacsmplcfg1 {
        &self.dacsmplcfg1
    }
    #[doc = "0x44 - DAC Value"]
    #[inline(always)]
    pub const fn dacvalue(&self) -> &Dacvalue {
        &self.dacvalue
    }
    #[doc = "0x48 - DAC Status"]
    #[inline(always)]
    pub const fn dacstat(&self) -> &Dacstat {
        &self.dacstat
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
#[doc = "DACCTL (rw) register accessor: DAC Control This register controls the analog part of the DAC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacctl`]
module"]
#[doc(alias = "DACCTL")]
pub type Dacctl = crate::Reg<dacctl::DacctlSpec>;
#[doc = "DAC Control This register controls the analog part of the DAC."]
pub mod dacctl;
#[doc = "LPMBIASCTL (rw) register accessor: Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpmbiasctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpmbiasctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmbiasctl`]
module"]
#[doc(alias = "LPMBIASCTL")]
pub type Lpmbiasctl = crate::Reg<lpmbiasctl::LpmbiasctlSpec>;
#[doc = "Low Power Mode Bias Control The low power mode bias module provides bias current to DAC and Comparator A when AUX_SYSIF:OPMODEREQ.REQ differers from A."]
pub mod lpmbiasctl;
#[doc = "DACSMPLCTL (rw) register accessor: DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacsmplctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacsmplctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacsmplctl`]
module"]
#[doc(alias = "DACSMPLCTL")]
pub type Dacsmplctl = crate::Reg<dacsmplctl::DacsmplctlSpec>;
#[doc = "DAC Sample Control The DAC sample clock maintains the DAC voltage stored in the sample-and-hold capacitor. The DAC sample clock waveform consists of a setup phase followed by a hold phase. In the setup phase the sample-and-hold capacitor charges to the programmed voltage. The hold phase maintains the voltage with minimal power. DACSMPLCFG0 and DACSMPLCFG1 configure the DAC sample clock waveform."]
pub mod dacsmplctl;
#[doc = "DACSMPLCFG0 (rw) register accessor: DAC Sample Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacsmplcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacsmplcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacsmplcfg0`]
module"]
#[doc(alias = "DACSMPLCFG0")]
pub type Dacsmplcfg0 = crate::Reg<dacsmplcfg0::Dacsmplcfg0Spec>;
#[doc = "DAC Sample Configuration 0"]
pub mod dacsmplcfg0;
#[doc = "DACSMPLCFG1 (rw) register accessor: DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) &lt; 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacsmplcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacsmplcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacsmplcfg1`]
module"]
#[doc(alias = "DACSMPLCFG1")]
pub type Dacsmplcfg1 = crate::Reg<dacsmplcfg1::Dacsmplcfg1Spec>;
#[doc = "DAC Sample Configuration 1 The sample clock period equals (high time + low time) * base period. DACSMPLCFG0.CLKDIV determines the base period. Timing requirements (DAC Buffer On / DAC Buffer Off): - (high time + low time) * base period > (4 us / 1 us) - (high time * base period) > (2 us / 0.5 us) - (low time * base period) > (2 us / 0.5 us) - (low time * base period + HOLD_INTERVAL * sample clock period) &lt; 32 us If AUX_SYSIF:OPMODEREQ.REQ equals PDLP, you must set: - H_PER = L_PER = HOLD_INTERVAL = 0."]
pub mod dacsmplcfg1;
#[doc = "DACVALUE (rw) register accessor: DAC Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacvalue::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacvalue::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacvalue`]
module"]
#[doc(alias = "DACVALUE")]
pub type Dacvalue = crate::Reg<dacvalue::DacvalueSpec>;
#[doc = "DAC Value"]
pub mod dacvalue;
#[doc = "DACSTAT (rw) register accessor: DAC Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dacstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dacstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dacstat`]
module"]
#[doc(alias = "DACSTAT")]
pub type Dacstat = crate::Reg<dacstat::DacstatSpec>;
#[doc = "DAC Status"]
pub mod dacstat;
