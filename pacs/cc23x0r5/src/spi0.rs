#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    desc: Desc,
    _reserved1: [u8; 0x40],
    imask: Imask,
    ris: Ris,
    mis: Mis,
    iset: Iset,
    iclr: Iclr,
    imset: Imset,
    imclr: Imclr,
    emu: Emu,
    dtb: Dtb,
    _reserved10: [u8; 0x98],
    ctl0: Ctl0,
    ctl1: Ctl1,
    clkcfg0: Clkcfg0,
    clkcfg1: Clkcfg1,
    ifls: Ifls,
    dmacr: Dmacr,
    rxcrc: Rxcrc,
    txcrc: Txcrc,
    txfhdr32: Txfhdr32,
    txfhdr24: Txfhdr24,
    txfhdr16: Txfhdr16,
    txfhdr8: Txfhdr8,
    txfhdrc: Txfhdrc,
    _reserved23: [u8; 0x0c],
    rxdata: Rxdata,
    _reserved24: [u8; 0x0c],
    txdata: Txdata,
    _reserved25: [u8; 0x0c],
    stat: Stat,
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
    #[doc = "0x48 - Raw interrupt status. This register reflects the state of all pending interrupts, regardless of masking. This register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x4c - Masked interrupt status. This register is simply a bitwise AND of the contents of IMASK and RIS.*\\]
