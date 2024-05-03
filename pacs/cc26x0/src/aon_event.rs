#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcuwusel: Mcuwusel,
    auxwusel: Auxwusel,
    evtomcusel: Evtomcusel,
    rtcsel: Rtcsel,
}
impl RegisterBlock {
    #[doc = "0x00 - Wake-up Selector For MCU This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure."]
    #[inline(always)]
    pub const fn mcuwusel(&self) -> &Mcuwusel {
        &self.mcuwusel
    }
    #[doc = "0x04 - Wake-up Selector For AUX This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX. Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\\]
) as it will speed up the wakeup procedure."]
    #[inline(always)]
    pub const fn auxwusel(&self) -> &Auxwusel {
        &self.auxwusel
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
#[doc = "MCUWUSEL (rw) register accessor: Wake-up Selector For MCU This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcuwusel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcuwusel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuwusel`]
module"]
#[doc(alias = "MCUWUSEL")]
pub type Mcuwusel = crate::Reg<mcuwusel::McuwuselSpec>;
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure."]
pub mod mcuwusel;
#[doc = "AUXWUSEL (rw) register accessor: Wake-up Selector For AUX This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX. Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\\]
) as it will speed up the wakeup procedure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxwusel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxwusel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxwusel`]
module"]
#[doc(alias = "AUXWUSEL")]
pub type Auxwusel = crate::Reg<auxwusel::AuxwuselSpec>;
#[doc = "Wake-up Selector For AUX This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX. Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\\]
) as it will speed up the wakeup procedure."]
pub mod auxwusel;
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
