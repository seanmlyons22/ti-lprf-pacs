#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    #[doc = "0x04 - AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    pub auxsceclk: AUXSCECLK,
    #[doc = "0x08 - RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
    pub ramcfg: RAMCFG,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    pub pwrctl: PWRCTL,
    #[doc = "0x14 - AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
    pub pwrstat: PWRSTAT,
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    pub shutdown: SHUTDOWN,
    #[doc = "0x1c - Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
    pub rechargecfg: RECHARGECFG,
    #[doc = "0x20 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    pub rechargestat: RECHARGESTAT,
    #[doc = "0x24 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    pub osccfg: OSCCFG,
    #[doc = "0x28 - Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
    pub resetctl: RESETCTL,
    #[doc = "0x2c - Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    pub sleepctl: SLEEPCTL,
    _reserved10: [u8; 0x04],
    #[doc = "0x34 - JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
    pub jtagcfg: JTAGCFG,
    _reserved11: [u8; 0x04],
    #[doc = "0x3c - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    pub jtagusercode: JTAGUSERCODE,
    _reserved12: [u8; 0x84],
    #[doc = "0xc4 - Configuration Load Value register"]
    pub wdtload: WDTLOAD,
    #[doc = "0xc8 - Test Mode"]
    pub wdttest: WDTTEST,
    _reserved14: [u8; 0x04],
    #[doc = "0xd0 - Lock"]
    pub wdtlock: WDTLOCK,
}
#[doc = "AUXSCECLK (rw) register accessor: an alias for `Reg<AUXSCECLK_SPEC>`"]
pub type AUXSCECLK = crate::Reg<auxsceclk::AUXSCECLK_SPEC>;
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxsceclk;
#[doc = "RAMCFG (rw) register accessor: an alias for `Reg<RAMCFG_SPEC>`"]
pub type RAMCFG = crate::Reg<ramcfg::RAMCFG_SPEC>;
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
pub mod ramcfg;
#[doc = "PWRCTL (rw) register accessor: an alias for `Reg<PWRCTL_SPEC>`"]
pub type PWRCTL = crate::Reg<pwrctl::PWRCTL_SPEC>;
#[doc = "Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "PWRSTAT (rw) register accessor: an alias for `Reg<PWRSTAT_SPEC>`"]
pub type PWRSTAT = crate::Reg<pwrstat::PWRSTAT_SPEC>;
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
pub mod pwrstat;
#[doc = "SHUTDOWN (rw) register accessor: an alias for `Reg<SHUTDOWN_SPEC>`"]
pub type SHUTDOWN = crate::Reg<shutdown::SHUTDOWN_SPEC>;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "RECHARGECFG (rw) register accessor: an alias for `Reg<RECHARGECFG_SPEC>`"]
pub type RECHARGECFG = crate::Reg<rechargecfg::RECHARGECFG_SPEC>;
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
pub mod rechargecfg;
#[doc = "RECHARGESTAT (rw) register accessor: an alias for `Reg<RECHARGESTAT_SPEC>`"]
pub type RECHARGESTAT = crate::Reg<rechargestat::RECHARGESTAT_SPEC>;
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub mod rechargestat;
#[doc = "OSCCFG (rw) register accessor: an alias for `Reg<OSCCFG_SPEC>`"]
pub type OSCCFG = crate::Reg<osccfg::OSCCFG_SPEC>;
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub mod osccfg;
#[doc = "RESETCTL (rw) register accessor: an alias for `Reg<RESETCTL_SPEC>`"]
pub type RESETCTL = crate::Reg<resetctl::RESETCTL_SPEC>;
#[doc = "Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "SLEEPCTL (rw) register accessor: an alias for `Reg<SLEEPCTL_SPEC>`"]
pub type SLEEPCTL = crate::Reg<sleepctl::SLEEPCTL_SPEC>;
#[doc = "Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;
#[doc = "JTAGCFG (rw) register accessor: an alias for `Reg<JTAGCFG_SPEC>`"]
pub type JTAGCFG = crate::Reg<jtagcfg::JTAGCFG_SPEC>;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAGUSERCODE (rw) register accessor: an alias for `Reg<JTAGUSERCODE_SPEC>`"]
pub type JTAGUSERCODE = crate::Reg<jtagusercode::JTAGUSERCODE_SPEC>;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;
#[doc = "WDTLOAD (rw) register accessor: an alias for `Reg<WDTLOAD_SPEC>`"]
pub type WDTLOAD = crate::Reg<wdtload::WDTLOAD_SPEC>;
#[doc = "Configuration Load Value register"]
pub mod wdtload;
#[doc = "WDTTEST (rw) register accessor: an alias for `Reg<WDTTEST_SPEC>`"]
pub type WDTTEST = crate::Reg<wdttest::WDTTEST_SPEC>;
#[doc = "Test Mode"]
pub mod wdttest;
#[doc = "WDTLOCK (rw) register accessor: an alias for `Reg<WDTLOCK_SPEC>`"]
pub type WDTLOCK = crate::Reg<wdtlock::WDTLOCK_SPEC>;
#[doc = "Lock"]
pub mod wdtlock;
