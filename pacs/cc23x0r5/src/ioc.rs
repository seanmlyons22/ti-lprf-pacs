#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    _reserved2: [u8; 0x5c],
    dtb: Dtb,
    _reserved3: [u8; 0x98],
    ioc0: Ioc0,
    ioc1: Ioc1,
    ioc2: Ioc2,
    ioc3: Ioc3,
    ioc4: Ioc4,
    ioc5: Ioc5,
    ioc6: Ioc6,
    ioc7: Ioc7,
    ioc8: Ioc8,
    ioc9: Ioc9,
    ioc10: Ioc10,
    ioc11: Ioc11,
    ioc12: Ioc12,
    ioc13: Ioc13,
    ioc14: Ioc14,
    ioc15: Ioc15,
    ioc16: Ioc16,
    ioc17: Ioc17,
    ioc18: Ioc18,
    ioc19: Ioc19,
    ioc20: Ioc20,
    ioc21: Ioc21,
    ioc22: Ioc22,
    ioc23: Ioc23,
    ioc24: Ioc24,
    ioc25: Ioc25,
    _reserved29: [u8; 0x0a98],
    dtbcfg: Dtbcfg,
    dtboe: Dtboe,
    evtcfg: Evtcfg,
    test: Test,
    dtbstat: Dtbstat,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Extended Description Register. This register provides configuration details of the IP to software drivers and end users."]
    #[inline(always)]
    pub const fn descex(&self) -> &Descex {
        &self.descex
    }
    #[doc = "0x64 - Digital Test Bus. This register is used to bring out some internal signals of the peripheral on digital test bus (DTB)."]
    #[inline(always)]
    pub const fn dtb(&self) -> &Dtb {
        &self.dtb
    }
    #[doc = "0x100 - Configuration of DIO0"]
    #[inline(always)]
    pub const fn ioc0(&self) -> &Ioc0 {
        &self.ioc0
    }
    #[doc = "0x104 - Configuration of DIO1"]
    #[inline(always)]
    pub const fn ioc1(&self) -> &Ioc1 {
        &self.ioc1
    }
    #[doc = "0x108 - Configuration of DIO2"]
    #[inline(always)]
    pub const fn ioc2(&self) -> &Ioc2 {
        &self.ioc2
    }
    #[doc = "0x10c - Configuration of DIO3"]
    #[inline(always)]
    pub const fn ioc3(&self) -> &Ioc3 {
        &self.ioc3
    }
    #[doc = "0x110 - Configuration of DIO4"]
    #[inline(always)]
    pub const fn ioc4(&self) -> &Ioc4 {
        &self.ioc4
    }
    #[doc = "0x114 - Configuration of DIO5"]
    #[inline(always)]
    pub const fn ioc5(&self) -> &Ioc5 {
        &self.ioc5
    }
    #[doc = "0x118 - Configuration of DIO6"]
    #[inline(always)]
    pub const fn ioc6(&self) -> &Ioc6 {
        &self.ioc6
    }
    #[doc = "0x11c - Configuration of DIO7"]
    #[inline(always)]
    pub const fn ioc7(&self) -> &Ioc7 {
        &self.ioc7
    }
    #[doc = "0x120 - Configuration of DIO8"]
    #[inline(always)]
    pub const fn ioc8(&self) -> &Ioc8 {
        &self.ioc8
    }
    #[doc = "0x124 - Configuration of DIO9"]
    #[inline(always)]
    pub const fn ioc9(&self) -> &Ioc9 {
        &self.ioc9
    }
    #[doc = "0x128 - Configuration of DIO10"]
    #[inline(always)]
    pub const fn ioc10(&self) -> &Ioc10 {
        &self.ioc10
    }
    #[doc = "0x12c - Configuration of DIO11"]
    #[inline(always)]
    pub const fn ioc11(&self) -> &Ioc11 {
        &self.ioc11
    }
    #[doc = "0x130 - Configuration of DIO12"]
    #[inline(always)]
    pub const fn ioc12(&self) -> &Ioc12 {
        &self.ioc12
    }
    #[doc = "0x134 - Configuration of DIO13"]
    #[inline(always)]
    pub const fn ioc13(&self) -> &Ioc13 {
        &self.ioc13
    }
    #[doc = "0x138 - Configuration of DIO14"]
    #[inline(always)]
    pub const fn ioc14(&self) -> &Ioc14 {
        &self.ioc14
    }
    #[doc = "0x13c - Configuration of DIO15"]
    #[inline(always)]
    pub const fn ioc15(&self) -> &Ioc15 {
        &self.ioc15
    }
    #[doc = "0x140 - Configuration of DIO16"]
    #[inline(always)]
    pub const fn ioc16(&self) -> &Ioc16 {
        &self.ioc16
    }
    #[doc = "0x144 - Configuration of DIO17"]
    #[inline(always)]
    pub const fn ioc17(&self) -> &Ioc17 {
        &self.ioc17
    }
    #[doc = "0x148 - Configuration of DIO18"]
    #[inline(always)]
    pub const fn ioc18(&self) -> &Ioc18 {
        &self.ioc18
    }
    #[doc = "0x14c - Configuration of DIO19"]
    #[inline(always)]
    pub const fn ioc19(&self) -> &Ioc19 {
        &self.ioc19
    }
    #[doc = "0x150 - Configuration of DIO20"]
    #[inline(always)]
    pub const fn ioc20(&self) -> &Ioc20 {
        &self.ioc20
    }
    #[doc = "0x154 - Configuration of DIO21"]
    #[inline(always)]
    pub const fn ioc21(&self) -> &Ioc21 {
        &self.ioc21
    }
    #[doc = "0x158 - Configuration of DIO22"]
    #[inline(always)]
    pub const fn ioc22(&self) -> &Ioc22 {
        &self.ioc22
    }
    #[doc = "0x15c - Configuration of DIO23"]
    #[inline(always)]
    pub const fn ioc23(&self) -> &Ioc23 {
        &self.ioc23
    }
    #[doc = "0x160 - This field controls input hysteresis"]
    #[inline(always)]
    pub const fn ioc24(&self) -> &Ioc24 {
        &self.ioc24
    }
    #[doc = "0x164 - Configuration of DIO25"]
    #[inline(always)]
    pub const fn ioc25(&self) -> &Ioc25 {
        &self.ioc25
    }
    #[doc = "0xc00 - DTB configuration"]
    #[inline(always)]
    pub const fn dtbcfg(&self) -> &Dtbcfg {
        &self.dtbcfg
    }
    #[doc = "0xc04 - DTB output enable"]
    #[inline(always)]
    pub const fn dtboe(&self) -> &Dtboe {
        &self.dtboe
    }
    #[doc = "0xc08 - Event configuration. This register is used to select DIO for IOC to publish event on AON event fabric. It also contains enable bit that is used to mask the event and event flag bit."]
    #[inline(always)]
    pub const fn evtcfg(&self) -> &Evtcfg {
        &self.evtcfg
    }
    #[doc = "0xc0c - Test register."]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0xc10 - DTB status register."]
    #[inline(always)]
    pub const fn dtbstat(&self) -> &Dtbstat {
        &self.dtbstat
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "DESCEX (rw) register accessor: Extended Description Register. This register provides configuration details of the IP to software drivers and end users.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`descex::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`descex::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@descex`]
module"]
#[doc(alias = "DESCEX")]
pub type Descex = crate::Reg<descex::DescexSpec>;
#[doc = "Extended Description Register. This register provides configuration details of the IP to software drivers and end users."]
pub mod descex;
#[doc = "DTB (rw) register accessor: Digital Test Bus. This register is used to bring out some internal signals of the peripheral on digital test bus (DTB).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb`]
module"]
#[doc(alias = "DTB")]
pub type Dtb = crate::Reg<dtb::DtbSpec>;
#[doc = "Digital Test Bus. This register is used to bring out some internal signals of the peripheral on digital test bus (DTB)."]
pub mod dtb;
#[doc = "IOC0 (rw) register accessor: Configuration of DIO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc0`]
module"]
#[doc(alias = "IOC0")]
pub type Ioc0 = crate::Reg<ioc0::Ioc0Spec>;
#[doc = "Configuration of DIO0"]
pub mod ioc0;
#[doc = "IOC1 (rw) register accessor: Configuration of DIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc1`]
module"]
#[doc(alias = "IOC1")]
pub type Ioc1 = crate::Reg<ioc1::Ioc1Spec>;
#[doc = "Configuration of DIO1"]
pub mod ioc1;
#[doc = "IOC2 (rw) register accessor: Configuration of DIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc2`]
module"]
#[doc(alias = "IOC2")]
pub type Ioc2 = crate::Reg<ioc2::Ioc2Spec>;
#[doc = "Configuration of DIO2"]
pub mod ioc2;
#[doc = "IOC3 (rw) register accessor: Configuration of DIO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc3`]
module"]
#[doc(alias = "IOC3")]
pub type Ioc3 = crate::Reg<ioc3::Ioc3Spec>;
#[doc = "Configuration of DIO3"]
pub mod ioc3;
#[doc = "IOC4 (rw) register accessor: Configuration of DIO4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc4`]
module"]
#[doc(alias = "IOC4")]
pub type Ioc4 = crate::Reg<ioc4::Ioc4Spec>;
#[doc = "Configuration of DIO4"]
pub mod ioc4;
#[doc = "IOC5 (rw) register accessor: Configuration of DIO5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc5`]
module"]
#[doc(alias = "IOC5")]
pub type Ioc5 = crate::Reg<ioc5::Ioc5Spec>;
#[doc = "Configuration of DIO5"]
pub mod ioc5;
#[doc = "IOC6 (rw) register accessor: Configuration of DIO6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc6`]
module"]
#[doc(alias = "IOC6")]
pub type Ioc6 = crate::Reg<ioc6::Ioc6Spec>;
#[doc = "Configuration of DIO6"]
pub mod ioc6;
#[doc = "IOC7 (rw) register accessor: Configuration of DIO7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc7`]
module"]
#[doc(alias = "IOC7")]
pub type Ioc7 = crate::Reg<ioc7::Ioc7Spec>;
#[doc = "Configuration of DIO7"]
pub mod ioc7;
#[doc = "IOC8 (rw) register accessor: Configuration of DIO8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc8`]
module"]
#[doc(alias = "IOC8")]
pub type Ioc8 = crate::Reg<ioc8::Ioc8Spec>;
#[doc = "Configuration of DIO8"]
pub mod ioc8;
#[doc = "IOC9 (rw) register accessor: Configuration of DIO9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc9`]
module"]
#[doc(alias = "IOC9")]
pub type Ioc9 = crate::Reg<ioc9::Ioc9Spec>;
#[doc = "Configuration of DIO9"]
pub mod ioc9;
#[doc = "IOC10 (rw) register accessor: Configuration of DIO10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc10`]
module"]
#[doc(alias = "IOC10")]
pub type Ioc10 = crate::Reg<ioc10::Ioc10Spec>;
#[doc = "Configuration of DIO10"]
pub mod ioc10;
#[doc = "IOC11 (rw) register accessor: Configuration of DIO11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc11`]
module"]
#[doc(alias = "IOC11")]
pub type Ioc11 = crate::Reg<ioc11::Ioc11Spec>;
#[doc = "Configuration of DIO11"]
pub mod ioc11;
#[doc = "IOC12 (rw) register accessor: Configuration of DIO12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc12`]
module"]
#[doc(alias = "IOC12")]
pub type Ioc12 = crate::Reg<ioc12::Ioc12Spec>;
#[doc = "Configuration of DIO12"]
pub mod ioc12;
#[doc = "IOC13 (rw) register accessor: Configuration of DIO13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc13`]
module"]
#[doc(alias = "IOC13")]
pub type Ioc13 = crate::Reg<ioc13::Ioc13Spec>;
#[doc = "Configuration of DIO13"]
pub mod ioc13;
#[doc = "IOC14 (rw) register accessor: Configuration of DIO14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc14`]
module"]
#[doc(alias = "IOC14")]
pub type Ioc14 = crate::Reg<ioc14::Ioc14Spec>;
#[doc = "Configuration of DIO14"]
pub mod ioc14;
#[doc = "IOC15 (rw) register accessor: Configuration of DIO15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc15`]
module"]
#[doc(alias = "IOC15")]
pub type Ioc15 = crate::Reg<ioc15::Ioc15Spec>;
#[doc = "Configuration of DIO15"]
pub mod ioc15;
#[doc = "IOC16 (rw) register accessor: Configuration of DIO16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc16`]
module"]
#[doc(alias = "IOC16")]
pub type Ioc16 = crate::Reg<ioc16::Ioc16Spec>;
#[doc = "Configuration of DIO16"]
pub mod ioc16;
#[doc = "IOC17 (rw) register accessor: Configuration of DIO17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc17`]
module"]
#[doc(alias = "IOC17")]
pub type Ioc17 = crate::Reg<ioc17::Ioc17Spec>;
#[doc = "Configuration of DIO17"]
pub mod ioc17;
#[doc = "IOC18 (rw) register accessor: Configuration of DIO18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc18`]
module"]
#[doc(alias = "IOC18")]
pub type Ioc18 = crate::Reg<ioc18::Ioc18Spec>;
#[doc = "Configuration of DIO18"]
pub mod ioc18;
#[doc = "IOC19 (rw) register accessor: Configuration of DIO19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc19`]
module"]
#[doc(alias = "IOC19")]
pub type Ioc19 = crate::Reg<ioc19::Ioc19Spec>;
#[doc = "Configuration of DIO19"]
pub mod ioc19;
#[doc = "IOC20 (rw) register accessor: Configuration of DIO20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc20`]
module"]
#[doc(alias = "IOC20")]
pub type Ioc20 = crate::Reg<ioc20::Ioc20Spec>;
#[doc = "Configuration of DIO20"]
pub mod ioc20;
#[doc = "IOC21 (rw) register accessor: Configuration of DIO21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc21`]
module"]
#[doc(alias = "IOC21")]
pub type Ioc21 = crate::Reg<ioc21::Ioc21Spec>;
#[doc = "Configuration of DIO21"]
pub mod ioc21;
#[doc = "IOC22 (rw) register accessor: Configuration of DIO22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc22`]
module"]
#[doc(alias = "IOC22")]
pub type Ioc22 = crate::Reg<ioc22::Ioc22Spec>;
#[doc = "Configuration of DIO22"]
pub mod ioc22;
#[doc = "IOC23 (rw) register accessor: Configuration of DIO23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc23`]
module"]
#[doc(alias = "IOC23")]
pub type Ioc23 = crate::Reg<ioc23::Ioc23Spec>;
#[doc = "Configuration of DIO23"]
pub mod ioc23;
#[doc = "IOC24 (rw) register accessor: This field controls input hysteresis\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc24`]
module"]
#[doc(alias = "IOC24")]
pub type Ioc24 = crate::Reg<ioc24::Ioc24Spec>;
#[doc = "This field controls input hysteresis"]
pub mod ioc24;
#[doc = "IOC25 (rw) register accessor: Configuration of DIO25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioc25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioc25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioc25`]
module"]
#[doc(alias = "IOC25")]
pub type Ioc25 = crate::Reg<ioc25::Ioc25Spec>;
#[doc = "Configuration of DIO25"]
pub mod ioc25;
#[doc = "DTBCFG (rw) register accessor: DTB configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtbcfg`]
module"]
#[doc(alias = "DTBCFG")]
pub type Dtbcfg = crate::Reg<dtbcfg::DtbcfgSpec>;
#[doc = "DTB configuration"]
pub mod dtbcfg;
#[doc = "DTBOE (rw) register accessor: DTB output enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtboe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtboe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtboe`]
module"]
#[doc(alias = "DTBOE")]
pub type Dtboe = crate::Reg<dtboe::DtboeSpec>;
#[doc = "DTB output enable"]
pub mod dtboe;
#[doc = "EVTCFG (rw) register accessor: Event configuration. This register is used to select DIO for IOC to publish event on AON event fabric. It also contains enable bit that is used to mask the event and event flag bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtcfg`]
module"]
#[doc(alias = "EVTCFG")]
pub type Evtcfg = crate::Reg<evtcfg::EvtcfgSpec>;
#[doc = "Event configuration. This register is used to select DIO for IOC to publish event on AON event fabric. It also contains enable bit that is used to mask the event and event flag bit."]
pub mod evtcfg;
#[doc = "TEST (rw) register accessor: Test register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`test::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`]
module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "Test register."]
pub mod test;
#[doc = "DTBSTAT (rw) register accessor: DTB status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtbstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtbstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtbstat`]
module"]
#[doc(alias = "DTBSTAT")]
pub type Dtbstat = crate::Reg<dtbstat::DtbstatSpec>;
#[doc = "DTB status register."]
pub mod dtbstat;
