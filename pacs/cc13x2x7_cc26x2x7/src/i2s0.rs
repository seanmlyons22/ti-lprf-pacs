#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - WCLK Source Selection"]
    pub aifwclksrc: AIFWCLKSRC,
    #[doc = "0x04 - DMA Buffer Size Configuration"]
    pub aifdmacfg: AIFDMACFG,
    #[doc = "0x08 - Pin Direction"]
    pub aifdircfg: AIFDIRCFG,
    #[doc = "0x0c - Serial Interface Format Configuration"]
    pub aiffmtcfg: AIFFMTCFG,
    #[doc = "0x10 - Word Selection Bit Mask for Pin 0"]
    pub aifwmask0: AIFWMASK0,
    #[doc = "0x14 - Word Selection Bit Mask for Pin 1"]
    pub aifwmask1: AIFWMASK1,
    #[doc = "0x18 - Internal. Only to be used through TI provided API."]
    pub aifwmask2: AIFWMASK2,
    #[doc = "0x1c - Audio Interface PWM Debug Value"]
    pub aifpwmvalue: AIFPWMVALUE,
    #[doc = "0x20 - DMA Input Buffer Next Pointer"]
    pub aifinptrnext: AIFINPTRNEXT,
    #[doc = "0x24 - DMA Input Buffer Current Pointer"]
    pub aifinptr: AIFINPTR,
    #[doc = "0x28 - DMA Output Buffer Next Pointer"]
    pub aifoutptrnext: AIFOUTPTRNEXT,
    #[doc = "0x2c - DMA Output Buffer Current Pointer"]
    pub aifoutptr: AIFOUTPTR,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Samplestamp Generator Control Register"]
    pub stmpctl: STMPCTL,
    #[doc = "0x38 - Captured XOSC Counter Value, Capture Channel 0"]
    pub stmpxcntcapt0: STMPXCNTCAPT0,
    #[doc = "0x3c - XOSC Period Value"]
    pub stmpxper: STMPXPER,
    #[doc = "0x40 - Captured WCLK Counter Value, Capture Channel 0"]
    pub stmpwcntcapt0: STMPWCNTCAPT0,
    #[doc = "0x44 - WCLK Counter Period Value"]
    pub stmpwper: STMPWPER,
    #[doc = "0x48 - WCLK Counter Trigger Value for Input Pins"]
    pub stmpintrig: STMPINTRIG,
    #[doc = "0x4c - WCLK Counter Trigger Value for Output Pins"]
    pub stmpouttrig: STMPOUTTRIG,
    #[doc = "0x50 - WCLK Counter Set Operation"]
    pub stmpwset: STMPWSET,
    #[doc = "0x54 - WCLK Counter Add Operation"]
    pub stmpwadd: STMPWADD,
    #[doc = "0x58 - XOSC Minimum Period Value Minimum Value of STMPXPER"]
    pub stmpxpermin: STMPXPERMIN,
    #[doc = "0x5c - Current Value of WCNT"]
    pub stmpwcnt: STMPWCNT,
    #[doc = "0x60 - Current Value of XCNT"]
    pub stmpxcnt: STMPXCNT,
    #[doc = "0x64 - Internal. Only to be used through TI provided API."]
    pub stmpxcntcapt1: STMPXCNTCAPT1,
    #[doc = "0x68 - Internal. Only to be used through TI provided API."]
    pub stmpwcntcapt1: STMPWCNTCAPT1,
    _reserved26: [u8; 0x04],
    #[doc = "0x70 - Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
    pub irqmask: IRQMASK,
    #[doc = "0x74 - Raw Interrupt Status Register"]
    pub irqflags: IRQFLAGS,
    #[doc = "0x78 - Interrupt Set Register"]
    pub irqset: IRQSET,
    #[doc = "0x7c - Interrupt Clear Register"]
    pub irqclr: IRQCLR,
}
#[doc = "AIFWCLKSRC (rw) register accessor: an alias for `Reg<AIFWCLKSRC_SPEC>`"]
pub type AIFWCLKSRC = crate::Reg<aifwclksrc::AIFWCLKSRC_SPEC>;
#[doc = "WCLK Source Selection"]
pub mod aifwclksrc;
#[doc = "AIFDMACFG (rw) register accessor: an alias for `Reg<AIFDMACFG_SPEC>`"]
pub type AIFDMACFG = crate::Reg<aifdmacfg::AIFDMACFG_SPEC>;
#[doc = "DMA Buffer Size Configuration"]
pub mod aifdmacfg;
#[doc = "AIFDIRCFG (rw) register accessor: an alias for `Reg<AIFDIRCFG_SPEC>`"]
pub type AIFDIRCFG = crate::Reg<aifdircfg::AIFDIRCFG_SPEC>;
#[doc = "Pin Direction"]
pub mod aifdircfg;
#[doc = "AIFFMTCFG (rw) register accessor: an alias for `Reg<AIFFMTCFG_SPEC>`"]
pub type AIFFMTCFG = crate::Reg<aiffmtcfg::AIFFMTCFG_SPEC>;
#[doc = "Serial Interface Format Configuration"]
pub mod aiffmtcfg;
#[doc = "AIFWMASK0 (rw) register accessor: an alias for `Reg<AIFWMASK0_SPEC>`"]
pub type AIFWMASK0 = crate::Reg<aifwmask0::AIFWMASK0_SPEC>;
#[doc = "Word Selection Bit Mask for Pin 0"]
pub mod aifwmask0;
#[doc = "AIFWMASK1 (rw) register accessor: an alias for `Reg<AIFWMASK1_SPEC>`"]
pub type AIFWMASK1 = crate::Reg<aifwmask1::AIFWMASK1_SPEC>;
#[doc = "Word Selection Bit Mask for Pin 1"]
pub mod aifwmask1;
#[doc = "AIFWMASK2 (rw) register accessor: an alias for `Reg<AIFWMASK2_SPEC>`"]
pub type AIFWMASK2 = crate::Reg<aifwmask2::AIFWMASK2_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod aifwmask2;
#[doc = "AIFPWMVALUE (rw) register accessor: an alias for `Reg<AIFPWMVALUE_SPEC>`"]
pub type AIFPWMVALUE = crate::Reg<aifpwmvalue::AIFPWMVALUE_SPEC>;
#[doc = "Audio Interface PWM Debug Value"]
pub mod aifpwmvalue;
#[doc = "AIFINPTRNEXT (rw) register accessor: an alias for `Reg<AIFINPTRNEXT_SPEC>`"]
pub type AIFINPTRNEXT = crate::Reg<aifinptrnext::AIFINPTRNEXT_SPEC>;
#[doc = "DMA Input Buffer Next Pointer"]
pub mod aifinptrnext;
#[doc = "AIFINPTR (rw) register accessor: an alias for `Reg<AIFINPTR_SPEC>`"]
pub type AIFINPTR = crate::Reg<aifinptr::AIFINPTR_SPEC>;
#[doc = "DMA Input Buffer Current Pointer"]
pub mod aifinptr;
#[doc = "AIFOUTPTRNEXT (rw) register accessor: an alias for `Reg<AIFOUTPTRNEXT_SPEC>`"]
pub type AIFOUTPTRNEXT = crate::Reg<aifoutptrnext::AIFOUTPTRNEXT_SPEC>;
#[doc = "DMA Output Buffer Next Pointer"]
pub mod aifoutptrnext;
#[doc = "AIFOUTPTR (rw) register accessor: an alias for `Reg<AIFOUTPTR_SPEC>`"]
pub type AIFOUTPTR = crate::Reg<aifoutptr::AIFOUTPTR_SPEC>;
#[doc = "DMA Output Buffer Current Pointer"]
pub mod aifoutptr;
#[doc = "STMPCTL (rw) register accessor: an alias for `Reg<STMPCTL_SPEC>`"]
pub type STMPCTL = crate::Reg<stmpctl::STMPCTL_SPEC>;
#[doc = "Samplestamp Generator Control Register"]
pub mod stmpctl;
#[doc = "STMPXCNTCAPT0 (rw) register accessor: an alias for `Reg<STMPXCNTCAPT0_SPEC>`"]
pub type STMPXCNTCAPT0 = crate::Reg<stmpxcntcapt0::STMPXCNTCAPT0_SPEC>;
#[doc = "Captured XOSC Counter Value, Capture Channel 0"]
pub mod stmpxcntcapt0;
#[doc = "STMPXPER (rw) register accessor: an alias for `Reg<STMPXPER_SPEC>`"]
pub type STMPXPER = crate::Reg<stmpxper::STMPXPER_SPEC>;
#[doc = "XOSC Period Value"]
pub mod stmpxper;
#[doc = "STMPWCNTCAPT0 (rw) register accessor: an alias for `Reg<STMPWCNTCAPT0_SPEC>`"]
pub type STMPWCNTCAPT0 = crate::Reg<stmpwcntcapt0::STMPWCNTCAPT0_SPEC>;
#[doc = "Captured WCLK Counter Value, Capture Channel 0"]
pub mod stmpwcntcapt0;
#[doc = "STMPWPER (rw) register accessor: an alias for `Reg<STMPWPER_SPEC>`"]
pub type STMPWPER = crate::Reg<stmpwper::STMPWPER_SPEC>;
#[doc = "WCLK Counter Period Value"]
pub mod stmpwper;
#[doc = "STMPINTRIG (rw) register accessor: an alias for `Reg<STMPINTRIG_SPEC>`"]
pub type STMPINTRIG = crate::Reg<stmpintrig::STMPINTRIG_SPEC>;
#[doc = "WCLK Counter Trigger Value for Input Pins"]
pub mod stmpintrig;
#[doc = "STMPOUTTRIG (rw) register accessor: an alias for `Reg<STMPOUTTRIG_SPEC>`"]
pub type STMPOUTTRIG = crate::Reg<stmpouttrig::STMPOUTTRIG_SPEC>;
#[doc = "WCLK Counter Trigger Value for Output Pins"]
pub mod stmpouttrig;
#[doc = "STMPWSET (rw) register accessor: an alias for `Reg<STMPWSET_SPEC>`"]
pub type STMPWSET = crate::Reg<stmpwset::STMPWSET_SPEC>;
#[doc = "WCLK Counter Set Operation"]
pub mod stmpwset;
#[doc = "STMPWADD (rw) register accessor: an alias for `Reg<STMPWADD_SPEC>`"]
pub type STMPWADD = crate::Reg<stmpwadd::STMPWADD_SPEC>;
#[doc = "WCLK Counter Add Operation"]
pub mod stmpwadd;
#[doc = "STMPXPERMIN (rw) register accessor: an alias for `Reg<STMPXPERMIN_SPEC>`"]
pub type STMPXPERMIN = crate::Reg<stmpxpermin::STMPXPERMIN_SPEC>;
#[doc = "XOSC Minimum Period Value Minimum Value of STMPXPER"]
pub mod stmpxpermin;
#[doc = "STMPWCNT (rw) register accessor: an alias for `Reg<STMPWCNT_SPEC>`"]
pub type STMPWCNT = crate::Reg<stmpwcnt::STMPWCNT_SPEC>;
#[doc = "Current Value of WCNT"]
pub mod stmpwcnt;
#[doc = "STMPXCNT (rw) register accessor: an alias for `Reg<STMPXCNT_SPEC>`"]
pub type STMPXCNT = crate::Reg<stmpxcnt::STMPXCNT_SPEC>;
#[doc = "Current Value of XCNT"]
pub mod stmpxcnt;
#[doc = "STMPXCNTCAPT1 (rw) register accessor: an alias for `Reg<STMPXCNTCAPT1_SPEC>`"]
pub type STMPXCNTCAPT1 = crate::Reg<stmpxcntcapt1::STMPXCNTCAPT1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod stmpxcntcapt1;
#[doc = "STMPWCNTCAPT1 (rw) register accessor: an alias for `Reg<STMPWCNTCAPT1_SPEC>`"]
pub type STMPWCNTCAPT1 = crate::Reg<stmpwcntcapt1::STMPWCNTCAPT1_SPEC>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod stmpwcntcapt1;
#[doc = "IRQMASK (rw) register accessor: an alias for `Reg<IRQMASK_SPEC>`"]
pub type IRQMASK = crate::Reg<irqmask::IRQMASK_SPEC>;
#[doc = "Interrupt Mask Register Selects mask states of the flags in IRQFLAGS that contribute to the I2S_IRQ event."]
pub mod irqmask;
#[doc = "IRQFLAGS (rw) register accessor: an alias for `Reg<IRQFLAGS_SPEC>`"]
pub type IRQFLAGS = crate::Reg<irqflags::IRQFLAGS_SPEC>;
#[doc = "Raw Interrupt Status Register"]
pub mod irqflags;
#[doc = "IRQSET (rw) register accessor: an alias for `Reg<IRQSET_SPEC>`"]
pub type IRQSET = crate::Reg<irqset::IRQSET_SPEC>;
#[doc = "Interrupt Set Register"]
pub mod irqset;
#[doc = "IRQCLR (rw) register accessor: an alias for `Reg<IRQCLR_SPEC>`"]
pub type IRQCLR = crate::Reg<irqclr::IRQCLR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod irqclr;
