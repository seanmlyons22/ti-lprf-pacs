#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration"]
    pub veccfg0: VECCFG0,
    #[doc = "0x04 - Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration"]
    pub veccfg1: VECCFG1,
    #[doc = "0x08 - Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions"]
    pub scewevsel: SCEWEVSEL,
    #[doc = "0x0c - Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
    pub evtoaonflags: EVTOAONFLAGS,
    #[doc = "0x10 - Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
    pub evtoaonpol: EVTOAONPOL,
    #[doc = "0x14 - Direct Memory Access Control"]
    pub dmactl: DMACTL,
    #[doc = "0x18 - Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
    pub swevset: SWEVSET,
    #[doc = "0x1c - Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC."]
    pub evstat0: EVSTAT0,
    #[doc = "0x20 - Event Status 1 Current event source levels, 31:16"]
    pub evstat1: EVSTAT1,
    #[doc = "0x24 - Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
    pub evtomcupol: EVTOMCUPOL,
    #[doc = "0x28 - Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
    pub evtomcuflags: EVTOMCUFLAGS,
    #[doc = "0x2c - Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
    pub combevtomcumask: COMBEVTOMCUMASK,
    _reserved12: [u8; 0x04],
    #[doc = "0x34 - Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n."]
    pub vecflags: VECFLAGS,
    #[doc = "0x38 - Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    pub evtomcuflagsclr: EVTOMCUFLAGSCLR,
    #[doc = "0x3c - Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    pub evtoaonflagsclr: EVTOAONFLAGSCLR,
    #[doc = "0x40 - Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
    pub vecflagsclr: VECFLAGSCLR,
}
#[doc = "VECCFG0 (rw) register accessor: an alias for `Reg<VECCFG0_SPEC>`"]
pub type VECCFG0 = crate::Reg<veccfg0::VECCFG0_SPEC>;
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration"]
pub mod veccfg0;
#[doc = "VECCFG1 (rw) register accessor: an alias for `Reg<VECCFG1_SPEC>`"]
pub type VECCFG1 = crate::Reg<veccfg1::VECCFG1_SPEC>;
#[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration"]
pub mod veccfg1;
#[doc = "SCEWEVSEL (rw) register accessor: an alias for `Reg<SCEWEVSEL_SPEC>`"]
pub type SCEWEVSEL = crate::Reg<scewevsel::SCEWEVSEL_SPEC>;
#[doc = "Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions"]
pub mod scewevsel;
#[doc = "EVTOAONFLAGS (rw) register accessor: an alias for `Reg<EVTOAONFLAGS_SPEC>`"]
pub type EVTOAONFLAGS = crate::Reg<evtoaonflags::EVTOAONFLAGS_SPEC>;
#[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
pub mod evtoaonflags;
#[doc = "EVTOAONPOL (rw) register accessor: an alias for `Reg<EVTOAONPOL_SPEC>`"]
pub type EVTOAONPOL = crate::Reg<evtoaonpol::EVTOAONPOL_SPEC>;
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
pub mod evtoaonpol;
#[doc = "DMACTL (rw) register accessor: an alias for `Reg<DMACTL_SPEC>`"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "Direct Memory Access Control"]
pub mod dmactl;
#[doc = "SWEVSET (rw) register accessor: an alias for `Reg<SWEVSET_SPEC>`"]
pub type SWEVSET = crate::Reg<swevset::SWEVSET_SPEC>;
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
pub mod swevset;
#[doc = "EVSTAT0 (rw) register accessor: an alias for `Reg<EVSTAT0_SPEC>`"]
pub type EVSTAT0 = crate::Reg<evstat0::EVSTAT0_SPEC>;
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC."]
pub mod evstat0;
#[doc = "EVSTAT1 (rw) register accessor: an alias for `Reg<EVSTAT1_SPEC>`"]
pub type EVSTAT1 = crate::Reg<evstat1::EVSTAT1_SPEC>;
#[doc = "Event Status 1 Current event source levels, 31:16"]
pub mod evstat1;
#[doc = "EVTOMCUPOL (rw) register accessor: an alias for `Reg<EVTOMCUPOL_SPEC>`"]
pub type EVTOMCUPOL = crate::Reg<evtomcupol::EVTOMCUPOL_SPEC>;
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
pub mod evtomcupol;
#[doc = "EVTOMCUFLAGS (rw) register accessor: an alias for `Reg<EVTOMCUFLAGS_SPEC>`"]
pub type EVTOMCUFLAGS = crate::Reg<evtomcuflags::EVTOMCUFLAGS_SPEC>;
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
pub mod evtomcuflags;
#[doc = "COMBEVTOMCUMASK (rw) register accessor: an alias for `Reg<COMBEVTOMCUMASK_SPEC>`"]
pub type COMBEVTOMCUMASK = crate::Reg<combevtomcumask::COMBEVTOMCUMASK_SPEC>;
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
pub mod combevtomcumask;
#[doc = "VECFLAGS (rw) register accessor: an alias for `Reg<VECFLAGS_SPEC>`"]
pub type VECFLAGS = crate::Reg<vecflags::VECFLAGS_SPEC>;
#[doc = "Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n."]
pub mod vecflags;
#[doc = "EVTOMCUFLAGSCLR (rw) register accessor: an alias for `Reg<EVTOMCUFLAGSCLR_SPEC>`"]
pub type EVTOMCUFLAGSCLR = crate::Reg<evtomcuflagsclr::EVTOMCUFLAGSCLR_SPEC>;
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtomcuflagsclr;
#[doc = "EVTOAONFLAGSCLR (rw) register accessor: an alias for `Reg<EVTOAONFLAGSCLR_SPEC>`"]
pub type EVTOAONFLAGSCLR = crate::Reg<evtoaonflagsclr::EVTOAONFLAGSCLR_SPEC>;
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtoaonflagsclr;
#[doc = "VECFLAGSCLR (rw) register accessor: an alias for `Reg<VECFLAGSCLR_SPEC>`"]
pub type VECFLAGSCLR = crate::Reg<vecflagsclr::VECFLAGSCLR_SPEC>;
#[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
pub mod vecflagsclr;
