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
    ter: Ter,
    _reserved33: [u8; 0x3c],
    tpr: Tpr,
    _reserved34: [u8; 0x3c],
    tcr: Tcr,
    _reserved35: [u8; 0x012c],
    lar: Lar,
    lsr: Lsr,
}
impl RegisterBlock {
    #[doc = "0x00 - Stimulus Port 0"]
    #[inline(always)]
    pub const fn stim0(&self) -> &Stim0 {
        &self.stim0
    }
    #[doc = "0x04 - Stimulus Port 1"]
    #[inline(always)]
    pub const fn stim1(&self) -> &Stim1 {
        &self.stim1
    }
    #[doc = "0x08 - Stimulus Port 2"]
    #[inline(always)]
    pub const fn stim2(&self) -> &Stim2 {
        &self.stim2
    }
    #[doc = "0x0c - Stimulus Port 3"]
    #[inline(always)]
    pub const fn stim3(&self) -> &Stim3 {
        &self.stim3
    }
    #[doc = "0x10 - Stimulus Port 4"]
    #[inline(always)]
    pub const fn stim4(&self) -> &Stim4 {
        &self.stim4
    }
    #[doc = "0x14 - Stimulus Port 5"]
    #[inline(always)]
    pub const fn stim5(&self) -> &Stim5 {
        &self.stim5
    }
    #[doc = "0x18 - Stimulus Port 6"]
    #[inline(always)]
    pub const fn stim6(&self) -> &Stim6 {
        &self.stim6
    }
    #[doc = "0x1c - Stimulus Port 7"]
    #[inline(always)]
    pub const fn stim7(&self) -> &Stim7 {
        &self.stim7
    }
    #[doc = "0x20 - Stimulus Port 8"]
    #[inline(always)]
    pub const fn stim8(&self) -> &Stim8 {
        &self.stim8
    }
    #[doc = "0x24 - Stimulus Port 9"]
    #[inline(always)]
    pub const fn stim9(&self) -> &Stim9 {
        &self.stim9
    }
    #[doc = "0x28 - Stimulus Port 10"]
    #[inline(always)]
    pub const fn stim10(&self) -> &Stim10 {
        &self.stim10
    }
    #[doc = "0x2c - Stimulus Port 11"]
    #[inline(always)]
    pub const fn stim11(&self) -> &Stim11 {
        &self.stim11
    }
    #[doc = "0x30 - Stimulus Port 12"]
    #[inline(always)]
    pub const fn stim12(&self) -> &Stim12 {
        &self.stim12
    }
    #[doc = "0x34 - Stimulus Port 13"]
    #[inline(always)]
    pub const fn stim13(&self) -> &Stim13 {
        &self.stim13
    }
    #[doc = "0x38 - Stimulus Port 14"]
    #[inline(always)]
    pub const fn stim14(&self) -> &Stim14 {
        &self.stim14
    }
    #[doc = "0x3c - Stimulus Port 15"]
    #[inline(always)]
    pub const fn stim15(&self) -> &Stim15 {
        &self.stim15
    }
    #[doc = "0x40 - Stimulus Port 16"]
    #[inline(always)]
    pub const fn stim16(&self) -> &Stim16 {
        &self.stim16
    }
    #[doc = "0x44 - Stimulus Port 17"]
    #[inline(always)]
    pub const fn stim17(&self) -> &Stim17 {
        &self.stim17
    }
    #[doc = "0x48 - Stimulus Port 18"]
    #[inline(always)]
    pub const fn stim18(&self) -> &Stim18 {
        &self.stim18
    }
    #[doc = "0x4c - Stimulus Port 19"]
    #[inline(always)]
    pub const fn stim19(&self) -> &Stim19 {
        &self.stim19
    }
    #[doc = "0x50 - Stimulus Port 20"]
    #[inline(always)]
    pub const fn stim20(&self) -> &Stim20 {
        &self.stim20
    }
    #[doc = "0x54 - Stimulus Port 21"]
    #[inline(always)]
    pub const fn stim21(&self) -> &Stim21 {
        &self.stim21
    }
    #[doc = "0x58 - Stimulus Port 22"]
    #[inline(always)]
    pub const fn stim22(&self) -> &Stim22 {
        &self.stim22
    }
    #[doc = "0x5c - Stimulus Port 23"]
    #[inline(always)]
    pub const fn stim23(&self) -> &Stim23 {
        &self.stim23
    }
    #[doc = "0x60 - Stimulus Port 24"]
    #[inline(always)]
    pub const fn stim24(&self) -> &Stim24 {
        &self.stim24
    }
    #[doc = "0x64 - Stimulus Port 25"]
    #[inline(always)]
    pub const fn stim25(&self) -> &Stim25 {
        &self.stim25
    }
    #[doc = "0x68 - Stimulus Port 26"]
    #[inline(always)]
    pub const fn stim26(&self) -> &Stim26 {
        &self.stim26
    }
    #[doc = "0x6c - Stimulus Port 27"]
    #[inline(always)]
    pub const fn stim27(&self) -> &Stim27 {
        &self.stim27
    }
    #[doc = "0x70 - Stimulus Port 28"]
    #[inline(always)]
    pub const fn stim28(&self) -> &Stim28 {
        &self.stim28
    }
    #[doc = "0x74 - Stimulus Port 29"]
    #[inline(always)]
    pub const fn stim29(&self) -> &Stim29 {
        &self.stim29
    }
    #[doc = "0x78 - Stimulus Port 30"]
    #[inline(always)]
    pub const fn stim30(&self) -> &Stim30 {
        &self.stim30
    }
    #[doc = "0x7c - Stimulus Port 31"]
    #[inline(always)]
    pub const fn stim31(&self) -> &Stim31 {
        &self.stim31
    }
    #[doc = "0xe00 - Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
    #[inline(always)]
    pub const fn ter(&self) -> &Ter {
        &self.ter
    }
    #[doc = "0xe40 - Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
    #[inline(always)]
    pub const fn tpr(&self) -> &Tpr {
        &self.tpr
    }
    #[doc = "0xe80 - Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0xfb0 - Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
    #[inline(always)]
    pub const fn lar(&self) -> &Lar {
        &self.lar
    }
    #[doc = "0xfb4 - Lock Status Use this register to enable write accesses to the Control Register."]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
}
#[doc = "STIM0 (rw) register accessor: Stimulus Port 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim0`]
module"]
#[doc(alias = "STIM0")]
pub type Stim0 = crate::Reg<stim0::Stim0Spec>;
#[doc = "Stimulus Port 0"]
pub mod stim0;
#[doc = "STIM1 (rw) register accessor: Stimulus Port 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim1`]
module"]
#[doc(alias = "STIM1")]
pub type Stim1 = crate::Reg<stim1::Stim1Spec>;
#[doc = "Stimulus Port 1"]
pub mod stim1;
#[doc = "STIM2 (rw) register accessor: Stimulus Port 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim2`]
module"]
#[doc(alias = "STIM2")]
pub type Stim2 = crate::Reg<stim2::Stim2Spec>;
#[doc = "Stimulus Port 2"]
pub mod stim2;
#[doc = "STIM3 (rw) register accessor: Stimulus Port 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim3`]
module"]
#[doc(alias = "STIM3")]
pub type Stim3 = crate::Reg<stim3::Stim3Spec>;
#[doc = "Stimulus Port 3"]
pub mod stim3;
#[doc = "STIM4 (rw) register accessor: Stimulus Port 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim4`]
module"]
#[doc(alias = "STIM4")]
pub type Stim4 = crate::Reg<stim4::Stim4Spec>;
#[doc = "Stimulus Port 4"]
pub mod stim4;
#[doc = "STIM5 (rw) register accessor: Stimulus Port 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim5`]
module"]
#[doc(alias = "STIM5")]
pub type Stim5 = crate::Reg<stim5::Stim5Spec>;
#[doc = "Stimulus Port 5"]
pub mod stim5;
#[doc = "STIM6 (rw) register accessor: Stimulus Port 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim6`]
module"]
#[doc(alias = "STIM6")]
pub type Stim6 = crate::Reg<stim6::Stim6Spec>;
#[doc = "Stimulus Port 6"]
pub mod stim6;
#[doc = "STIM7 (rw) register accessor: Stimulus Port 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim7`]
module"]
#[doc(alias = "STIM7")]
pub type Stim7 = crate::Reg<stim7::Stim7Spec>;
#[doc = "Stimulus Port 7"]
pub mod stim7;
#[doc = "STIM8 (rw) register accessor: Stimulus Port 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim8`]
module"]
#[doc(alias = "STIM8")]
pub type Stim8 = crate::Reg<stim8::Stim8Spec>;
#[doc = "Stimulus Port 8"]
pub mod stim8;
#[doc = "STIM9 (rw) register accessor: Stimulus Port 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim9`]
module"]
#[doc(alias = "STIM9")]
pub type Stim9 = crate::Reg<stim9::Stim9Spec>;
#[doc = "Stimulus Port 9"]
pub mod stim9;
#[doc = "STIM10 (rw) register accessor: Stimulus Port 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim10`]
module"]
#[doc(alias = "STIM10")]
pub type Stim10 = crate::Reg<stim10::Stim10Spec>;
#[doc = "Stimulus Port 10"]
pub mod stim10;
#[doc = "STIM11 (rw) register accessor: Stimulus Port 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim11`]
module"]
#[doc(alias = "STIM11")]
pub type Stim11 = crate::Reg<stim11::Stim11Spec>;
#[doc = "Stimulus Port 11"]
pub mod stim11;
#[doc = "STIM12 (rw) register accessor: Stimulus Port 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim12`]
module"]
#[doc(alias = "STIM12")]
pub type Stim12 = crate::Reg<stim12::Stim12Spec>;
#[doc = "Stimulus Port 12"]
pub mod stim12;
#[doc = "STIM13 (rw) register accessor: Stimulus Port 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim13`]
module"]
#[doc(alias = "STIM13")]
pub type Stim13 = crate::Reg<stim13::Stim13Spec>;
#[doc = "Stimulus Port 13"]
pub mod stim13;
#[doc = "STIM14 (rw) register accessor: Stimulus Port 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim14`]
module"]
#[doc(alias = "STIM14")]
pub type Stim14 = crate::Reg<stim14::Stim14Spec>;
#[doc = "Stimulus Port 14"]
pub mod stim14;
#[doc = "STIM15 (rw) register accessor: Stimulus Port 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim15`]
module"]
#[doc(alias = "STIM15")]
pub type Stim15 = crate::Reg<stim15::Stim15Spec>;
#[doc = "Stimulus Port 15"]
pub mod stim15;
#[doc = "STIM16 (rw) register accessor: Stimulus Port 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim16`]
module"]
#[doc(alias = "STIM16")]
pub type Stim16 = crate::Reg<stim16::Stim16Spec>;
#[doc = "Stimulus Port 16"]
pub mod stim16;
#[doc = "STIM17 (rw) register accessor: Stimulus Port 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim17`]
module"]
#[doc(alias = "STIM17")]
pub type Stim17 = crate::Reg<stim17::Stim17Spec>;
#[doc = "Stimulus Port 17"]
pub mod stim17;
#[doc = "STIM18 (rw) register accessor: Stimulus Port 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim18`]
module"]
#[doc(alias = "STIM18")]
pub type Stim18 = crate::Reg<stim18::Stim18Spec>;
#[doc = "Stimulus Port 18"]
pub mod stim18;
#[doc = "STIM19 (rw) register accessor: Stimulus Port 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim19`]
module"]
#[doc(alias = "STIM19")]
pub type Stim19 = crate::Reg<stim19::Stim19Spec>;
#[doc = "Stimulus Port 19"]
pub mod stim19;
#[doc = "STIM20 (rw) register accessor: Stimulus Port 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim20`]
module"]
#[doc(alias = "STIM20")]
pub type Stim20 = crate::Reg<stim20::Stim20Spec>;
#[doc = "Stimulus Port 20"]
pub mod stim20;
#[doc = "STIM21 (rw) register accessor: Stimulus Port 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim21`]
module"]
#[doc(alias = "STIM21")]
pub type Stim21 = crate::Reg<stim21::Stim21Spec>;
#[doc = "Stimulus Port 21"]
pub mod stim21;
#[doc = "STIM22 (rw) register accessor: Stimulus Port 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim22`]
module"]
#[doc(alias = "STIM22")]
pub type Stim22 = crate::Reg<stim22::Stim22Spec>;
#[doc = "Stimulus Port 22"]
pub mod stim22;
#[doc = "STIM23 (rw) register accessor: Stimulus Port 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim23`]
module"]
#[doc(alias = "STIM23")]
pub type Stim23 = crate::Reg<stim23::Stim23Spec>;
#[doc = "Stimulus Port 23"]
pub mod stim23;
#[doc = "STIM24 (rw) register accessor: Stimulus Port 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim24`]
module"]
#[doc(alias = "STIM24")]
pub type Stim24 = crate::Reg<stim24::Stim24Spec>;
#[doc = "Stimulus Port 24"]
pub mod stim24;
#[doc = "STIM25 (rw) register accessor: Stimulus Port 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim25`]
module"]
#[doc(alias = "STIM25")]
pub type Stim25 = crate::Reg<stim25::Stim25Spec>;
#[doc = "Stimulus Port 25"]
pub mod stim25;
#[doc = "STIM26 (rw) register accessor: Stimulus Port 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim26`]
module"]
#[doc(alias = "STIM26")]
pub type Stim26 = crate::Reg<stim26::Stim26Spec>;
#[doc = "Stimulus Port 26"]
pub mod stim26;
#[doc = "STIM27 (rw) register accessor: Stimulus Port 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim27`]
module"]
#[doc(alias = "STIM27")]
pub type Stim27 = crate::Reg<stim27::Stim27Spec>;
#[doc = "Stimulus Port 27"]
pub mod stim27;
#[doc = "STIM28 (rw) register accessor: Stimulus Port 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim28`]
module"]
#[doc(alias = "STIM28")]
pub type Stim28 = crate::Reg<stim28::Stim28Spec>;
#[doc = "Stimulus Port 28"]
pub mod stim28;
#[doc = "STIM29 (rw) register accessor: Stimulus Port 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim29`]
module"]
#[doc(alias = "STIM29")]
pub type Stim29 = crate::Reg<stim29::Stim29Spec>;
#[doc = "Stimulus Port 29"]
pub mod stim29;
#[doc = "STIM30 (rw) register accessor: Stimulus Port 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim30`]
module"]
#[doc(alias = "STIM30")]
pub type Stim30 = crate::Reg<stim30::Stim30Spec>;
#[doc = "Stimulus Port 30"]
pub mod stim30;
#[doc = "STIM31 (rw) register accessor: Stimulus Port 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stim31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stim31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stim31`]
module"]
#[doc(alias = "STIM31")]
pub type Stim31 = crate::Reg<stim31::Stim31Spec>;
#[doc = "Stimulus Port 31"]
pub mod stim31;
#[doc = "TER (rw) register accessor: Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ter`]
module"]
#[doc(alias = "TER")]
pub type Ter = crate::Reg<ter::TerSpec>;
#[doc = "Trace Enable Use the Trace Enable Register to generate trace data by writing to the corresponding stimulus port. Note: Privileged writes are accepted to this register if TCR.ITMENA is set. User writes are accepted to this register if TCR.ITMENA is set and the appropriate privilege mask is cleared. Privileged access to the stimulus ports enables an RTOS kernel to guarantee instrumentation slots or bandwidth as required."]
pub mod ter;
#[doc = "TPR (rw) register accessor: Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tpr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tpr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tpr`]
module"]
#[doc(alias = "TPR")]
pub type Tpr = crate::Reg<tpr::TprSpec>;
#[doc = "Trace Privilege This register is used to enable an operating system to control which stimulus ports are accessible by user code. This register can only be used in privileged mode."]
pub mod tpr;
#[doc = "TCR (rw) register accessor: Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`]
module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Trace Control Use this register to configure and control ITM transfers. This register can only be written in privilege mode. DWT is not enabled in the ITM block. However, DWT stimulus entry into the FIFO is controlled by DWTENA. If DWT requires timestamping, the TSENA bit must be set."]
pub mod tcr;
#[doc = "LAR (rw) register accessor: Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lar`]
module"]
#[doc(alias = "LAR")]
pub type Lar = crate::Reg<lar::LarSpec>;
#[doc = "Lock Access This register is used to prevent write accesses to the Control Registers: TER, TPR and TCR."]
pub mod lar;
#[doc = "LSR (rw) register accessor: Lock Status Use this register to enable write accesses to the Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Lock Status Use this register to enable write accesses to the Control Register."]
pub mod lsr;
