#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
    pub mcuwusel: MCUWUSEL,
    #[doc = "0x04 - Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
    pub mcuwusel1: MCUWUSEL1,
    #[doc = "0x08 - Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
    pub evtomcusel: EVTOMCUSEL,
    #[doc = "0x0c - RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
    pub rtcsel: RTCSEL,
}
#[doc = "MCUWUSEL (rw) register accessor: an alias for `Reg<MCUWUSEL_SPEC>`"]
pub type MCUWUSEL = crate::Reg<mcuwusel::MCUWUSEL_SPEC>;
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 0 to 3) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
pub mod mcuwusel;
#[doc = "MCUWUSEL1 (rw) register accessor: an alias for `Reg<MCUWUSEL1_SPEC>`"]
pub type MCUWUSEL1 = crate::Reg<mcuwusel1::MCUWUSEL1_SPEC>;
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 of 8 events (events 4 to 7) which are routed to AON_PMCTRL as wakeup sources for MCU. AON_PMCTRL will start a wakeup sequence for the MCU domain when either of the 8 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is required to setup a wakeup event in AON_EVENT before MCU is requesting powerdown ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO )."]
pub mod mcuwusel1;
#[doc = "EVTOMCUSEL (rw) register accessor: an alias for `Reg<EVTOMCUSEL_SPEC>`"]
pub type EVTOMCUSEL = crate::Reg<evtomcusel::EVTOMCUSEL_SPEC>;
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
pub mod evtomcusel;
#[doc = "RTCSEL (rw) register accessor: an alias for `Reg<RTCSEL_SPEC>`"]
pub type RTCSEL = crate::Reg<rtcsel::RTCSEL_SPEC>;
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
pub mod rtcsel;
