#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    _reserved1: [u8; 0x40],
    imask: Imask,
    ris: Ris,
    mis: Mis,
    iset: Iset,
    iclr: Iclr,
    imset: Imset,
    imclr: Imclr,
    _reserved8: [u8; 0x24],
    hfxtctl: Hfxtctl,
    _reserved9: [u8; 0x04],
    lfoscctl: Lfoscctl,
    lfxtctl: Lfxtctl,
    lfqualctl: Lfqualctl,
    lfincctl: Lfincctl,
    lfincovr: Lfincovr,
    _reserved14: [u8; 0x04],
    hftrackctl: Hftrackctl,
    _reserved15: [u8; 0x04],
    nabiasctl: Nabiasctl,
    lfmonctl: Lfmonctl,
    _reserved17: [u8; 0x0c],
    lfclksel: Lfclksel,
    _reserved18: [u8; 0x04],
    adcclksel: Adcclksel,
    _reserved19: [u8; 0x14],
    lfclkstat: Lfclkstat,
    hfxtstat: Hfxtstat,
    _reserved21: [u8; 0x04],
    trackstat: Trackstat,
    ampstat: Ampstat,
    _reserved23: [u8; 0x14],
    dtbctl: Dtbctl,
    _reserved24: [u8; 0x0c],
    hfxtinit: Hfxtinit,
    hfxttarg: Hfxttarg,
    hfxtdyn: Hfxtdyn,
    ampcfg0: Ampcfg0,
    ampcfg1: Ampcfg1,
    loopcfg: Loopcfg,
    _reserved30: [u8; 0x01d0],
    wdtcnt: Wdtcnt,
    wdttest: Wdttest,
    wdtlock: Wdtlock,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x44 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x48 - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x4c - Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x50 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x54 - Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x58 - Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imset(&self) -> &Imset {
        &self.imset
    }
    #[doc = "0x5c - Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imclr(&self) -> &Imclr {
        &self.imclr
    }
    #[doc = "0x84 - High frequency crystal control"]
    #[inline(always)]
    pub const fn hfxtctl(&self) -> &Hfxtctl {
        &self.hfxtctl
    }
    #[doc = "0x8c - Low frequency oscillator control"]
    #[inline(always)]
    pub const fn lfoscctl(&self) -> &Lfoscctl {
        &self.lfoscctl
    }
    #[doc = "0x90 - Low frequency crystal control"]
    #[inline(always)]
    pub const fn lfxtctl(&self) -> &Lfxtctl {
        &self.lfxtctl
    }
    #[doc = "0x94 - Low frequency clock qualification control"]
    #[inline(always)]
    pub const fn lfqualctl(&self) -> &Lfqualctl {
        &self.lfqualctl
    }
    #[doc = "0x98 - Low frequency time increment control"]
    #[inline(always)]
    pub const fn lfincctl(&self) -> &Lfincctl {
        &self.lfincctl
    }
    #[doc = "0x9c - Low frequency time increment override control"]
    #[inline(always)]
    pub const fn lfincovr(&self) -> &Lfincovr {
        &self.lfincovr
    }
    #[doc = "0xa4 - High frequency tracking loop control"]
    #[inline(always)]
    pub const fn hftrackctl(&self) -> &Hftrackctl {
        &self.hftrackctl
    }
    #[doc = "0xac - Nanoamp-bias control"]
    #[inline(always)]
    pub const fn nabiasctl(&self) -> &Nabiasctl {
        &self.nabiasctl
    }
    #[doc = "0xb0 - Low-frequency clock-monitor control"]
    #[inline(always)]
    pub const fn lfmonctl(&self) -> &Lfmonctl {
        &self.lfmonctl
    }
    #[doc = "0xc0 - Low frequency clock selection"]
    #[inline(always)]
    pub const fn lfclksel(&self) -> &Lfclksel {
        &self.lfclksel
    }
    #[doc = "0xc8 - ADC clock selection"]
    #[inline(always)]
    pub const fn adcclksel(&self) -> &Adcclksel {
        &self.adcclksel
    }
    #[doc = "0xe0 - Low-frequency clock status"]
    #[inline(always)]
    pub const fn lfclkstat(&self) -> &Lfclkstat {
        &self.lfclkstat
    }
    #[doc = "0xe4 - HFXT status information"]
    #[inline(always)]
    pub const fn hfxtstat(&self) -> &Hfxtstat {
        &self.hfxtstat
    }
    #[doc = "0xec - HFOSC tracking loop status information"]
    #[inline(always)]
    pub const fn trackstat(&self) -> &Trackstat {
        &self.trackstat
    }
    #[doc = "0xf0 - HFXT Amplitude Compensation Status"]
    #[inline(always)]
    pub const fn ampstat(&self) -> &Ampstat {
        &self.ampstat
    }
    #[doc = "0x108 - Digital test bus mux control"]
    #[inline(always)]
    pub const fn dtbctl(&self) -> &Dtbctl {
        &self.dtbctl
    }
    #[doc = "0x118 - Initial values for HFXT ramping"]
    #[inline(always)]
    pub const fn hfxtinit(&self) -> &Hfxtinit {
        &self.hfxtinit
    }
    #[doc = "0x11c - Target values for HFXT ramping"]
    #[inline(always)]
    pub const fn hfxttarg(&self) -> &Hfxttarg {
        &self.hfxttarg
    }
    #[doc = "0x120 - Alternative target values for HFXT configuration Software can change these values to dynamically transition the HFXT configuration while HFXT is running. Set SEL to select the alternative set of target values."]
    #[inline(always)]
    pub const fn hfxtdyn(&self) -> &Hfxtdyn {
        &self.hfxtdyn
    }
    #[doc = "0x124 - Amplitude Compensation Configuration 0"]
    #[inline(always)]
    pub const fn ampcfg0(&self) -> &Ampcfg0 {
        &self.ampcfg0
    }
    #[doc = "0x128 - Amplitude Compensation Configuration 1"]
    #[inline(always)]
    pub const fn ampcfg1(&self) -> &Ampcfg1 {
        &self.ampcfg1
    }
    #[doc = "0x12c - Configuration Register for the Tracking Loop"]
    #[inline(always)]
    pub const fn loopcfg(&self) -> &Loopcfg {
        &self.loopcfg
    }
    #[doc = "0x300 - WDT counter value register"]
    #[inline(always)]
    pub const fn wdtcnt(&self) -> &Wdtcnt {
        &self.wdtcnt
    }
    #[doc = "0x304 - WDT test mode register"]
    #[inline(always)]
    pub const fn wdttest(&self) -> &Wdttest {
        &self.wdttest
    }
    #[doc = "0x308 - WDT lock register"]
    #[inline(always)]
    pub const fn wdtlock(&self) -> &Wdtlock {
        &self.wdtlock
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "IMASK (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask;
#[doc = "RIS (rw) register accessor: Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod mis;
#[doc = "ISET (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
pub mod iclr;
#[doc = "IMSET (rw) register accessor: Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imset`]
module"]
#[doc(alias = "IMSET")]
pub type Imset = crate::Reg<imset::ImsetSpec>;
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
pub mod imset;
#[doc = "IMCLR (rw) register accessor: Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imclr`]
module"]
#[doc(alias = "IMCLR")]
pub type Imclr = crate::Reg<imclr::ImclrSpec>;
#[doc = "Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
pub mod imclr;
#[doc = "HFXTCTL (rw) register accessor: High frequency crystal control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxtctl`]
module"]
#[doc(alias = "HFXTCTL")]
pub type Hfxtctl = crate::Reg<hfxtctl::HfxtctlSpec>;
#[doc = "High frequency crystal control"]
pub mod hfxtctl;
#[doc = "LFOSCCTL (rw) register accessor: Low frequency oscillator control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfoscctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfoscctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfoscctl`]
module"]
#[doc(alias = "LFOSCCTL")]
pub type Lfoscctl = crate::Reg<lfoscctl::LfoscctlSpec>;
#[doc = "Low frequency oscillator control"]
pub mod lfoscctl;
#[doc = "LFXTCTL (rw) register accessor: Low frequency crystal control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfxtctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfxtctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfxtctl`]
module"]
#[doc(alias = "LFXTCTL")]
pub type Lfxtctl = crate::Reg<lfxtctl::LfxtctlSpec>;
#[doc = "Low frequency crystal control"]
pub mod lfxtctl;
#[doc = "LFQUALCTL (rw) register accessor: Low frequency clock qualification control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfqualctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfqualctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfqualctl`]
module"]
#[doc(alias = "LFQUALCTL")]
pub type Lfqualctl = crate::Reg<lfqualctl::LfqualctlSpec>;
#[doc = "Low frequency clock qualification control"]
pub mod lfqualctl;
#[doc = "LFINCCTL (rw) register accessor: Low frequency time increment control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfincctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfincctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfincctl`]
module"]
#[doc(alias = "LFINCCTL")]
pub type Lfincctl = crate::Reg<lfincctl::LfincctlSpec>;
#[doc = "Low frequency time increment control"]
pub mod lfincctl;
#[doc = "LFINCOVR (rw) register accessor: Low frequency time increment override control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfincovr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfincovr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfincovr`]
module"]
#[doc(alias = "LFINCOVR")]
pub type Lfincovr = crate::Reg<lfincovr::LfincovrSpec>;
#[doc = "Low frequency time increment override control"]
pub mod lfincovr;
#[doc = "HFTRACKCTL (rw) register accessor: High frequency tracking loop control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hftrackctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hftrackctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hftrackctl`]
module"]
#[doc(alias = "HFTRACKCTL")]
pub type Hftrackctl = crate::Reg<hftrackctl::HftrackctlSpec>;
#[doc = "High frequency tracking loop control"]
pub mod hftrackctl;
#[doc = "NABIASCTL (rw) register accessor: Nanoamp-bias control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nabiasctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nabiasctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nabiasctl`]
module"]
#[doc(alias = "NABIASCTL")]
pub type Nabiasctl = crate::Reg<nabiasctl::NabiasctlSpec>;
#[doc = "Nanoamp-bias control"]
pub mod nabiasctl;
#[doc = "LFMONCTL (rw) register accessor: Low-frequency clock-monitor control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfmonctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfmonctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfmonctl`]
module"]
#[doc(alias = "LFMONCTL")]
pub type Lfmonctl = crate::Reg<lfmonctl::LfmonctlSpec>;
#[doc = "Low-frequency clock-monitor control"]
pub mod lfmonctl;
#[doc = "LFCLKSEL (rw) register accessor: Low frequency clock selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclksel`]
module"]
#[doc(alias = "LFCLKSEL")]
pub type Lfclksel = crate::Reg<lfclksel::LfclkselSpec>;
#[doc = "Low frequency clock selection"]
pub mod lfclksel;
#[doc = "ADCCLKSEL (rw) register accessor: ADC clock selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcclksel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcclksel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcclksel`]
module"]
#[doc(alias = "ADCCLKSEL")]
pub type Adcclksel = crate::Reg<adcclksel::AdcclkselSpec>;
#[doc = "ADC clock selection"]
pub mod adcclksel;
#[doc = "LFCLKSTAT (rw) register accessor: Low-frequency clock status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lfclkstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lfclkstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lfclkstat`]
module"]
#[doc(alias = "LFCLKSTAT")]
pub type Lfclkstat = crate::Reg<lfclkstat::LfclkstatSpec>;
#[doc = "Low-frequency clock status"]
pub mod lfclkstat;
#[doc = "HFXTSTAT (rw) register accessor: HFXT status information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxtstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxtstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxtstat`]
module"]
#[doc(alias = "HFXTSTAT")]
pub type Hfxtstat = crate::Reg<hfxtstat::HfxtstatSpec>;
#[doc = "HFXT status information"]
pub mod hfxtstat;
#[doc = "TRACKSTAT (rw) register accessor: HFOSC tracking loop status information\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trackstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trackstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trackstat`]
module"]
#[doc(alias = "TRACKSTAT")]
pub type Trackstat = crate::Reg<trackstat::TrackstatSpec>;
#[doc = "HFOSC tracking loop status information"]
pub mod trackstat;
#[doc = "AMPSTAT (rw) register accessor: HFXT Amplitude Compensation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampstat`]
module"]
#[doc(alias = "AMPSTAT")]
pub type Ampstat = crate::Reg<ampstat::AmpstatSpec>;
#[doc = "HFXT Amplitude Compensation Status"]
pub mod ampstat;
#[doc = "DTBCTL (rw) register accessor: Digital test bus mux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtbctl`]
module"]
#[doc(alias = "DTBCTL")]
pub type Dtbctl = crate::Reg<dtbctl::DtbctlSpec>;
#[doc = "Digital test bus mux control"]
pub mod dtbctl;
#[doc = "HFXTINIT (rw) register accessor: Initial values for HFXT ramping\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxtinit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxtinit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxtinit`]
module"]
#[doc(alias = "HFXTINIT")]
pub type Hfxtinit = crate::Reg<hfxtinit::HfxtinitSpec>;
#[doc = "Initial values for HFXT ramping"]
pub mod hfxtinit;
#[doc = "HFXTTARG (rw) register accessor: Target values for HFXT ramping\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxttarg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxttarg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxttarg`]
module"]
#[doc(alias = "HFXTTARG")]
pub type Hfxttarg = crate::Reg<hfxttarg::HfxttargSpec>;
#[doc = "Target values for HFXT ramping"]
pub mod hfxttarg;
#[doc = "HFXTDYN (rw) register accessor: Alternative target values for HFXT configuration Software can change these values to dynamically transition the HFXT configuration while HFXT is running. Set SEL to select the alternative set of target values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hfxtdyn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hfxtdyn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxtdyn`]
module"]
#[doc(alias = "HFXTDYN")]
pub type Hfxtdyn = crate::Reg<hfxtdyn::HfxtdynSpec>;
#[doc = "Alternative target values for HFXT configuration Software can change these values to dynamically transition the HFXT configuration while HFXT is running. Set SEL to select the alternative set of target values."]
pub mod hfxtdyn;
#[doc = "AMPCFG0 (rw) register accessor: Amplitude Compensation Configuration 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcfg0`]
module"]
#[doc(alias = "AMPCFG0")]
pub type Ampcfg0 = crate::Reg<ampcfg0::Ampcfg0Spec>;
#[doc = "Amplitude Compensation Configuration 0"]
pub mod ampcfg0;
#[doc = "AMPCFG1 (rw) register accessor: Amplitude Compensation Configuration 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ampcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ampcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ampcfg1`]
module"]
#[doc(alias = "AMPCFG1")]
pub type Ampcfg1 = crate::Reg<ampcfg1::Ampcfg1Spec>;
#[doc = "Amplitude Compensation Configuration 1"]
pub mod ampcfg1;
#[doc = "LOOPCFG (rw) register accessor: Configuration Register for the Tracking Loop\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`loopcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`loopcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@loopcfg`]
module"]
#[doc(alias = "LOOPCFG")]
pub type Loopcfg = crate::Reg<loopcfg::LoopcfgSpec>;
#[doc = "Configuration Register for the Tracking Loop"]
pub mod loopcfg;
#[doc = "WDTCNT (rw) register accessor: WDT counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtcnt`]
module"]
#[doc(alias = "WDTCNT")]
pub type Wdtcnt = crate::Reg<wdtcnt::WdtcntSpec>;
#[doc = "WDT counter value register"]
pub mod wdtcnt;
#[doc = "WDTTEST (rw) register accessor: WDT test mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdttest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdttest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdttest`]
module"]
#[doc(alias = "WDTTEST")]
pub type Wdttest = crate::Reg<wdttest::WdttestSpec>;
#[doc = "WDT test mode register"]
pub mod wdttest;
#[doc = "WDTLOCK (rw) register accessor: WDT lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtlock`]
module"]
#[doc(alias = "WDTLOCK")]
pub type Wdtlock = crate::Reg<wdtlock::WdtlockSpec>;
#[doc = "WDT lock register"]
pub mod wdtlock;
