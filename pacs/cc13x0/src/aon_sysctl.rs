#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwrctl: Pwrctl,
    resetctl: Resetctl,
    sleepctl: Sleepctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
    #[inline(always)]
    pub const fn pwrctl(&self) -> &Pwrctl {
        &self.pwrctl
    }
    #[doc = "0x04 - Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
    #[inline(always)]
    pub const fn resetctl(&self) -> &Resetctl {
        &self.resetctl
    }
    #[doc = "0x08 - Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
    #[inline(always)]
    pub const fn sleepctl(&self) -> &Sleepctl {
        &self.sleepctl
    }
}
#[doc = "PWRCTL (rw) register accessor: Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwrctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwrctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctl`]
module"]
#[doc(alias = "PWRCTL")]
pub type Pwrctl = crate::Reg<pwrctl::PwrctlSpec>;
#[doc = "Power Management This register controls bitfields for setting low level power management features such as selection of regulator for VDDR supply and control of IO ring where certain segments can be enabled / disabled."]
pub mod pwrctl;
#[doc = "RESETCTL (rw) register accessor: Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`resetctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`resetctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@resetctl`]
module"]
#[doc(alias = "RESETCTL")]
pub type Resetctl = crate::Reg<resetctl::ResetctlSpec>;
#[doc = "Reset Management This register contains bitfields releated to system reset such as reset source and reset request and control of brown out resets."]
pub mod resetctl;
#[doc = "SLEEPCTL (rw) register accessor: Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sleepctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sleepctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepctl`]
module"]
#[doc(alias = "SLEEPCTL")]
pub type Sleepctl = crate::Reg<sleepctl::SleepctlSpec>;
#[doc = "Sleep Mode This register is used to unfreeze the IO pad ring after waking up from SHUTDOWN"]
pub mod sleepctl;
