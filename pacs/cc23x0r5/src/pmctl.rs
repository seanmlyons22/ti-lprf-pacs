#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    shtdwn: Shtdwn,
    slpctl: Slpctl,
    wusta: Wusta,
    vddrctl: Vddrctl,
    iosegset: Iosegset,
    iosegclr: Iosegclr,
    sysfset: Sysfset,
    sysfclr: Sysfclr,
    sysfsta: Sysfsta,
    rstctl: Rstctl,
    rststa: Rststa,
    bootsta: Bootsta,
    _reserved14: [u8; 0x04],
    aonrsta1: Aonrsta1,
    aonrset1: Aonrset1,
    aonrclr1: Aonrclr1,
    _reserved17: [u8; 0x04],
    delta: Delta,
    wutime: Wutime,
    prepuctl: Prepuctl,
    swstmp: Swstmp,
    _reserved21: [u8; 0x08],
    etpp: Etpp,
    _reserved22: [u8; 0x14],
    retcfg0: Retcfg0,
    retcfg1: Retcfg1,
    retcfg2: Retcfg2,
    retcfg3: Retcfg3,
    retcfg4: Retcfg4,
    retcfg5: Retcfg5,
    retcfg6: Retcfg6,
    retcfg7: Retcfg7,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Extended Description Register. This register shows ULL IP availability and memory size configuration."]
    #[inline(always)]
    pub const fn descex(&self) -> &Descex {
        &self.descex
    }
    #[doc = "0x08 - Shutdown Register. This register controls SHUTDOWN mode entry."]
    #[inline(always)]
    pub const fn shtdwn(&self) -> &Shtdwn {
        &self.shtdwn
    }
    #[doc = "0x0c - Sleep Control Register. This register controls I/O pad sleep mode. When I/O pad sleep mode is enabled all I/O pad outputs and I/O pad configurations are latched. Inputs are transparent if I/O pad is configured as input."]
    #[inline(always)]
    pub const fn slpctl(&self) -> &Slpctl {
        &self.slpctl
    }
    #[doc = "0x10 - Wakeup Status Register This register shows the device wakeup source. Used to distinguish between wakeup from STANDBY, SHUTDOWN and reset."]
    #[inline(always)]
    pub const fn wusta(&self) -> &Wusta {
        &self.wusta
    }
    #[doc = "0x14 - VDDR Control Register. This register contains VDDR regulator settings for the device."]
    #[inline(always)]
    pub const fn vddrctl(&self) -> &Vddrctl {
        &self.vddrctl
    }
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn iosegset(&self) -> &Iosegset {
        &self.iosegset
    }
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn iosegclr(&self) -> &Iosegclr {
        &self.iosegclr
    }
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn sysfset(&self) -> &Sysfset {
        &self.sysfset
    }
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn sysfclr(&self) -> &Sysfclr {
        &self.sysfclr
    }
    #[doc = "0x28 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn sysfsta(&self) -> &Sysfsta {
        &self.sysfsta
    }
    #[doc = "0x2c - Reset Control Register. This register configures and controls system reset."]
    #[inline(always)]
    pub const fn rstctl(&self) -> &Rstctl {
        &self.rstctl
    }
    #[doc = "0x30 - Reset Status. This register contains the reset source and SHUTDOWN wakeup source for the system. Check WUSTA.SRC first to ensure that wakeup from STANDBY is not set. The capture feature is not rearmed until all of the possible reset sources have been released and the result has been copied to this register. During the copy and rearm process it is one 24MHz period in which an eventual new system reset will be reported as Power on reset regardless of the root cause."]
    #[inline(always)]
    pub const fn rststa(&self) -> &Rststa {
        &self.rststa
    }
    #[doc = "0x34 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn bootsta(&self) -> &Bootsta {
        &self.bootsta
    }
    #[doc = "0x3c - AON Register Status 1. This register contains the general purpose AON flags for SW, and is updated through AONRSET1.FLAG and AONRCLR1.FLAG. The register is only reset on a POR event."]
    #[inline(always)]
    pub const fn aonrsta1(&self) -> &Aonrsta1 {
        &self.aonrsta1
    }
    #[doc = "0x40 - AON Register Set 1. This register sets the AON flags that can be read through AONRSTA1.FLAG."]
    #[inline(always)]
    pub const fn aonrset1(&self) -> &Aonrset1 {
        &self.aonrset1
    }
    #[doc = "0x44 - AON Register Clear 1. This register clears the AON flags that can be read through AONRSTA1.FLAG."]
    #[inline(always)]
    pub const fn aonrclr1(&self) -> &Aonrclr1 {
        &self.aonrclr1
    }
    #[doc = "0x4c - Delta Time Register. This register contains the measured delta time during wakeup from STANDBY mode."]
    #[inline(always)]
    pub const fn delta(&self) -> &Delta {
        &self.delta
    }
    #[doc = "0x50 - WakeUp Time Register This register contains the measured wakeup times from STANDBY mode."]
    #[inline(always)]
    pub const fn wutime(&self) -> &Wutime {
        &self.wutime
    }
    #[doc = "0x54 - Pre Power-Up Control Register. This register contains settings and control for pre-powerup, STANDBY and wakeup measurements."]
    #[inline(always)]
    pub const fn prepuctl(&self) -> &Prepuctl {
        &self.prepuctl
    }
    #[doc = "0x58 - SW Time Stamp Register. This register is used to set the SW time stamp for the delta time measurement."]
    #[inline(always)]
    pub const fn swstmp(&self) -> &Swstmp {
        &self.swstmp
    }
    #[doc = "0x64 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn etpp(&self) -> &Etpp {
        &self.etpp
    }
    #[doc = "0x7c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg0(&self) -> &Retcfg0 {
        &self.retcfg0
    }
    #[doc = "0x80 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg1(&self) -> &Retcfg1 {
        &self.retcfg1
    }
    #[doc = "0x84 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg2(&self) -> &Retcfg2 {
        &self.retcfg2
    }
    #[doc = "0x88 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg3(&self) -> &Retcfg3 {
        &self.retcfg3
    }
    #[doc = "0x8c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg4(&self) -> &Retcfg4 {
        &self.retcfg4
    }
    #[doc = "0x90 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg5(&self) -> &Retcfg5 {
        &self.retcfg5
    }
    #[doc = "0x94 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg6(&self) -> &Retcfg6 {
        &self.retcfg6
    }
    #[doc = "0x98 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn retcfg7(&self) -> &Retcfg7 {
        &self.retcfg7
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "DESCEX (rw) register accessor: Extended Description Register. This register shows ULL IP availability and memory size configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex`]
module"]
#[doc(alias = "DESCEX")]
pub type Descex = crate::Reg<descex::DescexSpec>;
#[doc = "Extended Description Register. This register shows ULL IP availability and memory size configuration."]
pub mod descex;
#[doc = "SHTDWN (rw) register accessor: Shutdown Register. This register controls SHUTDOWN mode entry.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shtdwn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shtdwn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shtdwn`]
module"]
#[doc(alias = "SHTDWN")]
pub type Shtdwn = crate::Reg<shtdwn::ShtdwnSpec>;
#[doc = "Shutdown Register. This register controls SHUTDOWN mode entry."]
pub mod shtdwn;
#[doc = "SLPCTL (rw) register accessor: Sleep Control Register. This register controls I/O pad sleep mode. When I/O pad sleep mode is enabled all I/O pad outputs and I/O pad configurations are latched. Inputs are transparent if I/O pad is configured as input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@slpctl`]
module"]
#[doc(alias = "SLPCTL")]
pub type Slpctl = crate::Reg<slpctl::SlpctlSpec>;
#[doc = "Sleep Control Register. This register controls I/O pad sleep mode. When I/O pad sleep mode is enabled all I/O pad outputs and I/O pad configurations are latched. Inputs are transparent if I/O pad is configured as input."]
pub mod slpctl;
#[doc = "WUSTA (rw) register accessor: Wakeup Status Register This register shows the device wakeup source. Used to distinguish between wakeup from STANDBY, SHUTDOWN and reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wusta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wusta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wusta`]
module"]
#[doc(alias = "WUSTA")]
pub type Wusta = crate::Reg<wusta::WustaSpec>;
#[doc = "Wakeup Status Register This register shows the device wakeup source. Used to distinguish between wakeup from STANDBY, SHUTDOWN and reset."]
pub mod wusta;
#[doc = "VDDRCTL (rw) register accessor: VDDR Control Register. This register contains VDDR regulator settings for the device.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vddrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vddrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vddrctl`]
module"]
#[doc(alias = "VDDRCTL")]
pub type Vddrctl = crate::Reg<vddrctl::VddrctlSpec>;
#[doc = "VDDR Control Register. This register contains VDDR regulator settings for the device."]
pub mod vddrctl;
#[doc = "IOSEGSET (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iosegset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iosegset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iosegset`]
module"]
#[doc(alias = "IOSEGSET")]
pub type Iosegset = crate::Reg<iosegset::IosegsetSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iosegset;
#[doc = "IOSEGCLR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iosegclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iosegclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iosegclr`]
module"]
#[doc(alias = "IOSEGCLR")]
pub type Iosegclr = crate::Reg<iosegclr::IosegclrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iosegclr;
#[doc = "SYSFSET (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysfset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysfset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysfset`]
module"]
#[doc(alias = "SYSFSET")]
pub type Sysfset = crate::Reg<sysfset::SysfsetSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sysfset;
#[doc = "SYSFCLR (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysfclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysfclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysfclr`]
module"]
#[doc(alias = "SYSFCLR")]
pub type Sysfclr = crate::Reg<sysfclr::SysfclrSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sysfclr;
#[doc = "SYSFSTA (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysfsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysfsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysfsta`]
module"]
#[doc(alias = "SYSFSTA")]
pub type Sysfsta = crate::Reg<sysfsta::SysfstaSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod sysfsta;
#[doc = "RSTCTL (rw) register accessor: Reset Control Register. This register configures and controls system reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctl`]
module"]
#[doc(alias = "RSTCTL")]
pub type Rstctl = crate::Reg<rstctl::RstctlSpec>;
#[doc = "Reset Control Register. This register configures and controls system reset."]
pub mod rstctl;
#[doc = "RSTSTA (rw) register accessor: Reset Status. This register contains the reset source and SHUTDOWN wakeup source for the system. Check WUSTA.SRC first to ensure that wakeup from STANDBY is not set. The capture feature is not rearmed until all of the possible reset sources have been released and the result has been copied to this register. During the copy and rearm process it is one 24MHz period in which an eventual new system reset will be reported as Power on reset regardless of the root cause.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rststa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rststa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rststa`]
module"]
#[doc(alias = "RSTSTA")]
pub type Rststa = crate::Reg<rststa::RststaSpec>;
#[doc = "Reset Status. This register contains the reset source and SHUTDOWN wakeup source for the system. Check WUSTA.SRC first to ensure that wakeup from STANDBY is not set. The capture feature is not rearmed until all of the possible reset sources have been released and the result has been copied to this register. During the copy and rearm process it is one 24MHz period in which an eventual new system reset will be reported as Power on reset regardless of the root cause."]
pub mod rststa;
#[doc = "BOOTSTA (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bootsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bootsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bootsta`]
module"]
#[doc(alias = "BOOTSTA")]
pub type Bootsta = crate::Reg<bootsta::BootstaSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod bootsta;
#[doc = "AONRSTA1 (rw) register accessor: AON Register Status 1. This register contains the general purpose AON flags for SW, and is updated through AONRSET1.FLAG and AONRCLR1.FLAG. The register is only reset on a POR event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonrsta1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonrsta1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aonrsta1`]
module"]
#[doc(alias = "AONRSTA1")]
pub type Aonrsta1 = crate::Reg<aonrsta1::Aonrsta1Spec>;
#[doc = "AON Register Status 1. This register contains the general purpose AON flags for SW, and is updated through AONRSET1.FLAG and AONRCLR1.FLAG. The register is only reset on a POR event."]
pub mod aonrsta1;
#[doc = "AONRSET1 (rw) register accessor: AON Register Set 1. This register sets the AON flags that can be read through AONRSTA1.FLAG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonrset1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonrset1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aonrset1`]
module"]
#[doc(alias = "AONRSET1")]
pub type Aonrset1 = crate::Reg<aonrset1::Aonrset1Spec>;
#[doc = "AON Register Set 1. This register sets the AON flags that can be read through AONRSTA1.FLAG."]
pub mod aonrset1;
#[doc = "AONRCLR1 (rw) register accessor: AON Register Clear 1. This register clears the AON flags that can be read through AONRSTA1.FLAG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aonrclr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aonrclr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aonrclr1`]
module"]
#[doc(alias = "AONRCLR1")]
pub type Aonrclr1 = crate::Reg<aonrclr1::Aonrclr1Spec>;
#[doc = "AON Register Clear 1. This register clears the AON flags that can be read through AONRSTA1.FLAG."]
pub mod aonrclr1;
#[doc = "DELTA (rw) register accessor: Delta Time Register. This register contains the measured delta time during wakeup from STANDBY mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delta`]
module"]
#[doc(alias = "DELTA")]
pub type Delta = crate::Reg<delta::DeltaSpec>;
#[doc = "Delta Time Register. This register contains the measured delta time during wakeup from STANDBY mode."]
pub mod delta;
#[doc = "WUTIME (rw) register accessor: WakeUp Time Register This register contains the measured wakeup times from STANDBY mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutime`]
module"]
#[doc(alias = "WUTIME")]
pub type Wutime = crate::Reg<wutime::WutimeSpec>;
#[doc = "WakeUp Time Register This register contains the measured wakeup times from STANDBY mode."]
pub mod wutime;
#[doc = "PREPUCTL (rw) register accessor: Pre Power-Up Control Register. This register contains settings and control for pre-powerup, STANDBY and wakeup measurements.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prepuctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prepuctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prepuctl`]
module"]
#[doc(alias = "PREPUCTL")]
pub type Prepuctl = crate::Reg<prepuctl::PrepuctlSpec>;
#[doc = "Pre Power-Up Control Register. This register contains settings and control for pre-powerup, STANDBY and wakeup measurements."]
pub mod prepuctl;
#[doc = "SWSTMP (rw) register accessor: SW Time Stamp Register. This register is used to set the SW time stamp for the delta time measurement.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swstmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swstmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swstmp`]
module"]
#[doc(alias = "SWSTMP")]
pub type Swstmp = crate::Reg<swstmp::SwstmpSpec>;
#[doc = "SW Time Stamp Register. This register is used to set the SW time stamp for the delta time measurement."]
pub mod swstmp;
#[doc = "ETPP (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etpp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etpp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@etpp`]
module"]
#[doc(alias = "ETPP")]
pub type Etpp = crate::Reg<etpp::EtppSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod etpp;
#[doc = "RETCFG0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg0`]
module"]
#[doc(alias = "RETCFG0")]
pub type Retcfg0 = crate::Reg<retcfg0::Retcfg0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg0;
#[doc = "RETCFG1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg1`]
module"]
#[doc(alias = "RETCFG1")]
pub type Retcfg1 = crate::Reg<retcfg1::Retcfg1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg1;
#[doc = "RETCFG2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg2`]
module"]
#[doc(alias = "RETCFG2")]
pub type Retcfg2 = crate::Reg<retcfg2::Retcfg2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg2;
#[doc = "RETCFG3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg3`]
module"]
#[doc(alias = "RETCFG3")]
pub type Retcfg3 = crate::Reg<retcfg3::Retcfg3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg3;
#[doc = "RETCFG4 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg4`]
module"]
#[doc(alias = "RETCFG4")]
pub type Retcfg4 = crate::Reg<retcfg4::Retcfg4Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg4;
#[doc = "RETCFG5 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg5`]
module"]
#[doc(alias = "RETCFG5")]
pub type Retcfg5 = crate::Reg<retcfg5::Retcfg5Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg5;
#[doc = "RETCFG6 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg6`]
module"]
#[doc(alias = "RETCFG6")]
pub type Retcfg6 = crate::Reg<retcfg6::Retcfg6Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg6;
#[doc = "RETCFG7 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retcfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retcfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@retcfg7`]
module"]
#[doc(alias = "RETCFG7")]
pub type Retcfg7 = crate::Reg<retcfg7::Retcfg7Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod retcfg7;
