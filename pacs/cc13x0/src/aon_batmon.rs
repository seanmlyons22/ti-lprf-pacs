#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    meascfg: Meascfg,
    _reserved2: [u8; 0x04],
    tempp0: Tempp0,
    tempp1: Tempp1,
    tempp2: Tempp2,
    batmonp0: Batmonp0,
    batmonp1: Batmonp1,
    iostrp0: Iostrp0,
    flashpumpp0: Flashpumpp0,
    bat: Bat,
    batupd: Batupd,
    temp: Temp,
    tempupd: Tempupd,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn meascfg(&self) -> &Meascfg {
        &self.meascfg
    }
    #[doc = "0x0c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn tempp0(&self) -> &Tempp0 {
        &self.tempp0
    }
    #[doc = "0x10 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn tempp1(&self) -> &Tempp1 {
        &self.tempp1
    }
    #[doc = "0x14 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn tempp2(&self) -> &Tempp2 {
        &self.tempp2
    }
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn batmonp0(&self) -> &Batmonp0 {
        &self.batmonp0
    }
    #[doc = "0x1c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn batmonp1(&self) -> &Batmonp1 {
        &self.batmonp1
    }
    #[doc = "0x20 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn iostrp0(&self) -> &Iostrp0 {
        &self.iostrp0
    }
    #[doc = "0x24 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn flashpumpp0(&self) -> &Flashpumpp0 {
        &self.flashpumpp0
    }
    #[doc = "0x28 - Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
    #[inline(always)]
    pub const fn bat(&self) -> &Bat {
        &self.bat
    }
    #[doc = "0x2c - Battery Update Indicates BAT Updates"]
    #[inline(always)]
    pub const fn batupd(&self) -> &Batupd {
        &self.batupd
    }
    #[doc = "0x30 - Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    #[doc = "0x34 - Temperature Update Indicates TEMP Updates"]
    #[inline(always)]
    pub const fn tempupd(&self) -> &Tempupd {
        &self.tempupd
    }
}
#[doc = "CTL (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod ctl;
#[doc = "MEASCFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`meascfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`meascfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meascfg`]
module"]
#[doc(alias = "MEASCFG")]
pub type Meascfg = crate::Reg<meascfg::MeascfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod meascfg;
#[doc = "TEMPP0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempp0`]
module"]
#[doc(alias = "TEMPP0")]
pub type Tempp0 = crate::Reg<tempp0::Tempp0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp0;
#[doc = "TEMPP1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempp1`]
module"]
#[doc(alias = "TEMPP1")]
pub type Tempp1 = crate::Reg<tempp1::Tempp1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp1;
#[doc = "TEMPP2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempp2`]
module"]
#[doc(alias = "TEMPP2")]
pub type Tempp2 = crate::Reg<tempp2::Tempp2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tempp2;
#[doc = "BATMONP0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmonp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmonp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batmonp0`]
module"]
#[doc(alias = "BATMONP0")]
pub type Batmonp0 = crate::Reg<batmonp0::Batmonp0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp0;
#[doc = "BATMONP1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batmonp1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batmonp1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batmonp1`]
module"]
#[doc(alias = "BATMONP1")]
pub type Batmonp1 = crate::Reg<batmonp1::Batmonp1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod batmonp1;
#[doc = "IOSTRP0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iostrp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iostrp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iostrp0`]
module"]
#[doc(alias = "IOSTRP0")]
pub type Iostrp0 = crate::Reg<iostrp0::Iostrp0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrp0;
#[doc = "FLASHPUMPP0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flashpumpp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flashpumpp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashpumpp0`]
module"]
#[doc(alias = "FLASHPUMPP0")]
pub type Flashpumpp0 = crate::Reg<flashpumpp0::Flashpumpp0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod flashpumpp0;
#[doc = "BAT (rw) register accessor: Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bat`]
module"]
#[doc(alias = "BAT")]
pub type Bat = crate::Reg<bat::BatSpec>;
#[doc = "Last Measured Battery Voltage This register may be read while BATUPD.STAT = 1"]
pub mod bat;
#[doc = "BATUPD (rw) register accessor: Battery Update Indicates BAT Updates\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`batupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`batupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@batupd`]
module"]
#[doc(alias = "BATUPD")]
pub type Batupd = crate::Reg<batupd::BatupdSpec>;
#[doc = "Battery Update Indicates BAT Updates"]
pub mod batupd;
#[doc = "TEMP (rw) register accessor: Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`temp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`temp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`]
module"]
#[doc(alias = "TEMP")]
pub type Temp = crate::Reg<temp::TempSpec>;
#[doc = "Temperature Last Measured Temperature in Degrees Celsius This register may be read while TEMPUPD.STAT = 1."]
pub mod temp;
#[doc = "TEMPUPD (rw) register accessor: Temperature Update Indicates TEMP Updates\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tempupd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tempupd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tempupd`]
module"]
#[doc(alias = "TEMPUPD")]
pub type Tempupd = crate::Reg<tempupd::TempupdSpec>;
#[doc = "Temperature Update Indicates TEMP Updates"]
pub mod tempupd;
