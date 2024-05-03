#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dout3_0: Dout3_0,
    dout7_4: Dout7_4,
    dout11_8: Dout11_8,
    dout15_12: Dout15_12,
    dout19_16: Dout19_16,
    dout23_20: Dout23_20,
    dout27_24: Dout27_24,
    dout31_28: Dout31_28,
    dout35_32: Dout35_32,
    dout39_36: Dout39_36,
    dout43_40: Dout43_40,
    dout47_44: Dout47_44,
    _reserved12: [u8; 0x50],
    dout31_0: Dout31_0,
    dout47_32: Dout47_32,
    _reserved14: [u8; 0x08],
    doutset31_0: Doutset31_0,
    doutset47_32: Doutset47_32,
    _reserved16: [u8; 0x08],
    doutclr31_0: Doutclr31_0,
    doutclr47_32: Doutclr47_32,
    _reserved18: [u8; 0x08],
    douttgl31_0: Douttgl31_0,
    douttgl47_32: Douttgl47_32,
    _reserved20: [u8; 0x08],
    din31_0: Din31_0,
    din47_32: Din47_32,
    _reserved22: [u8; 0x08],
    doe31_0: Doe31_0,
    doe47_32: Doe47_32,
    _reserved24: [u8; 0x08],
    evflags31_0: Evflags31_0,
    evflags47_32: Evflags47_32,
}
impl RegisterBlock {
    #[doc = "0x00 - Data Out 0 to 3 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout3_0(&self) -> &Dout3_0 {
        &self.dout3_0
    }
    #[doc = "0x04 - Data Out 4 to 7 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout7_4(&self) -> &Dout7_4 {
        &self.dout7_4
    }
    #[doc = "0x08 - Data Out 8 to 11 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout11_8(&self) -> &Dout11_8 {
        &self.dout11_8
    }
    #[doc = "0x0c - Data Out 12 to 15 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout15_12(&self) -> &Dout15_12 {
        &self.dout15_12
    }
    #[doc = "0x10 - Data Out 16 to 19 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout19_16(&self) -> &Dout19_16 {
        &self.dout19_16
    }
    #[doc = "0x14 - Data Out 20 to 23 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout23_20(&self) -> &Dout23_20 {
        &self.dout23_20
    }
    #[doc = "0x18 - Data Out 24 to 27 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout27_24(&self) -> &Dout27_24 {
        &self.dout27_24
    }
    #[doc = "0x1c - Data Out 28 to 31 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout31_28(&self) -> &Dout31_28 {
        &self.dout31_28
    }
    #[doc = "0x20 - Data Out 35 to 32 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout35_32(&self) -> &Dout35_32 {
        &self.dout35_32
    }
    #[doc = "0x24 - Data Out 39 to 36 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout39_36(&self) -> &Dout39_36 {
        &self.dout39_36
    }
    #[doc = "0x28 - Data Out 43 to 40 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout43_40(&self) -> &Dout43_40 {
        &self.dout43_40
    }
    #[doc = "0x2c - Data Out 47 to 44 Alias register for byte access to each bit in DOUT47_0"]
    #[inline(always)]
    pub const fn dout47_44(&self) -> &Dout47_44 {
        &self.dout47_44
    }
    #[doc = "0x80 - Data Output for DIO 0 to 31"]
    #[inline(always)]
    pub const fn dout31_0(&self) -> &Dout31_0 {
        &self.dout31_0
    }
    #[doc = "0x84 - Data Output for DIO 0 to 31"]
    #[inline(always)]
    pub const fn dout47_32(&self) -> &Dout47_32 {
        &self.dout47_32
    }
    #[doc = "0x90 - Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
    #[inline(always)]
    pub const fn doutset31_0(&self) -> &Doutset31_0 {
        &self.doutset31_0
    }
    #[doc = "0x94 - Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
    #[inline(always)]
    pub const fn doutset47_32(&self) -> &Doutset47_32 {
        &self.doutset47_32
    }
    #[doc = "0xa0 - Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
    #[inline(always)]
    pub const fn doutclr31_0(&self) -> &Doutclr31_0 {
        &self.doutclr31_0
    }
    #[doc = "0xa4 - Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
    #[inline(always)]
    pub const fn doutclr47_32(&self) -> &Doutclr47_32 {
        &self.doutclr47_32
    }
    #[doc = "0xb0 - Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    #[inline(always)]
    pub const fn douttgl31_0(&self) -> &Douttgl31_0 {
        &self.douttgl31_0
    }
    #[doc = "0xb4 - Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    #[inline(always)]
    pub const fn douttgl47_32(&self) -> &Douttgl47_32 {
        &self.douttgl47_32
    }
    #[doc = "0xc0 - Data Input from DIO 0 to 31"]
    #[inline(always)]
    pub const fn din31_0(&self) -> &Din31_0 {
        &self.din31_0
    }
    #[doc = "0xc4 - Data Input from DIO 32 to 47"]
    #[inline(always)]
    pub const fn din47_32(&self) -> &Din47_32 {
        &self.din47_32
    }
    #[doc = "0xd0 - Data Output Enable for DIO 0 to 31"]
    #[inline(always)]
    pub const fn doe31_0(&self) -> &Doe31_0 {
        &self.doe31_0
    }
    #[doc = "0xd4 - Data Output Enable for DIO 32 to 47"]
    #[inline(always)]
    pub const fn doe47_32(&self) -> &Doe47_32 {
        &self.doe47_32
    }
    #[doc = "0xe0 - Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    #[inline(always)]
    pub const fn evflags31_0(&self) -> &Evflags31_0 {
        &self.evflags31_0
    }
    #[doc = "0xe4 - Event Register for DIO 32 to 47 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    #[inline(always)]
    pub const fn evflags47_32(&self) -> &Evflags47_32 {
        &self.evflags47_32
    }
}
#[doc = "DOUT3_0 (rw) register accessor: Data Out 0 to 3 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout3_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout3_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout3_0`]
module"]
#[doc(alias = "DOUT3_0")]
pub type Dout3_0 = crate::Reg<dout3_0::Dout3_0Spec>;
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout3_0;
#[doc = "DOUT7_4 (rw) register accessor: Data Out 4 to 7 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout7_4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout7_4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout7_4`]
module"]
#[doc(alias = "DOUT7_4")]
pub type Dout7_4 = crate::Reg<dout7_4::Dout7_4Spec>;
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout7_4;
#[doc = "DOUT11_8 (rw) register accessor: Data Out 8 to 11 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout11_8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout11_8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout11_8`]
module"]
#[doc(alias = "DOUT11_8")]
pub type Dout11_8 = crate::Reg<dout11_8::Dout11_8Spec>;
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout11_8;
#[doc = "DOUT15_12 (rw) register accessor: Data Out 12 to 15 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout15_12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout15_12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout15_12`]
module"]
#[doc(alias = "DOUT15_12")]
pub type Dout15_12 = crate::Reg<dout15_12::Dout15_12Spec>;
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout15_12;
#[doc = "DOUT19_16 (rw) register accessor: Data Out 16 to 19 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout19_16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout19_16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout19_16`]
module"]
#[doc(alias = "DOUT19_16")]
pub type Dout19_16 = crate::Reg<dout19_16::Dout19_16Spec>;
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout19_16;
#[doc = "DOUT23_20 (rw) register accessor: Data Out 20 to 23 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout23_20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout23_20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout23_20`]
module"]
#[doc(alias = "DOUT23_20")]
pub type Dout23_20 = crate::Reg<dout23_20::Dout23_20Spec>;
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout23_20;
#[doc = "DOUT27_24 (rw) register accessor: Data Out 24 to 27 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout27_24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout27_24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout27_24`]
module"]
#[doc(alias = "DOUT27_24")]
pub type Dout27_24 = crate::Reg<dout27_24::Dout27_24Spec>;
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout27_24;
#[doc = "DOUT31_28 (rw) register accessor: Data Out 28 to 31 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout31_28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout31_28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout31_28`]
module"]
#[doc(alias = "DOUT31_28")]
pub type Dout31_28 = crate::Reg<dout31_28::Dout31_28Spec>;
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout31_28;
#[doc = "DOUT35_32 (rw) register accessor: Data Out 35 to 32 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout35_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout35_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout35_32`]
module"]
#[doc(alias = "DOUT35_32")]
pub type Dout35_32 = crate::Reg<dout35_32::Dout35_32Spec>;
#[doc = "Data Out 35 to 32 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout35_32;
#[doc = "DOUT39_36 (rw) register accessor: Data Out 39 to 36 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout39_36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout39_36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout39_36`]
module"]
#[doc(alias = "DOUT39_36")]
pub type Dout39_36 = crate::Reg<dout39_36::Dout39_36Spec>;
#[doc = "Data Out 39 to 36 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout39_36;
#[doc = "DOUT43_40 (rw) register accessor: Data Out 43 to 40 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout43_40::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout43_40::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout43_40`]
module"]
#[doc(alias = "DOUT43_40")]
pub type Dout43_40 = crate::Reg<dout43_40::Dout43_40Spec>;
#[doc = "Data Out 43 to 40 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout43_40;
#[doc = "DOUT47_44 (rw) register accessor: Data Out 47 to 44 Alias register for byte access to each bit in DOUT47_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout47_44::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout47_44::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout47_44`]
module"]
#[doc(alias = "DOUT47_44")]
pub type Dout47_44 = crate::Reg<dout47_44::Dout47_44Spec>;
#[doc = "Data Out 47 to 44 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout47_44;
#[doc = "DOUT31_0 (rw) register accessor: Data Output for DIO 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout31_0`]
module"]
#[doc(alias = "DOUT31_0")]
pub type Dout31_0 = crate::Reg<dout31_0::Dout31_0Spec>;
#[doc = "Data Output for DIO 0 to 31"]
pub mod dout31_0;
#[doc = "DOUT47_32 (rw) register accessor: Data Output for DIO 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dout47_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dout47_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dout47_32`]
module"]
#[doc(alias = "DOUT47_32")]
pub type Dout47_32 = crate::Reg<dout47_32::Dout47_32Spec>;
#[doc = "Data Output for DIO 0 to 31"]
pub mod dout47_32;
#[doc = "DOUTSET31_0 (rw) register accessor: Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutset31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutset31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutset31_0`]
module"]
#[doc(alias = "DOUTSET31_0")]
pub type Doutset31_0 = crate::Reg<doutset31_0::Doutset31_0Spec>;
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
pub mod doutset31_0;
#[doc = "DOUTSET47_32 (rw) register accessor: Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutset47_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutset47_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutset47_32`]
module"]
#[doc(alias = "DOUTSET47_32")]
pub type Doutset47_32 = crate::Reg<doutset47_32::Doutset47_32Spec>;
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
pub mod doutset47_32;
#[doc = "DOUTCLR31_0 (rw) register accessor: Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutclr31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutclr31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutclr31_0`]
module"]
#[doc(alias = "DOUTCLR31_0")]
pub type Doutclr31_0 = crate::Reg<doutclr31_0::Doutclr31_0Spec>;
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
pub mod doutclr31_0;
#[doc = "DOUTCLR47_32 (rw) register accessor: Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doutclr47_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doutclr47_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doutclr47_32`]
module"]
#[doc(alias = "DOUTCLR47_32")]
pub type Doutclr47_32 = crate::Reg<doutclr47_32::Doutclr47_32Spec>;
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
pub mod doutclr47_32;
#[doc = "DOUTTGL31_0 (rw) register accessor: Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl31_0`]
module"]
#[doc(alias = "DOUTTGL31_0")]
pub type Douttgl31_0 = crate::Reg<douttgl31_0::Douttgl31_0Spec>;
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub mod douttgl31_0;
#[doc = "DOUTTGL47_32 (rw) register accessor: Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`douttgl47_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`douttgl47_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@douttgl47_32`]
module"]
#[doc(alias = "DOUTTGL47_32")]
pub type Douttgl47_32 = crate::Reg<douttgl47_32::Douttgl47_32Spec>;
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub mod douttgl47_32;
#[doc = "DIN31_0 (rw) register accessor: Data Input from DIO 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din31_0`]
module"]
#[doc(alias = "DIN31_0")]
pub type Din31_0 = crate::Reg<din31_0::Din31_0Spec>;
#[doc = "Data Input from DIO 0 to 31"]
pub mod din31_0;
#[doc = "DIN47_32 (rw) register accessor: Data Input from DIO 32 to 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`din47_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`din47_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@din47_32`]
module"]
#[doc(alias = "DIN47_32")]
pub type Din47_32 = crate::Reg<din47_32::Din47_32Spec>;
#[doc = "Data Input from DIO 32 to 47"]
pub mod din47_32;
#[doc = "DOE31_0 (rw) register accessor: Data Output Enable for DIO 0 to 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe31_0`]
module"]
#[doc(alias = "DOE31_0")]
pub type Doe31_0 = crate::Reg<doe31_0::Doe31_0Spec>;
#[doc = "Data Output Enable for DIO 0 to 31"]
pub mod doe31_0;
#[doc = "DOE47_32 (rw) register accessor: Data Output Enable for DIO 32 to 47\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`doe47_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doe47_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doe47_32`]
module"]
#[doc(alias = "DOE47_32")]
pub type Doe47_32 = crate::Reg<doe47_32::Doe47_32Spec>;
#[doc = "Data Output Enable for DIO 32 to 47"]
pub mod doe47_32;
#[doc = "EVFLAGS31_0 (rw) register accessor: Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evflags31_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflags31_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evflags31_0`]
module"]
#[doc(alias = "EVFLAGS31_0")]
pub type Evflags31_0 = crate::Reg<evflags31_0::Evflags31_0Spec>;
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub mod evflags31_0;
#[doc = "EVFLAGS47_32 (rw) register accessor: Event Register for DIO 32 to 47 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evflags47_32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evflags47_32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evflags47_32`]
module"]
#[doc(alias = "EVFLAGS47_32")]
pub type Evflags47_32 = crate::Reg<evflags47_32::Evflags47_32Spec>;
#[doc = "Event Register for DIO 32 to 47 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub mod evflags47_32;
