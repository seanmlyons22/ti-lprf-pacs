#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl0: Ctl0,
    ctl1: Ctl1,
    radcextcfg: Radcextcfg,
    ampcompctl: Ampcompctl,
    ampcompth1: Ampcompth1,
    ampcompth2: Ampcompth2,
    anabypassval1: Anabypassval1,
    anabypassval2: Anabypassval2,
    atestctl: Atestctl,
    adcdoublernanoampctl: Adcdoublernanoampctl,
    xoschfctl: Xoschfctl,
    lfoscctl: Lfoscctl,
    rcoschfctl: Rcoschfctl,
    rcoscmfctl: Rcoscmfctl,
    _reserved14: [u8; 0x04],
    stat0: Stat0,
    stat1: Stat1,
    stat2: Stat2,
}
impl RegisterBlock {
    #[doc = "0x00 - Control 0 Controls clock source selects"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x04 - Control 1 This register contains OSC_DIG configuration"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x08 - RADC External Configuration"]
    #[inline(always)]
    pub const fn radcextcfg(&self) -> &Radcextcfg {
        &self.radcextcfg
    }
    #[doc = "0x0c - Amplitude Compensation Control"]
    #[inline(always)]
    pub const fn ampcompctl(&self) -> &Ampcompctl {
        &self.ampcompctl
    }
    #[doc = "0x10 - Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm"]
    #[inline(always)]
    pub const fn ampcompth1(&self) -> &Ampcompth1 {
        &self.ampcompth1
    }
    #[doc = "0x14 - Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
    #[inline(always)]
    pub const fn ampcompth2(&self) -> &Ampcompth2 {
        &self.ampcompth2
    }
    #[doc = "0x18 - Analog Bypass Values 1"]
    #[inline(always)]
    pub const fn anabypassval1(&self) -> &Anabypassval1 {
        &self.anabypassval1
    }
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn anabypassval2(&self) -> &Anabypassval2 {
        &self.anabypassval2
    }
    #[doc = "0x20 - Analog Test Control"]
    #[inline(always)]
    pub const fn atestctl(&self) -> &Atestctl {
        &self.atestctl
    }
    #[doc = "0x24 - ADC Doubler Nanoamp Control"]
    #[inline(always)]
    pub const fn adcdoublernanoampctl(&self) -> &Adcdoublernanoampctl {
        &self.adcdoublernanoampctl
    }
    #[doc = "0x28 - XOSCHF Control"]
    #[inline(always)]
    pub const fn xoschfctl(&self) -> &Xoschfctl {
        &self.xoschfctl
    }
    #[doc = "0x2c - Low Frequency Oscillator Control"]
    #[inline(always)]
    pub const fn lfoscctl(&self) -> &Lfoscctl {
        &self.lfoscctl
    }
    #[doc = "0x30 - RCOSCHF Control"]
    #[inline(always)]
    pub const fn rcoschfctl(&self) -> &Rcoschfctl {
        &self.rcoschfctl
    }
    #[doc = "0x34 - RCOSC_MF Control"]
    #[inline(always)]
    pub const fn rcoscmfctl(&self) -> &Rcoscmfctl {
        &self.rcoscmfctl
    }
    #[doc = "0x3c - Status 0 This register contains status signals from OSC_DIG"]
    #[inline(always)]
    pub const fn stat0(&self) -> &Stat0 {
        &self.stat0
    }
    #[doc = "0x40 - Status 1 This register contains status signals from OSC_DIG"]
    #[inline(always)]
    pub const fn stat1(&self) -> &Stat1 {
        &self.stat1
    }
    #[doc = "0x44 - Status 2 This register contains status signals from AMPCOMP FSM"]
    #[inline(always)]
    pub const fn stat2(&self) -> &Stat2 {
        &self.stat2
    }
}
#[doc = "CTL0 (rw) register accessor: Control 0 Controls clock source selects\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control 0 Controls clock source selects"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control 1 This register contains OSC_DIG configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control 1 This register contains OSC_DIG configuration"]
pub mod ctl1;
#[doc = "RADCEXTCFG (rw) register accessor: RADC External Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`radcextcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`radcextcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@radcextcfg`]
module"]
#[doc(alias = "RADCEXTCFG")]
pub type Radcextcfg = crate::Reg<radcextcfg::RadcextcfgSpec>;
#[doc = "RADC External Configuration"]
pub mod radcextcfg;
#[doc = "AMPCOMPCTL (rw) register accessor: Amplitude Compensation Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcompctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcompctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcompctl`]
module"]
#[doc(alias = "AMPCOMPCTL")]
pub type Ampcompctl = crate::Reg<ampcompctl::AmpcompctlSpec>;
#[doc = "Amplitude Compensation Control"]
pub mod ampcompctl;
#[doc = "AMPCOMPTH1 (rw) register accessor: Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcompth1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcompth1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcompth1`]
module"]
#[doc(alias = "AMPCOMPTH1")]
pub type Ampcompth1 = crate::Reg<ampcompth1::Ampcompth1Spec>;
#[doc = "Amplitude Compensation Threshold 1 This register contains threshold values for amplitude compensation algorithm"]
pub mod ampcompth1;
#[doc = "AMPCOMPTH2 (rw) register accessor: Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcompth2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcompth2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcompth2`]
module"]
#[doc(alias = "AMPCOMPTH2")]
pub type Ampcompth2 = crate::Reg<ampcompth2::Ampcompth2Spec>;
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm."]
pub mod ampcompth2;
#[doc = "ANABYPASSVAL1 (rw) register accessor: Analog Bypass Values 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anabypassval1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anabypassval1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anabypassval1`]
module"]
#[doc(alias = "ANABYPASSVAL1")]
pub type Anabypassval1 = crate::Reg<anabypassval1::Anabypassval1Spec>;
#[doc = "Analog Bypass Values 1"]
pub mod anabypassval1;
#[doc = "ANABYPASSVAL2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`anabypassval2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`anabypassval2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@anabypassval2`]
module"]
#[doc(alias = "ANABYPASSVAL2")]
pub type Anabypassval2 = crate::Reg<anabypassval2::Anabypassval2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod anabypassval2;
#[doc = "ATESTCTL (rw) register accessor: Analog Test Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atestctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atestctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atestctl`]
module"]
#[doc(alias = "ATESTCTL")]
pub type Atestctl = crate::Reg<atestctl::AtestctlSpec>;
#[doc = "Analog Test Control"]
pub mod atestctl;
#[doc = "ADCDOUBLERNANOAMPCTL (rw) register accessor: ADC Doubler Nanoamp Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcdoublernanoampctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcdoublernanoampctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcdoublernanoampctl`]
module"]
#[doc(alias = "ADCDOUBLERNANOAMPCTL")]
pub type Adcdoublernanoampctl = crate::Reg<adcdoublernanoampctl::AdcdoublernanoampctlSpec>;
#[doc = "ADC Doubler Nanoamp Control"]
pub mod adcdoublernanoampctl;
#[doc = "XOSCHFCTL (rw) register accessor: XOSCHF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xoschfctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xoschfctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xoschfctl`]
module"]
#[doc(alias = "XOSCHFCTL")]
pub type Xoschfctl = crate::Reg<xoschfctl::XoschfctlSpec>;
#[doc = "XOSCHF Control"]
pub mod xoschfctl;
#[doc = "LFOSCCTL (rw) register accessor: Low Frequency Oscillator Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfoscctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfoscctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfoscctl`]
module"]
#[doc(alias = "LFOSCCTL")]
pub type Lfoscctl = crate::Reg<lfoscctl::LfoscctlSpec>;
#[doc = "Low Frequency Oscillator Control"]
pub mod lfoscctl;
#[doc = "RCOSCHFCTL (rw) register accessor: RCOSCHF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcoschfctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcoschfctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcoschfctl`]
module"]
#[doc(alias = "RCOSCHFCTL")]
pub type Rcoschfctl = crate::Reg<rcoschfctl::RcoschfctlSpec>;
#[doc = "RCOSCHF Control"]
pub mod rcoschfctl;
#[doc = "RCOSCMFCTL (rw) register accessor: RCOSC_MF Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcoscmfctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcoscmfctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcoscmfctl`]
module"]
#[doc(alias = "RCOSCMFCTL")]
pub type Rcoscmfctl = crate::Reg<rcoscmfctl::RcoscmfctlSpec>;
#[doc = "RCOSC_MF Control"]
pub mod rcoscmfctl;
#[doc = "STAT0 (rw) register accessor: Status 0 This register contains status signals from OSC_DIG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat0`]
module"]
#[doc(alias = "STAT0")]
pub type Stat0 = crate::Reg<stat0::Stat0Spec>;
#[doc = "Status 0 This register contains status signals from OSC_DIG"]
pub mod stat0;
#[doc = "STAT1 (rw) register accessor: Status 1 This register contains status signals from OSC_DIG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat1`]
module"]
#[doc(alias = "STAT1")]
pub type Stat1 = crate::Reg<stat1::Stat1Spec>;
#[doc = "Status 1 This register contains status signals from OSC_DIG"]
pub mod stat1;
#[doc = "STAT2 (rw) register accessor: Status 2 This register contains status signals from AMPCOMP FSM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat2`]
module"]
#[doc(alias = "STAT2")]
pub type Stat2 = crate::Reg<stat2::Stat2Spec>;
#[doc = "Status 2 This register contains status signals from AMPCOMP FSM"]
pub mod stat2;
