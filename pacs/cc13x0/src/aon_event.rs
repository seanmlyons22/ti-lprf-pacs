#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Wake-up Selector For MCU This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure."]
    pub mcuwusel: MCUWUSEL,
    #[doc = "0x04 - Wake-up Selector For AUX This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX. Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\\]
) as it will speed up the wakeup procedure."]
    pub auxwusel: AUXWUSEL,
    #[doc = "0x08 - Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
    pub evtomcusel: EVTOMCUSEL,
    #[doc = "0x0c - RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
    pub rtcsel: RTCSEL,
}
#[doc = "MCUWUSEL (rw) register accessor: an alias for `Reg<MCUWUSEL_SPEC>`"]
pub type MCUWUSEL = crate::Reg<mcuwusel::MCUWUSEL_SPEC>;
#[doc = "Wake-up Selector For MCU This register contains pointers to 4 events which are routed to AON_WUC as wakeup sources for MCU. AON_WUC will start a wakeup sequence for the MCU domain when either of the 4 selected events are asserted. A wakeup sequence will guarantee that the MCU power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for MCU. Note: It is recommended ( or required when AON_WUC:MCUCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before MCU is requesting powerdown. ( PRCM requests uLDO, see conditions in PRCM:VDCTL.ULDO ) as it will speed up the wakeup procedure."]
pub mod mcuwusel;
#[doc = "AUXWUSEL (rw) register accessor: an alias for `Reg<AUXWUSEL_SPEC>`"]
pub type AUXWUSEL = crate::Reg<auxwusel::AUXWUSEL_SPEC>;
#[doc = "Wake-up Selector For AUX This register contains pointers to 3 events which are routed to AON_WUC as wakeup sources for AUX. AON_WUC will start a wakeup sequence for the AUX domain when either of the 3 selected events are asserted. A wakeup sequence will guarantee that the AUX power switches are turned on, LDO resources are available and SCLK_HF is available and selected as clock source for AUX. Note: It is recommended ( or required when AON_WUC:AUXCLK.PWR_DWN_SRC=NONE) to also setup a wakeup event here before AUX is requesting powerdown. ( AUX_WUC:PWRDWNREQ.REQ is asserted\\]
) as it will speed up the wakeup procedure."]
pub mod auxwusel;
#[doc = "EVTOMCUSEL (rw) register accessor: an alias for `Reg<EVTOMCUSEL_SPEC>`"]
pub type EVTOMCUSEL = crate::Reg<evtomcusel::EVTOMCUSEL_SPEC>;
#[doc = "Event Selector For MCU Event Fabric This register contains pointers for 3 AON events that are routed to the MCU Event Fabric EVENT"]
pub mod evtomcusel;
#[doc = "RTCSEL (rw) register accessor: an alias for `Reg<RTCSEL_SPEC>`"]
pub type RTCSEL = crate::Reg<rtcsel::RTCSEL_SPEC>;
#[doc = "RTC Capture Event Selector For AON_RTC This register contains a pointer to select an AON event for RTC capture. Please refer to AON_RTC:CH1CAPT"]
pub mod rtcsel;
