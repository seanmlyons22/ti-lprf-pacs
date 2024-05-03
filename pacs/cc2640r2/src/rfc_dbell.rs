#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cmdr: Cmdr,
    cmdsta: Cmdsta,
    rfhwifg: Rfhwifg,
    rfhwien: Rfhwien,
    rfcpeifg: Rfcpeifg,
    rfcpeien: Rfcpeien,
    rfcpeisl: Rfcpeisl,
    rfackifg: Rfackifg,
    sysgpoctl: Sysgpoctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Doorbell Command Register"]
    #[inline(always)]
    pub const fn cmdr(&self) -> &Cmdr {
        &self.cmdr
    }
    #[doc = "0x04 - Doorbell Command Status Register"]
    #[inline(always)]
    pub const fn cmdsta(&self) -> &Cmdsta {
        &self.cmdsta
    }
    #[doc = "0x08 - Interrupt Flags From RF Hardware Modules"]
    #[inline(always)]
    pub const fn rfhwifg(&self) -> &Rfhwifg {
        &self.rfhwifg
    }
    #[doc = "0x0c - Interrupt Enable For RF Hardware Modules"]
    #[inline(always)]
    pub const fn rfhwien(&self) -> &Rfhwien {
        &self.rfhwien
    }
    #[doc = "0x10 - Interrupt Flags For Command and Packet Engine Generated Interrupts"]
    #[inline(always)]
    pub const fn rfcpeifg(&self) -> &Rfcpeifg {
        &self.rfcpeifg
    }
    #[doc = "0x14 - Interrupt Enable For Command and Packet Engine Generated Interrupts"]
    #[inline(always)]
    pub const fn rfcpeien(&self) -> &Rfcpeien {
        &self.rfcpeien
    }
    #[doc = "0x18 - Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
    #[inline(always)]
    pub const fn rfcpeisl(&self) -> &Rfcpeisl {
        &self.rfcpeisl
    }
    #[doc = "0x1c - Doorbell Command Acknowledgement Interrupt Flag"]
    #[inline(always)]
    pub const fn rfackifg(&self) -> &Rfackifg {
        &self.rfackifg
    }
    #[doc = "0x20 - RF Core General Purpose Output Control"]
    #[inline(always)]
    pub const fn sysgpoctl(&self) -> &Sysgpoctl {
        &self.sysgpoctl
    }
}
#[doc = "CMDR (rw) register accessor: Doorbell Command Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdr`]
module"]
#[doc(alias = "CMDR")]
pub type Cmdr = crate::Reg<cmdr::CmdrSpec>;
#[doc = "Doorbell Command Register"]
pub mod cmdr;
#[doc = "CMDSTA (rw) register accessor: Doorbell Command Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmdsta::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmdsta::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmdsta`]
module"]
#[doc(alias = "CMDSTA")]
pub type Cmdsta = crate::Reg<cmdsta::CmdstaSpec>;
#[doc = "Doorbell Command Status Register"]
pub mod cmdsta;
#[doc = "RFHWIFG (rw) register accessor: Interrupt Flags From RF Hardware Modules\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfhwifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfhwifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfhwifg`]
module"]
#[doc(alias = "RFHWIFG")]
pub type Rfhwifg = crate::Reg<rfhwifg::RfhwifgSpec>;
#[doc = "Interrupt Flags From RF Hardware Modules"]
pub mod rfhwifg;
#[doc = "RFHWIEN (rw) register accessor: Interrupt Enable For RF Hardware Modules\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfhwien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfhwien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfhwien`]
module"]
#[doc(alias = "RFHWIEN")]
pub type Rfhwien = crate::Reg<rfhwien::RfhwienSpec>;
#[doc = "Interrupt Enable For RF Hardware Modules"]
pub mod rfhwien;
#[doc = "RFCPEIFG (rw) register accessor: Interrupt Flags For Command and Packet Engine Generated Interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcpeifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcpeifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcpeifg`]
module"]
#[doc(alias = "RFCPEIFG")]
pub type Rfcpeifg = crate::Reg<rfcpeifg::RfcpeifgSpec>;
#[doc = "Interrupt Flags For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeifg;
#[doc = "RFCPEIEN (rw) register accessor: Interrupt Enable For Command and Packet Engine Generated Interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcpeien::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcpeien::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcpeien`]
module"]
#[doc(alias = "RFCPEIEN")]
pub type Rfcpeien = crate::Reg<rfcpeien::RfcpeienSpec>;
#[doc = "Interrupt Enable For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeien;
#[doc = "RFCPEISL (rw) register accessor: Interrupt Vector Selection For Command and Packet Engine Generated Interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcpeisl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcpeisl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcpeisl`]
module"]
#[doc(alias = "RFCPEISL")]
pub type Rfcpeisl = crate::Reg<rfcpeisl::RfcpeislSpec>;
#[doc = "Interrupt Vector Selection For Command and Packet Engine Generated Interrupts"]
pub mod rfcpeisl;
#[doc = "RFACKIFG (rw) register accessor: Doorbell Command Acknowledgement Interrupt Flag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfackifg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfackifg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfackifg`]
module"]
#[doc(alias = "RFACKIFG")]
pub type Rfackifg = crate::Reg<rfackifg::RfackifgSpec>;
#[doc = "Doorbell Command Acknowledgement Interrupt Flag"]
pub mod rfackifg;
#[doc = "SYSGPOCTL (rw) register accessor: RF Core General Purpose Output Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sysgpoctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysgpoctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysgpoctl`]
module"]
#[doc(alias = "SYSGPOCTL")]
pub type Sysgpoctl = crate::Reg<sysgpoctl::SysgpoctlSpec>;
#[doc = "RF Core General Purpose Output Control"]
pub mod sysgpoctl;
