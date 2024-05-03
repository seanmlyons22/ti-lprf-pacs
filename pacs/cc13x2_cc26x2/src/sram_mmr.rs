#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    per_ctl: PerCtl,
    per_chk: PerChk,
    per_dbg: PerDbg,
    mem_ctl: MemCtl,
}
impl RegisterBlock {
    #[doc = "0x00 - Parity Error Control Parity error check controls"]
    #[inline(always)]
    pub const fn per_ctl(&self) -> &PerCtl {
        &self.per_ctl
    }
    #[doc = "0x04 - Parity Error Check Parity error check results"]
    #[inline(always)]
    pub const fn per_chk(&self) -> &PerChk {
        &self.per_chk
    }
    #[doc = "0x08 - Parity Error Debug Parity error check debug address setting"]
    #[inline(always)]
    pub const fn per_dbg(&self) -> &PerDbg {
        &self.per_dbg
    }
    #[doc = "0x0c - Memory Control Controls memory initialization"]
    #[inline(always)]
    pub const fn mem_ctl(&self) -> &MemCtl {
        &self.mem_ctl
    }
}
#[doc = "PER_CTL (rw) register accessor: Parity Error Control Parity error check controls\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_ctl`]
module"]
#[doc(alias = "PER_CTL")]
pub type PerCtl = crate::Reg<per_ctl::PerCtlSpec>;
#[doc = "Parity Error Control Parity error check controls"]
pub mod per_ctl;
#[doc = "PER_CHK (rw) register accessor: Parity Error Check Parity error check results\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_chk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_chk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_chk`]
module"]
#[doc(alias = "PER_CHK")]
pub type PerChk = crate::Reg<per_chk::PerChkSpec>;
#[doc = "Parity Error Check Parity error check results"]
pub mod per_chk;
#[doc = "PER_DBG (rw) register accessor: Parity Error Debug Parity error check debug address setting\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`per_dbg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`per_dbg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@per_dbg`]
module"]
#[doc(alias = "PER_DBG")]
pub type PerDbg = crate::Reg<per_dbg::PerDbgSpec>;
#[doc = "Parity Error Debug Parity error check debug address setting"]
pub mod per_dbg;
#[doc = "MEM_CTL (rw) register accessor: Memory Control Controls memory initialization\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mem_ctl`]
module"]
#[doc(alias = "MEM_CTL")]
pub type MemCtl = crate::Reg<mem_ctl::MemCtlSpec>;
#[doc = "Memory Control Controls memory initialization"]
pub mod mem_ctl;
