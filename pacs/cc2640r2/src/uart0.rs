#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
    pub dr: DR,
    _reserved_1_ecr: [u8; 0x04],
    #[doc = "0x08 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved0: RESERVED0,
    _reserved3: [u8; 0x0c],
    #[doc = "0x18 - Flag Reads from this register return the UART flags."]
    pub fr: FR,
    #[doc = "0x1c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved2: RESERVED2,
    _reserved5: [u8; 0x04],
    #[doc = "0x24 - Integer Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
    pub ibrd: IBRD,
    #[doc = "0x28 - Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
    pub fbrd: FBRD,
    #[doc = "0x2c - Line Control"]
    pub lcrh: LCRH,
    #[doc = "0x30 - Control"]
    pub ctl: CTL,
    #[doc = "0x34 - Interrupt FIFO Level Select"]
    pub ifls: IFLS,
    #[doc = "0x38 - Interrupt Mask Set/Clear"]
    pub imsc: IMSC,
    #[doc = "0x3c - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x40 - Masked Interrupt Status"]
    pub mis: MIS,
    #[doc = "0x44 - Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
    pub icr: ICR,
    #[doc = "0x48 - DMA Control"]
    pub dmactl: DMACTL,
    #[doc = "0x4c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved1: RESERVED1,
    _reserved16: [u8; 0x40],
    #[doc = "0x90 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved3: RESERVED3,
    _reserved17: [u8; 0x0f3c],
    #[doc = "0xfd0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub reserved4: RESERVED4,
}
impl RegisterBlock {
    #[doc = "0x04 - Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
    #[inline(always)]
    pub const fn ecr(&self) -> &ECR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
    #[doc = "0x04 - Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
    #[inline(always)]
    pub const fn rsr(&self) -> &RSR {
        unsafe { &*(self as *const Self).cast::<u8>().add(4usize).cast() }
    }
}
#[doc = "DR (rw) register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "Data For words to be transmitted: - if the FIFOs are enabled (LCRH.FEN = 1), data written to this location is pushed onto the transmit FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), data is stored in the transmitter holding register (the bottom word of the transmit FIFO). The write operation initiates transmission from the UART. The data is prefixed with a start bit, appended with the appropriate parity bit (if parity is enabled), and a stop bit. The resultant word is then transmitted. For received words: - if the FIFOs are enabled (LCRH.FEN = 1), the data byte and the 4-bit status (break, frame, parity, and overrun) is pushed onto the 12-bit wide receive FIFO - if the FIFOs are not enabled (LCRH.FEN = 0), the data byte and status are stored in the receiving holding register (the bottom word of the receive FIFO). The received data byte is read by performing reads from this register along with the corresponding status information. The status information can also be read by a read of the RSR register."]
pub mod dr;
#[doc = "RSR (rw) register accessor: an alias for `Reg<RSR_SPEC>`"]
pub type RSR = crate::Reg<rsr::RSR_SPEC>;
#[doc = "Status This register is mapped to the same address as ECR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors). If the status is read from this register, then the status information for break, framing and parity corresponds to the data character read from the Data Register, DR prior to reading the RSR. The status information for overrun is set immediately when an overrun condition occurs."]
pub mod rsr;
#[doc = "ECR (rw) register accessor: an alias for `Reg<ECR_SPEC>`"]
pub type ECR = crate::Reg<ecr::ECR_SPEC>;
#[doc = "Error Clear This register is mapped to the same address as RSR register. Reads from this address are associated with RSR register and return the receive status. Writes to this address are associated with ECR register and clear the receive status flags (framing, parity, break, and overrun errors)."]
pub mod ecr;
#[doc = "RESERVED0 (rw) register accessor: an alias for `Reg<RESERVED0_SPEC>`"]
pub type RESERVED0 = crate::Reg<reserved0::RESERVED0_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved0;
#[doc = "FR (rw) register accessor: an alias for `Reg<FR_SPEC>`"]
pub type FR = crate::Reg<fr::FR_SPEC>;
#[doc = "Flag Reads from this register return the UART flags."]
pub mod fr;
#[doc = "RESERVED2 (rw) register accessor: an alias for `Reg<RESERVED2_SPEC>`"]
pub type RESERVED2 = crate::Reg<reserved2::RESERVED2_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved2;
#[doc = "IBRD (rw) register accessor: an alias for `Reg<IBRD_SPEC>`"]
pub type IBRD = crate::Reg<ibrd::IBRD_SPEC>;
#[doc = "Integer Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub mod ibrd;
#[doc = "FBRD (rw) register accessor: an alias for `Reg<FBRD_SPEC>`"]
pub type FBRD = crate::Reg<fbrd::FBRD_SPEC>;
#[doc = "Fractional Baud-Rate Divisor If this register is modified while trasmission or reception is on-going, the baudrate will not be updated until transmission or reception of the current character is complete."]
pub mod fbrd;
#[doc = "LCRH (rw) register accessor: an alias for `Reg<LCRH_SPEC>`"]
pub type LCRH = crate::Reg<lcrh::LCRH_SPEC>;
#[doc = "Line Control"]
pub mod lcrh;
#[doc = "CTL (rw) register accessor: an alias for `Reg<CTL_SPEC>`"]
pub type CTL = crate::Reg<ctl::CTL_SPEC>;
#[doc = "Control"]
pub mod ctl;
#[doc = "IFLS (rw) register accessor: an alias for `Reg<IFLS_SPEC>`"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "Interrupt FIFO Level Select"]
pub mod ifls;
#[doc = "IMSC (rw) register accessor: an alias for `Reg<IMSC_SPEC>`"]
pub type IMSC = crate::Reg<imsc::IMSC_SPEC>;
#[doc = "Interrupt Mask Set/Clear"]
pub mod imsc;
#[doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked Interrupt Status"]
pub mod mis;
#[doc = "ICR (rw) register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear On a write of 1, the corresponding interrupt is cleared. A write of 0 has no effect."]
pub mod icr;
#[doc = "DMACTL (rw) register accessor: an alias for `Reg<DMACTL_SPEC>`"]
pub type DMACTL = crate::Reg<dmactl::DMACTL_SPEC>;
#[doc = "DMA Control"]
pub mod dmactl;
#[doc = "RESERVED1 (rw) register accessor: an alias for `Reg<RESERVED1_SPEC>`"]
pub type RESERVED1 = crate::Reg<reserved1::RESERVED1_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved1;
#[doc = "RESERVED3 (rw) register accessor: an alias for `Reg<RESERVED3_SPEC>`"]
pub type RESERVED3 = crate::Reg<reserved3::RESERVED3_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved3;
#[doc = "RESERVED4 (rw) register accessor: an alias for `Reg<RESERVED4_SPEC>`"]
pub type RESERVED4 = crate::Reg<reserved4::RESERVED4_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod reserved4;
