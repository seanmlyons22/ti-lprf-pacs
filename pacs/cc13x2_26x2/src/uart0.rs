#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dr: Dr,
    _reserved_1_ecr: [u8; 0x04],
    reserved0: Reserved0,
    _reserved3: [u8; 0x0c],
    fr: Fr,
    reserved2: Reserved2,
    _reserved5: [u8; 0x04],
    ibrd: Ibrd,
    fbrd: Fbrd,
    lcrh: Lcrh,
    ctl: Ctl,
    ifls: Ifls,
    imsc: Imsc,
    ris: Ris,
    mis: Mis,
    icr: Icr,
    dmactl: Dmactl,
    reserved1: Reserved1,
    _reserved16: [u8; 0x40],
    reserved3: Reserved3,
    _reserved17: [u8; 0x0f3c],
    reserved4: Reserved4,
}
impl RegisterBlock {
    #[doc = "0x00 - Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x04 - Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
    #[inline(always)]
    pub const fn ecr(&self) -> &Ecr {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved0(&self) -> &Reserved0 {
        &self.reserved0
    }
    #[doc = "0x18 - Flag Reads from this register return the UART flags."]
    #[inline(always)]
    pub const fn fr(&self) -> &Fr {
        &self.fr
    }
    #[doc = "0x1c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved2(&self) -> &Reserved2 {
        &self.reserved2
    }
    #[doc = "0x24 - Integer Baud-Rate Divisor If this register is modified while transmission or reception is on-going, the baud rate will not be updated until transmission or reception of the current character is complete."]
    #[inline(always)]
    pub const fn ibrd(&self) -> &Ibrd {
        &self.ibrd
    }
    #[doc = "0x28 - Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
    #[inline(always)]
    pub const fn fbrd(&self) -> &Fbrd {
        &self.fbrd
    }
    #[doc = "0x2c - Line Control"]
    #[inline(always)]
    pub const fn lcrh(&self) -> &Lcrh {
        &self.lcrh
    }
    #[doc = "0x30 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x34 - Interrupt FIFO Level Select"]
    #[inline(always)]
    pub const fn ifls(&self) -> &Ifls {
        &self.ifls
    }
    #[doc = "0x38 - Interrupt Mask Set/Clear"]
    #[inline(always)]
    pub const fn imsc(&self) -> &Imsc {
        &self.imsc
    }
    #[doc = "0x3c - Raw Interrupt Status"]
    #[inline(always)]
    pub const fn ris(&self) -> &Ris {
        &self.ris
    }
    #[doc = "0x40 - Masked Interrupt Status"]
    #[inline(always)]
    pub const fn mis(&self) -> &Mis {
        &self.mis
    }
    #[doc = "0x44 - Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x48 - DMA Control"]
    #[inline(always)]
    pub const fn dmactl(&self) -> &Dmactl {
        &self.dmactl
    }
    #[doc = "0x4c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved1(&self) -> &Reserved1 {
        &self.reserved1
    }
    #[doc = "0x90 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved3(&self) -> &Reserved3 {
        &self.reserved3
    }
    #[doc = "0xfd0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn reserved4(&self) -> &Reserved4 {
        &self.reserved4
    }
}
#[doc = "DR (rw) register accessor: Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
pub mod dr;
#[doc = "RSR (rw) register accessor: Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`]
module"]
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
#[doc = "Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
pub mod rsr;
#[doc = "ECR (rw) register accessor: Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecr`]
module"]
#[doc(alias = "ECR")]
pub type Ecr = crate::Reg<ecr::EcrSpec>;
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
pub mod ecr;
#[doc = "RESERVED0 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved0`]
module"]
#[doc(alias = "RESERVED0")]
pub type Reserved0 = crate::Reg<reserved0::Reserved0Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved0;
#[doc = "FR (rw) register accessor: Flag Reads from this register return the UART flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fr`]
module"]
#[doc(alias = "FR")]
pub type Fr = crate::Reg<fr::FrSpec>;
#[doc = "Flag Reads from this register return the UART flags."]
pub mod fr;
#[doc = "RESERVED2 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved2`]
module"]
#[doc(alias = "RESERVED2")]
pub type Reserved2 = crate::Reg<reserved2::Reserved2Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved2;
#[doc = "IBRD (rw) register accessor: Integer Baud-Rate Divisor If this register is modified while transmission or reception is on-going, the baud rate will not be updated until transmission or reception of the current character is complete.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ibrd`]
module"]
#[doc(alias = "IBRD")]
pub type Ibrd = crate::Reg<ibrd::IbrdSpec>;
#[doc = "Integer Baud-Rate Divisor If this register is modified while transmission or reception is on-going, the baud rate will not be updated until transmission or reception of the current character is complete."]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fbrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fbrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fbrd`]
module"]
#[doc(alias = "FBRD")]
pub type Fbrd = crate::Reg<fbrd::FbrdSpec>;
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub mod fbrd;
#[doc = "LCRH (rw) register accessor: Line Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcrh`]
module"]
#[doc(alias = "LCRH")]
pub type Lcrh = crate::Reg<lcrh::LcrhSpec>;
#[doc = "Line Control"]
pub mod lcrh;
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "IFLS (rw) register accessor: Interrupt FIFO Level Select\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ifls::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifls::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifls`]
module"]
#[doc(alias = "IFLS")]
pub type Ifls = crate::Reg<ifls::IflsSpec>;
#[doc = "Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "IMSC (rw) register accessor: Interrupt Mask Set/Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imsc`]
module"]
#[doc(alias = "IMSC")]
pub type Imsc = crate::Reg<imsc::ImscSpec>;
#[doc = "Interrupt Mask Set/Clear"]
pub mod imsc;
#[doc = "RIS (rw) register accessor: Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ris::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ris::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ris`]
module"]
#[doc(alias = "RIS")]
pub type Ris = crate::Reg<ris::RisSpec>;
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mis::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mis::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mis`]
module"]
#[doc(alias = "MIS")]
pub type Mis = crate::Reg<mis::MisSpec>;
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR (rw) register accessor: Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub mod icr;
#[doc = "DMACTL (rw) register accessor: DMA Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmactl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmactl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmactl`]
module"]
#[doc(alias = "DMACTL")]
pub type Dmactl = crate::Reg<dmactl::DmactlSpec>;
#[doc = "DMA Control"]
pub mod dmactl;
#[doc = "RESERVED1 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved1`]
module"]
#[doc(alias = "RESERVED1")]
pub type Reserved1 = crate::Reg<reserved1::Reserved1Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved1;
#[doc = "RESERVED3 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved3`]
module"]
#[doc(alias = "RESERVED3")]
pub type Reserved3 = crate::Reg<reserved3::Reserved3Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved3;
#[doc = "RESERVED4 (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reserved4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reserved4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reserved4`]
module"]
#[doc(alias = "RESERVED4")]
pub type Reserved4 = crate::Reg<reserved4::Reserved4Spec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved4;
