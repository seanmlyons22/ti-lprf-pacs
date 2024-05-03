#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    veccfg0: Veccfg0,
    veccfg1: Veccfg1,
    scewevsel: Scewevsel,
    evtoaonflags: Evtoaonflags,
    evtoaonpol: Evtoaonpol,
    dmactl: Dmactl,
    swevset: Swevset,
    evstat0: Evstat0,
    evstat1: Evstat1,
    evtomcupol: Evtomcupol,
    evtomcuflags: Evtomcuflags,
    combevtomcumask: Combevtomcumask,
    _reserved12: [u8; 0x04],
    vecflags: Vecflags,
    evtomcuflagsclr: Evtomcuflagsclr,
    evtoaonflagsclr: Evtoaonflagsclr,
    vecflagsclr: Vecflagsclr,
}
impl RegisterBlock {
    #[doc = "0x00 - Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration"]
    #[inline(always)]
    pub const fn veccfg0(&self) -> &Veccfg0 {
        &self.veccfg0
    }
    #[doc = "0x04 - Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration"]
    #[inline(always)]
    pub const fn veccfg1(&self) -> &Veccfg1 {
        &self.veccfg1
    }
    #[doc = "0x08 - Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions"]
    #[inline(always)]
    pub const fn scewevsel(&self) -> &Scewevsel {
        &self.scewevsel
    }
    #[doc = "0x0c - Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
    #[inline(always)]
    pub const fn evtoaonflags(&self) -> &Evtoaonflags {
        &self.evtoaonflags
    }
    #[doc = "0x10 - Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
    #[inline(always)]
    pub const fn evtoaonpol(&self) -> &Evtoaonpol {
        &self.evtoaonpol
    }
    #[doc = "0x14 - Direct Memory Access Control"]
    #[inline(always)]
    pub const fn dmactl(&self) -> &Dmactl {
        &self.dmactl
    }
    #[doc = "0x18 - Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
    #[inline(always)]
    pub const fn swevset(&self) -> &Swevset {
        &self.swevset
    }
    #[doc = "0x1c - Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC."]
    #[inline(always)]
    pub const fn evstat0(&self) -> &Evstat0 {
        &self.evstat0
    }
    #[doc = "0x20 - Event Status 1 Current event source levels, 31:16"]
    #[inline(always)]
    pub const fn evstat1(&self) -> &Evstat1 {
        &self.evstat1
    }
    #[doc = "0x24 - Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
    #[inline(always)]
    pub const fn evtomcupol(&self) -> &Evtomcupol {
        &self.evtomcupol
    }
    #[doc = "0x28 - Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
    #[inline(always)]
    pub const fn evtomcuflags(&self) -> &Evtomcuflags {
        &self.evtomcuflags
    }
    #[doc = "0x2c - Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
    #[inline(always)]
    pub const fn combevtomcumask(&self) -> &Combevtomcumask {
        &self.combevtomcumask
    }
    #[doc = "0x34 - Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n."]
    #[inline(always)]
    pub const fn vecflags(&self) -> &Vecflags {
        &self.vecflags
    }
    #[doc = "0x38 - Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    #[inline(always)]
    pub const fn evtomcuflagsclr(&self) -> &Evtomcuflagsclr {
        &self.evtomcuflagsclr
    }
    #[doc = "0x3c - Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    #[inline(always)]
    pub const fn evtoaonflagsclr(&self) -> &Evtoaonflagsclr {
        &self.evtoaonflagsclr
    }
    #[doc = "0x40 - Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
    #[inline(always)]
    pub const fn vecflagsclr(&self) -> &Vecflagsclr {
        &self.vecflagsclr
    }
}
#[doc = "VECCFG0 (rw) register accessor: Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg0`]
module"]
#[doc(alias = "VECCFG0")]
pub type Veccfg0 = crate::Reg<veccfg0::Veccfg0Spec>;
#[doc = "Vector Configuration 0 AUX_SCE wakeup vector 0 and 1 configuration"]
pub mod veccfg0;
#[doc = "VECCFG1 (rw) register accessor: Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`veccfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`veccfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@veccfg1`]
module"]
#[doc(alias = "VECCFG1")]
pub type Veccfg1 = crate::Reg<veccfg1::Veccfg1Spec>;
#[doc = "Vector Configuration 1 AUX_SCE event vectors 2 and 3 configuration"]
pub mod veccfg1;
#[doc = "SCEWEVSEL (rw) register accessor: Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scewevsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scewevsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scewevsel`]
module"]
#[doc(alias = "SCEWEVSEL")]
pub type Scewevsel = crate::Reg<scewevsel::ScewevselSpec>;
#[doc = "Sensor Controller Engine Wait Event Selection Configuration of this register controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions"]
pub mod scewevsel;
#[doc = "EVTOAONFLAGS (rw) register accessor: Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtoaonflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtoaonflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtoaonflags`]
module"]
#[doc(alias = "EVTOAONFLAGS")]
pub type Evtoaonflags = crate::Reg<evtoaonflags::EvtoaonflagsSpec>;
#[doc = "Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
pub mod evtoaonflags;
#[doc = "EVTOAONPOL (rw) register accessor: Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtoaonpol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtoaonpol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtoaonpol`]
module"]
#[doc(alias = "EVTOAONPOL")]
pub type Evtoaonpol = crate::Reg<evtoaonpol::EvtoaonpolSpec>;
#[doc = "Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
pub mod evtoaonpol;
#[doc = "DMACTL (rw) register accessor: Direct Memory Access Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl`]
module"]
#[doc(alias = "DMACTL")]
pub type Dmactl = crate::Reg<dmactl::DmactlSpec>;
#[doc = "Direct Memory Access Control"]
pub mod dmactl;
#[doc = "SWEVSET (rw) register accessor: Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swevset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swevset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swevset`]
module"]
#[doc(alias = "SWEVSET")]
pub type Swevset = crate::Reg<swevset::SwevsetSpec>;
#[doc = "Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
pub mod swevset;
#[doc = "EVSTAT0 (rw) register accessor: Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat0`]
module"]
#[doc(alias = "EVSTAT0")]
pub type Evstat0 = crate::Reg<evstat0::Evstat0Spec>;
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 32-bit event bus that is synchronous to AUX clock. The following subscribers use the asynchronous version of events in this register. - AUX_ANAIF. - AUX_TDC."]
pub mod evstat0;
#[doc = "EVSTAT1 (rw) register accessor: Event Status 1 Current event source levels, 31:16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat1`]
module"]
#[doc(alias = "EVSTAT1")]
pub type Evstat1 = crate::Reg<evstat1::Evstat1Spec>;
#[doc = "Event Status 1 Current event source levels, 31:16"]
pub mod evstat1;
#[doc = "EVTOMCUPOL (rw) register accessor: Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcupol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcupol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtomcupol`]
module"]
#[doc(alias = "EVTOMCUPOL")]
pub type Evtomcupol = crate::Reg<evtomcupol::EvtomcupolSpec>;
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
pub mod evtomcupol;
#[doc = "EVTOMCUFLAGS (rw) register accessor: Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcuflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcuflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtomcuflags`]
module"]
#[doc(alias = "EVTOMCUFLAGS")]
pub type Evtomcuflags = crate::Reg<evtomcuflags::EvtomcuflagsSpec>;
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
pub mod evtomcuflags;
#[doc = "COMBEVTOMCUMASK (rw) register accessor: Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`combevtomcumask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`combevtomcumask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@combevtomcumask`]
module"]
#[doc(alias = "COMBEVTOMCUMASK")]
pub type Combevtomcumask = crate::Reg<combevtomcumask::CombevtomcumaskSpec>;
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
pub mod combevtomcumask;
#[doc = "VECFLAGS (rw) register accessor: Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vecflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vecflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vecflags`]
module"]
#[doc(alias = "VECFLAGS")]
pub type Vecflags = crate::Reg<vecflags::VecflagsSpec>;
#[doc = "Vector Flags If a vector flag becomes 1 and AUX_SCE sleeps, AUX_SCE will wake up and execute the corresponding vector. The vector with the lowest index will execute first if multiple vectors flags are set. AUX_SCE must return to sleep to execute the next vector. During execution of a vector, AUX_SCE must clear the vector flag that triggered execution. Write 1 to bit index n in VECFLAGSCLR to clear vector flag n."]
pub mod vecflags;
#[doc = "EVTOMCUFLAGSCLR (rw) register accessor: Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcuflagsclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcuflagsclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtomcuflagsclr`]
module"]
#[doc(alias = "EVTOMCUFLAGSCLR")]
pub type Evtomcuflagsclr = crate::Reg<evtomcuflagsclr::EvtomcuflagsclrSpec>;
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtomcuflagsclr;
#[doc = "EVTOAONFLAGSCLR (rw) register accessor: Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtoaonflagsclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtoaonflagsclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtoaonflagsclr`]
module"]
#[doc(alias = "EVTOAONFLAGSCLR")]
pub type Evtoaonflagsclr = crate::Reg<evtoaonflagsclr::EvtoaonflagsclrSpec>;
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtoaonflagsclr;
#[doc = "VECFLAGSCLR (rw) register accessor: Vector Flags Clear Strobes for clearing flags in VECFLAGS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vecflagsclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vecflagsclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vecflagsclr`]
module"]
#[doc(alias = "VECFLAGSCLR")]
pub type Vecflagsclr = crate::Reg<vecflagsclr::VecflagsclrSpec>;
#[doc = "Vector Flags Clear Strobes for clearing flags in VECFLAGS."]
pub mod vecflagsclr;
