#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    auxsceclk: Auxsceclk,
    ramcfg: Ramcfg,
    _reserved2: [u8; 0x04],
    pwrctl: Pwrctl,
    pwrstat: Pwrstat,
    shutdown: Shutdown,
    rechargecfg: Rechargecfg,
    rechargestat: Rechargestat,
    osccfg: Osccfg,
    resetctl: Resetctl,
    sleepctl: Sleepctl,
    _reserved10: [u8; 0x04],
    jtagcfg: Jtagcfg,
    _reserved11: [u8; 0x04],
    jtagusercode: Jtagusercode,
    _reserved12: [u8; 0x84],
    wdtload: Wdtload,
    wdttest: Wdttest,
    _reserved14: [u8; 0x04],
    wdtlock: Wdtlock,
}
impl RegisterBlock {
    #[doc = "0x04 - AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    #[inline(always)]
    pub const fn auxsceclk(&self) -> &Auxsceclk {
        &self.auxsceclk
    }
    #[doc = "0x08 - RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
    #[inline(always)]
    pub const fn ramcfg(&self) -> &Ramcfg {
        &self.ramcfg
    }
    #[doc = "0x10 - Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    #[inline(always)]
    pub const fn pwrctl(&self) -> &Pwrctl {
        &self.pwrctl
    }
    #[doc = "0x14 - AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
    #[inline(always)]
    pub const fn pwrstat(&self) -> &Pwrstat {
        &self.pwrstat
    }
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    #[inline(always)]
    pub const fn shutdown(&self) -> &Shutdown {
        &self.shutdown
    }
    #[doc = "0x1c - Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
    #[inline(always)]
    pub const fn rechargecfg(&self) -> &Rechargecfg {
        &self.rechargecfg
    }
    #[doc = "0x20 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    #[inline(always)]
    pub const fn rechargestat(&self) -> &Rechargestat {
        &self.rechargestat
    }
    #[doc = "0x24 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    #[inline(always)]
    pub const fn osccfg(&self) -> &Osccfg {
        &self.osccfg
    }
    #[doc = "0x28 - Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
    #[inline(always)]
    pub const fn resetctl(&self) -> &Resetctl {
        &self.resetctl
    }
    #[doc = "0x2c - Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    #[inline(always)]
    pub const fn sleepctl(&self) -> &Sleepctl {
        &self.sleepctl
    }
    #[doc = "0x34 - JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
    #[inline(always)]
    pub const fn jtagcfg(&self) -> &Jtagcfg {
        &self.jtagcfg
    }
    #[doc = "0x3c - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    #[inline(always)]
    pub const fn jtagusercode(&self) -> &Jtagusercode {
        &self.jtagusercode
    }
    #[doc = "0xc4 - Configuration Load Value register"]
    #[inline(always)]
    pub const fn wdtload(&self) -> &Wdtload {
        &self.wdtload
    }
    #[doc = "0xc8 - Test Mode"]
    #[inline(always)]
    pub const fn wdttest(&self) -> &Wdttest {
        &self.wdttest
    }
    #[doc = "0xd0 - Lock"]
    #[inline(always)]
    pub const fn wdtlock(&self) -> &Wdtlock {
        &self.wdtlock
    }
}
#[doc = "AUXSCECLK (rw) register accessor: AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxsceclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxsceclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxsceclk`]
module"]
#[doc(alias = "AUXSCECLK")]
pub type Auxsceclk = crate::Reg<auxsceclk::AuxsceclkSpec>;
#[doc = "AUX SCE Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxsceclk;
#[doc = "RAMCFG (rw) register accessor: RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ramcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ramcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ramcfg`]
module"]
#[doc(alias = "RAMCFG")]
pub type Ramcfg = crate::Reg<ramcfg::RamcfgSpec>;
#[doc = "RAM Configuration This register contains power management related configuration for the SRAM in the MCU and AUX domain."]
pub mod ramcfg;
#[doc = "PWRCTL (rw) register accessor: Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctl`]
module"]
#[doc(alias = "PWRCTL")]
pub type Pwrctl = crate::Reg<pwrctl::PwrctlSpec>;
#[doc = "Power Management Control This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "PWRSTAT (rw) register accessor: AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrstat`]
module"]
#[doc(alias = "PWRSTAT")]
pub type Pwrstat = crate::Reg<pwrstat::PwrstatSpec>;
#[doc = "AON Power and Reset Status This register is used to monitor various power management related signals in AON. All other signals than JTAG_PD_ON, AUX_BUS_RESET_DONE, and AUX_RESET_DONE are for test, calibration and debug purpose only."]
pub mod pwrstat;
#[doc = "SHUTDOWN (rw) register accessor: Shutdown Control This register contains bitfields required for entering shutdown mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shutdown::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shutdown::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdown`]
module"]
#[doc(alias = "SHUTDOWN")]
pub type Shutdown = crate::Reg<shutdown::ShutdownSpec>;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "RECHARGECFG (rw) register accessor: Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rechargecfg`]
module"]
#[doc(alias = "RECHARGECFG")]
pub type Rechargecfg = crate::Reg<rechargecfg::RechargecfgSpec>;
#[doc = "Recharge Controller Configuration This register sets all relevant parameters for controlling the recharge algorithm."]
pub mod rechargecfg;
#[doc = "RECHARGESTAT (rw) register accessor: Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargestat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargestat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rechargestat`]
module"]
#[doc(alias = "RECHARGESTAT")]
pub type Rechargestat = crate::Reg<rechargestat::RechargestatSpec>;
#[doc = "Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
pub mod rechargestat;
#[doc = "OSCCFG (rw) register accessor: Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osccfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osccfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osccfg`]
module"]
#[doc(alias = "OSCCFG")]
pub type Osccfg = crate::Reg<osccfg::OsccfgSpec>;
#[doc = "Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
pub mod osccfg;
#[doc = "RESETCTL (rw) register accessor: Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetctl`]
module"]
#[doc(alias = "RESETCTL")]
pub type Resetctl = crate::Reg<resetctl::ResetctlSpec>;
#[doc = "Reset Management This register contains bitfields related to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "SLEEPCTL (rw) register accessor: Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepctl`]
module"]
#[doc(alias = "SLEEPCTL")]
pub type Sleepctl = crate::Reg<sleepctl::SleepctlSpec>;
#[doc = "Sleep Control This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;
#[doc = "JTAGCFG (rw) register accessor: JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtagcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtagcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtagcfg`]
module"]
#[doc(alias = "JTAGCFG")]
pub type Jtagcfg = crate::Reg<jtagcfg::JtagcfgSpec>;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain. This includes permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAGUSERCODE (rw) register accessor: JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtagusercode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtagusercode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtagusercode`]
module"]
#[doc(alias = "JTAGUSERCODE")]
pub type Jtagusercode = crate::Reg<jtagusercode::JtagusercodeSpec>;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;
#[doc = "WDTLOAD (rw) register accessor: Configuration Load Value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtload::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtload::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtload`]
module"]
#[doc(alias = "WDTLOAD")]
pub type Wdtload = crate::Reg<wdtload::WdtloadSpec>;
#[doc = "Configuration Load Value register"]
pub mod wdtload;
#[doc = "WDTTEST (rw) register accessor: Test Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdttest::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdttest::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdttest`]
module"]
#[doc(alias = "WDTTEST")]
pub type Wdttest = crate::Reg<wdttest::WdttestSpec>;
#[doc = "Test Mode"]
pub mod wdttest;
#[doc = "WDTLOCK (rw) register accessor: Lock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdtlock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdtlock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdtlock`]
module"]
#[doc(alias = "WDTLOCK")]
pub type Wdtlock = crate::Reg<wdtlock::WdtlockSpec>;
#[doc = "Lock"]
pub mod wdtlock;
