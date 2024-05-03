#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    stim0: Stim0,
    stim1: Stim1,
    stim2: Stim2,
    stim3: Stim3,
    stim4: Stim4,
    stim5: Stim5,
    stim6: Stim6,
    stim7: Stim7,
    stim8: Stim8,
    stim9: Stim9,
    stim10: Stim10,
    stim11: Stim11,
    stim12: Stim12,
    stim13: Stim13,
    stim14: Stim14,
    stim15: Stim15,
    stim16: Stim16,
    stim17: Stim17,
    stim18: Stim18,
    stim19: Stim19,
    stim20: Stim20,
    stim21: Stim21,
    stim22: Stim22,
    stim23: Stim23,
    stim24: Stim24,
    stim25: Stim25,
    stim26: Stim26,
    stim27: Stim27,
    stim28: Stim28,
    stim29: Stim29,
    stim30: Stim30,
    stim31: Stim31,
    _reserved32: [u8; 0x0d80],
    ter0: Ter0,
    _reserved33: [u8; 0x3c],
    tpr: Tpr,
    _reserved34: [u8; 0x3c],
    tcr: Tcr,
    _reserved35: [u8; 0x6c],
    int_atready: IntAtready,
    _reserved36: [u8; 0x04],
    int_atvalid: IntAtvalid,
    _reserved37: [u8; 0x04],
    itctrl: Itctrl,
    _reserved38: [u8; 0xb8],
    devarch: Devarch,
    _reserved39: [u8; 0x0c],
    devtype: Devtype,
    pidr4: Pidr4,
    pidr5: Pidr5,
    pidr6: Pidr6,
    pidr7: Pidr7,
    pidr0: Pidr0,
    pidr1: Pidr1,
    pidr2: Pidr2,
    pidr3: Pidr3,
    cidr0: Cidr0,
    cidr1: Cidr1,
    cidr2: Cidr2,
    cidr3: Cidr3,
}
impl RegisterBlock {
    #[doc = "0x00 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim0(&self) -> &Stim0 {
        &self.stim0
    }
    #[doc = "0x04 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim1(&self) -> &Stim1 {
        &self.stim1
    }
    #[doc = "0x08 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim2(&self) -> &Stim2 {
        &self.stim2
    }
    #[doc = "0x0c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim3(&self) -> &Stim3 {
        &self.stim3
    }
    #[doc = "0x10 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim4(&self) -> &Stim4 {
        &self.stim4
    }
    #[doc = "0x14 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim5(&self) -> &Stim5 {
        &self.stim5
    }
    #[doc = "0x18 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim6(&self) -> &Stim6 {
        &self.stim6
    }
    #[doc = "0x1c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim7(&self) -> &Stim7 {
        &self.stim7
    }
    #[doc = "0x20 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim8(&self) -> &Stim8 {
        &self.stim8
    }
    #[doc = "0x24 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim9(&self) -> &Stim9 {
        &self.stim9
    }
    #[doc = "0x28 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim10(&self) -> &Stim10 {
        &self.stim10
    }
    #[doc = "0x2c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim11(&self) -> &Stim11 {
        &self.stim11
    }
    #[doc = "0x30 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim12(&self) -> &Stim12 {
        &self.stim12
    }
    #[doc = "0x34 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim13(&self) -> &Stim13 {
        &self.stim13
    }
    #[doc = "0x38 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim14(&self) -> &Stim14 {
        &self.stim14
    }
    #[doc = "0x3c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim15(&self) -> &Stim15 {
        &self.stim15
    }
    #[doc = "0x40 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim16(&self) -> &Stim16 {
        &self.stim16
    }
    #[doc = "0x44 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim17(&self) -> &Stim17 {
        &self.stim17
    }
    #[doc = "0x48 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim18(&self) -> &Stim18 {
        &self.stim18
    }
    #[doc = "0x4c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim19(&self) -> &Stim19 {
        &self.stim19
    }
    #[doc = "0x50 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim20(&self) -> &Stim20 {
        &self.stim20
    }
    #[doc = "0x54 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim21(&self) -> &Stim21 {
        &self.stim21
    }
    #[doc = "0x58 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim22(&self) -> &Stim22 {
        &self.stim22
    }
    #[doc = "0x5c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim23(&self) -> &Stim23 {
        &self.stim23
    }
    #[doc = "0x60 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim24(&self) -> &Stim24 {
        &self.stim24
    }
    #[doc = "0x64 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim25(&self) -> &Stim25 {
        &self.stim25
    }
    #[doc = "0x68 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim26(&self) -> &Stim26 {
        &self.stim26
    }
    #[doc = "0x6c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim27(&self) -> &Stim27 {
        &self.stim27
    }
    #[doc = "0x70 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim28(&self) -> &Stim28 {
        &self.stim28
    }
    #[doc = "0x74 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim29(&self) -> &Stim29 {
        &self.stim29
    }
    #[doc = "0x78 - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim30(&self) -> &Stim30 {
        &self.stim30
    }
    #[doc = "0x7c - Provides the interface for generating Instrumentation packets"]
    #[inline(always)]
    pub const fn stim31(&self) -> &Stim31 {
        &self.stim31
    }
    #[doc = "0xe00 - Provide an individual enable bit for each ITM_STIM register"]
    #[inline(always)]
    pub const fn ter0(&self) -> &Ter0 {
        &self.ter0
    }
    #[doc = "0xe40 - Controls which stimulus ports can be accessed by unprivileged code"]
    #[inline(always)]
    pub const fn tpr(&self) -> &Tpr {
        &self.tpr
    }
    #[doc = "0xe80 - Configures and controls transfers through the ITM interface"]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0xef0 - Integration Mode: Read ATB Ready"]
    #[inline(always)]
    pub const fn int_atready(&self) -> &IntAtready {
        &self.int_atready
    }
    #[doc = "0xef8 - Integration Mode: Write ATB Valid"]
    #[inline(always)]
    pub const fn int_atvalid(&self) -> &IntAtvalid {
        &self.int_atvalid
    }
    #[doc = "0xf00 - Integration Mode Control Register"]
    #[inline(always)]
    pub const fn itctrl(&self) -> &Itctrl {
        &self.itctrl
    }
    #[doc = "0xfbc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn devarch(&self) -> &Devarch {
        &self.devarch
    }
    #[doc = "0xfcc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn devtype(&self) -> &Devtype {
        &self.devtype
    }
    #[doc = "0xfd0 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr4(&self) -> &Pidr4 {
        &self.pidr4
    }
    #[doc = "0xfd4 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr5(&self) -> &Pidr5 {
        &self.pidr5
    }
    #[doc = "0xfd8 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr6(&self) -> &Pidr6 {
        &self.pidr6
    }
    #[doc = "0xfdc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr7(&self) -> &Pidr7 {
        &self.pidr7
    }
    #[doc = "0xfe0 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr0(&self) -> &Pidr0 {
        &self.pidr0
    }
    #[doc = "0xfe4 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr1(&self) -> &Pidr1 {
        &self.pidr1
    }
    #[doc = "0xfe8 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr2(&self) -> &Pidr2 {
        &self.pidr2
    }
    #[doc = "0xfec - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn pidr3(&self) -> &Pidr3 {
        &self.pidr3
    }
    #[doc = "0xff0 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn cidr0(&self) -> &Cidr0 {
        &self.cidr0
    }
    #[doc = "0xff4 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn cidr1(&self) -> &Cidr1 {
        &self.cidr1
    }
    #[doc = "0xff8 - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn cidr2(&self) -> &Cidr2 {
        &self.cidr2
    }
    #[doc = "0xffc - Provides CoreSight discovery information for the ITM"]
    #[inline(always)]
    pub const fn cidr3(&self) -> &Cidr3 {
        &self.cidr3
    }
}
#[doc = "STIM0 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim0`]
module"]
#[doc(alias = "STIM0")]
pub type Stim0 = crate::Reg<stim0::Stim0Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim0;
#[doc = "STIM1 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim1`]
module"]
#[doc(alias = "STIM1")]
pub type Stim1 = crate::Reg<stim1::Stim1Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim1;
#[doc = "STIM2 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim2`]
module"]
#[doc(alias = "STIM2")]
pub type Stim2 = crate::Reg<stim2::Stim2Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim2;
#[doc = "STIM3 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim3`]
module"]
#[doc(alias = "STIM3")]
pub type Stim3 = crate::Reg<stim3::Stim3Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim3;
#[doc = "STIM4 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim4`]
module"]
#[doc(alias = "STIM4")]
pub type Stim4 = crate::Reg<stim4::Stim4Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim4;
#[doc = "STIM5 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim5`]
module"]
#[doc(alias = "STIM5")]
pub type Stim5 = crate::Reg<stim5::Stim5Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim5;
#[doc = "STIM6 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim6`]
module"]
#[doc(alias = "STIM6")]
pub type Stim6 = crate::Reg<stim6::Stim6Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim6;
#[doc = "STIM7 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim7`]
module"]
#[doc(alias = "STIM7")]
pub type Stim7 = crate::Reg<stim7::Stim7Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim7;
#[doc = "STIM8 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim8`]
module"]
#[doc(alias = "STIM8")]
pub type Stim8 = crate::Reg<stim8::Stim8Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim8;
#[doc = "STIM9 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim9`]
module"]
#[doc(alias = "STIM9")]
pub type Stim9 = crate::Reg<stim9::Stim9Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim9;
#[doc = "STIM10 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim10`]
module"]
#[doc(alias = "STIM10")]
pub type Stim10 = crate::Reg<stim10::Stim10Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim10;
#[doc = "STIM11 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim11`]
module"]
#[doc(alias = "STIM11")]
pub type Stim11 = crate::Reg<stim11::Stim11Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim11;
#[doc = "STIM12 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim12`]
module"]
#[doc(alias = "STIM12")]
pub type Stim12 = crate::Reg<stim12::Stim12Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim12;
#[doc = "STIM13 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim13`]
module"]
#[doc(alias = "STIM13")]
pub type Stim13 = crate::Reg<stim13::Stim13Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim13;
#[doc = "STIM14 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim14`]
module"]
#[doc(alias = "STIM14")]
pub type Stim14 = crate::Reg<stim14::Stim14Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim14;
#[doc = "STIM15 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim15`]
module"]
#[doc(alias = "STIM15")]
pub type Stim15 = crate::Reg<stim15::Stim15Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim15;
#[doc = "STIM16 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim16`]
module"]
#[doc(alias = "STIM16")]
pub type Stim16 = crate::Reg<stim16::Stim16Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim16;
#[doc = "STIM17 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim17`]
module"]
#[doc(alias = "STIM17")]
pub type Stim17 = crate::Reg<stim17::Stim17Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim17;
#[doc = "STIM18 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim18`]
module"]
#[doc(alias = "STIM18")]
pub type Stim18 = crate::Reg<stim18::Stim18Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim18;
#[doc = "STIM19 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim19`]
module"]
#[doc(alias = "STIM19")]
pub type Stim19 = crate::Reg<stim19::Stim19Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim19;
#[doc = "STIM20 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim20`]
module"]
#[doc(alias = "STIM20")]
pub type Stim20 = crate::Reg<stim20::Stim20Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim20;
#[doc = "STIM21 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim21`]
module"]
#[doc(alias = "STIM21")]
pub type Stim21 = crate::Reg<stim21::Stim21Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim21;
#[doc = "STIM22 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim22`]
module"]
#[doc(alias = "STIM22")]
pub type Stim22 = crate::Reg<stim22::Stim22Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim22;
#[doc = "STIM23 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim23`]
module"]
#[doc(alias = "STIM23")]
pub type Stim23 = crate::Reg<stim23::Stim23Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim23;
#[doc = "STIM24 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim24`]
module"]
#[doc(alias = "STIM24")]
pub type Stim24 = crate::Reg<stim24::Stim24Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim24;
#[doc = "STIM25 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim25`]
module"]
#[doc(alias = "STIM25")]
pub type Stim25 = crate::Reg<stim25::Stim25Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim25;
#[doc = "STIM26 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim26`]
module"]
#[doc(alias = "STIM26")]
pub type Stim26 = crate::Reg<stim26::Stim26Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim26;
#[doc = "STIM27 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim27`]
module"]
#[doc(alias = "STIM27")]
pub type Stim27 = crate::Reg<stim27::Stim27Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim27;
#[doc = "STIM28 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim28`]
module"]
#[doc(alias = "STIM28")]
pub type Stim28 = crate::Reg<stim28::Stim28Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim28;
#[doc = "STIM29 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim29`]
module"]
#[doc(alias = "STIM29")]
pub type Stim29 = crate::Reg<stim29::Stim29Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim29;
#[doc = "STIM30 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim30`]
module"]
#[doc(alias = "STIM30")]
pub type Stim30 = crate::Reg<stim30::Stim30Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim30;
#[doc = "STIM31 (rw) register accessor: Provides the interface for generating Instrumentation packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim31`]
module"]
#[doc(alias = "STIM31")]
pub type Stim31 = crate::Reg<stim31::Stim31Spec>;
#[doc = "Provides the interface for generating Instrumentation packets"]
pub mod stim31;
#[doc = "TER0 (rw) register accessor: Provide an individual enable bit for each ITM_STIM register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ter0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ter0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ter0`]
module"]
#[doc(alias = "TER0")]
pub type Ter0 = crate::Reg<ter0::Ter0Spec>;
#[doc = "Provide an individual enable bit for each ITM_STIM register"]
pub mod ter0;
#[doc = "TPR (rw) register accessor: Controls which stimulus ports can be accessed by unprivileged code\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`]
module"]
#[doc(alias = "TPR")]
pub type Tpr = crate::Reg<tpr::TprSpec>;
#[doc = "Controls which stimulus ports can be accessed by unprivileged code"]
pub mod tpr;
#[doc = "TCR (rw) register accessor: Configures and controls transfers through the ITM interface\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Configures and controls transfers through the ITM interface"]
pub mod tcr;
#[doc = "INT_ATREADY (rw) register accessor: Integration Mode: Read ATB Ready\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_atready::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_atready::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_atready`]
module"]
#[doc(alias = "INT_ATREADY")]
pub type IntAtready = crate::Reg<int_atready::IntAtreadySpec>;
#[doc = "Integration Mode: Read ATB Ready"]
pub mod int_atready;
#[doc = "INT_ATVALID (rw) register accessor: Integration Mode: Write ATB Valid\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_atvalid::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_atvalid::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_atvalid`]
module"]
#[doc(alias = "INT_ATVALID")]
pub type IntAtvalid = crate::Reg<int_atvalid::IntAtvalidSpec>;
#[doc = "Integration Mode: Write ATB Valid"]
pub mod int_atvalid;
#[doc = "ITCTRL (rw) register accessor: Integration Mode Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@itctrl`]
module"]
#[doc(alias = "ITCTRL")]
pub type Itctrl = crate::Reg<itctrl::ItctrlSpec>;
#[doc = "Integration Mode Control Register"]
pub mod itctrl;
#[doc = "DEVARCH (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devarch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devarch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devarch`]
module"]
#[doc(alias = "DEVARCH")]
pub type Devarch = crate::Reg<devarch::DevarchSpec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod devarch;
#[doc = "DEVTYPE (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`devtype::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`devtype::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devtype`]
module"]
#[doc(alias = "DEVTYPE")]
pub type Devtype = crate::Reg<devtype::DevtypeSpec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod devtype;
#[doc = "PIDR4 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr4`]
module"]
#[doc(alias = "PIDR4")]
pub type Pidr4 = crate::Reg<pidr4::Pidr4Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr4;
#[doc = "PIDR5 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr5`]
module"]
#[doc(alias = "PIDR5")]
pub type Pidr5 = crate::Reg<pidr5::Pidr5Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr5;
#[doc = "PIDR6 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr6`]
module"]
#[doc(alias = "PIDR6")]
pub type Pidr6 = crate::Reg<pidr6::Pidr6Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr6;
#[doc = "PIDR7 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr7`]
module"]
#[doc(alias = "PIDR7")]
pub type Pidr7 = crate::Reg<pidr7::Pidr7Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr7;
#[doc = "PIDR0 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr0`]
module"]
#[doc(alias = "PIDR0")]
pub type Pidr0 = crate::Reg<pidr0::Pidr0Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr0;
#[doc = "PIDR1 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr1`]
module"]
#[doc(alias = "PIDR1")]
pub type Pidr1 = crate::Reg<pidr1::Pidr1Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr1;
#[doc = "PIDR2 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr2`]
module"]
#[doc(alias = "PIDR2")]
pub type Pidr2 = crate::Reg<pidr2::Pidr2Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr2;
#[doc = "PIDR3 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pidr3`]
module"]
#[doc(alias = "PIDR3")]
pub type Pidr3 = crate::Reg<pidr3::Pidr3Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod pidr3;
#[doc = "CIDR0 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr0`]
module"]
#[doc(alias = "CIDR0")]
pub type Cidr0 = crate::Reg<cidr0::Cidr0Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr0;
#[doc = "CIDR1 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr1`]
module"]
#[doc(alias = "CIDR1")]
pub type Cidr1 = crate::Reg<cidr1::Cidr1Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr1;
#[doc = "CIDR2 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr2`]
module"]
#[doc(alias = "CIDR2")]
pub type Cidr2 = crate::Reg<cidr2::Cidr2Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr2;
#[doc = "CIDR3 (rw) register accessor: Provides CoreSight discovery information for the ITM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cidr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cidr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cidr3`]
module"]
#[doc(alias = "CIDR3")]
pub type Cidr3 = crate::Reg<cidr3::Cidr3Spec>;
#[doc = "Provides CoreSight discovery information for the ITM"]
pub mod cidr3;
