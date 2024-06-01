#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    evstat0: Evstat0,
    evstat1: Evstat1,
    evstat2: Evstat2,
    evstat3: Evstat3,
    scewevcfg0: Scewevcfg0,
    scewevcfg1: Scewevcfg1,
    dmactl: Dmactl,
    _reserved7: [u8; 0x04],
    swevset: Swevset,
    evtoaonflags: Evtoaonflags,
    evtoaonpol: Evtoaonpol,
    evtoaonflagsclr: Evtoaonflagsclr,
    evtomcuflags: Evtomcuflags,
    evtomcupol: Evtomcupol,
    evtomcuflagsclr: Evtomcuflagsclr,
    combevtomcumask: Combevtomcumask,
    evobscfg: Evobscfg,
    progdly: Progdly,
    manual: Manual,
    evstat0l: Evstat0l,
    evstat0h: Evstat0h,
    evstat1l: Evstat1l,
    evstat1h: Evstat1h,
    evstat2l: Evstat2l,
    evstat2h: Evstat2h,
    evstat3l: Evstat3l,
    evstat3h: Evstat3h,
}
impl RegisterBlock {
    #[doc = "0x00 - Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    #[inline(always)]
    pub const fn evstat0(&self) -> &Evstat0 {
        &self.evstat0
    }
    #[doc = "0x04 - Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    #[inline(always)]
    pub const fn evstat1(&self) -> &Evstat1 {
        &self.evstat1
    }
    #[doc = "0x08 - Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    #[inline(always)]
    pub const fn evstat2(&self) -> &Evstat2 {
        &self.evstat2
    }
    #[doc = "0x0c - Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
    #[inline(always)]
    pub const fn evstat3(&self) -> &Evstat3 {
        &self.evstat3
    }
    #[doc = "0x10 - Sensor Controller Engine Wait Event Configuration 0 Configuration of this register and SCEWEVCFG1 controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions. When COMB_EV_EN = 0: AUX_SCE:WUSTAT.EV_SIGNALS (7) = EV0_SEL event When COMB_EV_EN = 1: AUX_SCE:WUSTAT.EV_SIGNALS (7) = ( EV0_SEL event ) OR ( SCEWEVCFG1.EV1_SEL event ) Bit fields SCEWEVCFG1.EV0_POL and SCEWEVCFG1.EV1_POL control the polarity of selected events. Event combination is useful when there is a need to wait for a certain condition with timeout."]
    #[inline(always)]
    pub const fn scewevcfg0(&self) -> &Scewevcfg0 {
        &self.scewevcfg0
    }
    #[doc = "0x14 - Sensor Controller Engine Wait Event Configuration 1 See SCEWEVCFG0 for description."]
    #[inline(always)]
    pub const fn scewevcfg1(&self) -> &Scewevcfg1 {
        &self.scewevcfg1
    }
    #[doc = "0x18 - Direct Memory Access Control"]
    #[inline(always)]
    pub const fn dmactl(&self) -> &Dmactl {
        &self.dmactl
    }
    #[doc = "0x20 - Software Event Set Set software event flags from AUX domain to AON and MCU domains. CPUs in MCU domain can read the event flags from EVTOAONFLAGS and clear them in EVTOAONFLAGSCLR. Use of these event flags is software-defined."]
    #[inline(always)]
    pub const fn swevset(&self) -> &Swevset {
        &self.swevset
    }
    #[doc = "0x24 - Events To AON Flags This register contains a collection of event flags routed to AON_EVENT. To clear an event flag, write to EVTOAONFLAGSCLR or write 0 to event flag in this register."]
    #[inline(always)]
    pub const fn evtoaonflags(&self) -> &Evtoaonflags {
        &self.evtoaonflags
    }
    #[doc = "0x28 - Events To AON Polarity Event source polarity configuration for EVTOAONFLAGS."]
    #[inline(always)]
    pub const fn evtoaonpol(&self) -> &Evtoaonpol {
        &self.evtoaonpol
    }
    #[doc = "0x2c - Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    #[inline(always)]
    pub const fn evtoaonflagsclr(&self) -> &Evtoaonflagsclr {
        &self.evtoaonflagsclr
    }
    #[doc = "0x30 - Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
    #[inline(always)]
    pub const fn evtomcuflags(&self) -> &Evtomcuflags {
        &self.evtomcuflags
    }
    #[doc = "0x34 - Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
    #[inline(always)]
    pub const fn evtomcupol(&self) -> &Evtomcupol {
        &self.evtomcupol
    }
    #[doc = "0x38 - Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
    #[inline(always)]
    pub const fn evtomcuflagsclr(&self) -> &Evtomcuflagsclr {
        &self.evtomcuflagsclr
    }
    #[doc = "0x3c - Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
    #[inline(always)]
    pub const fn combevtomcumask(&self) -> &Combevtomcumask {
        &self.combevtomcumask
    }
    #[doc = "0x40 - Event Observation Configuration"]
    #[inline(always)]
    pub const fn evobscfg(&self) -> &Evobscfg {
        &self.evobscfg
    }
    #[doc = "0x44 - Programmable Delay"]
    #[inline(always)]
    pub const fn progdly(&self) -> &Progdly {
        &self.progdly
    }
    #[doc = "0x48 - Manual Programmable event."]
    #[inline(always)]
    pub const fn manual(&self) -> &Manual {
        &self.manual
    }
    #[doc = "0x4c - Event Status 0 Low"]
    #[inline(always)]
    pub const fn evstat0l(&self) -> &Evstat0l {
        &self.evstat0l
    }
    #[doc = "0x50 - Event Status 0 High"]
    #[inline(always)]
    pub const fn evstat0h(&self) -> &Evstat0h {
        &self.evstat0h
    }
    #[doc = "0x54 - Event Status 1 Low"]
    #[inline(always)]
    pub const fn evstat1l(&self) -> &Evstat1l {
        &self.evstat1l
    }
    #[doc = "0x58 - Event Status 1 High"]
    #[inline(always)]
    pub const fn evstat1h(&self) -> &Evstat1h {
        &self.evstat1h
    }
    #[doc = "0x5c - Event Status 2 Low"]
    #[inline(always)]
    pub const fn evstat2l(&self) -> &Evstat2l {
        &self.evstat2l
    }
    #[doc = "0x60 - Event Status 2 High"]
    #[inline(always)]
    pub const fn evstat2h(&self) -> &Evstat2h {
        &self.evstat2h
    }
    #[doc = "0x64 - Event Status 3 Low"]
    #[inline(always)]
    pub const fn evstat3l(&self) -> &Evstat3l {
        &self.evstat3l
    }
    #[doc = "0x68 - Event Status 3 High"]
    #[inline(always)]
    pub const fn evstat3h(&self) -> &Evstat3h {
        &self.evstat3h
    }
}
#[doc = "EVSTAT0 (rw) register accessor: Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat0`]
module"]
#[doc(alias = "EVSTAT0")]
pub type Evstat0 = crate::Reg<evstat0::Evstat0Spec>;
#[doc = "Event Status 0 Register holds events 0 thru 15 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat0;
#[doc = "EVSTAT1 (rw) register accessor: Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat1`]
module"]
#[doc(alias = "EVSTAT1")]
pub type Evstat1 = crate::Reg<evstat1::Evstat1Spec>;
#[doc = "Event Status 1 Register holds events 16 thru 31 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat1;
#[doc = "EVSTAT2 (rw) register accessor: Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat2`]
module"]
#[doc(alias = "EVSTAT2")]
pub type Evstat2 = crate::Reg<evstat2::Evstat2Spec>;
#[doc = "Event Status 2 Register holds events 32 thru 47 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC. - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat2;
#[doc = "EVSTAT3 (rw) register accessor: Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat3`]
module"]
#[doc(alias = "EVSTAT3")]
pub type Evstat3 = crate::Reg<evstat3::Evstat3Spec>;
#[doc = "Event Status 3 Register holds events 48 thru 63 of the 64-bit event bus that is synchronous to AUX clock. All events read through this register are synchronized at SCE clock rate, unless otherwise noted. The following subscribers use the asynchronous version of events in this register. - AUX_TIMER2. - AUX_ANAIF. - AUX_TDC . - AUX_SYSIF. - AUX_AIODIOn. - EVOBSCFG."]
pub mod evstat3;
#[doc = "SCEWEVCFG0 (rw) register accessor: Sensor Controller Engine Wait Event Configuration 0 Configuration of this register and SCEWEVCFG1 controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions. When COMB_EV_EN = 0: AUX_SCE:WUSTAT.EV_SIGNALS (7) = EV0_SEL event When COMB_EV_EN = 1: AUX_SCE:WUSTAT.EV_SIGNALS (7) = ( EV0_SEL event ) OR ( SCEWEVCFG1.EV1_SEL event ) Bit fields SCEWEVCFG1.EV0_POL and SCEWEVCFG1.EV1_POL control the polarity of selected events. Event combination is useful when there is a need to wait for a certain condition with timeout.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scewevcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scewevcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scewevcfg0`]
module"]
#[doc(alias = "SCEWEVCFG0")]
pub type Scewevcfg0 = crate::Reg<scewevcfg0::Scewevcfg0Spec>;
#[doc = "Sensor Controller Engine Wait Event Configuration 0 Configuration of this register and SCEWEVCFG1 controls bit index 7 in AUX_SCE:WUSTAT.EV_SIGNALS. This bit can be used by AUX_SCE WEV0, WEV1, BEV0 and BEV1 instructions. When COMB_EV_EN = 0: AUX_SCE:WUSTAT.EV_SIGNALS (7) = EV0_SEL event When COMB_EV_EN = 1: AUX_SCE:WUSTAT.EV_SIGNALS (7) = ( EV0_SEL event ) OR ( SCEWEVCFG1.EV1_SEL event ) Bit fields SCEWEVCFG1.EV0_POL and SCEWEVCFG1.EV1_POL control the polarity of selected events. Event combination is useful when there is a need to wait for a certain condition with timeout."]
pub mod scewevcfg0;
#[doc = "SCEWEVCFG1 (rw) register accessor: Sensor Controller Engine Wait Event Configuration 1 See SCEWEVCFG0 for description.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scewevcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scewevcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scewevcfg1`]
module"]
#[doc(alias = "SCEWEVCFG1")]
pub type Scewevcfg1 = crate::Reg<scewevcfg1::Scewevcfg1Spec>;
#[doc = "Sensor Controller Engine Wait Event Configuration 1 See SCEWEVCFG0 for description."]
pub mod scewevcfg1;
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
#[doc = "EVTOAONFLAGSCLR (rw) register accessor: Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtoaonflagsclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtoaonflagsclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtoaonflagsclr`]
module"]
#[doc(alias = "EVTOAONFLAGSCLR")]
pub type Evtoaonflagsclr = crate::Reg<evtoaonflagsclr::EvtoaonflagsclrSpec>;
#[doc = "Events To AON Clear Clear event flags in EVTOAONFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtoaonflagsclr;
#[doc = "EVTOMCUFLAGS (rw) register accessor: Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcuflags::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcuflags::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtomcuflags`]
module"]
#[doc(alias = "EVTOMCUFLAGS")]
pub type Evtomcuflags = crate::Reg<evtomcuflags::EvtomcuflagsSpec>;
#[doc = "Events to MCU Flags This register contains a collection of event flags routed to MCU domain. To clear an event flag, write to EVTOMCUFLAGSCLR or write 0 to event flag in this register. Follow procedure described in AUX_SYSIF:WUCLR to clear AUX_WU_EV event flag."]
pub mod evtomcuflags;
#[doc = "EVTOMCUPOL (rw) register accessor: Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcupol::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcupol::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtomcupol`]
module"]
#[doc(alias = "EVTOMCUPOL")]
pub type Evtomcupol = crate::Reg<evtomcupol::EvtomcupolSpec>;
#[doc = "Event To MCU Polarity Event source polarity configuration for EVTOMCUFLAGS."]
pub mod evtomcupol;
#[doc = "EVTOMCUFLAGSCLR (rw) register accessor: Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcuflagsclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcuflagsclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtomcuflagsclr`]
module"]
#[doc(alias = "EVTOMCUFLAGSCLR")]
pub type Evtomcuflagsclr = crate::Reg<evtomcuflagsclr::EvtomcuflagsclrSpec>;
#[doc = "Events To MCU Flags Clear Clear event flags in EVTOMCUFLAGS. In order to clear a level sensitive event flag, the event must be deasserted."]
pub mod evtomcuflagsclr;
#[doc = "COMBEVTOMCUMASK (rw) register accessor: Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`combevtomcumask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`combevtomcumask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@combevtomcumask`]
module"]
#[doc(alias = "COMBEVTOMCUMASK")]
pub type Combevtomcumask = crate::Reg<combevtomcumask::CombevtomcumaskSpec>;
#[doc = "Combined Event To MCU Mask Select event flags in EVTOMCUFLAGS that contribute to the AUX_COMB event to EVENT and system CPU. The AUX_COMB event is high as long as one or more of the included event flags are set."]
pub mod combevtomcumask;
#[doc = "EVOBSCFG (rw) register accessor: Event Observation Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evobscfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evobscfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evobscfg`]
module"]
#[doc(alias = "EVOBSCFG")]
pub type Evobscfg = crate::Reg<evobscfg::EvobscfgSpec>;
#[doc = "Event Observation Configuration"]
pub mod evobscfg;
#[doc = "PROGDLY (rw) register accessor: Programmable Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`progdly::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`progdly::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@progdly`]
module"]
#[doc(alias = "PROGDLY")]
pub type Progdly = crate::Reg<progdly::ProgdlySpec>;
#[doc = "Programmable Delay"]
pub mod progdly;
#[doc = "MANUAL (rw) register accessor: Manual Programmable event.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`manual::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`manual::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@manual`]
module"]
#[doc(alias = "MANUAL")]
pub type Manual = crate::Reg<manual::ManualSpec>;
#[doc = "Manual Programmable event."]
pub mod manual;
#[doc = "EVSTAT0L (rw) register accessor: Event Status 0 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat0l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat0l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat0l`]
module"]
#[doc(alias = "EVSTAT0L")]
pub type Evstat0l = crate::Reg<evstat0l::Evstat0lSpec>;
#[doc = "Event Status 0 Low"]
pub mod evstat0l;
#[doc = "EVSTAT0H (rw) register accessor: Event Status 0 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat0h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat0h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat0h`]
module"]
#[doc(alias = "EVSTAT0H")]
pub type Evstat0h = crate::Reg<evstat0h::Evstat0hSpec>;
#[doc = "Event Status 0 High"]
pub mod evstat0h;
#[doc = "EVSTAT1L (rw) register accessor: Event Status 1 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat1l`]
module"]
#[doc(alias = "EVSTAT1L")]
pub type Evstat1l = crate::Reg<evstat1l::Evstat1lSpec>;
#[doc = "Event Status 1 Low"]
pub mod evstat1l;
#[doc = "EVSTAT1H (rw) register accessor: Event Status 1 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat1h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat1h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat1h`]
module"]
#[doc(alias = "EVSTAT1H")]
pub type Evstat1h = crate::Reg<evstat1h::Evstat1hSpec>;
#[doc = "Event Status 1 High"]
pub mod evstat1h;
#[doc = "EVSTAT2L (rw) register accessor: Event Status 2 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat2l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat2l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat2l`]
module"]
#[doc(alias = "EVSTAT2L")]
pub type Evstat2l = crate::Reg<evstat2l::Evstat2lSpec>;
#[doc = "Event Status 2 Low"]
pub mod evstat2l;
#[doc = "EVSTAT2H (rw) register accessor: Event Status 2 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat2h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat2h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat2h`]
module"]
#[doc(alias = "EVSTAT2H")]
pub type Evstat2h = crate::Reg<evstat2h::Evstat2hSpec>;
#[doc = "Event Status 2 High"]
pub mod evstat2h;
#[doc = "EVSTAT3L (rw) register accessor: Event Status 3 Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat3l::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat3l::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat3l`]
module"]
#[doc(alias = "EVSTAT3L")]
pub type Evstat3l = crate::Reg<evstat3l::Evstat3lSpec>;
#[doc = "Event Status 3 Low"]
pub mod evstat3l;
#[doc = "EVSTAT3H (rw) register accessor: Event Status 3 High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evstat3h::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evstat3h::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evstat3h`]
module"]
#[doc(alias = "EVSTAT3H")]
pub type Evstat3h = crate::Reg<evstat3h::Evstat3hSpec>;
#[doc = "Event Status 3 High"]
pub mod evstat3h;
