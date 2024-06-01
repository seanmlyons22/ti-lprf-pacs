#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iocfg0: Iocfg0,
    iocfg1: Iocfg1,
    iocfg2: Iocfg2,
    iocfg3: Iocfg3,
    iocfg4: Iocfg4,
    iocfg5: Iocfg5,
    iocfg6: Iocfg6,
    iocfg7: Iocfg7,
    iocfg8: Iocfg8,
    iocfg9: Iocfg9,
    iocfg10: Iocfg10,
    iocfg11: Iocfg11,
    iocfg12: Iocfg12,
    iocfg13: Iocfg13,
    iocfg14: Iocfg14,
    iocfg15: Iocfg15,
    iocfg16: Iocfg16,
    iocfg17: Iocfg17,
    iocfg18: Iocfg18,
    iocfg19: Iocfg19,
    iocfg20: Iocfg20,
    iocfg21: Iocfg21,
    iocfg22: Iocfg22,
    iocfg23: Iocfg23,
    iocfg24: Iocfg24,
    iocfg25: Iocfg25,
    iocfg26: Iocfg26,
    iocfg27: Iocfg27,
    iocfg28: Iocfg28,
    iocfg29: Iocfg29,
    iocfg30: Iocfg30,
    iocfg31: Iocfg31,
}
impl RegisterBlock {
    #[doc = "0x00 - Configuration of DIO0"]
    #[inline(always)]
    pub const fn iocfg0(&self) -> &Iocfg0 {
        &self.iocfg0
    }
    #[doc = "0x04 - Configuration of DIO1"]
    #[inline(always)]
    pub const fn iocfg1(&self) -> &Iocfg1 {
        &self.iocfg1
    }
    #[doc = "0x08 - Configuration of DIO2"]
    #[inline(always)]
    pub const fn iocfg2(&self) -> &Iocfg2 {
        &self.iocfg2
    }
    #[doc = "0x0c - Configuration of DIO3"]
    #[inline(always)]
    pub const fn iocfg3(&self) -> &Iocfg3 {
        &self.iocfg3
    }
    #[doc = "0x10 - Configuration of DIO4"]
    #[inline(always)]
    pub const fn iocfg4(&self) -> &Iocfg4 {
        &self.iocfg4
    }
    #[doc = "0x14 - Configuration of DIO5"]
    #[inline(always)]
    pub const fn iocfg5(&self) -> &Iocfg5 {
        &self.iocfg5
    }
    #[doc = "0x18 - Configuration of DIO6"]
    #[inline(always)]
    pub const fn iocfg6(&self) -> &Iocfg6 {
        &self.iocfg6
    }
    #[doc = "0x1c - Configuration of DIO7"]
    #[inline(always)]
    pub const fn iocfg7(&self) -> &Iocfg7 {
        &self.iocfg7
    }
    #[doc = "0x20 - Configuration of DIO8"]
    #[inline(always)]
    pub const fn iocfg8(&self) -> &Iocfg8 {
        &self.iocfg8
    }
    #[doc = "0x24 - Configuration of DIO9"]
    #[inline(always)]
    pub const fn iocfg9(&self) -> &Iocfg9 {
        &self.iocfg9
    }
    #[doc = "0x28 - Configuration of DIO10"]
    #[inline(always)]
    pub const fn iocfg10(&self) -> &Iocfg10 {
        &self.iocfg10
    }
    #[doc = "0x2c - Configuration of DIO11"]
    #[inline(always)]
    pub const fn iocfg11(&self) -> &Iocfg11 {
        &self.iocfg11
    }
    #[doc = "0x30 - Configuration of DIO12"]
    #[inline(always)]
    pub const fn iocfg12(&self) -> &Iocfg12 {
        &self.iocfg12
    }
    #[doc = "0x34 - Configuration of DIO13"]
    #[inline(always)]
    pub const fn iocfg13(&self) -> &Iocfg13 {
        &self.iocfg13
    }
    #[doc = "0x38 - Configuration of DIO14"]
    #[inline(always)]
    pub const fn iocfg14(&self) -> &Iocfg14 {
        &self.iocfg14
    }
    #[doc = "0x3c - Configuration of DIO15"]
    #[inline(always)]
    pub const fn iocfg15(&self) -> &Iocfg15 {
        &self.iocfg15
    }
    #[doc = "0x40 - Configuration of DIO16"]
    #[inline(always)]
    pub const fn iocfg16(&self) -> &Iocfg16 {
        &self.iocfg16
    }
    #[doc = "0x44 - Configuration of DIO17"]
    #[inline(always)]
    pub const fn iocfg17(&self) -> &Iocfg17 {
        &self.iocfg17
    }
    #[doc = "0x48 - Configuration of DIO18"]
    #[inline(always)]
    pub const fn iocfg18(&self) -> &Iocfg18 {
        &self.iocfg18
    }
    #[doc = "0x4c - Configuration of DIO19"]
    #[inline(always)]
    pub const fn iocfg19(&self) -> &Iocfg19 {
        &self.iocfg19
    }
    #[doc = "0x50 - Configuration of DIO20"]
    #[inline(always)]
    pub const fn iocfg20(&self) -> &Iocfg20 {
        &self.iocfg20
    }
    #[doc = "0x54 - Configuration of DIO21"]
    #[inline(always)]
    pub const fn iocfg21(&self) -> &Iocfg21 {
        &self.iocfg21
    }
    #[doc = "0x58 - Configuration of DIO22"]
    #[inline(always)]
    pub const fn iocfg22(&self) -> &Iocfg22 {
        &self.iocfg22
    }
    #[doc = "0x5c - Configuration of DIO23"]
    #[inline(always)]
    pub const fn iocfg23(&self) -> &Iocfg23 {
        &self.iocfg23
    }
    #[doc = "0x60 - Configuration of DIO24"]
    #[inline(always)]
    pub const fn iocfg24(&self) -> &Iocfg24 {
        &self.iocfg24
    }
    #[doc = "0x64 - Configuration of DIO25"]
    #[inline(always)]
    pub const fn iocfg25(&self) -> &Iocfg25 {
        &self.iocfg25
    }
    #[doc = "0x68 - Configuration of DIO26"]
    #[inline(always)]
    pub const fn iocfg26(&self) -> &Iocfg26 {
        &self.iocfg26
    }
    #[doc = "0x6c - Configuration of DIO27"]
    #[inline(always)]
    pub const fn iocfg27(&self) -> &Iocfg27 {
        &self.iocfg27
    }
    #[doc = "0x70 - Configuration of DIO28"]
    #[inline(always)]
    pub const fn iocfg28(&self) -> &Iocfg28 {
        &self.iocfg28
    }
    #[doc = "0x74 - Configuration of DIO29"]
    #[inline(always)]
    pub const fn iocfg29(&self) -> &Iocfg29 {
        &self.iocfg29
    }
    #[doc = "0x78 - Configuration of DIO30"]
    #[inline(always)]
    pub const fn iocfg30(&self) -> &Iocfg30 {
        &self.iocfg30
    }
    #[doc = "0x7c - Configuration of DIO31"]
    #[inline(always)]
    pub const fn iocfg31(&self) -> &Iocfg31 {
        &self.iocfg31
    }
}
#[doc = "IOCFG0 (rw) register accessor: Configuration of DIO0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg0`]
module"]
#[doc(alias = "IOCFG0")]
pub type Iocfg0 = crate::Reg<iocfg0::Iocfg0Spec>;
#[doc = "Configuration of DIO0"]
pub mod iocfg0;
#[doc = "IOCFG1 (rw) register accessor: Configuration of DIO1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg1`]
module"]
#[doc(alias = "IOCFG1")]
pub type Iocfg1 = crate::Reg<iocfg1::Iocfg1Spec>;
#[doc = "Configuration of DIO1"]
pub mod iocfg1;
#[doc = "IOCFG2 (rw) register accessor: Configuration of DIO2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg2`]
module"]
#[doc(alias = "IOCFG2")]
pub type Iocfg2 = crate::Reg<iocfg2::Iocfg2Spec>;
#[doc = "Configuration of DIO2"]
pub mod iocfg2;
#[doc = "IOCFG3 (rw) register accessor: Configuration of DIO3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg3`]
module"]
#[doc(alias = "IOCFG3")]
pub type Iocfg3 = crate::Reg<iocfg3::Iocfg3Spec>;
#[doc = "Configuration of DIO3"]
pub mod iocfg3;
#[doc = "IOCFG4 (rw) register accessor: Configuration of DIO4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg4`]
module"]
#[doc(alias = "IOCFG4")]
pub type Iocfg4 = crate::Reg<iocfg4::Iocfg4Spec>;
#[doc = "Configuration of DIO4"]
pub mod iocfg4;
#[doc = "IOCFG5 (rw) register accessor: Configuration of DIO5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg5`]
module"]
#[doc(alias = "IOCFG5")]
pub type Iocfg5 = crate::Reg<iocfg5::Iocfg5Spec>;
#[doc = "Configuration of DIO5"]
pub mod iocfg5;
#[doc = "IOCFG6 (rw) register accessor: Configuration of DIO6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg6`]
module"]
#[doc(alias = "IOCFG6")]
pub type Iocfg6 = crate::Reg<iocfg6::Iocfg6Spec>;
#[doc = "Configuration of DIO6"]
pub mod iocfg6;
#[doc = "IOCFG7 (rw) register accessor: Configuration of DIO7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg7`]
module"]
#[doc(alias = "IOCFG7")]
pub type Iocfg7 = crate::Reg<iocfg7::Iocfg7Spec>;
#[doc = "Configuration of DIO7"]
pub mod iocfg7;
#[doc = "IOCFG8 (rw) register accessor: Configuration of DIO8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg8`]
module"]
#[doc(alias = "IOCFG8")]
pub type Iocfg8 = crate::Reg<iocfg8::Iocfg8Spec>;
#[doc = "Configuration of DIO8"]
pub mod iocfg8;
#[doc = "IOCFG9 (rw) register accessor: Configuration of DIO9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg9`]
module"]
#[doc(alias = "IOCFG9")]
pub type Iocfg9 = crate::Reg<iocfg9::Iocfg9Spec>;
#[doc = "Configuration of DIO9"]
pub mod iocfg9;
#[doc = "IOCFG10 (rw) register accessor: Configuration of DIO10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg10`]
module"]
#[doc(alias = "IOCFG10")]
pub type Iocfg10 = crate::Reg<iocfg10::Iocfg10Spec>;
#[doc = "Configuration of DIO10"]
pub mod iocfg10;
#[doc = "IOCFG11 (rw) register accessor: Configuration of DIO11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg11`]
module"]
#[doc(alias = "IOCFG11")]
pub type Iocfg11 = crate::Reg<iocfg11::Iocfg11Spec>;
#[doc = "Configuration of DIO11"]
pub mod iocfg11;
#[doc = "IOCFG12 (rw) register accessor: Configuration of DIO12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg12`]
module"]
#[doc(alias = "IOCFG12")]
pub type Iocfg12 = crate::Reg<iocfg12::Iocfg12Spec>;
#[doc = "Configuration of DIO12"]
pub mod iocfg12;
#[doc = "IOCFG13 (rw) register accessor: Configuration of DIO13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg13`]
module"]
#[doc(alias = "IOCFG13")]
pub type Iocfg13 = crate::Reg<iocfg13::Iocfg13Spec>;
#[doc = "Configuration of DIO13"]
pub mod iocfg13;
#[doc = "IOCFG14 (rw) register accessor: Configuration of DIO14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg14`]
module"]
#[doc(alias = "IOCFG14")]
pub type Iocfg14 = crate::Reg<iocfg14::Iocfg14Spec>;
#[doc = "Configuration of DIO14"]
pub mod iocfg14;
#[doc = "IOCFG15 (rw) register accessor: Configuration of DIO15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg15`]
module"]
#[doc(alias = "IOCFG15")]
pub type Iocfg15 = crate::Reg<iocfg15::Iocfg15Spec>;
#[doc = "Configuration of DIO15"]
pub mod iocfg15;
#[doc = "IOCFG16 (rw) register accessor: Configuration of DIO16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg16`]
module"]
#[doc(alias = "IOCFG16")]
pub type Iocfg16 = crate::Reg<iocfg16::Iocfg16Spec>;
#[doc = "Configuration of DIO16"]
pub mod iocfg16;
#[doc = "IOCFG17 (rw) register accessor: Configuration of DIO17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg17`]
module"]
#[doc(alias = "IOCFG17")]
pub type Iocfg17 = crate::Reg<iocfg17::Iocfg17Spec>;
#[doc = "Configuration of DIO17"]
pub mod iocfg17;
#[doc = "IOCFG18 (rw) register accessor: Configuration of DIO18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg18`]
module"]
#[doc(alias = "IOCFG18")]
pub type Iocfg18 = crate::Reg<iocfg18::Iocfg18Spec>;
#[doc = "Configuration of DIO18"]
pub mod iocfg18;
#[doc = "IOCFG19 (rw) register accessor: Configuration of DIO19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg19`]
module"]
#[doc(alias = "IOCFG19")]
pub type Iocfg19 = crate::Reg<iocfg19::Iocfg19Spec>;
#[doc = "Configuration of DIO19"]
pub mod iocfg19;
#[doc = "IOCFG20 (rw) register accessor: Configuration of DIO20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg20`]
module"]
#[doc(alias = "IOCFG20")]
pub type Iocfg20 = crate::Reg<iocfg20::Iocfg20Spec>;
#[doc = "Configuration of DIO20"]
pub mod iocfg20;
#[doc = "IOCFG21 (rw) register accessor: Configuration of DIO21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg21`]
module"]
#[doc(alias = "IOCFG21")]
pub type Iocfg21 = crate::Reg<iocfg21::Iocfg21Spec>;
#[doc = "Configuration of DIO21"]
pub mod iocfg21;
#[doc = "IOCFG22 (rw) register accessor: Configuration of DIO22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg22`]
module"]
#[doc(alias = "IOCFG22")]
pub type Iocfg22 = crate::Reg<iocfg22::Iocfg22Spec>;
#[doc = "Configuration of DIO22"]
pub mod iocfg22;
#[doc = "IOCFG23 (rw) register accessor: Configuration of DIO23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg23`]
module"]
#[doc(alias = "IOCFG23")]
pub type Iocfg23 = crate::Reg<iocfg23::Iocfg23Spec>;
#[doc = "Configuration of DIO23"]
pub mod iocfg23;
#[doc = "IOCFG24 (rw) register accessor: Configuration of DIO24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg24`]
module"]
#[doc(alias = "IOCFG24")]
pub type Iocfg24 = crate::Reg<iocfg24::Iocfg24Spec>;
#[doc = "Configuration of DIO24"]
pub mod iocfg24;
#[doc = "IOCFG25 (rw) register accessor: Configuration of DIO25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg25`]
module"]
#[doc(alias = "IOCFG25")]
pub type Iocfg25 = crate::Reg<iocfg25::Iocfg25Spec>;
#[doc = "Configuration of DIO25"]
pub mod iocfg25;
#[doc = "IOCFG26 (rw) register accessor: Configuration of DIO26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg26`]
module"]
#[doc(alias = "IOCFG26")]
pub type Iocfg26 = crate::Reg<iocfg26::Iocfg26Spec>;
#[doc = "Configuration of DIO26"]
pub mod iocfg26;
#[doc = "IOCFG27 (rw) register accessor: Configuration of DIO27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg27`]
module"]
#[doc(alias = "IOCFG27")]
pub type Iocfg27 = crate::Reg<iocfg27::Iocfg27Spec>;
#[doc = "Configuration of DIO27"]
pub mod iocfg27;
#[doc = "IOCFG28 (rw) register accessor: Configuration of DIO28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg28`]
module"]
#[doc(alias = "IOCFG28")]
pub type Iocfg28 = crate::Reg<iocfg28::Iocfg28Spec>;
#[doc = "Configuration of DIO28"]
pub mod iocfg28;
#[doc = "IOCFG29 (rw) register accessor: Configuration of DIO29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg29`]
module"]
#[doc(alias = "IOCFG29")]
pub type Iocfg29 = crate::Reg<iocfg29::Iocfg29Spec>;
#[doc = "Configuration of DIO29"]
pub mod iocfg29;
#[doc = "IOCFG30 (rw) register accessor: Configuration of DIO30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg30`]
module"]
#[doc(alias = "IOCFG30")]
pub type Iocfg30 = crate::Reg<iocfg30::Iocfg30Spec>;
#[doc = "Configuration of DIO30"]
pub mod iocfg30;
#[doc = "IOCFG31 (rw) register accessor: Configuration of DIO31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iocfg31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iocfg31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iocfg31`]
module"]
#[doc(alias = "IOCFG31")]
pub type Iocfg31 = crate::Reg<iocfg31::Iocfg31Spec>;
#[doc = "Configuration of DIO31"]
pub mod iocfg31;
