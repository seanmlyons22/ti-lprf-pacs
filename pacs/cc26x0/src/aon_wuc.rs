#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcuclk: Mcuclk,
    auxclk: Auxclk,
    mcucfg: Mcucfg,
    auxcfg: Auxcfg,
    auxctl: Auxctl,
    pwrstat: Pwrstat,
    shutdown: Shutdown,
    _reserved7: [u8; 0x04],
    ctl0: Ctl0,
    ctl1: Ctl1,
    _reserved9: [u8; 0x08],
    rechargecfg: Rechargecfg,
    rechargestat: Rechargestat,
    osccfg: Osccfg,
    _reserved12: [u8; 0x04],
    jtagcfg: Jtagcfg,
    jtagusercode: Jtagusercode,
}
impl RegisterBlock {
    #[doc = "0x00 - MCU Clock Management This register contains bitfields related to the MCU clock."]
    #[inline(always)]
    pub const fn mcuclk(&self) -> &Mcuclk {
        &self.mcuclk
    }
    #[doc = "0x04 - AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
    #[inline(always)]
    pub const fn auxclk(&self) -> &Auxclk {
        &self.auxclk
    }
    #[doc = "0x08 - MCU Configuration This register contains power management related bitfields for the MCU domain."]
    #[inline(always)]
    pub const fn mcucfg(&self) -> &Mcucfg {
        &self.mcucfg
    }
    #[doc = "0x0c - AUX Configuration This register contains power management related signals for the AUX domain."]
    #[inline(always)]
    pub const fn auxcfg(&self) -> &Auxcfg {
        &self.auxcfg
    }
    #[doc = "0x10 - AUX Control This register contains events and control signals for the AUX domain."]
    #[inline(always)]
    pub const fn auxctl(&self) -> &Auxctl {
        &self.auxctl
    }
    #[doc = "0x14 - Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
    #[inline(always)]
    pub const fn pwrstat(&self) -> &Pwrstat {
        &self.pwrstat
    }
    #[doc = "0x18 - Shutdown Control This register contains bitfields required for entering shutdown mode"]
    #[inline(always)]
    pub const fn shutdown(&self) -> &Shutdown {
        &self.shutdown
    }
    #[doc = "0x20 - Control 0 This register contains various chip level control and debug bitfields."]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x24 - Control 1 This register contains various chip level control and debug bitfields."]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x30 - Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
    #[inline(always)]
    pub const fn rechargecfg(&self) -> &Rechargecfg {
        &self.rechargecfg
    }
    #[doc = "0x34 - Recharge Controller Status This register controls various status registers which are updated during recharge. The register is mostly intended for test and debug."]
    #[inline(always)]
    pub const fn rechargestat(&self) -> &Rechargestat {
        &self.rechargestat
    }
    #[doc = "0x38 - Oscillator Configuration This register sets the period for Amplitude compensation requests sent to the oscillator control system. The amplitude compensations is only applicable when XOSC_HF is running in low power mode."]
    #[inline(always)]
    pub const fn osccfg(&self) -> &Osccfg {
        &self.osccfg
    }
    #[doc = "0x40 - JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
    #[inline(always)]
    pub const fn jtagcfg(&self) -> &Jtagcfg {
        &self.jtagcfg
    }
    #[doc = "0x44 - JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
    #[inline(always)]
    pub const fn jtagusercode(&self) -> &Jtagusercode {
        &self.jtagusercode
    }
}
#[doc = "MCUCLK (rw) register accessor: MCU Clock Management This register contains bitfields related to the MCU clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcuclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcuclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcuclk`]
module"]
#[doc(alias = "MCUCLK")]
pub type Mcuclk = crate::Reg<mcuclk::McuclkSpec>;
#[doc = "MCU Clock Management This register contains bitfields related to the MCU clock."]
pub mod mcuclk;
#[doc = "AUXCLK (rw) register accessor: AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxclk`]
module"]
#[doc(alias = "AUXCLK")]
pub type Auxclk = crate::Reg<auxclk::AuxclkSpec>;
#[doc = "AUX Clock Management This register contains bitfields that are relevant for setting up the clock to the AUX domain."]
pub mod auxclk;
#[doc = "MCUCFG (rw) register accessor: MCU Configuration This register contains power management related bitfields for the MCU domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcucfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcucfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcucfg`]
module"]
#[doc(alias = "MCUCFG")]
pub type Mcucfg = crate::Reg<mcucfg::McucfgSpec>;
#[doc = "MCU Configuration This register contains power management related bitfields for the MCU domain."]
pub mod mcucfg;
#[doc = "AUXCFG (rw) register accessor: AUX Configuration This register contains power management related signals for the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxcfg`]
module"]
#[doc(alias = "AUXCFG")]
pub type Auxcfg = crate::Reg<auxcfg::AuxcfgSpec>;
#[doc = "AUX Configuration This register contains power management related signals for the AUX domain."]
pub mod auxcfg;
#[doc = "AUXCTL (rw) register accessor: AUX Control This register contains events and control signals for the AUX domain.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxctl`]
module"]
#[doc(alias = "AUXCTL")]
pub type Auxctl = crate::Reg<auxctl::AuxctlSpec>;
#[doc = "AUX Control This register contains events and control signals for the AUX domain."]
pub mod auxctl;
#[doc = "PWRSTAT (rw) register accessor: Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrstat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrstat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrstat`]
module"]
#[doc(alias = "PWRSTAT")]
pub type Pwrstat = crate::Reg<pwrstat::PwrstatSpec>;
#[doc = "Power Status This register is used to monitor various power management related signals in AON. Most signals are for test, calibration and debug purpose only, and others can be used to detect that AUX or JTAG domains are powered up."]
pub mod pwrstat;
#[doc = "SHUTDOWN (rw) register accessor: Shutdown Control This register contains bitfields required for entering shutdown mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shutdown::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shutdown::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shutdown`]
module"]
#[doc(alias = "SHUTDOWN")]
pub type Shutdown = crate::Reg<shutdown::ShutdownSpec>;
#[doc = "Shutdown Control This register contains bitfields required for entering shutdown mode"]
pub mod shutdown;
#[doc = "CTL0 (rw) register accessor: Control 0 This register contains various chip level control and debug bitfields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "Control 0 This register contains various chip level control and debug bitfields."]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: Control 1 This register contains various chip level control and debug bitfields.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "Control 1 This register contains various chip level control and debug bitfields."]
pub mod ctl1;
#[doc = "RECHARGECFG (rw) register accessor: Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rechargecfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rechargecfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rechargecfg`]
module"]
#[doc(alias = "RECHARGECFG")]
pub type Rechargecfg = crate::Reg<rechargecfg::RechargecfgSpec>;
#[doc = "Recharge Controller Configuration This register sets all relevant patameters for controlling the recharge algorithm."]
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
#[doc = "JTAGCFG (rw) register accessor: JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtagcfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtagcfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtagcfg`]
module"]
#[doc(alias = "JTAGCFG")]
pub type Jtagcfg = crate::Reg<jtagcfg::JtagcfgSpec>;
#[doc = "JTAG Configuration This register contains control for configuration of the JTAG domain,- hereunder access permissions for each TAP."]
pub mod jtagcfg;
#[doc = "JTAGUSERCODE (rw) register accessor: JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jtagusercode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jtagusercode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jtagusercode`]
module"]
#[doc(alias = "JTAGUSERCODE")]
pub type Jtagusercode = crate::Reg<jtagusercode::JtagusercodeSpec>;
#[doc = "JTAG USERCODE Boot code copies the JTAG USERCODE to this register from where it is forwarded to the debug subsystem."]
pub mod jtagusercode;
