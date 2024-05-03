#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iostrmin: Iostrmin,
    iostrmed: Iostrmed,
    iostrmax: Iostrmax,
    ioclatch: Ioclatch,
    clk32kctl: Clk32kctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn iostrmin(&self) -> &Iostrmin {
        &self.iostrmin
    }
    #[doc = "0x04 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn iostrmed(&self) -> &Iostrmed {
        &self.iostrmed
    }
    #[doc = "0x08 - Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub const fn iostrmax(&self) -> &Iostrmax {
        &self.iostrmax
    }
    #[doc = "0x0c - IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC"]
    #[inline(always)]
    pub const fn ioclatch(&self) -> &Ioclatch {
        &self.ioclatch
    }
    #[doc = "0x10 - SCLK_LF External Output Control"]
    #[inline(always)]
    pub const fn clk32kctl(&self) -> &Clk32kctl {
        &self.clk32kctl
    }
}
#[doc = "IOSTRMIN (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iostrmin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iostrmin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iostrmin`]
module"]
#[doc(alias = "IOSTRMIN")]
pub type Iostrmin = crate::Reg<iostrmin::IostrminSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmin;
#[doc = "IOSTRMED (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iostrmed::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iostrmed::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iostrmed`]
module"]
#[doc(alias = "IOSTRMED")]
pub type Iostrmed = crate::Reg<iostrmed::IostrmedSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmed;
#[doc = "IOSTRMAX (rw) register accessor: Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iostrmax::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iostrmax::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iostrmax`]
module"]
#[doc(alias = "IOSTRMAX")]
pub type Iostrmax = crate::Reg<iostrmax::IostrmaxSpec>;
#[doc = "Internal. Only to be used through TI provided API."]
pub mod iostrmax;
#[doc = "IOCLATCH (rw) register accessor: IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioclatch::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioclatch::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioclatch`]
module"]
#[doc(alias = "IOCLATCH")]
pub type Ioclatch = crate::Reg<ioclatch::IoclatchSpec>;
#[doc = "IO Latch Control Controls transparency of all latches holding I/O or configuration state from the MCU IOC"]
pub mod ioclatch;
#[doc = "CLK32KCTL (rw) register accessor: SCLK_LF External Output Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk32kctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk32kctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk32kctl`]
module"]
#[doc(alias = "CLK32KCTL")]
pub type Clk32kctl = crate::Reg<clk32kctl::Clk32kctlSpec>;
#[doc = "SCLK_LF External Output Control"]
pub mod clk32kctl;
