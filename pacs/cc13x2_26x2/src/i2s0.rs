#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aifwclksrc: Aifwclksrc,
    aifdmacfg: Aifdmacfg,
    aifdircfg: Aifdircfg,
    aiffmtcfg: Aiffmtcfg,
    aifwmask0: Aifwmask0,
    aifwmask1: Aifwmask1,
    aifwmask2: Aifwmask2,
    aifpwmvalue: Aifpwmvalue,
    aifinptrnext: Aifinptrnext,
    aifinptr: Aifinptr,
    aifoutptrnext: Aifoutptrnext,
    aifoutptr: Aifoutptr,
    _reserved12: [u8; 0x04],
    stmpctl: Stmpctl,
    stmpxcntcapt0: Stmpxcntcapt0,
    stmpxper: Stmpxper,
    stmpwcntcapt0: Stmpwcntcapt0,
    stmpwper: Stmpwper,
    stmpintrig: Stmpintrig,
    stmpouttrig: Stmpouttrig,
    stmpwset: Stmpwset,
    stmpwadd: Stmpwadd,
    stmpxpermin: Stmpxpermin,
    stmpwcnt: Stmpwcnt,
    stmpxcnt: Stmpxcnt,
    stmpxcntcapt1: Stmpxcntcapt1,
    stmpwcntcapt1: Stmpwcntcapt1,
    _reserved26: [u8; 0x04],
    irqmask: Irqmask,
    irqflags: Irqflags,
    irqset: Irqset,
    irqclr: Irqclr,
}
impl RegisterBlock {
    #[doc = "0x00 - WCLK Source Selection"]
    #[inline(always)]
    pub const fn aifwclksrc(&self) -> &Aifwclksrc {
        &self.aifwclksrc
    }
    #[doc = "0x04 - DMA Buffer Size Configuration"]
    #[inline(always)]
    pub const fn aifdmacfg(&self) -> &Aifdmacfg {
        &self.aifdmacfg
    }
    #[doc = "0x08 - Pin Direction"]
    #[inline(always)]
    pub const fn aifdircfg(&self) -> &Aifdircfg {
        &self.aifdircfg
    }
    #[doc = "0x0c - Serial Interface Format Configuration"]
    #[inline(always)]
    pub const fn aiffmtcfg(&self) -> &Aiffmtcfg {
        &self.aiffmtcfg
    }
    #[doc = "0x10 - Word Selection Bit Mask for Pin 0"]
    #[inline(always)]
    pub const fn aifwmask0(&self) -> &Aifwmask0 {
        &self.aifwmask0
    }
    #[doc = "0x14 - Word Selection Bit Mask for Pin 1"]
    #[inline(always)]
    pub const fn aifwmask1(&self) -> &Aifwmask1 {
        &self.aifwmask1
    }
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn aifwmask2(&self) -> &Aifwmask2 {
        &self.aifwmask2
    }
    #[doc = "0x1c - Audio Interface PWM Debug Value"]
    #[inline(always)]
    pub const fn aifpwmvalue(&self) -> &Aifpwmvalue {
        &self.aifpwmvalue
    }
    #[doc = "0x20 - DMA Input Buffer Next Pointer"]
    #[inline(always)]
    pub const fn aifinptrnext(&self) -> &Aifinptrnext {
        &self.aifinptrnext
    }
    #[doc = "0x24 - DMA Input Buffer Current Pointer"]
    #[inline(always)]
    pub const fn aifinptr(&self) -> &Aifinptr {
        &self.aifinptr
    }
    #[doc = "0x28 - DMA Output Buffer Next Pointer"]
    #[inline(always)]
    pub const fn aifoutptrnext(&self) -> &Aifoutptrnext {
        &self.aifoutptrnext
    }
    #[doc = "0x2c - DMA Output Buffer Current Pointer"]
    #[inline(always)]
    pub const fn aifoutptr(&self) -> &Aifoutptr {
        &self.aifoutptr
    }
    #[doc = "0x34 - Samplestamp Generator Control Register"]
    #[inline(always)]
    pub const fn stmpctl(&self) -> &Stmpctl {
        &self.stmpctl
    }
    #[doc = "0x38 - Captured XOSC Counter Value, Capture Channel 0"]
    #[inline(always)]
    pub const fn stmpxcntcapt0(&self) -> &Stmpxcntcapt0 {
        &self.stmpxcntcapt0
    }
    #[doc = "0x3c - XOSC Period Value"]
    #[inline(always)]
    pub const fn stmpxper(&self) -> &Stmpxper {
        &self.stmpxper
    }
    #[doc = "0x40 - Captured WCLK Counter Value, Capture Channel 0"]
    #[inline(always)]
    pub const fn stmpwcntcapt0(&self) -> &Stmpwcntcapt0 {
        &self.stmpwcntcapt0
    }
    #[doc = "0x44 - WCLK Counter Period Value"]
    #[inline(always)]
    pub const fn stmpwper(&self) -> &Stmpwper {
        &self.stmpwper
    }
    #[doc = "0x48 - WCLK Counter Trigger Value for Input Pins"]
    #[inline(always)]
    pub const fn stmpintrig(&self) -> &Stmpintrig {
        &self.stmpintrig
    }
    #[doc = "0x4c - WCLK Counter Trigger Value for Output Pins"]
    #[inline(always)]
    pub const fn stmpouttrig(&self) -> &Stmpouttrig {
        &self.stmpouttrig
    }
    #[doc = "0x50 - WCLK Counter Set Operation"]
    #[inline(always)]
    pub const fn stmpwset(&self) -> &Stmpwset {
        &self.stmpwset
    }
    #[doc = "0x54 - WCLK Counter Add Operation"]
    #[inline(always)]
    pub const fn stmpwadd(&self) -> &Stmpwadd {
        &self.stmpwadd
    }
    #[doc = "0x58 - XOSC Minimum Period Value Minimum Value of STMPXPER"]
    #[inline(always)]
    pub const fn stmpxpermin(&self) -> &Stmpxpermin {
        &self.stmpxpermin
    }
    #[doc = "0x5c - Current Value of WCNT"]
    #[inline(always)]
    pub const fn stmpwcnt(&self) -> &Stmpwcnt {
        &self.stmpwcnt
    }
    #[doc = "0x60 - Current Value of XCNT"]
    #[inline(always)]
    pub const fn stmpxcnt(&self) -> &Stmpxcnt {
        &self.stmpxcnt
    }
    #[doc = "0x64 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn stmpxcntcapt1(&self) -> &Stmpxcntcapt1 {
        &self.stmpxcntcapt1
    }
    #[doc = "0x68 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn stmpwcntcapt1(&self) -> &Stmpwcntcapt1 {
        &self.stmpwcntcapt1
    }
    #[doc = "0x70 - Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
    #[inline(always)]
    pub const fn irqmask(&self) -> &Irqmask {
        &self.irqmask
    }
    #[doc = "0x74 - Raw Interrupt Status Register"]
    #[inline(always)]
    pub const fn irqflags(&self) -> &Irqflags {
        &self.irqflags
    }
    #[doc = "0x78 - Interrupt Set Register"]
    #[inline(always)]
    pub const fn irqset(&self) -> &Irqset {
        &self.irqset
    }
    #[doc = "0x7c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn irqclr(&self) -> &Irqclr {
        &self.irqclr
    }
}
#[doc = "AIFWCLKSRC (rw) register accessor: WCLK Source Selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifwclksrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifwclksrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifwclksrc`]
module"]
#[doc(alias = "AIFWCLKSRC")]
pub type Aifwclksrc = crate::Reg<aifwclksrc::AifwclksrcSpec>;
#[doc = "WCLK Source Selection"]
pub mod aifwclksrc;
#[doc = "AIFDMACFG (rw) register accessor: DMA Buffer Size Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifdmacfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifdmacfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifdmacfg`]
module"]
#[doc(alias = "AIFDMACFG")]
pub type Aifdmacfg = crate::Reg<aifdmacfg::AifdmacfgSpec>;
#[doc = "DMA Buffer Size Configuration"]
pub mod aifdmacfg;
#[doc = "AIFDIRCFG (rw) register accessor: Pin Direction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifdircfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifdircfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifdircfg`]
module"]
#[doc(alias = "AIFDIRCFG")]
pub type Aifdircfg = crate::Reg<aifdircfg::AifdircfgSpec>;
#[doc = "Pin Direction"]
pub mod aifdircfg;
#[doc = "AIFFMTCFG (rw) register accessor: Serial Interface Format Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aiffmtcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aiffmtcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aiffmtcfg`]
module"]
#[doc(alias = "AIFFMTCFG")]
pub type Aiffmtcfg = crate::Reg<aiffmtcfg::AiffmtcfgSpec>;
#[doc = "Serial Interface Format Configuration"]
pub mod aiffmtcfg;
#[doc = "AIFWMASK0 (rw) register accessor: Word Selection Bit Mask for Pin 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifwmask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifwmask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifwmask0`]
module"]
#[doc(alias = "AIFWMASK0")]
pub type Aifwmask0 = crate::Reg<aifwmask0::Aifwmask0Spec>;
#[doc = "Word Selection Bit Mask for Pin 0"]
pub mod aifwmask0;
#[doc = "AIFWMASK1 (rw) register accessor: Word Selection Bit Mask for Pin 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifwmask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifwmask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifwmask1`]
module"]
#[doc(alias = "AIFWMASK1")]
pub type Aifwmask1 = crate::Reg<aifwmask1::Aifwmask1Spec>;
#[doc = "Word Selection Bit Mask for Pin 1"]
pub mod aifwmask1;
#[doc = "AIFWMASK2 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifwmask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifwmask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifwmask2`]
module"]
#[doc(alias = "AIFWMASK2")]
pub type Aifwmask2 = crate::Reg<aifwmask2::Aifwmask2Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod aifwmask2;
#[doc = "AIFPWMVALUE (rw) register accessor: Audio Interface PWM Debug Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifpwmvalue::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifpwmvalue::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifpwmvalue`]
module"]
#[doc(alias = "AIFPWMVALUE")]
pub type Aifpwmvalue = crate::Reg<aifpwmvalue::AifpwmvalueSpec>;
#[doc = "Audio Interface PWM Debug Value"]
pub mod aifpwmvalue;
#[doc = "AIFINPTRNEXT (rw) register accessor: DMA Input Buffer Next Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifinptrnext::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifinptrnext::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifinptrnext`]
module"]
#[doc(alias = "AIFINPTRNEXT")]
pub type Aifinptrnext = crate::Reg<aifinptrnext::AifinptrnextSpec>;
#[doc = "DMA Input Buffer Next Pointer"]
pub mod aifinptrnext;
#[doc = "AIFINPTR (rw) register accessor: DMA Input Buffer Current Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifinptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifinptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifinptr`]
module"]
#[doc(alias = "AIFINPTR")]
pub type Aifinptr = crate::Reg<aifinptr::AifinptrSpec>;
#[doc = "DMA Input Buffer Current Pointer"]
pub mod aifinptr;
#[doc = "AIFOUTPTRNEXT (rw) register accessor: DMA Output Buffer Next Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifoutptrnext::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifoutptrnext::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifoutptrnext`]
module"]
#[doc(alias = "AIFOUTPTRNEXT")]
pub type Aifoutptrnext = crate::Reg<aifoutptrnext::AifoutptrnextSpec>;
#[doc = "DMA Output Buffer Next Pointer"]
pub mod aifoutptrnext;
#[doc = "AIFOUTPTR (rw) register accessor: DMA Output Buffer Current Pointer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aifoutptr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aifoutptr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aifoutptr`]
module"]
#[doc(alias = "AIFOUTPTR")]
pub type Aifoutptr = crate::Reg<aifoutptr::AifoutptrSpec>;
#[doc = "DMA Output Buffer Current Pointer"]
pub mod aifoutptr;
#[doc = "STMPCTL (rw) register accessor: Samplestamp Generator Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpctl`]
module"]
#[doc(alias = "STMPCTL")]
pub type Stmpctl = crate::Reg<stmpctl::StmpctlSpec>;
#[doc = "Samplestamp Generator Control Register"]
pub mod stmpctl;
#[doc = "STMPXCNTCAPT0 (rw) register accessor: Captured XOSC Counter Value, Capture Channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxcntcapt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxcntcapt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpxcntcapt0`]
module"]
#[doc(alias = "STMPXCNTCAPT0")]
pub type Stmpxcntcapt0 = crate::Reg<stmpxcntcapt0::Stmpxcntcapt0Spec>;
#[doc = "Captured XOSC Counter Value, Capture Channel 0"]
pub mod stmpxcntcapt0;
#[doc = "STMPXPER (rw) register accessor: XOSC Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpxper`]
module"]
#[doc(alias = "STMPXPER")]
pub type Stmpxper = crate::Reg<stmpxper::StmpxperSpec>;
#[doc = "XOSC Period Value"]
pub mod stmpxper;
#[doc = "STMPWCNTCAPT0 (rw) register accessor: Captured WCLK Counter Value, Capture Channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwcntcapt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwcntcapt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpwcntcapt0`]
module"]
#[doc(alias = "STMPWCNTCAPT0")]
pub type Stmpwcntcapt0 = crate::Reg<stmpwcntcapt0::Stmpwcntcapt0Spec>;
#[doc = "Captured WCLK Counter Value, Capture Channel 0"]
pub mod stmpwcntcapt0;
#[doc = "STMPWPER (rw) register accessor: WCLK Counter Period Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpwper`]
module"]
#[doc(alias = "STMPWPER")]
pub type Stmpwper = crate::Reg<stmpwper::StmpwperSpec>;
#[doc = "WCLK Counter Period Value"]
pub mod stmpwper;
#[doc = "STMPINTRIG (rw) register accessor: WCLK Counter Trigger Value for Input Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpintrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpintrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpintrig`]
module"]
#[doc(alias = "STMPINTRIG")]
pub type Stmpintrig = crate::Reg<stmpintrig::StmpintrigSpec>;
#[doc = "WCLK Counter Trigger Value for Input Pins"]
pub mod stmpintrig;
#[doc = "STMPOUTTRIG (rw) register accessor: WCLK Counter Trigger Value for Output Pins\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpouttrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpouttrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpouttrig`]
module"]
#[doc(alias = "STMPOUTTRIG")]
pub type Stmpouttrig = crate::Reg<stmpouttrig::StmpouttrigSpec>;
#[doc = "WCLK Counter Trigger Value for Output Pins"]
pub mod stmpouttrig;
#[doc = "STMPWSET (rw) register accessor: WCLK Counter Set Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpwset`]
module"]
#[doc(alias = "STMPWSET")]
pub type Stmpwset = crate::Reg<stmpwset::StmpwsetSpec>;
#[doc = "WCLK Counter Set Operation"]
pub mod stmpwset;
#[doc = "STMPWADD (rw) register accessor: WCLK Counter Add Operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwadd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwadd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpwadd`]
module"]
#[doc(alias = "STMPWADD")]
pub type Stmpwadd = crate::Reg<stmpwadd::StmpwaddSpec>;
#[doc = "WCLK Counter Add Operation"]
pub mod stmpwadd;
#[doc = "STMPXPERMIN (rw) register accessor: XOSC Minimum Period Value Minimum Value of STMPXPER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxpermin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxpermin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpxpermin`]
module"]
#[doc(alias = "STMPXPERMIN")]
pub type Stmpxpermin = crate::Reg<stmpxpermin::StmpxperminSpec>;
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER"]
pub mod stmpxpermin;
#[doc = "STMPWCNT (rw) register accessor: Current Value of WCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpwcnt`]
module"]
#[doc(alias = "STMPWCNT")]
pub type Stmpwcnt = crate::Reg<stmpwcnt::StmpwcntSpec>;
#[doc = "Current Value of WCNT"]
pub mod stmpwcnt;
#[doc = "STMPXCNT (rw) register accessor: Current Value of XCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpxcnt`]
module"]
#[doc(alias = "STMPXCNT")]
pub type Stmpxcnt = crate::Reg<stmpxcnt::StmpxcntSpec>;
#[doc = "Current Value of XCNT"]
pub mod stmpxcnt;
#[doc = "STMPXCNTCAPT1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpxcntcapt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpxcntcapt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpxcntcapt1`]
module"]
#[doc(alias = "STMPXCNTCAPT1")]
pub type Stmpxcntcapt1 = crate::Reg<stmpxcntcapt1::Stmpxcntcapt1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod stmpxcntcapt1;
#[doc = "STMPWCNTCAPT1 (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stmpwcntcapt1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stmpwcntcapt1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmpwcntcapt1`]
module"]
#[doc(alias = "STMPWCNTCAPT1")]
pub type Stmpwcntcapt1 = crate::Reg<stmpwcntcapt1::Stmpwcntcapt1Spec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod stmpwcntcapt1;
#[doc = "IRQMASK (rw) register accessor: Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqmask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqmask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqmask`]
module"]
#[doc(alias = "IRQMASK")]
pub type Irqmask = crate::Reg<irqmask::IrqmaskSpec>;
#[doc = "Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
pub mod irqmask;
#[doc = "IRQFLAGS (rw) register accessor: Raw Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqflags`]
module"]
#[doc(alias = "IRQFLAGS")]
pub type Irqflags = crate::Reg<irqflags::IrqflagsSpec>;
#[doc = "Raw Interrupt Status Register"]
pub mod irqflags;
#[doc = "IRQSET (rw) register accessor: Interrupt Set Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqset`]
module"]
#[doc(alias = "IRQSET")]
pub type Irqset = crate::Reg<irqset::IrqsetSpec>;
#[doc = "Interrupt Set Register"]
pub mod irqset;
#[doc = "IRQCLR (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irqclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irqclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irqclr`]
module"]
#[doc(alias = "IRQCLR")]
pub type Irqclr = crate::Reg<irqclr::IrqclrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod irqclr;
