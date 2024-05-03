#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    dhcsr: Dhcsr,
    dcrsr: Dcrsr,
    dcrdr: Dcrdr,
    demcr: Demcr,
    _reserved4: [u8; 0x04],
    dauthctrl: Dauthctrl,
    dscsr: Dscsr,
}
impl RegisterBlock {
    #[doc = "0x10 - Controls halting debug"]
    #[inline(always)]
    pub const fn dhcsr(&self) -> &Dhcsr {
        &self.dhcsr
    }
    #[doc = "0x14 - With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer"]
    #[inline(always)]
    pub const fn dcrsr(&self) -> &Dcrsr {
        &self.dcrsr
    }
    #[doc = "0x18 - With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE"]
    #[inline(always)]
    pub const fn dcrdr(&self) -> &Dcrdr {
        &self.dcrdr
    }
    #[doc = "0x1c - Manages vector catch behavior and DebugMonitor handling when debugging"]
    #[inline(always)]
    pub const fn demcr(&self) -> &Demcr {
        &self.demcr
    }
    #[doc = "0x24 - This register allows the external authentication interface to be overridden from software."]
    #[inline(always)]
    pub const fn dauthctrl(&self) -> &Dauthctrl {
        &self.dauthctrl
    }
    #[doc = "0x28 - Provides control and status information for Secure debug"]
    #[inline(always)]
    pub const fn dscsr(&self) -> &Dscsr {
        &self.dscsr
    }
}
#[doc = "DHCSR (rw) register accessor: Controls halting debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dhcsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dhcsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhcsr`]
module"]
#[doc(alias = "DHCSR")]
pub type Dhcsr = crate::Reg<dhcsr::DhcsrSpec>;
#[doc = "Controls halting debug"]
pub mod dhcsr;
#[doc = "DCRSR (rw) register accessor: With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrsr`]
module"]
#[doc(alias = "DCRSR")]
pub type Dcrsr = crate::Reg<dcrsr::DcrsrSpec>;
#[doc = "With the DCRDR, provides debug access to the general-purpose registers, special-purpose registers, and the FP extension registers. A write to the DCRSR specifies the register to transfer, whether the transfer is a read or write, and starts the transfer"]
pub mod dcrsr;
#[doc = "DCRDR (rw) register accessor: With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcrdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcrdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcrdr`]
module"]
#[doc(alias = "DCRDR")]
pub type Dcrdr = crate::Reg<dcrdr::DcrdrSpec>;
#[doc = "With the DCRSR, provides debug access to the general-purpose registers, special-purpose registers, and the FP Extension registers. If the Main Extension is implemented, it can also be used for message passing between an external debugger and a debug agent running on the PE"]
pub mod dcrdr;
#[doc = "DEMCR (rw) register accessor: Manages vector catch behavior and DebugMonitor handling when debugging\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`demcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`demcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@demcr`]
module"]
#[doc(alias = "DEMCR")]
pub type Demcr = crate::Reg<demcr::DemcrSpec>;
#[doc = "Manages vector catch behavior and DebugMonitor handling when debugging"]
pub mod demcr;
#[doc = "DAUTHCTRL (rw) register accessor: This register allows the external authentication interface to be overridden from software.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dauthctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dauthctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dauthctrl`]
module"]
#[doc(alias = "DAUTHCTRL")]
pub type Dauthctrl = crate::Reg<dauthctrl::DauthctrlSpec>;
#[doc = "This register allows the external authentication interface to be overridden from software."]
pub mod dauthctrl;
#[doc = "DSCSR (rw) register accessor: Provides control and status information for Secure debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dscsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dscsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dscsr`]
module"]
#[doc(alias = "DSCSR")]
pub type Dscsr = crate::Reg<dscsr::DscsrSpec>;
#[doc = "Provides control and status information for Secure debug"]
pub mod dscsr;
