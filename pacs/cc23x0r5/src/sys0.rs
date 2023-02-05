#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    descex: Descex,
    glock: Glock,
    munlock: Munlock,
    status: Status,
    _reserved5: [u8; 0xec],
    atestcfg: Atestcfg,
    i2vcfg: I2vcfg,
    tsenscfg: Tsenscfg,
    lpcmpcfg: Lpcmpcfg,
    _reserved9: [u8; 0x10],
    tsdtest: Tsdtest,
    _reserved10: [u8; 0x02d8],
    deviceid: Deviceid,
    timmute0: Timmute0,
    timmute1: Timmute1,
    timmute2: Timmute2,
    timmute3: Timmute3,
    _reserved15: [u8; 0x03e8],
    partid: Partid,
    lifecyc: Lifecyc,
    tmute0: Tmute0,
    tmute1: Tmute1,
    tmute2: Tmute2,
    tmute3: Tmute3,
    tmute4: Tmute4,
    tmute5: Tmute5,
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
    #[doc = "0x08 - Global Lock Register. Locks registers in both mutable and immutable sections"]
    #[inline(always)]
    pub const fn glock(&self) -> &Glock {
        &self.glock
    }
    #[doc = "0x0c - Unlocks registers in mutable section"]
    #[inline(always)]
    pub const fn munlock(&self) -> &Munlock {
        &self.munlock
    }
    #[doc = "0x10 - Lock Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x100 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn atestcfg(&self) -> &Atestcfg {
        &self.atestcfg
    }
    #[doc = "0x104 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn i2vcfg(&self) -> &I2vcfg {
        &self.i2vcfg
    }
    #[doc = "0x108 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn tsenscfg(&self) -> &Tsenscfg {
        &self.tsenscfg
    }
    #[doc = "0x10c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn lpcmpcfg(&self) -> &Lpcmpcfg {
        &self.lpcmpcfg
    }
    #[doc = "0x120 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn tsdtest(&self) -> &Tsdtest {
        &self.tsdtest
    }
    #[doc = "0x3fc - This register provides Device ID information. Note: This 32-bit register value is provided as output to DEBUGSS."]
    #[inline(always)]
    pub const fn deviceid(&self) -> &Deviceid {
        &self.deviceid
    }
    #[doc = "0x400 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn timmute0(&self) -> &Timmute0 {
        &self.timmute0
    }
    #[doc = "0x404 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn timmute1(&self) -> &Timmute1 {
        &self.timmute1
    }
    #[doc = "0x408 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn timmute2(&self) -> &Timmute2 {
        &self.timmute2
    }
    #[doc = "0x40c - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn timmute3(&self) -> &Timmute3 {
        &self.timmute3
    }
    #[doc = "0x7f8 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn partid(&self) -> &Partid {
        &self.partid
    }
    #[doc = "0x7fc - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn lifecyc(&self) -> &Lifecyc {
        &self.lifecyc
    }
    #[doc = "0x800 - TMUTE0 trim Register"]
    #[inline(always)]
    pub const fn tmute0(&self) -> &Tmute0 {
        &self.tmute0
    }
    #[doc = "0x804 - TMUTE1 trim Register"]
    #[inline(always)]
    pub const fn tmute1(&self) -> &Tmute1 {
        &self.tmute1
    }
    #[doc = "0x808 - TMUTE2 trim Register"]
    #[inline(always)]
    pub const fn tmute2(&self) -> &Tmute2 {
        &self.tmute2
    }
    #[doc = "0x80c - TMUTE3 trim Register"]
    #[inline(always)]
    pub const fn tmute3(&self) -> &Tmute3 {
        &self.tmute3
    }
    #[doc = "0x810 - TMUTE4 trim Register"]
    #[inline(always)]
    pub const fn tmute4(&self) -> &Tmute4 {
        &self.tmute4
    }
    #[doc = "0x814 - TMUTE5 trim Register"]
    #[inline(always)]
    pub const fn tmute5(&self) -> &Tmute5 {
        &self.tmute5
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
#[doc = "GLOCK (rw) register accessor: Global Lock Register. Locks registers in both mutable and immutable sections\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glock`]
module"]
#[doc(alias = "GLOCK")]
pub type Glock = crate::Reg<glock::GlockSpec>;
#[doc = "Global Lock Register. Locks registers in both mutable and immutable sections"]
pub mod glock;
#[doc = "MUNLOCK (rw) register accessor: Unlocks registers in mutable section\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`munlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`munlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@munlock`]
module"]
#[doc(alias = "MUNLOCK")]
pub type Munlock = crate::Reg<munlock::MunlockSpec>;
#[doc = "Unlocks registers in mutable section"]
pub mod munlock;
#[doc = "STATUS (rw) register accessor: Lock Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Lock Status Register"]
pub mod status;
#[doc = "ATESTCFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atestcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atestcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@atestcfg`]
module"]
#[doc(alias = "ATESTCFG")]
pub type Atestcfg = crate::Reg<atestcfg::AtestcfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod atestcfg;
#[doc = "I2VCFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2vcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2vcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2vcfg`]
module"]
#[doc(alias = "I2VCFG")]
pub type I2vcfg = crate::Reg<i2vcfg::I2vcfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod i2vcfg;
#[doc = "TSENSCFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsenscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsenscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsenscfg`]
module"]
#[doc(alias = "TSENSCFG")]
pub type Tsenscfg = crate::Reg<tsenscfg::TsenscfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tsenscfg;
#[doc = "LPCMPCFG (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpcmpcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpcmpcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcmpcfg`]
module"]
#[doc(alias = "LPCMPCFG")]
pub type Lpcmpcfg = crate::Reg<lpcmpcfg::LpcmpcfgSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod lpcmpcfg;
#[doc = "TSDTEST (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsdtest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsdtest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsdtest`]
module"]
#[doc(alias = "TSDTEST")]
pub type Tsdtest = crate::Reg<tsdtest::TsdtestSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod tsdtest;
#[doc = "DEVICEID (rw) register accessor: This register provides Device ID information. Note: This 32-bit register value is provided as output to DEBUGSS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`deviceid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`deviceid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deviceid`]
module"]
#[doc(alias = "DEVICEID")]
pub type Deviceid = crate::Reg<deviceid::DeviceidSpec>;
#[doc = "This register provides Device ID information. Note: This 32-bit register value is provided as output to DEBUGSS."]
pub mod deviceid;
#[doc = "TIMMUTE0 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timmute0`]
module"]
#[doc(alias = "TIMMUTE0")]
pub type Timmute0 = crate::Reg<timmute0::Timmute0Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod timmute0;
#[doc = "TIMMUTE1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timmute1`]
module"]
#[doc(alias = "TIMMUTE1")]
pub type Timmute1 = crate::Reg<timmute1::Timmute1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod timmute1;
#[doc = "TIMMUTE2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timmute2`]
module"]
#[doc(alias = "TIMMUTE2")]
pub type Timmute2 = crate::Reg<timmute2::Timmute2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod timmute2;
#[doc = "TIMMUTE3 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timmute3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timmute3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timmute3`]
module"]
#[doc(alias = "TIMMUTE3")]
pub type Timmute3 = crate::Reg<timmute3::Timmute3Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod timmute3;
#[doc = "PARTID (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`partid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`partid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@partid`]
module"]
#[doc(alias = "PARTID")]
pub type Partid = crate::Reg<partid::PartidSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod partid;
#[doc = "LIFECYC (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lifecyc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lifecyc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lifecyc`]
module"]
#[doc(alias = "LIFECYC")]
pub type Lifecyc = crate::Reg<lifecyc::LifecycSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod lifecyc;
#[doc = "TMUTE0 (rw) register accessor: TMUTE0 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmute0`]
module"]
#[doc(alias = "TMUTE0")]
pub type Tmute0 = crate::Reg<tmute0::Tmute0Spec>;
#[doc = "TMUTE0 trim Register"]
pub mod tmute0;
#[doc = "TMUTE1 (rw) register accessor: TMUTE1 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmute1`]
module"]
#[doc(alias = "TMUTE1")]
pub type Tmute1 = crate::Reg<tmute1::Tmute1Spec>;
#[doc = "TMUTE1 trim Register"]
pub mod tmute1;
#[doc = "TMUTE2 (rw) register accessor: TMUTE2 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmute2`]
module"]
#[doc(alias = "TMUTE2")]
pub type Tmute2 = crate::Reg<tmute2::Tmute2Spec>;
#[doc = "TMUTE2 trim Register"]
pub mod tmute2;
#[doc = "TMUTE3 (rw) register accessor: TMUTE3 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmute3`]
module"]
#[doc(alias = "TMUTE3")]
pub type Tmute3 = crate::Reg<tmute3::Tmute3Spec>;
#[doc = "TMUTE3 trim Register"]
pub mod tmute3;
#[doc = "TMUTE4 (rw) register accessor: TMUTE4 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmute4`]
module"]
#[doc(alias = "TMUTE4")]
pub type Tmute4 = crate::Reg<tmute4::Tmute4Spec>;
#[doc = "TMUTE4 trim Register"]
pub mod tmute4;
#[doc = "TMUTE5 (rw) register accessor: TMUTE5 trim Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmute5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmute5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmute5`]
module"]
#[doc(alias = "TMUTE5")]
pub type Tmute5 = crate::Reg<tmute5::Tmute5Spec>;
#[doc = "TMUTE5 trim Register"]
pub mod tmute5;
