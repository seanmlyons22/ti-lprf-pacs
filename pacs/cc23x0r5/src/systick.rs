#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    rvr: Rvr,
    cvr: Cvr,
    calib: Calib,
}
impl RegisterBlock {
    #[doc = "0x00 - Use the SysTick Control and Status Register to enable the SysTick features."]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN."]
    #[inline(always)]
    pub const fn rvr(&self) -> &Rvr {
        &self.rvr
    }
    #[doc = "0x08 - Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
    #[inline(always)]
    pub const fn cvr(&self) -> &Cvr {
        &self.cvr
    }
    #[doc = "0x0c - Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
    #[inline(always)]
    pub const fn calib(&self) -> &Calib {
        &self.calib
    }
}
#[doc = "CSR (rw) register accessor: Use the SysTick Control and Status Register to enable the SysTick features.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features."]
pub mod csr;
#[doc = "RVR (rw) register accessor: Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rvr`]
module"]
#[doc(alias = "RVR")]
pub type Rvr = crate::Reg<rvr::RvrSpec>;
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN."]
pub mod rvr;
#[doc = "CVR (rw) register accessor: Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cvr`]
module"]
#[doc(alias = "CVR")]
pub type Cvr = crate::Reg<cvr::CvrSpec>;
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
pub mod cvr;
#[doc = "CALIB (rw) register accessor: Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calib::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calib::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calib`]
module"]
#[doc(alias = "CALIB")]
pub type Calib = crate::Reg<calib::CalibSpec>;
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
pub mod calib;
