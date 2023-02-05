#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    _reserved1: [u8; 0x40],
    imask: Imask,
    _reserved2: [u8; 0x04],
    ris: Ris,
    _reserved3: [u8; 0x04],
    mis: Mis,
    _reserved4: [u8; 0x04],
    iset: Iset,
    _reserved5: [u8; 0x04],
    iclr: Iclr,
    _reserved6: [u8; 0x04],
    imset: Imset,
    _reserved7: [u8; 0x04],
    imclr: Imclr,
    _reserved8: [u8; 0x88],
    txd: Txd,
    txctl: Txctl,
    rxd: Rxd,
    rxctl: Rxctl,
    txdpeek: Txdpeek,
    rxdpeek: Rxdpeek,
    _reserved14: [u8; 0xe8],
    special_auth: SpecialAuth,
    special_auth_set: SpecialAuthSet,
    special_auth_clr: SpecialAuthClr,
    _reserved17: [u8; 0x04],
    app_auth: AppAuth,
    app_auth_set: AppAuthSet,
    app_auth_clr: AppAuthClr,
    dbgctl: Dbgctl,
}
impl RegisterBlock {
    #[doc = "0x00 - Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
    #[inline(always)]
    pub const fn desc(&self) -> &Desc {
        &self.desc
    }
    #[doc = "0x44 - Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
    #[inline(always)]
    pub const fn imask(&self) -> &Imask {
        &self.imask
    }
    #[doc = "0x4c - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x54 - Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x5c - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x64 - Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x6c - Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imset(&self) -> &Imset {
        &self.imset
    }
    #[doc = "0x74 - Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imclr(&self) -> &Imclr {
        &self.imclr
    }
    #[doc = "0x100 - Transmit data register. This register is used for sending SACI command parameter data from the host to the device. The host (SWD interface) can write this register. This updates the value of TXD, and sets TXCTL.TXDSTA = FULL The host should only write TXD while TXCTL.TXDSTA = EMPTY. If the host incorrectly writes TXD while TXCTL.TXDSTA = FULL, this will just update the value of TXD. The host (SWD interface) can read the TXD register. This does not affect TXCTL.TXDSTA. The device (boot code) can only read the TXD register. This sets TXCTL.TXDSTA = EMPTY. The device should only read TXD while TXCTL.TXDSTA = FULL. If the device incorrectly reads TXD while TXCTL.TXDSTA = EMPTY, this will just return the value of TXD. If the host writes TXD on the same clock cycle as the device reads TXD: The device reads the old TXD value. TXD is updated with the new value, and TXCTL.TXDSTA is set to FULL."]
    #[inline(always)]
    pub const fn txd(&self) -> &Txd {
        &self.txd
    }
    #[doc = "0x104 - Transmit control register. This register contains status of the TXD register (full/empty), and also software defined flags that are used by the SACI protocol. The host (SWD interface) can write the FLAGS field of the TXCTL register. The host (SWD interface) can read the TXCTL register. The device (boot code) can only read the TXCTL register."]
    #[inline(always)]
    pub const fn txctl(&self) -> &Txctl {
        &self.txctl
    }
    #[doc = "0x108 - Receive data register. This register is used to send SACI command response data from the device to the host. The device (boot code) can write the RXD register. This updates the value of RXD, and sets RXCTL.RXDSTA = FULL. The device should only write RXD while RXCTL.RXDSTA = EMPTY. If the device incorrectly writes RXD while RXCTL.RXDSTA = FULL, this will just update the value of RXD. The device (boot code) can read the RXD register in order to flush it. This sets RXCTL.RXDSTA = EMPTY. The host (SWD interface) can only read the RXD register. This sets RXCTL.RXDSTA = EMPTY. The host should only read RXD while RXCTL.RXDSTA = FULL. If the host incorrectly reads RXD while RXCTL.RXDSTA = EMPTY, this will just return the value of RXD. If the device writes RXD on the same clock cycle as the host reads RXD: The host reads the old RXD value. RXD is updated with the new value, and RXCTL.RXDSTA is set to FULL."]
    #[inline(always)]
    pub const fn rxd(&self) -> &Rxd {
        &self.rxd
    }
    #[doc = "0x10c - Receive control register. This register contains status of the RXD register (full/empty), and also software defined flags that are used by the SACI protocol. The device (boot code) can write the FLAGS field of the RXCTL register. The device (boot code) can read the RXCTL register. The host (SWD interface) can only read the RXCTL register"]
    #[inline(always)]
    pub const fn rxctl(&self) -> &Rxctl {
        &self.rxctl
    }
    #[doc = "0x110 - Transmit data peek register . This register is a read-only version of the TXD register that can be read by host and device without any side-effects. This register is used to peek at the values in TXD without affecting the FULL/EMPTY flag."]
    #[inline(always)]
    pub const fn txdpeek(&self) -> &Txdpeek {
        &self.txdpeek
    }
    #[doc = "0x114 - Receive data peek register. The RXDPEEK register is a read-only version of the RXD register that can be read by host and device without any side-effects This register is used to peek at the values in Receive Data Register without affecting the FULL/EMPTY flag."]
    #[inline(always)]
    pub const fn rxdpeek(&self) -> &Rxdpeek {
        &self.rxdpeek
    }
    #[doc = "0x200 - This register indicates the status of different AP firewalls and fakestandby enable."]
    #[inline(always)]
    pub const fn special_auth(&self) -> &SpecialAuth {
        &self.special_auth
    }
    #[doc = "0x204 - This register is used for setting bits in SPECIAL_AUTH register. This register is configured and locked during device boot."]
    #[inline(always)]
    pub const fn special_auth_set(&self) -> &SpecialAuthSet {
        &self.special_auth_set
    }
    #[doc = "0x208 - This register is used for clearing bits in SPECIAL_AUTH register. This register is configured and locked during device boot."]
    #[inline(always)]
    pub const fn special_auth_clr(&self) -> &SpecialAuthClr {
        &self.special_auth_clr
    }
    #[doc = "0x210 - This register indicates the debug privileges of ARM Cortex CPU."]
    #[inline(always)]
    pub const fn app_auth(&self) -> &AppAuth {
        &self.app_auth
    }
    #[doc = "0x214 - This register is used for setting bits in APP_AUTH register. This register is configured and locked during device boot."]
    #[inline(always)]
    pub const fn app_auth_set(&self) -> &AppAuthSet {
        &self.app_auth_set
    }
    #[doc = "0x218 - This register is used for clearing bits in APP_AUTH register. This register is configured and locked during device boot."]
    #[inline(always)]
    pub const fn app_auth_clr(&self) -> &AppAuthClr {
        &self.app_auth_clr
    }
    #[doc = "0x21c - Debug control register. This register is used for controlling debug connection and read out debug status."]
    #[inline(always)]
    pub const fn dbgctl(&self) -> &Dbgctl {
        &self.dbgctl
    }
}
#[doc = "DESC (rw) register accessor: Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`desc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`desc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@desc`]
module"]
#[doc(alias = "DESC")]
pub type Desc = crate::Reg<desc::DescSpec>;
#[doc = "Description Register. This register provides IP module ID, revision information, instance index and standard MMR registers offset."]
pub mod desc;
#[doc = "IMASK (rw) register accessor: Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imask`]
module"]
#[doc(alias = "IMASK")]
pub type Imask = crate::Reg<imask::ImaskSpec>;
#[doc = "Interrupt mask. This register selects interrupt sources which are allowed to pass from RIS to MIS when the corresponding bit-fields are set to 1."]
pub mod imask;
#[doc = "RIS (rw) register accessor: Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
pub mod mis;
#[doc = "ISET (rw) register accessor: Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iset`]
module"]
#[doc(alias = "ISET")]
pub type Iset = crate::Reg<iset::IsetSpec>;
#[doc = "Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iclr`]
module"]
#[doc(alias = "ICLR")]
pub type Iclr = crate::Reg<iclr::IclrSpec>;
#[doc = "Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
pub mod iclr;
#[doc = "IMSET (rw) register accessor: Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imset`]
module"]
#[doc(alias = "IMSET")]
pub type Imset = crate::Reg<imset::ImsetSpec>;
#[doc = "Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
pub mod imset;
#[doc = "IMCLR (rw) register accessor: Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imclr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imclr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imclr`]
module"]
#[doc(alias = "IMCLR")]
pub type Imclr = crate::Reg<imclr::ImclrSpec>;
#[doc = "Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
pub mod imclr;
#[doc = "TXD (rw) register accessor: Transmit data register. This register is used for sending SACI command parameter data from the host to the device. The host (SWD interface) can write this register. This updates the value of TXD, and sets TXCTL.TXDSTA = FULL The host should only write TXD while TXCTL.TXDSTA = EMPTY. If the host incorrectly writes TXD while TXCTL.TXDSTA = FULL, this will just update the value of TXD. The host (SWD interface) can read the TXD register. This does not affect TXCTL.TXDSTA. The device (boot code) can only read the TXD register. This sets TXCTL.TXDSTA = EMPTY. The device should only read TXD while TXCTL.TXDSTA = FULL. If the device incorrectly reads TXD while TXCTL.TXDSTA = EMPTY, this will just return the value of TXD. If the host writes TXD on the same clock cycle as the device reads TXD: The device reads the old TXD value. TXD is updated with the new value, and TXCTL.TXDSTA is set to FULL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txd`]
module"]
#[doc(alias = "TXD")]
pub type Txd = crate::Reg<txd::TxdSpec>;
#[doc = "Transmit data register. This register is used for sending SACI command parameter data from the host to the device. The host (SWD interface) can write this register. This updates the value of TXD, and sets TXCTL.TXDSTA = FULL The host should only write TXD while TXCTL.TXDSTA = EMPTY. If the host incorrectly writes TXD while TXCTL.TXDSTA = FULL, this will just update the value of TXD. The host (SWD interface) can read the TXD register. This does not affect TXCTL.TXDSTA. The device (boot code) can only read the TXD register. This sets TXCTL.TXDSTA = EMPTY. The device should only read TXD while TXCTL.TXDSTA = FULL. If the device incorrectly reads TXD while TXCTL.TXDSTA = EMPTY, this will just return the value of TXD. If the host writes TXD on the same clock cycle as the device reads TXD: The device reads the old TXD value. TXD is updated with the new value, and TXCTL.TXDSTA is set to FULL."]
pub mod txd;
#[doc = "TXCTL (rw) register accessor: Transmit control register. This register contains status of the TXD register (full/empty), and also software defined flags that are used by the SACI protocol. The host (SWD interface) can write the FLAGS field of the TXCTL register. The host (SWD interface) can read the TXCTL register. The device (boot code) can only read the TXCTL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txctl`]
module"]
#[doc(alias = "TXCTL")]
pub type Txctl = crate::Reg<txctl::TxctlSpec>;
#[doc = "Transmit control register. This register contains status of the TXD register (full/empty), and also software defined flags that are used by the SACI protocol. The host (SWD interface) can write the FLAGS field of the TXCTL register. The host (SWD interface) can read the TXCTL register. The device (boot code) can only read the TXCTL register."]
pub mod txctl;
#[doc = "RXD (rw) register accessor: Receive data register. This register is used to send SACI command response data from the device to the host. The device (boot code) can write the RXD register. This updates the value of RXD, and sets RXCTL.RXDSTA = FULL. The device should only write RXD while RXCTL.RXDSTA = EMPTY. If the device incorrectly writes RXD while RXCTL.RXDSTA = FULL, this will just update the value of RXD. The device (boot code) can read the RXD register in order to flush it. This sets RXCTL.RXDSTA = EMPTY. The host (SWD interface) can only read the RXD register. This sets RXCTL.RXDSTA = EMPTY. The host should only read RXD while RXCTL.RXDSTA = FULL. If the host incorrectly reads RXD while RXCTL.RXDSTA = EMPTY, this will just return the value of RXD. If the device writes RXD on the same clock cycle as the host reads RXD: The host reads the old RXD value. RXD is updated with the new value, and RXCTL.RXDSTA is set to FULL.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxd`]
module"]
#[doc(alias = "RXD")]
pub type Rxd = crate::Reg<rxd::RxdSpec>;
#[doc = "Receive data register. This register is used to send SACI command response data from the device to the host. The device (boot code) can write the RXD register. This updates the value of RXD, and sets RXCTL.RXDSTA = FULL. The device should only write RXD while RXCTL.RXDSTA = EMPTY. If the device incorrectly writes RXD while RXCTL.RXDSTA = FULL, this will just update the value of RXD. The device (boot code) can read the RXD register in order to flush it. This sets RXCTL.RXDSTA = EMPTY. The host (SWD interface) can only read the RXD register. This sets RXCTL.RXDSTA = EMPTY. The host should only read RXD while RXCTL.RXDSTA = FULL. If the host incorrectly reads RXD while RXCTL.RXDSTA = EMPTY, this will just return the value of RXD. If the device writes RXD on the same clock cycle as the host reads RXD: The host reads the old RXD value. RXD is updated with the new value, and RXCTL.RXDSTA is set to FULL."]
pub mod rxd;
#[doc = "RXCTL (rw) register accessor: Receive control register. This register contains status of the RXD register (full/empty), and also software defined flags that are used by the SACI protocol. The device (boot code) can write the FLAGS field of the RXCTL register. The device (boot code) can read the RXCTL register. The host (SWD interface) can only read the RXCTL register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxctl`]
module"]
#[doc(alias = "RXCTL")]
pub type Rxctl = crate::Reg<rxctl::RxctlSpec>;
#[doc = "Receive control register. This register contains status of the RXD register (full/empty), and also software defined flags that are used by the SACI protocol. The device (boot code) can write the FLAGS field of the RXCTL register. The device (boot code) can read the RXCTL register. The host (SWD interface) can only read the RXCTL register"]
pub mod rxctl;
#[doc = "TXDPEEK (rw) register accessor: Transmit data peek register . This register is a read-only version of the TXD register that can be read by host and device without any side-effects. This register is used to peek at the values in TXD without affecting the FULL/EMPTY flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdpeek::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdpeek::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdpeek`]
module"]
#[doc(alias = "TXDPEEK")]
pub type Txdpeek = crate::Reg<txdpeek::TxdpeekSpec>;
#[doc = "Transmit data peek register . This register is a read-only version of the TXD register that can be read by host and device without any side-effects. This register is used to peek at the values in TXD without affecting the FULL/EMPTY flag."]
pub mod txdpeek;
#[doc = "RXDPEEK (rw) register accessor: Receive data peek register. The RXDPEEK register is a read-only version of the RXD register that can be read by host and device without any side-effects This register is used to peek at the values in Receive Data Register without affecting the FULL/EMPTY flag.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdpeek::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdpeek::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdpeek`]
module"]
#[doc(alias = "RXDPEEK")]
pub type Rxdpeek = crate::Reg<rxdpeek::RxdpeekSpec>;
#[doc = "Receive data peek register. The RXDPEEK register is a read-only version of the RXD register that can be read by host and device without any side-effects This register is used to peek at the values in Receive Data Register without affecting the FULL/EMPTY flag."]
pub mod rxdpeek;
#[doc = "SPECIAL_AUTH (rw) register accessor: This register indicates the status of different AP firewalls and fakestandby enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`special_auth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`special_auth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@special_auth`]
module"]
#[doc(alias = "SPECIAL_AUTH")]
pub type SpecialAuth = crate::Reg<special_auth::SpecialAuthSpec>;
#[doc = "This register indicates the status of different AP firewalls and fakestandby enable."]
pub mod special_auth;
#[doc = "SPECIAL_AUTH_SET (rw) register accessor: This register is used for setting bits in SPECIAL_AUTH register. This register is configured and locked during device boot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`special_auth_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`special_auth_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@special_auth_set`]
module"]
#[doc(alias = "SPECIAL_AUTH_SET")]
pub type SpecialAuthSet = crate::Reg<special_auth_set::SpecialAuthSetSpec>;
#[doc = "This register is used for setting bits in SPECIAL_AUTH register. This register is configured and locked during device boot."]
pub mod special_auth_set;
#[doc = "SPECIAL_AUTH_CLR (rw) register accessor: This register is used for clearing bits in SPECIAL_AUTH register. This register is configured and locked during device boot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`special_auth_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`special_auth_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@special_auth_clr`]
module"]
#[doc(alias = "SPECIAL_AUTH_CLR")]
pub type SpecialAuthClr = crate::Reg<special_auth_clr::SpecialAuthClrSpec>;
#[doc = "This register is used for clearing bits in SPECIAL_AUTH register. This register is configured and locked during device boot."]
pub mod special_auth_clr;
#[doc = "APP_AUTH (rw) register accessor: This register indicates the debug privileges of ARM Cortex CPU.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_auth::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_auth::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_auth`]
module"]
#[doc(alias = "APP_AUTH")]
pub type AppAuth = crate::Reg<app_auth::AppAuthSpec>;
#[doc = "This register indicates the debug privileges of ARM Cortex CPU."]
pub mod app_auth;
#[doc = "APP_AUTH_SET (rw) register accessor: This register is used for setting bits in APP_AUTH register. This register is configured and locked during device boot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_auth_set::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_auth_set::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_auth_set`]
module"]
#[doc(alias = "APP_AUTH_SET")]
pub type AppAuthSet = crate::Reg<app_auth_set::AppAuthSetSpec>;
#[doc = "This register is used for setting bits in APP_AUTH register. This register is configured and locked during device boot."]
pub mod app_auth_set;
#[doc = "APP_AUTH_CLR (rw) register accessor: This register is used for clearing bits in APP_AUTH register. This register is configured and locked during device boot.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`app_auth_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`app_auth_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@app_auth_clr`]
module"]
#[doc(alias = "APP_AUTH_CLR")]
pub type AppAuthClr = crate::Reg<app_auth_clr::AppAuthClrSpec>;
#[doc = "This register is used for clearing bits in APP_AUTH register. This register is configured and locked during device boot."]
pub mod app_auth_clr;
#[doc = "DBGCTL (rw) register accessor: Debug control register. This register is used for controlling debug connection and read out debug status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgctl`]
module"]
#[doc(alias = "DBGCTL")]
pub type Dbgctl = crate::Reg<dbgctl::DbgctlSpec>;
#[doc = "Debug control register. This register is used for controlling debug connection and read out debug status."]
pub mod dbgctl;
