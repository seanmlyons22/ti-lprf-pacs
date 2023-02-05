#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat0: EVSTAT0,
    #[doc = "0x04 - Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat1: EVSTAT1,
    #[doc = "0x08 - Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat2: EVSTAT2,
    #[doc = "0x0c - Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    pub evstat3: EVSTAT3,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - Direct Memory Access Control"]
    pub dmactl: DMACTL,
    _reserved5: [u8; 0x04],
    #[doc = "0x20 - Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
    pub swevset: SWEVSET,
    #[doc = "0x24 - Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
    pub evtoaonflags: EVTOAONFLAGS,
    #[doc = "0x28 - Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
    pub evtoaonpol: EVTOAONPOL,
    #[doc = "0x2c - Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    pub evtoaonflagsclr: EVTOAONFLAGSCLR,
    #[doc = "0x30 - Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
    pub evtomcuflags: EVTOMCUFLAGS,
    #[doc = "0x34 - Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
    pub evtomcupol: EVTOMCUPOL,
    #[doc = "0x38 - Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    pub evtomcuflagsclr: EVTOMCUFLAGSCLR,
    #[doc = "0x3c - Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
    pub combevtomcumask: COMBEVTOMCUMASK,
    #[doc = "0x40 - Event Observation Configuration"]
    pub evobscfg: EVOBSCFG,
    _reserved14: [u8; 0x04],
    #[doc = "0x48 - Manual Programmable event."]
    pub manual: MANUAL,
}
#[doc = "EVSTAT0 (rw) register accessor: an alias for `Reg<EVSTAT0_SPEC>`"]
pub type EVSTAT0 = crate::Reg<evstat0::EVSTAT0_SPEC>;
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat0;
#[doc = "EVSTAT1 (rw) register accessor: an alias for `Reg<EVSTAT1_SPEC>`"]
pub type EVSTAT1 = crate::Reg<evstat1::EVSTAT1_SPEC>;
#[doc = "Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat1;
#[doc = "EVSTAT2 (rw) register accessor: an alias for `Reg<EVSTAT2_SPEC>`"]
pub type EVSTAT2 = crate::Reg<evstat2::EVSTAT2_SPEC>;
#[doc = "Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat2;
#[doc = "EVSTAT3 (rw) register accessor: an alias for `Reg<EVSTAT3_SPEC>`"]
pub type EVSTAT3 = crate::Reg<evstat3::EVSTAT3_SPEC>;
#[doc = "Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat3;
#[doc = "DMACTL (rw) register accessor: an alias for `Reg<DMACTL_SPEC>`"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "Direct Memory Access Control"]
pub mod dmactl;
#[doc = "SWEVSET (rw) register accessor: an alias for `Reg<SWEVSET_SPEC>`"]
pub type SWEVSET = crate::Reg<swevset::SWEVSET_SPEC>;
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
pub mod swevset;
#[doc = "EVTOAONFLAGS (rw) register accessor: an alias for `Reg<EVTOAONFLAGS_SPEC>`"]
pub type EVTOAONFLAGS = crate::Reg<evtoaonflags::EVTOAONFLAGS_SPEC>;
#[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
pub mod evtoaonflags;
#[doc = "EVTOAONPOL (rw) register accessor: an alias for `Reg<EVTOAONPOL_SPEC>`"]
pub type EVTOAONPOL = crate::Reg<evtoaonpol::EVTOAONPOL_SPEC>;
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
pub mod evtoaonpol;
#[doc = "EVTOAONFLAGSCLR (rw) register accessor: an alias for `Reg<EVTOAONFLAGSCLR_SPEC>`"]
pub type EVTOAONFLAGSCLR = crate::Reg<evtoaonflagsclr::EVTOAONFLAGSCLR_SPEC>;
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtoaonflagsclr;
#[doc = "EVTOMCUFLAGS (rw) register accessor: an alias for `Reg<EVTOMCUFLAGS_SPEC>`"]
pub type EVTOMCUFLAGS = crate::Reg<evtomcuflags::EVTOMCUFLAGS_SPEC>;
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
pub mod evtomcuflags;
#[doc = "EVTOMCUPOL (rw) register accessor: an alias for `Reg<EVTOMCUPOL_SPEC>`"]
pub type EVTOMCUPOL = crate::Reg<evtomcupol::EVTOMCUPOL_SPEC>;
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
pub mod evtomcupol;
#[doc = "EVTOMCUFLAGSCLR (rw) register accessor: an alias for `Reg<EVTOMCUFLAGSCLR_SPEC>`"]
pub type EVTOMCUFLAGSCLR = crate::Reg<evtomcuflagsclr::EVTOMCUFLAGSCLR_SPEC>;
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtomcuflagsclr;
#[doc = "COMBEVTOMCUMASK (rw) register accessor: an alias for `Reg<COMBEVTOMCUMASK_SPEC>`"]
pub type COMBEVTOMCUMASK = crate::Reg<combevtomcumask::COMBEVTOMCUMASK_SPEC>;
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
pub mod combevtomcumask;
#[doc = "EVOBSCFG (rw) register accessor: an alias for `Reg<EVOBSCFG_SPEC>`"]
pub type EVOBSCFG = crate::Reg<evobscfg::EVOBSCFG_SPEC>;
#[doc = "Event Observation Configuration"]
pub mod evobscfg;
#[doc = "MANUAL (rw) register accessor: an alias for `Reg<MANUAL_SPEC>`"]
pub type MANUAL = crate::Reg<manual::MANUAL_SPEC>;
#[doc = "Manual Programmable event."]
pub mod manual;
