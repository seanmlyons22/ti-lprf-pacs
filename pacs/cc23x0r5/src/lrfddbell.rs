#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    clkctl: Clkctl,
    dmacfg: Dmacfg,
    systimoev: Systimoev,
    systdmatrig: Systdmatrig,
    gposel0: Gposel0,
    gposel1: Gposel1,
    _reserved7: [u8; 0x28],
    imask0: Imask0,
    ris0: Ris0,
    mis0: Mis0,
    iset0: Iset0,
    iclr0: Iclr0,
    _reserved12: [u8; 0x2c],
    imask1: Imask1,
    ris1: Ris1,
    mis1: Mis1,
    iset1: Iset1,
    iclr1: Iclr1,
    _reserved17: [u8; 0x2c],
    imask2: Imask2,
    ris2: Ris2,
    mis2: Mis2,
    iset2: Iset2,
    iclr2: Iclr2,
}
impl RegisterBlock {
    #[doc = "0x00 - This register identifies the peripheral and its exact version."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x04 - Controls the functional clock gates for the individual sub-modules. Writing a bit to zero does not necessarily switch off the corresponding clock. It can also be requested internally. A clock will only be switched off if internal and external requests are removed"]
    #[inline(always)]
    pub const fn clkctl(&self) -> &Clkctl {
        &self.clkctl
    }
    #[doc = "0x08 - DMA Configuration"]
    #[inline(always)]
    pub const fn dmacfg(&self) -> &Dmacfg {
        &self.dmacfg
    }
    #[doc = "0x0c - Systimer Output Event Control Register. Controls routing of internal events to the three systimer output events"]
    #[inline(always)]
    pub const fn systimoev(&self) -> &Systimoev {
        &self.systimoev
    }
    #[doc = "0x10 - Manual triggering of systimer capture event or DMA trigger This comes on top of any HW driven sources configured in SYSTIMOEV"]
    #[inline(always)]
    pub const fn systdmatrig(&self) -> &Systdmatrig {
        &self.systdmatrig
    }
    #[doc = "0x14 - Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines"]
    #[inline(always)]
    pub const fn gposel0(&self) -> &Gposel0 {
        &self.gposel0
    }
    #[doc = "0x18 - Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines"]
    #[inline(always)]
    pub const fn gposel1(&self) -> &Gposel1 {
        &self.gposel1
    }
    #[doc = "0x44 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask0(&self) -> &Imask0 {
        &self.imask0
    }
    #[doc = "0x48 - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn ris0(&self) -> &Ris0 {
        &self.ris0
    }
    #[doc = "0x4c - Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn mis0(&self) -> &Mis0 {
        &self.mis0
    }
    #[doc = "0x50 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset0(&self) -> &Iset0 {
        &self.iset0
    }
    #[doc = "0x54 - Interrupt clear. Write a 1 to clear corresponding Interrupt."]
    #[inline(always)]
    pub const fn iclr0(&self) -> &Iclr0 {
        &self.iclr0
    }
    #[doc = "0x84 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask1(&self) -> &Imask1 {
        &self.imask1
    }
    #[doc = "0x88 - Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS0 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
    #[inline(always)]
    pub const fn ris1(&self) -> &Ris1 {
        &self.ris1
    }
    #[doc = "0x8c - Masked interrupt status. This is an AND of the IMASK and RIS registers."]
    #[inline(always)]
    pub const fn mis1(&self) -> &Mis1 {
        &self.mis1
    }
    #[doc = "0x90 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset1(&self) -> &Iset1 {
        &self.iset1
    }
    #[doc = "0x94 - Interrupt clear. Write a 1 to clear corresponding Interrupt."]
    #[inline(always)]
    pub const fn iclr1(&self) -> &Iclr1 {
        &self.iclr1
    }
    #[doc = "0xc4 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask2(&self) -> &Imask2 {
        &self.imask2
    }
    #[doc = "0xc8 - Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS0 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
    #[inline(always)]
    pub const fn ris2(&self) -> &Ris2 {
        &self.ris2
    }
    #[doc = "0xcc - Masked interrupt status. This is an AND of the IMASK and RIS registers."]
    #[inline(always)]
    pub const fn mis2(&self) -> &Mis2 {
        &self.mis2
    }
    #[doc = "0xd0 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset2(&self) -> &Iset2 {
        &self.iset2
    }
    #[doc = "0xd4 - Interrupt clear. Write a 1 to clear corresponding Interrupt."]
    #[inline(always)]
    pub const fn iclr2(&self) -> &Iclr2 {
        &self.iclr2
    }
}
#[doc = "DESC (rw) register accessor: This register identifies the peripheral and its exact version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "This register identifies the peripheral and its exact version."]
pub mod desc;
#[doc = "CLKCTL (rw) register accessor: Controls the functional clock gates for the individual sub-modules. Writing a bit to zero does not necessarily switch off the corresponding clock. It can also be requested internally. A clock will only be switched off if internal and external requests are removed\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkctl`]
module"]
#[doc(alias = "CLKCTL")]
pub type Clkctl = crate::Reg<clkctl::ClkctlSpec>;
#[doc = "Controls the functional clock gates for the individual sub-modules. Writing a bit to zero does not necessarily switch off the corresponding clock. It can also be requested internally. A clock will only be switched off if internal and external requests are removed"]
pub mod clkctl;
#[doc = "DMACFG (rw) register accessor: DMA Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacfg`]
module"]
#[doc(alias = "DMACFG")]
pub type Dmacfg = crate::Reg<dmacfg::DmacfgSpec>;
#[doc = "DMA Configuration"]
pub mod dmacfg;
#[doc = "SYSTIMOEV (rw) register accessor: Systimer Output Event Control Register. Controls routing of internal events to the three systimer output events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systimoev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimoev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systimoev`]
module"]
#[doc(alias = "SYSTIMOEV")]
pub type Systimoev = crate::Reg<systimoev::SystimoevSpec>;
#[doc = "Systimer Output Event Control Register. Controls routing of internal events to the three systimer output events"]
pub mod systimoev;
#[doc = "SYSTDMATRIG (rw) register accessor: Manual triggering of systimer capture event or DMA trigger This comes on top of any HW driven sources configured in SYSTIMOEV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`systdmatrig::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systdmatrig::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@systdmatrig`]
module"]
#[doc(alias = "SYSTDMATRIG")]
pub type Systdmatrig = crate::Reg<systdmatrig::SystdmatrigSpec>;
#[doc = "Manual triggering of systimer capture event or DMA trigger This comes on top of any HW driven sources configured in SYSTIMOEV"]
pub mod systdmatrig;
#[doc = "GPOSEL0 (rw) register accessor: Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gposel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gposel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gposel0`]
module"]
#[doc(alias = "GPOSEL0")]
pub type Gposel0 = crate::Reg<gposel0::Gposel0Spec>;
#[doc = "Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines"]
pub mod gposel0;
#[doc = "GPOSEL1 (rw) register accessor: Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gposel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gposel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gposel1`]
module"]
#[doc(alias = "GPOSEL1")]
pub type Gposel1 = crate::Reg<gposel1::Gposel1Spec>;
#[doc = "Controls routing of GPO signals from MDM, RFE and PBE to the radio GPO lines"]
pub mod gposel1;
#[doc = "IMASK0 (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask0`]
module"]
#[doc(alias = "IMASK0")]
pub type Imask0 = crate::Reg<imask0::Imask0Spec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask0;
#[doc = "RIS0 (rw) register accessor: Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris0`]
module"]
#[doc(alias = "RIS0")]
pub type Ris0 = crate::Reg<ris0::Ris0Spec>;
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod ris0;
#[doc = "MIS0 (rw) register accessor: Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis0`]
module"]
#[doc(alias = "MIS0")]
pub type Mis0 = crate::Reg<mis0::Mis0Spec>;
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod mis0;
#[doc = "ISET0 (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset0`]
module"]
#[doc(alias = "ISET0")]
pub type Iset0 = crate::Reg<iset0::Iset0Spec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset0;
#[doc = "ICLR0 (rw) register accessor: Interrupt clear. Write a 1 to clear corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr0`]
module"]
#[doc(alias = "ICLR0")]
pub type Iclr0 = crate::Reg<iclr0::Iclr0Spec>;
#[doc = "Interrupt clear. Write a 1 to clear corresponding Interrupt."]
pub mod iclr0;
#[doc = "IMASK1 (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask1`]
module"]
#[doc(alias = "IMASK1")]
pub type Imask1 = crate::Reg<imask1::Imask1Spec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask1;
#[doc = "RIS1 (rw) register accessor: Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS0 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris1`]
module"]
#[doc(alias = "RIS1")]
pub type Ris1 = crate::Reg<ris1::Ris1Spec>;
#[doc = "Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS0 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
pub mod ris1;
#[doc = "MIS1 (rw) register accessor: Masked interrupt status. This is an AND of the IMASK and RIS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis1`]
module"]
#[doc(alias = "MIS1")]
pub type Mis1 = crate::Reg<mis1::Mis1Spec>;
#[doc = "Masked interrupt status. This is an AND of the IMASK and RIS registers."]
pub mod mis1;
#[doc = "ISET1 (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset1`]
module"]
#[doc(alias = "ISET1")]
pub type Iset1 = crate::Reg<iset1::Iset1Spec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset1;
#[doc = "ICLR1 (rw) register accessor: Interrupt clear. Write a 1 to clear corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr1`]
module"]
#[doc(alias = "ICLR1")]
pub type Iclr1 = crate::Reg<iclr1::Iclr1Spec>;
#[doc = "Interrupt clear. Write a 1 to clear corresponding Interrupt."]
pub mod iclr1;
#[doc = "IMASK2 (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask2`]
module"]
#[doc(alias = "IMASK2")]
pub type Imask2 = crate::Reg<imask2::Imask2Spec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask2;
#[doc = "RIS2 (rw) register accessor: Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS0 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris2`]
module"]
#[doc(alias = "RIS2")]
pub type Ris2 = crate::Reg<ris2::Ris2Spec>;
#[doc = "Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS0 register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
pub mod ris2;
#[doc = "MIS2 (rw) register accessor: Masked interrupt status. This is an AND of the IMASK and RIS registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis2`]
module"]
#[doc(alias = "MIS2")]
pub type Mis2 = crate::Reg<mis2::Mis2Spec>;
#[doc = "Masked interrupt status. This is an AND of the IMASK and RIS registers."]
pub mod mis2;
#[doc = "ISET2 (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset2`]
module"]
#[doc(alias = "ISET2")]
pub type Iset2 = crate::Reg<iset2::Iset2Spec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset2;
#[doc = "ICLR2 (rw) register accessor: Interrupt clear. Write a 1 to clear corresponding Interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr2`]
module"]
#[doc(alias = "ICLR2")]
pub type Iclr2 = crate::Reg<iclr2::Iclr2Spec>;
#[doc = "Interrupt clear. Write a 1 to clear corresponding Interrupt."]
pub mod iclr2;
