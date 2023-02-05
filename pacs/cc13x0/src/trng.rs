#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Random Number Lower Word Readout Value"]
    pub out0: OUT0,
    #[doc = "0x04 - Random Number Upper Word Readout Value"]
    pub out1: OUT1,
    #[doc = "0x08 - Interrupt Status"]
    pub irqflagstat: IRQFLAGSTAT,
    #[doc = "0x0c - Interrupt Mask"]
    pub irqflagmask: IRQFLAGMASK,
    #[doc = "0x10 - Interrupt Flag Clear"]
    pub irqflagclr: IRQFLAGCLR,
    #[doc = "0x14 - Control"]
    pub ctl: CTL,
    #[doc = "0x18 - Configuration 0"]
    pub cfg0: CFG0,
    #[doc = "0x1c - Alarm Control"]
    pub alarmcnt: ALARMCNT,
    #[doc = "0x20 - FRO Enable"]
    pub froen: FROEN,
    #[doc = "0x24 - FRO De-tune Bit"]
    pub frodetune: FRODETUNE,
    #[doc = "0x28 - Alarm Event"]
    pub alarmmask: ALARMMASK,
    #[doc = "0x2c - Alarm Shutdown"]
    pub alarmstop: ALARMSTOP,
    #[doc = "0x30 - LFSR Readout Value"]
    pub lfsr0: LFSR0,
    #[doc = "0x34 - LFSR Readout Value"]
    pub lfsr1: LFSR1,
    #[doc = "0x38 - LFSR Readout Value"]
    pub lfsr2: LFSR2,
    _reserved15: [u8; 0x3c],
    #[doc = "0x78 - TRNG Engine Options Information"]
    pub hwopt: HWOPT,
    #[doc = "0x7c - HW Version 0 EIP Number And Core Revision"]
    pub hwver0: HWVER0,
    _reserved17: [u8; 0x1f58],
    #[doc = "0x1fd8 - Interrupt Status After Masking"]
    pub irqstatmask: IRQSTATMASK,
    _reserved18: [u8; 0x04],
    #[doc = "0x1fe0 - HW Version 1 TRNG Revision Number"]
    pub hwver1: HWVER1,
    _reserved19: [u8; 0x08],
    #[doc = "0x1fec - Interrupt Set"]
    pub irqset: IRQSET,
    #[doc = "0x1ff0 - SW Reset Control"]
    pub swreset: SWRESET,
    _reserved21: [u8; 0x04],
    #[doc = "0x1ff8 - Interrupt Status"]
    pub irqstat: IRQSTAT,
}
#[doc = "OUT0 (rw) register accessor: an alias for `Reg<OUT0_SPEC>`"]
pub type OUT0 = crate::Reg<out0::OUT0_SPEC>;
#[doc = "Random Number Lower Word Readout Value"]
pub mod out0;
#[doc = "OUT1 (rw) register accessor: an alias for `Reg<OUT1_SPEC>`"]
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
#[doc = "Random Number Upper Word Readout Value"]
pub mod out1;
#[doc = "IRQFLAGSTAT (rw) register accessor: an alias for `Reg<IRQFLAGSTAT_SPEC>`"]
pub type IRQFLAGSTAT = crate::Reg<irqflagstat::IRQFLAGSTAT_SPEC>;
#[doc = "Interrupt Status"]
pub mod irqflagstat;
#[doc = "IRQFLAGMASK (rw) register accessor: an alias for `Reg<IRQFLAGMASK_SPEC>`"]
pub type IRQFLAGMASK = crate::Reg<irqflagmask::IRQFLAGMASK_SPEC>;
#[doc = "Interrupt Mask"]
pub mod irqflagmask;
#[doc = "IRQFLAGCLR (rw) register accessor: an alias for `Reg<IRQFLAGCLR_SPEC>`"]
pub type IRQFLAGCLR = crate::Reg<irqflagclr::IRQFLAGCLR_SPEC>;
#[doc = "Interrupt Flag Clear"]
pub mod irqflagclr;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CFG0 (rw) register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "Configuration 0"]
pub mod cfg0;
#[doc = "ALARMCNT (rw) register accessor: an alias for `Reg<ALARMCNT_SPEC>`"]
pub type ALARMCNT = crate::Reg<alarmcnt::ALARMCNT_SPEC>;
#[doc = "Alarm Control"]
pub mod alarmcnt;
#[doc = "FROEN (rw) register accessor: an alias for `Reg<FROEN_SPEC>`"]
pub type FROEN = crate::Reg<froen::FROEN_SPEC>;
#[doc = "FRO Enable"]
pub mod froen;
#[doc = "FRODETUNE (rw) register accessor: an alias for `Reg<FRODETUNE_SPEC>`"]
pub type FRODETUNE = crate::Reg<frodetune::FRODETUNE_SPEC>;
#[doc = "FRO De-tune Bit"]
pub mod frodetune;
#[doc = "ALARMMASK (rw) register accessor: an alias for `Reg<ALARMMASK_SPEC>`"]
pub type ALARMMASK = crate::Reg<alarmmask::ALARMMASK_SPEC>;
#[doc = "Alarm Event"]
pub mod alarmmask;
#[doc = "ALARMSTOP (rw) register accessor: an alias for `Reg<ALARMSTOP_SPEC>`"]
pub type ALARMSTOP = crate::Reg<alarmstop::ALARMSTOP_SPEC>;
#[doc = "Alarm Shutdown"]
pub mod alarmstop;
#[doc = "LFSR0 (rw) register accessor: an alias for `Reg<LFSR0_SPEC>`"]
pub type LFSR0 = crate::Reg<lfsr0::LFSR0_SPEC>;
#[doc = "LFSR Readout Value"]
pub mod lfsr0;
#[doc = "LFSR1 (rw) register accessor: an alias for `Reg<LFSR1_SPEC>`"]
pub type LFSR1 = crate::Reg<lfsr1::LFSR1_SPEC>;
#[doc = "LFSR Readout Value"]
pub mod lfsr1;
#[doc = "LFSR2 (rw) register accessor: an alias for `Reg<LFSR2_SPEC>`"]
pub type LFSR2 = crate::Reg<lfsr2::LFSR2_SPEC>;
#[doc = "LFSR Readout Value"]
pub mod lfsr2;
#[doc = "HWOPT (rw) register accessor: an alias for `Reg<HWOPT_SPEC>`"]
pub type HWOPT = crate::Reg<hwopt::HWOPT_SPEC>;
#[doc = "TRNG Engine Options Information"]
pub mod hwopt;
#[doc = "HWVER0 (rw) register accessor: an alias for `Reg<HWVER0_SPEC>`"]
pub type HWVER0 = crate::Reg<hwver0::HWVER0_SPEC>;
#[doc = "HW Version 0 EIP Number And Core Revision"]
pub mod hwver0;
#[doc = "IRQSTATMASK (rw) register accessor: an alias for `Reg<IRQSTATMASK_SPEC>`"]
pub type IRQSTATMASK = crate::Reg<irqstatmask::IRQSTATMASK_SPEC>;
#[doc = "Interrupt Status After Masking"]
pub mod irqstatmask;
#[doc = "HWVER1 (rw) register accessor: an alias for `Reg<HWVER1_SPEC>`"]
pub type HWVER1 = crate::Reg<hwver1::HWVER1_SPEC>;
#[doc = "HW Version 1 TRNG Revision Number"]
pub mod hwver1;
#[doc = "IRQSET (rw) register accessor: an alias for `Reg<IRQSET_SPEC>`"]
pub type IRQSET = crate::Reg<irqset::IRQSET_SPEC>;
#[doc = "Interrupt Set"]
pub mod irqset;
#[doc = "SWRESET (rw) register accessor: an alias for `Reg<SWRESET_SPEC>`"]
pub type SWRESET = crate::Reg<swreset::SWRESET_SPEC>;
#[doc = "SW Reset Control"]
pub mod swreset;
#[doc = "IRQSTAT (rw) register accessor: an alias for `Reg<IRQSTAT_SPEC>`"]
pub type IRQSTAT = crate::Reg<irqstat::IRQSTAT_SPEC>;
#[doc = "Interrupt Status"]
pub mod irqstat;
