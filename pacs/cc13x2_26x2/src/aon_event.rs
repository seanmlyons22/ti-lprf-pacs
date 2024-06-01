#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcuwusel: Mcuwusel,
    mcuwusel1: Mcuwusel1,
    evtomcusel: Evtomcusel,
    rtcsel: Rtcsel,
}
impl RegisterBlock {
    #[doc = "0x00 - Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
    #[inline(always)]
    pub const fn mcuwusel(&self) -> &Mcuwusel {
        &self.mcuwusel
    }
    #[doc = "0x04 - Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
    #[inline(always)]
    pub const fn mcuwusel1(&self) -> &Mcuwusel1 {
        &self.mcuwusel1
    }
    #[doc = "0x08 - Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
    #[inline(always)]
    pub const fn evtomcusel(&self) -> &Evtomcusel {
        &self.evtomcusel
    }
    #[doc = "0x0c - RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
    #[inline(always)]
    pub const fn rtcsel(&self) -> &Rtcsel {
        &self.rtcsel
    }
}
#[doc = "MCUWUSEL (rw) register accessor: Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcuwusel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcuwusel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuwusel`]
module"]
#[doc(alias = "MCUWUSEL")]
pub type Mcuwusel = crate::Reg<mcuwusel::McuwuselSpec>;
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
pub mod mcuwusel;
#[doc = "MCUWUSEL1 (rw) register accessor: Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcuwusel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcuwusel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuwusel1`]
module"]
#[doc(alias = "MCUWUSEL1")]
pub type Mcuwusel1 = crate::Reg<mcuwusel1::Mcuwusel1Spec>;
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
pub mod mcuwusel1;
#[doc = "EVTOMCUSEL (rw) register accessor: Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evtomcusel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evtomcusel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evtomcusel`]
module"]
#[doc(alias = "EVTOMCUSEL")]
pub type Evtomcusel = crate::Reg<evtomcusel::EvtomcuselSpec>;
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
pub mod evtomcusel;
#[doc = "RTCSEL (rw) register accessor: RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtcsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtcsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtcsel`]
module"]
#[doc(alias = "RTCSEL")]
pub type Rtcsel = crate::Reg<rtcsel::RtcselSpec>;
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
pub mod rtcsel;
