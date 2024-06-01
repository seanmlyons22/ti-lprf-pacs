#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    smph0: Smph0,
    smph1: Smph1,
    smph2: Smph2,
    smph3: Smph3,
    smph4: Smph4,
    smph5: Smph5,
    smph6: Smph6,
    smph7: Smph7,
    smph8: Smph8,
    smph9: Smph9,
    smph10: Smph10,
    smph11: Smph11,
    smph12: Smph12,
    smph13: Smph13,
    smph14: Smph14,
    smph15: Smph15,
    smph16: Smph16,
    smph17: Smph17,
    smph18: Smph18,
    smph19: Smph19,
    smph20: Smph20,
    smph21: Smph21,
    smph22: Smph22,
    smph23: Smph23,
    smph24: Smph24,
    smph25: Smph25,
    smph26: Smph26,
    smph27: Smph27,
    smph28: Smph28,
    smph29: Smph29,
    smph30: Smph30,
    smph31: Smph31,
    _reserved32: [u8; 0x0780],
    peek0: Peek0,
    peek1: Peek1,
    peek2: Peek2,
    peek3: Peek3,
    peek4: Peek4,
    peek5: Peek5,
    peek6: Peek6,
    peek7: Peek7,
    peek8: Peek8,
    peek9: Peek9,
    peek10: Peek10,
    peek11: Peek11,
    peek12: Peek12,
    peek13: Peek13,
    peek14: Peek14,
    peek15: Peek15,
    peek16: Peek16,
    peek17: Peek17,
    peek18: Peek18,
    peek19: Peek19,
    peek20: Peek20,
    peek21: Peek21,
    peek22: Peek22,
    peek23: Peek23,
    peek24: Peek24,
    peek25: Peek25,
    peek26: Peek26,
    peek27: Peek27,
    peek28: Peek28,
    peek29: Peek29,
    peek30: Peek30,
    peek31: Peek31,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU SEMAPHORE 0"]
    #[inline(always)]
    pub const fn smph0(&self) -> &Smph0 {
        &self.smph0
    }
    #[doc = "0x04 - MCU SEMAPHORE 1"]
    #[inline(always)]
    pub const fn smph1(&self) -> &Smph1 {
        &self.smph1
    }
    #[doc = "0x08 - MCU SEMAPHORE 2"]
    #[inline(always)]
    pub const fn smph2(&self) -> &Smph2 {
        &self.smph2
    }
    #[doc = "0x0c - MCU SEMAPHORE 3"]
    #[inline(always)]
    pub const fn smph3(&self) -> &Smph3 {
        &self.smph3
    }
    #[doc = "0x10 - MCU SEMAPHORE 4"]
    #[inline(always)]
    pub const fn smph4(&self) -> &Smph4 {
        &self.smph4
    }
    #[doc = "0x14 - MCU SEMAPHORE 5"]
    #[inline(always)]
    pub const fn smph5(&self) -> &Smph5 {
        &self.smph5
    }
    #[doc = "0x18 - MCU SEMAPHORE 6"]
    #[inline(always)]
    pub const fn smph6(&self) -> &Smph6 {
        &self.smph6
    }
    #[doc = "0x1c - MCU SEMAPHORE 7"]
    #[inline(always)]
    pub const fn smph7(&self) -> &Smph7 {
        &self.smph7
    }
    #[doc = "0x20 - MCU SEMAPHORE 8"]
    #[inline(always)]
    pub const fn smph8(&self) -> &Smph8 {
        &self.smph8
    }
    #[doc = "0x24 - MCU SEMAPHORE 9"]
    #[inline(always)]
    pub const fn smph9(&self) -> &Smph9 {
        &self.smph9
    }
    #[doc = "0x28 - MCU SEMAPHORE 10"]
    #[inline(always)]
    pub const fn smph10(&self) -> &Smph10 {
        &self.smph10
    }
    #[doc = "0x2c - MCU SEMAPHORE 11"]
    #[inline(always)]
    pub const fn smph11(&self) -> &Smph11 {
        &self.smph11
    }
    #[doc = "0x30 - MCU SEMAPHORE 12"]
    #[inline(always)]
    pub const fn smph12(&self) -> &Smph12 {
        &self.smph12
    }
    #[doc = "0x34 - MCU SEMAPHORE 13"]
    #[inline(always)]
    pub const fn smph13(&self) -> &Smph13 {
        &self.smph13
    }
    #[doc = "0x38 - MCU SEMAPHORE 14"]
    #[inline(always)]
    pub const fn smph14(&self) -> &Smph14 {
        &self.smph14
    }
    #[doc = "0x3c - MCU SEMAPHORE 15"]
    #[inline(always)]
    pub const fn smph15(&self) -> &Smph15 {
        &self.smph15
    }
    #[doc = "0x40 - MCU SEMAPHORE 16"]
    #[inline(always)]
    pub const fn smph16(&self) -> &Smph16 {
        &self.smph16
    }
    #[doc = "0x44 - MCU SEMAPHORE 17"]
    #[inline(always)]
    pub const fn smph17(&self) -> &Smph17 {
        &self.smph17
    }
    #[doc = "0x48 - MCU SEMAPHORE 18"]
    #[inline(always)]
    pub const fn smph18(&self) -> &Smph18 {
        &self.smph18
    }
    #[doc = "0x4c - MCU SEMAPHORE 19"]
    #[inline(always)]
    pub const fn smph19(&self) -> &Smph19 {
        &self.smph19
    }
    #[doc = "0x50 - MCU SEMAPHORE 20"]
    #[inline(always)]
    pub const fn smph20(&self) -> &Smph20 {
        &self.smph20
    }
    #[doc = "0x54 - MCU SEMAPHORE 21"]
    #[inline(always)]
    pub const fn smph21(&self) -> &Smph21 {
        &self.smph21
    }
    #[doc = "0x58 - MCU SEMAPHORE 22"]
    #[inline(always)]
    pub const fn smph22(&self) -> &Smph22 {
        &self.smph22
    }
    #[doc = "0x5c - MCU SEMAPHORE 23"]
    #[inline(always)]
    pub const fn smph23(&self) -> &Smph23 {
        &self.smph23
    }
    #[doc = "0x60 - MCU SEMAPHORE 24"]
    #[inline(always)]
    pub const fn smph24(&self) -> &Smph24 {
        &self.smph24
    }
    #[doc = "0x64 - MCU SEMAPHORE 25"]
    #[inline(always)]
    pub const fn smph25(&self) -> &Smph25 {
        &self.smph25
    }
    #[doc = "0x68 - MCU SEMAPHORE 26"]
    #[inline(always)]
    pub const fn smph26(&self) -> &Smph26 {
        &self.smph26
    }
    #[doc = "0x6c - MCU SEMAPHORE 27"]
    #[inline(always)]
    pub const fn smph27(&self) -> &Smph27 {
        &self.smph27
    }
    #[doc = "0x70 - MCU SEMAPHORE 28"]
    #[inline(always)]
    pub const fn smph28(&self) -> &Smph28 {
        &self.smph28
    }
    #[doc = "0x74 - MCU SEMAPHORE 29"]
    #[inline(always)]
    pub const fn smph29(&self) -> &Smph29 {
        &self.smph29
    }
    #[doc = "0x78 - MCU SEMAPHORE 30"]
    #[inline(always)]
    pub const fn smph30(&self) -> &Smph30 {
        &self.smph30
    }
    #[doc = "0x7c - MCU SEMAPHORE 31"]
    #[inline(always)]
    pub const fn smph31(&self) -> &Smph31 {
        &self.smph31
    }
    #[doc = "0x800 - MCU SEMAPHORE 0 ALIAS"]
    #[inline(always)]
    pub const fn peek0(&self) -> &Peek0 {
        &self.peek0
    }
    #[doc = "0x804 - MCU SEMAPHORE 1 ALIAS"]
    #[inline(always)]
    pub const fn peek1(&self) -> &Peek1 {
        &self.peek1
    }
    #[doc = "0x808 - MCU SEMAPHORE 2 ALIAS"]
    #[inline(always)]
    pub const fn peek2(&self) -> &Peek2 {
        &self.peek2
    }
    #[doc = "0x80c - MCU SEMAPHORE 3 ALIAS"]
    #[inline(always)]
    pub const fn peek3(&self) -> &Peek3 {
        &self.peek3
    }
    #[doc = "0x810 - MCU SEMAPHORE 4 ALIAS"]
    #[inline(always)]
    pub const fn peek4(&self) -> &Peek4 {
        &self.peek4
    }
    #[doc = "0x814 - MCU SEMAPHORE 5 ALIAS"]
    #[inline(always)]
    pub const fn peek5(&self) -> &Peek5 {
        &self.peek5
    }
    #[doc = "0x818 - MCU SEMAPHORE 6 ALIAS"]
    #[inline(always)]
    pub const fn peek6(&self) -> &Peek6 {
        &self.peek6
    }
    #[doc = "0x81c - MCU SEMAPHORE 7 ALIAS"]
    #[inline(always)]
    pub const fn peek7(&self) -> &Peek7 {
        &self.peek7
    }
    #[doc = "0x820 - MCU SEMAPHORE 8 ALIAS"]
    #[inline(always)]
    pub const fn peek8(&self) -> &Peek8 {
        &self.peek8
    }
    #[doc = "0x824 - MCU SEMAPHORE 9 ALIAS"]
    #[inline(always)]
    pub const fn peek9(&self) -> &Peek9 {
        &self.peek9
    }
    #[doc = "0x828 - MCU SEMAPHORE 10 ALIAS"]
    #[inline(always)]
    pub const fn peek10(&self) -> &Peek10 {
        &self.peek10
    }
    #[doc = "0x82c - MCU SEMAPHORE 11 ALIAS"]
    #[inline(always)]
    pub const fn peek11(&self) -> &Peek11 {
        &self.peek11
    }
    #[doc = "0x830 - MCU SEMAPHORE 12 ALIAS"]
    #[inline(always)]
    pub const fn peek12(&self) -> &Peek12 {
        &self.peek12
    }
    #[doc = "0x834 - MCU SEMAPHORE 13 ALIAS"]
    #[inline(always)]
    pub const fn peek13(&self) -> &Peek13 {
        &self.peek13
    }
    #[doc = "0x838 - MCU SEMAPHORE 14 ALIAS"]
    #[inline(always)]
    pub const fn peek14(&self) -> &Peek14 {
        &self.peek14
    }
    #[doc = "0x83c - MCU SEMAPHORE 15 ALIAS"]
    #[inline(always)]
    pub const fn peek15(&self) -> &Peek15 {
        &self.peek15
    }
    #[doc = "0x840 - MCU SEMAPHORE 16 ALIAS"]
    #[inline(always)]
    pub const fn peek16(&self) -> &Peek16 {
        &self.peek16
    }
    #[doc = "0x844 - MCU SEMAPHORE 17 ALIAS"]
    #[inline(always)]
    pub const fn peek17(&self) -> &Peek17 {
        &self.peek17
    }
    #[doc = "0x848 - MCU SEMAPHORE 18 ALIAS"]
    #[inline(always)]
    pub const fn peek18(&self) -> &Peek18 {
        &self.peek18
    }
    #[doc = "0x84c - MCU SEMAPHORE 19 ALIAS"]
    #[inline(always)]
    pub const fn peek19(&self) -> &Peek19 {
        &self.peek19
    }
    #[doc = "0x850 - MCU SEMAPHORE 20 ALIAS"]
    #[inline(always)]
    pub const fn peek20(&self) -> &Peek20 {
        &self.peek20
    }
    #[doc = "0x854 - MCU SEMAPHORE 21 ALIAS"]
    #[inline(always)]
    pub const fn peek21(&self) -> &Peek21 {
        &self.peek21
    }
    #[doc = "0x858 - MCU SEMAPHORE 22 ALIAS"]
    #[inline(always)]
    pub const fn peek22(&self) -> &Peek22 {
        &self.peek22
    }
    #[doc = "0x85c - MCU SEMAPHORE 23 ALIAS"]
    #[inline(always)]
    pub const fn peek23(&self) -> &Peek23 {
        &self.peek23
    }
    #[doc = "0x860 - MCU SEMAPHORE 24 ALIAS"]
    #[inline(always)]
    pub const fn peek24(&self) -> &Peek24 {
        &self.peek24
    }
    #[doc = "0x864 - MCU SEMAPHORE 25 ALIAS"]
    #[inline(always)]
    pub const fn peek25(&self) -> &Peek25 {
        &self.peek25
    }
    #[doc = "0x868 - MCU SEMAPHORE 26 ALIAS"]
    #[inline(always)]
    pub const fn peek26(&self) -> &Peek26 {
        &self.peek26
    }
    #[doc = "0x86c - MCU SEMAPHORE 27 ALIAS"]
    #[inline(always)]
    pub const fn peek27(&self) -> &Peek27 {
        &self.peek27
    }
    #[doc = "0x870 - MCU SEMAPHORE 28 ALIAS"]
    #[inline(always)]
    pub const fn peek28(&self) -> &Peek28 {
        &self.peek28
    }
    #[doc = "0x874 - MCU SEMAPHORE 29 ALIAS"]
    #[inline(always)]
    pub const fn peek29(&self) -> &Peek29 {
        &self.peek29
    }
    #[doc = "0x878 - MCU SEMAPHORE 30 ALIAS"]
    #[inline(always)]
    pub const fn peek30(&self) -> &Peek30 {
        &self.peek30
    }
    #[doc = "0x87c - MCU SEMAPHORE 31 ALIAS"]
    #[inline(always)]
    pub const fn peek31(&self) -> &Peek31 {
        &self.peek31
    }
}
#[doc = "SMPH0 (rw) register accessor: MCU SEMAPHORE 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph0`]
module"]
#[doc(alias = "SMPH0")]
pub type Smph0 = crate::Reg<smph0::Smph0Spec>;
#[doc = "MCU SEMAPHORE 0"]
pub mod smph0;
#[doc = "SMPH1 (rw) register accessor: MCU SEMAPHORE 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph1`]
module"]
#[doc(alias = "SMPH1")]
pub type Smph1 = crate::Reg<smph1::Smph1Spec>;
#[doc = "MCU SEMAPHORE 1"]
pub mod smph1;
#[doc = "SMPH2 (rw) register accessor: MCU SEMAPHORE 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph2`]
module"]
#[doc(alias = "SMPH2")]
pub type Smph2 = crate::Reg<smph2::Smph2Spec>;
#[doc = "MCU SEMAPHORE 2"]
pub mod smph2;
#[doc = "SMPH3 (rw) register accessor: MCU SEMAPHORE 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph3`]
module"]
#[doc(alias = "SMPH3")]
pub type Smph3 = crate::Reg<smph3::Smph3Spec>;
#[doc = "MCU SEMAPHORE 3"]
pub mod smph3;
#[doc = "SMPH4 (rw) register accessor: MCU SEMAPHORE 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph4`]
module"]
#[doc(alias = "SMPH4")]
pub type Smph4 = crate::Reg<smph4::Smph4Spec>;
#[doc = "MCU SEMAPHORE 4"]
pub mod smph4;
#[doc = "SMPH5 (rw) register accessor: MCU SEMAPHORE 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph5`]
module"]
#[doc(alias = "SMPH5")]
pub type Smph5 = crate::Reg<smph5::Smph5Spec>;
#[doc = "MCU SEMAPHORE 5"]
pub mod smph5;
#[doc = "SMPH6 (rw) register accessor: MCU SEMAPHORE 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph6`]
module"]
#[doc(alias = "SMPH6")]
pub type Smph6 = crate::Reg<smph6::Smph6Spec>;
#[doc = "MCU SEMAPHORE 6"]
pub mod smph6;
#[doc = "SMPH7 (rw) register accessor: MCU SEMAPHORE 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph7`]
module"]
#[doc(alias = "SMPH7")]
pub type Smph7 = crate::Reg<smph7::Smph7Spec>;
#[doc = "MCU SEMAPHORE 7"]
pub mod smph7;
#[doc = "SMPH8 (rw) register accessor: MCU SEMAPHORE 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph8`]
module"]
#[doc(alias = "SMPH8")]
pub type Smph8 = crate::Reg<smph8::Smph8Spec>;
#[doc = "MCU SEMAPHORE 8"]
pub mod smph8;
#[doc = "SMPH9 (rw) register accessor: MCU SEMAPHORE 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph9`]
module"]
#[doc(alias = "SMPH9")]
pub type Smph9 = crate::Reg<smph9::Smph9Spec>;
#[doc = "MCU SEMAPHORE 9"]
pub mod smph9;
#[doc = "SMPH10 (rw) register accessor: MCU SEMAPHORE 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph10`]
module"]
#[doc(alias = "SMPH10")]
pub type Smph10 = crate::Reg<smph10::Smph10Spec>;
#[doc = "MCU SEMAPHORE 10"]
pub mod smph10;
#[doc = "SMPH11 (rw) register accessor: MCU SEMAPHORE 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph11`]
module"]
#[doc(alias = "SMPH11")]
pub type Smph11 = crate::Reg<smph11::Smph11Spec>;
#[doc = "MCU SEMAPHORE 11"]
pub mod smph11;
#[doc = "SMPH12 (rw) register accessor: MCU SEMAPHORE 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph12`]
module"]
#[doc(alias = "SMPH12")]
pub type Smph12 = crate::Reg<smph12::Smph12Spec>;
#[doc = "MCU SEMAPHORE 12"]
pub mod smph12;
#[doc = "SMPH13 (rw) register accessor: MCU SEMAPHORE 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph13`]
module"]
#[doc(alias = "SMPH13")]
pub type Smph13 = crate::Reg<smph13::Smph13Spec>;
#[doc = "MCU SEMAPHORE 13"]
pub mod smph13;
#[doc = "SMPH14 (rw) register accessor: MCU SEMAPHORE 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph14`]
module"]
#[doc(alias = "SMPH14")]
pub type Smph14 = crate::Reg<smph14::Smph14Spec>;
#[doc = "MCU SEMAPHORE 14"]
pub mod smph14;
#[doc = "SMPH15 (rw) register accessor: MCU SEMAPHORE 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph15`]
module"]
#[doc(alias = "SMPH15")]
pub type Smph15 = crate::Reg<smph15::Smph15Spec>;
#[doc = "MCU SEMAPHORE 15"]
pub mod smph15;
#[doc = "SMPH16 (rw) register accessor: MCU SEMAPHORE 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph16`]
module"]
#[doc(alias = "SMPH16")]
pub type Smph16 = crate::Reg<smph16::Smph16Spec>;
#[doc = "MCU SEMAPHORE 16"]
pub mod smph16;
#[doc = "SMPH17 (rw) register accessor: MCU SEMAPHORE 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph17`]
module"]
#[doc(alias = "SMPH17")]
pub type Smph17 = crate::Reg<smph17::Smph17Spec>;
#[doc = "MCU SEMAPHORE 17"]
pub mod smph17;
#[doc = "SMPH18 (rw) register accessor: MCU SEMAPHORE 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph18`]
module"]
#[doc(alias = "SMPH18")]
pub type Smph18 = crate::Reg<smph18::Smph18Spec>;
#[doc = "MCU SEMAPHORE 18"]
pub mod smph18;
#[doc = "SMPH19 (rw) register accessor: MCU SEMAPHORE 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph19`]
module"]
#[doc(alias = "SMPH19")]
pub type Smph19 = crate::Reg<smph19::Smph19Spec>;
#[doc = "MCU SEMAPHORE 19"]
pub mod smph19;
#[doc = "SMPH20 (rw) register accessor: MCU SEMAPHORE 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph20`]
module"]
#[doc(alias = "SMPH20")]
pub type Smph20 = crate::Reg<smph20::Smph20Spec>;
#[doc = "MCU SEMAPHORE 20"]
pub mod smph20;
#[doc = "SMPH21 (rw) register accessor: MCU SEMAPHORE 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph21`]
module"]
#[doc(alias = "SMPH21")]
pub type Smph21 = crate::Reg<smph21::Smph21Spec>;
#[doc = "MCU SEMAPHORE 21"]
pub mod smph21;
#[doc = "SMPH22 (rw) register accessor: MCU SEMAPHORE 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph22`]
module"]
#[doc(alias = "SMPH22")]
pub type Smph22 = crate::Reg<smph22::Smph22Spec>;
#[doc = "MCU SEMAPHORE 22"]
pub mod smph22;
#[doc = "SMPH23 (rw) register accessor: MCU SEMAPHORE 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph23`]
module"]
#[doc(alias = "SMPH23")]
pub type Smph23 = crate::Reg<smph23::Smph23Spec>;
#[doc = "MCU SEMAPHORE 23"]
pub mod smph23;
#[doc = "SMPH24 (rw) register accessor: MCU SEMAPHORE 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph24`]
module"]
#[doc(alias = "SMPH24")]
pub type Smph24 = crate::Reg<smph24::Smph24Spec>;
#[doc = "MCU SEMAPHORE 24"]
pub mod smph24;
#[doc = "SMPH25 (rw) register accessor: MCU SEMAPHORE 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph25`]
module"]
#[doc(alias = "SMPH25")]
pub type Smph25 = crate::Reg<smph25::Smph25Spec>;
#[doc = "MCU SEMAPHORE 25"]
pub mod smph25;
#[doc = "SMPH26 (rw) register accessor: MCU SEMAPHORE 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph26`]
module"]
#[doc(alias = "SMPH26")]
pub type Smph26 = crate::Reg<smph26::Smph26Spec>;
#[doc = "MCU SEMAPHORE 26"]
pub mod smph26;
#[doc = "SMPH27 (rw) register accessor: MCU SEMAPHORE 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph27`]
module"]
#[doc(alias = "SMPH27")]
pub type Smph27 = crate::Reg<smph27::Smph27Spec>;
#[doc = "MCU SEMAPHORE 27"]
pub mod smph27;
#[doc = "SMPH28 (rw) register accessor: MCU SEMAPHORE 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph28`]
module"]
#[doc(alias = "SMPH28")]
pub type Smph28 = crate::Reg<smph28::Smph28Spec>;
#[doc = "MCU SEMAPHORE 28"]
pub mod smph28;
#[doc = "SMPH29 (rw) register accessor: MCU SEMAPHORE 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph29`]
module"]
#[doc(alias = "SMPH29")]
pub type Smph29 = crate::Reg<smph29::Smph29Spec>;
#[doc = "MCU SEMAPHORE 29"]
pub mod smph29;
#[doc = "SMPH30 (rw) register accessor: MCU SEMAPHORE 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph30`]
module"]
#[doc(alias = "SMPH30")]
pub type Smph30 = crate::Reg<smph30::Smph30Spec>;
#[doc = "MCU SEMAPHORE 30"]
pub mod smph30;
#[doc = "SMPH31 (rw) register accessor: MCU SEMAPHORE 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smph31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smph31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smph31`]
module"]
#[doc(alias = "SMPH31")]
pub type Smph31 = crate::Reg<smph31::Smph31Spec>;
#[doc = "MCU SEMAPHORE 31"]
pub mod smph31;
#[doc = "PEEK0 (rw) register accessor: MCU SEMAPHORE 0 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek0`]
module"]
#[doc(alias = "PEEK0")]
pub type Peek0 = crate::Reg<peek0::Peek0Spec>;
#[doc = "MCU SEMAPHORE 0 ALIAS"]
pub mod peek0;
#[doc = "PEEK1 (rw) register accessor: MCU SEMAPHORE 1 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek1`]
module"]
#[doc(alias = "PEEK1")]
pub type Peek1 = crate::Reg<peek1::Peek1Spec>;
#[doc = "MCU SEMAPHORE 1 ALIAS"]
pub mod peek1;
#[doc = "PEEK2 (rw) register accessor: MCU SEMAPHORE 2 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek2`]
module"]
#[doc(alias = "PEEK2")]
pub type Peek2 = crate::Reg<peek2::Peek2Spec>;
#[doc = "MCU SEMAPHORE 2 ALIAS"]
pub mod peek2;
#[doc = "PEEK3 (rw) register accessor: MCU SEMAPHORE 3 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek3`]
module"]
#[doc(alias = "PEEK3")]
pub type Peek3 = crate::Reg<peek3::Peek3Spec>;
#[doc = "MCU SEMAPHORE 3 ALIAS"]
pub mod peek3;
#[doc = "PEEK4 (rw) register accessor: MCU SEMAPHORE 4 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek4`]
module"]
#[doc(alias = "PEEK4")]
pub type Peek4 = crate::Reg<peek4::Peek4Spec>;
#[doc = "MCU SEMAPHORE 4 ALIAS"]
pub mod peek4;
#[doc = "PEEK5 (rw) register accessor: MCU SEMAPHORE 5 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek5`]
module"]
#[doc(alias = "PEEK5")]
pub type Peek5 = crate::Reg<peek5::Peek5Spec>;
#[doc = "MCU SEMAPHORE 5 ALIAS"]
pub mod peek5;
#[doc = "PEEK6 (rw) register accessor: MCU SEMAPHORE 6 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek6`]
module"]
#[doc(alias = "PEEK6")]
pub type Peek6 = crate::Reg<peek6::Peek6Spec>;
#[doc = "MCU SEMAPHORE 6 ALIAS"]
pub mod peek6;
#[doc = "PEEK7 (rw) register accessor: MCU SEMAPHORE 7 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek7`]
module"]
#[doc(alias = "PEEK7")]
pub type Peek7 = crate::Reg<peek7::Peek7Spec>;
#[doc = "MCU SEMAPHORE 7 ALIAS"]
pub mod peek7;
#[doc = "PEEK8 (rw) register accessor: MCU SEMAPHORE 8 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek8`]
module"]
#[doc(alias = "PEEK8")]
pub type Peek8 = crate::Reg<peek8::Peek8Spec>;
#[doc = "MCU SEMAPHORE 8 ALIAS"]
pub mod peek8;
#[doc = "PEEK9 (rw) register accessor: MCU SEMAPHORE 9 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek9`]
module"]
#[doc(alias = "PEEK9")]
pub type Peek9 = crate::Reg<peek9::Peek9Spec>;
#[doc = "MCU SEMAPHORE 9 ALIAS"]
pub mod peek9;
#[doc = "PEEK10 (rw) register accessor: MCU SEMAPHORE 10 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek10`]
module"]
#[doc(alias = "PEEK10")]
pub type Peek10 = crate::Reg<peek10::Peek10Spec>;
#[doc = "MCU SEMAPHORE 10 ALIAS"]
pub mod peek10;
#[doc = "PEEK11 (rw) register accessor: MCU SEMAPHORE 11 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek11`]
module"]
#[doc(alias = "PEEK11")]
pub type Peek11 = crate::Reg<peek11::Peek11Spec>;
#[doc = "MCU SEMAPHORE 11 ALIAS"]
pub mod peek11;
#[doc = "PEEK12 (rw) register accessor: MCU SEMAPHORE 12 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek12`]
module"]
#[doc(alias = "PEEK12")]
pub type Peek12 = crate::Reg<peek12::Peek12Spec>;
#[doc = "MCU SEMAPHORE 12 ALIAS"]
pub mod peek12;
#[doc = "PEEK13 (rw) register accessor: MCU SEMAPHORE 13 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek13`]
module"]
#[doc(alias = "PEEK13")]
pub type Peek13 = crate::Reg<peek13::Peek13Spec>;
#[doc = "MCU SEMAPHORE 13 ALIAS"]
pub mod peek13;
#[doc = "PEEK14 (rw) register accessor: MCU SEMAPHORE 14 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek14`]
module"]
#[doc(alias = "PEEK14")]
pub type Peek14 = crate::Reg<peek14::Peek14Spec>;
#[doc = "MCU SEMAPHORE 14 ALIAS"]
pub mod peek14;
#[doc = "PEEK15 (rw) register accessor: MCU SEMAPHORE 15 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek15`]
module"]
#[doc(alias = "PEEK15")]
pub type Peek15 = crate::Reg<peek15::Peek15Spec>;
#[doc = "MCU SEMAPHORE 15 ALIAS"]
pub mod peek15;
#[doc = "PEEK16 (rw) register accessor: MCU SEMAPHORE 16 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek16`]
module"]
#[doc(alias = "PEEK16")]
pub type Peek16 = crate::Reg<peek16::Peek16Spec>;
#[doc = "MCU SEMAPHORE 16 ALIAS"]
pub mod peek16;
#[doc = "PEEK17 (rw) register accessor: MCU SEMAPHORE 17 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek17`]
module"]
#[doc(alias = "PEEK17")]
pub type Peek17 = crate::Reg<peek17::Peek17Spec>;
#[doc = "MCU SEMAPHORE 17 ALIAS"]
pub mod peek17;
#[doc = "PEEK18 (rw) register accessor: MCU SEMAPHORE 18 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek18`]
module"]
#[doc(alias = "PEEK18")]
pub type Peek18 = crate::Reg<peek18::Peek18Spec>;
#[doc = "MCU SEMAPHORE 18 ALIAS"]
pub mod peek18;
#[doc = "PEEK19 (rw) register accessor: MCU SEMAPHORE 19 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek19`]
module"]
#[doc(alias = "PEEK19")]
pub type Peek19 = crate::Reg<peek19::Peek19Spec>;
#[doc = "MCU SEMAPHORE 19 ALIAS"]
pub mod peek19;
#[doc = "PEEK20 (rw) register accessor: MCU SEMAPHORE 20 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek20`]
module"]
#[doc(alias = "PEEK20")]
pub type Peek20 = crate::Reg<peek20::Peek20Spec>;
#[doc = "MCU SEMAPHORE 20 ALIAS"]
pub mod peek20;
#[doc = "PEEK21 (rw) register accessor: MCU SEMAPHORE 21 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek21`]
module"]
#[doc(alias = "PEEK21")]
pub type Peek21 = crate::Reg<peek21::Peek21Spec>;
#[doc = "MCU SEMAPHORE 21 ALIAS"]
pub mod peek21;
#[doc = "PEEK22 (rw) register accessor: MCU SEMAPHORE 22 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek22`]
module"]
#[doc(alias = "PEEK22")]
pub type Peek22 = crate::Reg<peek22::Peek22Spec>;
#[doc = "MCU SEMAPHORE 22 ALIAS"]
pub mod peek22;
#[doc = "PEEK23 (rw) register accessor: MCU SEMAPHORE 23 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek23`]
module"]
#[doc(alias = "PEEK23")]
pub type Peek23 = crate::Reg<peek23::Peek23Spec>;
#[doc = "MCU SEMAPHORE 23 ALIAS"]
pub mod peek23;
#[doc = "PEEK24 (rw) register accessor: MCU SEMAPHORE 24 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek24`]
module"]
#[doc(alias = "PEEK24")]
pub type Peek24 = crate::Reg<peek24::Peek24Spec>;
#[doc = "MCU SEMAPHORE 24 ALIAS"]
pub mod peek24;
#[doc = "PEEK25 (rw) register accessor: MCU SEMAPHORE 25 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek25`]
module"]
#[doc(alias = "PEEK25")]
pub type Peek25 = crate::Reg<peek25::Peek25Spec>;
#[doc = "MCU SEMAPHORE 25 ALIAS"]
pub mod peek25;
#[doc = "PEEK26 (rw) register accessor: MCU SEMAPHORE 26 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek26`]
module"]
#[doc(alias = "PEEK26")]
pub type Peek26 = crate::Reg<peek26::Peek26Spec>;
#[doc = "MCU SEMAPHORE 26 ALIAS"]
pub mod peek26;
#[doc = "PEEK27 (rw) register accessor: MCU SEMAPHORE 27 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek27`]
module"]
#[doc(alias = "PEEK27")]
pub type Peek27 = crate::Reg<peek27::Peek27Spec>;
#[doc = "MCU SEMAPHORE 27 ALIAS"]
pub mod peek27;
#[doc = "PEEK28 (rw) register accessor: MCU SEMAPHORE 28 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek28`]
module"]
#[doc(alias = "PEEK28")]
pub type Peek28 = crate::Reg<peek28::Peek28Spec>;
#[doc = "MCU SEMAPHORE 28 ALIAS"]
pub mod peek28;
#[doc = "PEEK29 (rw) register accessor: MCU SEMAPHORE 29 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek29`]
module"]
#[doc(alias = "PEEK29")]
pub type Peek29 = crate::Reg<peek29::Peek29Spec>;
#[doc = "MCU SEMAPHORE 29 ALIAS"]
pub mod peek29;
#[doc = "PEEK30 (rw) register accessor: MCU SEMAPHORE 30 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek30`]
module"]
#[doc(alias = "PEEK30")]
pub type Peek30 = crate::Reg<peek30::Peek30Spec>;
#[doc = "MCU SEMAPHORE 30 ALIAS"]
pub mod peek30;
#[doc = "PEEK31 (rw) register accessor: MCU SEMAPHORE 31 ALIAS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peek31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peek31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@peek31`]
module"]
#[doc(alias = "PEEK31")]
pub type Peek31 = crate::Reg<peek31::Peek31Spec>;
#[doc = "MCU SEMAPHORE 31 ALIAS"]
pub mod peek31;