registers. A flag set in this register can be cleared by writing 1 to the corresponding ICLR register bit."]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x50 - Interrupt set register. This register can used by software for diagnostics and safety checking purposes. Writing a 1 to a bit in this register will set the event and the corresponding RIS bit also gets set. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets set."]
    #[inline(always)]
    pub const fn iset(&self) -> &Iset {
        &self.iset
    }
    #[doc = "0x54 - Interrupt clear register. This register allows software to clear interrupts. Writing a 1 to a bit in this register will clear the event and the corresponding RIS bit also gets cleared. If the corresponding IMASK bit is set, then the corresponding MIS register bit also gets cleared."]
    #[inline(always)]
    pub const fn iclr(&self) -> &Iclr {
        &self.iclr
    }
    #[doc = "0x58 - Interrupt mask set register. Writing a 1 to a bit in this register will set the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imset(&self) -> &Imset {
        &self.imset
    }
    #[doc = "0x5c - Interrupt mask clear register. Writing a 1 to a bit in this register will clear the corresponding IMASK bit."]
    #[inline(always)]
    pub const fn imclr(&self) -> &Imclr {
        &self.imclr
    }
    #[doc = "0x60 - Emulation control register. This register controls the behavior of the IP related to core halted input."]
    #[inline(always)]
    pub const fn emu(&self) -> &Emu {
        &self.emu
    }
    #[doc = "0x64 - Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
    #[inline(always)]
    pub const fn dtb(&self) -> &Dtb {
        &self.dtb
    }
    #[doc = "0x100 - SPI control register 0"]
    #[inline(always)]
    pub const fn ctl0(&self) -> &Ctl0 {
        &self.ctl0
    }
    #[doc = "0x104 - SPI control register 1"]
    #[inline(always)]
    pub const fn ctl1(&self) -> &Ctl1 {
        &self.ctl1
    }
    #[doc = "0x108 - Clock configuration register 0. This register is used to configure the clock prescaler."]
    #[inline(always)]
    pub const fn clkcfg0(&self) -> &Clkcfg0 {
        &self.clkcfg0
    }
    #[doc = "0x10c - Clock configuration register 1. This register is used to configure serial clock rate and clock count for delayed sampling in controller mode."]
    #[inline(always)]
    pub const fn clkcfg1(&self) -> &Clkcfg1 {
        &self.clkcfg1
    }
    #[doc = "0x110 - Interrupt FIFO level select register. This register can be used to define the levels at which the RIS.TX, RIS.RX flags are triggered. The interrupts are generated based on FIFO level. Out of reset, the IFLS.TXSEL and IFLS.RXSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark."]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x114 - DMA Control Register"]
    #[inline(always)]
    pub const fn dmacr(&self) -> &Dmacr {
        &self.dmacr
    }
    #[doc = "0x118 - Receive CRC register. Reading this register provides the computed CRC value from the receive side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0."]
    #[inline(always)]
    pub const fn rxcrc(&self) -> &Rxcrc {
        &self.rxcrc
    }
    #[doc = "0x11c - Transmit CRC register. Reading this register provides the computed CRC value from the transmit side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0."]
    #[inline(always)]
    pub const fn txcrc(&self) -> &Txcrc {
        &self.txcrc
    }
    #[doc = "0x120 - Header update reigster for 32 bits of header data."]
    #[inline(always)]
    pub const fn txfhdr32(&self) -> &Txfhdr32 {
        &self.txfhdr32
    }
    #[doc = "0x124 - Header update reigster for 24 bits of header data."]
    #[inline(always)]
    pub const fn txfhdr24(&self) -> &Txfhdr24 {
        &self.txfhdr24
    }
    #[doc = "0x128 - Header update reigster for 16 bits of data."]
    #[inline(always)]
    pub const fn txfhdr16(&self) -> &Txfhdr16 {
        &self.txfhdr16
    }
    #[doc = "0x12c - Header update reigster for 8 bits of header data."]
    #[inline(always)]
    pub const fn txfhdr8(&self) -> &Txfhdr8 {
        &self.txfhdr8
    }
    #[doc = "0x130 - Atomic Header Control register"]
    #[inline(always)]
    pub const fn txfhdrc(&self) -> &Txfhdrc {
        &self.txfhdrc
    }
    #[doc = "0x140 - RXDATA Register. Reading this register returns first value in the RX FIFO. If the FIFO is empty the last read value is returned. Writing has no effect and is ignored."]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x150 - TXDATA Register. Writing a value in this register puts the data into the TX FIFO. Reading this register returns the last writen value."]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x160 - Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
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
#[doc = "EMU (rw) register accessor: Emulation control register. This register controls the behavior of the IP related to core halted input.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emu::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emu::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emu`]
module"]
#[doc(alias = "EMU")]
pub type Emu = crate::Reg<emu::EmuSpec>;
#[doc = "Emulation control register. This register controls the behavior of the IP related to core halted input."]
pub mod emu;
#[doc = "DTB (rw) register accessor: Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dtb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dtb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtb`]
module"]
#[doc(alias = "DTB")]
pub type Dtb = crate::Reg<dtb::DtbSpec>;
#[doc = "Digital test bus control register. This register can be used to bring out IP internal signals to the pads for observation. 16 signals can be observed per select value."]
pub mod dtb;
#[doc = "CTL0 (rw) register accessor: SPI control register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl0`]
module"]
#[doc(alias = "CTL0")]
pub type Ctl0 = crate::Reg<ctl0::Ctl0Spec>;
#[doc = "SPI control register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: SPI control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl1`]
module"]
#[doc(alias = "CTL1")]
pub type Ctl1 = crate::Reg<ctl1::Ctl1Spec>;
#[doc = "SPI control register 1"]
pub mod ctl1;
#[doc = "CLKCFG0 (rw) register accessor: Clock configuration register 0. This register is used to configure the clock prescaler.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg0`]
module"]
#[doc(alias = "CLKCFG0")]
pub type Clkcfg0 = crate::Reg<clkcfg0::Clkcfg0Spec>;
#[doc = "Clock configuration register 0. This register is used to configure the clock prescaler."]
pub mod clkcfg0;
#[doc = "CLKCFG1 (rw) register accessor: Clock configuration register 1. This register is used to configure serial clock rate and clock count for delayed sampling in controller mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkcfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkcfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkcfg1`]
module"]
#[doc(alias = "CLKCFG1")]
pub type Clkcfg1 = crate::Reg<clkcfg1::Clkcfg1Spec>;
#[doc = "Clock configuration register 1. This register is used to configure serial clock rate and clock count for delayed sampling in controller mode."]
pub mod clkcfg1;
#[doc = "IFLS (rw) register accessor: Interrupt FIFO level select register. This register can be used to define the levels at which the RIS.TX, RIS.RX flags are triggered. The interrupts are generated based on FIFO level. Out of reset, the IFLS.TXSEL and IFLS.RXSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`]
module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "Interrupt FIFO level select register. This register can be used to define the levels at which the RIS.TX, RIS.RX flags are triggered. The interrupts are generated based on FIFO level. Out of reset, the IFLS.TXSEL and IFLS.RXSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark."]
pub mod ifls;
#[doc = "DMACR (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacr`]
module"]
#[doc(alias = "DMACR")]
pub type Dmacr = crate::Reg<dmacr::DmacrSpec>;
#[doc = "DMA Control Register"]
pub mod dmacr;
#[doc = "RXCRC (rw) register accessor: Receive CRC register. Reading this register provides the computed CRC value from the receive side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxcrc`]
module"]
#[doc(alias = "RXCRC")]
pub type Rxcrc = crate::Reg<rxcrc::RxcrcSpec>;
#[doc = "Receive CRC register. Reading this register provides the computed CRC value from the receive side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0."]
pub mod rxcrc;
#[doc = "TXCRC (rw) register accessor: Transmit CRC register. Reading this register provides the computed CRC value from the transmit side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txcrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txcrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txcrc`]
module"]
#[doc(alias = "TXCRC")]
pub type Txcrc = crate::Reg<txcrc::TxcrcSpec>;
#[doc = "Transmit CRC register. Reading this register provides the computed CRC value from the transmit side CRC unit. Reading this register or writing to this register with any value auto initializes the seed. The seed value is 0xFF when CTL0.CRCPOLY = 0 and 0xFFFF when CTL0.CRCPOLY = 1 for CCITT CRC polynomials. Bits\\[15:8\\]
are a don't care when CTL0.CRCPOLY = 0."]
pub mod txcrc;
#[doc = "TXFHDR32 (rw) register accessor: Header update reigster for 32 bits of header data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfhdr32`]
module"]
#[doc(alias = "TXFHDR32")]
pub type Txfhdr32 = crate::Reg<txfhdr32::Txfhdr32Spec>;
#[doc = "Header update reigster for 32 bits of header data."]
pub mod txfhdr32;
#[doc = "TXFHDR24 (rw) register accessor: Header update reigster for 24 bits of header data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfhdr24`]
module"]
#[doc(alias = "TXFHDR24")]
pub type Txfhdr24 = crate::Reg<txfhdr24::Txfhdr24Spec>;
#[doc = "Header update reigster for 24 bits of header data."]
pub mod txfhdr24;
#[doc = "TXFHDR16 (rw) register accessor: Header update reigster for 16 bits of data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfhdr16`]
module"]
#[doc(alias = "TXFHDR16")]
pub type Txfhdr16 = crate::Reg<txfhdr16::Txfhdr16Spec>;
#[doc = "Header update reigster for 16 bits of data."]
pub mod txfhdr16;
#[doc = "TXFHDR8 (rw) register accessor: Header update reigster for 8 bits of header data.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfhdr8`]
module"]
#[doc(alias = "TXFHDR8")]
pub type Txfhdr8 = crate::Reg<txfhdr8::Txfhdr8Spec>;
#[doc = "Header update reigster for 8 bits of header data."]
pub mod txfhdr8;
#[doc = "TXFHDRC (rw) register accessor: Atomic Header Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txfhdrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txfhdrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfhdrc`]
module"]
#[doc(alias = "TXFHDRC")]
pub type Txfhdrc = crate::Reg<txfhdrc::TxfhdrcSpec>;
#[doc = "Atomic Header Control register"]
pub mod txfhdrc;
#[doc = "RXDATA (rw) register accessor: RXDATA Register. Reading this register returns first value in the RX FIFO. If the FIFO is empty the last read value is returned. Writing has no effect and is ignored.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`]
module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "RXDATA Register. Reading this register returns first value in the RX FIFO. If the FIFO is empty the last read value is returned. Writing has no effect and is ignored."]
pub mod rxdata;
#[doc = "TXDATA (rw) register accessor: TXDATA Register. Writing a value in this register puts the data into the TX FIFO. Reading this register returns the last writen value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdata::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdata::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`]
module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "TXDATA Register. Writing a value in this register puts the data into the TX FIFO. Reading this register returns the last writen value."]
pub mod txdata;
#[doc = "STAT (rw) register accessor: Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Status Register"]
pub mod stat;
