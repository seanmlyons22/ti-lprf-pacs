#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x20],
    #[doc = "0x20 - This register provides the highest priority enabled interrupt index. Value 0x00 means no event pending. Interrupt 1 is the highest priority, and 31 is the least priority. That is, the least bit position that is set to 1 denotes the highest priority pending interrupt. The priority order is fixed. However, users can implement their own prioritization schemes using other registers that expose the full set of interrupts that have occurred. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flag in RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register is updated with the next highest priority interrupt, if none are pending, then it would display 0x0."]
    pub iidx: IIDX,
    _reserved1: [u8; 0x04],
    #[doc = "0x28 - Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
    pub imask: IMASK,
    _reserved2: [u8; 0x04],
    #[doc = "0x30 - Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
    pub ris: RIS,
    _reserved3: [u8; 0x04],
    #[doc = "0x38 - Masked interrupt status. This is an AND of the IMASK and RIS registers."]
    pub mis: MIS,
    _reserved4: [u8; 0x04],
    #[doc = "0x40 - Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
    pub iset: ISET,
    _reserved5: [u8; 0x04],
    #[doc = "0x48 - Interrupt clear. Write a 1 to clear the corresponding Interrupt."]
    pub iclr: ICLR,
    _reserved6: [u8; 0x94],
    #[doc = "0xe0 - Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS) Note: The recommendation is to use SPI in the software mode"]
    pub evt_mode: EVT_MODE,
    _reserved7: [u8; 0x18],
    #[doc = "0xfc - This register identifies the peripheral and its exact version."]
    pub desc: DESC,
    #[doc = "0x100 - SPI Control Register 0"]
    pub ctl0: CTL0,
    #[doc = "0x104 - SPI Control Register 1"]
    pub ctl1: CTL1,
    #[doc = "0x108 - Clock prescaler and divider register. This register contains the settings for the Clock prescaler and divider settings."]
    pub clkctl: CLKCTL,
    #[doc = "0x10c - The IFLS register is the interrupt FIFO level select register. This register can be used to define the levels at which the TX, RX FIFO interrupt flags are triggered. Out of reset, the TXIFLSEL and RXIFLSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark."]
    pub ifls: IFLS,
    #[doc = "0x110 - Status Register"]
    pub stat: STAT,
    #[doc = "0x114 - This register is used to specify a divide ratio of the SPI functional clock."]
    pub clkdiv2: CLKDIV2,
    #[doc = "0x118 - DMA Control Register"]
    pub dmacr: DMACR,
    _reserved15: [u8; 0x14],
    #[doc = "0x130 - RXDATA Register. Reading this register returns value in the RX FIFO pointed by the current FIFO read pointer. If the RX FIFO is empty, the last read value is returned. Writing has not effect and is ignored."]
    pub rxdata: RXDATA,
    _reserved16: [u8; 0x0c],
    #[doc = "0x140 - TXDATA Register. Writing into this register puts the data into the TX FIFO. Reading this register returns the last written value, pointed by the current FIFO write pointer."]
    pub txdata: TXDATA,
}
#[doc = "IIDX (rw) register accessor: an alias for `Reg<IIDX_SPEC>`"]
pub type IIDX = crate::Reg<iidx::IIDX_SPEC>;
#[doc = "This register provides the highest priority enabled interrupt index. Value 0x00 means no event pending. Interrupt 1 is the highest priority, and 31 is the least priority. That is, the least bit position that is set to 1 denotes the highest priority pending interrupt. The priority order is fixed. However, users can implement their own prioritization schemes using other registers that expose the full set of interrupts that have occurred. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flag in RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register is updated with the next highest priority interrupt, if none are pending, then it would display 0x0."]
pub mod iidx;
#[doc = "IMASK (rw) register accessor: an alias for `Reg<IMASK_SPEC>`"]
pub type IMASK = crate::Reg<imask::IMASK_SPEC>;
#[doc = "Interrupt Mask. If a bit is set, then corresponding interrupt is un-masked. Un-masking the interrupt causes the raw interrupt to be visible in IIDX, as well as MIS."]
pub mod imask;
#[doc = "RIS (rw) register accessor: an alias for `Reg<RIS_SPEC>`"]
pub type RIS = crate::Reg<ris::RIS_SPEC>;
#[doc = "Raw interrupt status. Reflects all pending interrupts, regardless of masking. The RIS register allows the user to implement a poll scheme. A flag set in this register can be cleared by writing 1 to the ICLR register bit even if the corresponding IMASK bit is not enabled."]
pub mod ris;
#[doc = "MIS (rw) register accessor: an alias for `Reg<MIS_SPEC>`"]
pub type MIS = crate::Reg<mis::MIS_SPEC>;
#[doc = "Masked interrupt status. This is an AND of the IMASK and RIS registers."]
pub mod mis;
#[doc = "ISET (rw) register accessor: an alias for `Reg<ISET_SPEC>`"]
pub type ISET = crate::Reg<iset::ISET_SPEC>;
#[doc = "Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set."]
pub mod iset;
#[doc = "ICLR (rw) register accessor: an alias for `Reg<ICLR_SPEC>`"]
pub type ICLR = crate::Reg<iclr::ICLR_SPEC>;
#[doc = "Interrupt clear. Write a 1 to clear the corresponding Interrupt."]
pub mod iclr;
#[doc = "EVT_MODE (rw) register accessor: an alias for `Reg<EVT_MODE_SPEC>`"]
pub type EVT_MODE = crate::Reg<evt_mode::EVT_MODE_SPEC>;
#[doc = "Event mode register. It is used to select whether each line is disabled, in software mode (software clears the RIS) or in hardware mode (hardware clears the RIS) Note: The recommendation is to use SPI in the software mode"]
pub mod evt_mode;
#[doc = "DESC (rw) register accessor: an alias for `Reg<DESC_SPEC>`"]
pub type DESC = crate::Reg<desc::DESC_SPEC>;
#[doc = "This register identifies the peripheral and its exact version."]
pub mod desc;
#[doc = "CTL0 (rw) register accessor: an alias for `Reg<CTL0_SPEC>`"]
pub type CTL0 = crate::Reg<ctl0::CTL0_SPEC>;
#[doc = "SPI Control Register 0"]
pub mod ctl0;
#[doc = "CTL1 (rw) register accessor: an alias for `Reg<CTL1_SPEC>`"]
pub type CTL1 = crate::Reg<ctl1::CTL1_SPEC>;
#[doc = "SPI Control Register 1"]
pub mod ctl1;
#[doc = "CLKCTL (rw) register accessor: an alias for `Reg<CLKCTL_SPEC>`"]
pub type CLKCTL = crate::Reg<clkctl::CLKCTL_SPEC>;
#[doc = "Clock prescaler and divider register. This register contains the settings for the Clock prescaler and divider settings."]
pub mod clkctl;
#[doc = "IFLS (rw) register accessor: an alias for `Reg<IFLS_SPEC>`"]
pub type IFLS = crate::Reg<ifls::IFLS_SPEC>;
#[doc = "The IFLS register is the interrupt FIFO level select register. This register can be used to define the levels at which the TX, RX FIFO interrupt flags are triggered. Out of reset, the TXIFLSEL and RXIFLSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark."]
pub mod ifls;
#[doc = "STAT (rw) register accessor: an alias for `Reg<STAT_SPEC>`"]
pub type STAT = crate::Reg<stat::STAT_SPEC>;
#[doc = "Status Register"]
pub mod stat;
#[doc = "CLKDIV2 (rw) register accessor: an alias for `Reg<CLKDIV2_SPEC>`"]
pub type CLKDIV2 = crate::Reg<clkdiv2::CLKDIV2_SPEC>;
#[doc = "This register is used to specify a divide ratio of the SPI functional clock."]
pub mod clkdiv2;
#[doc = "DMACR (rw) register accessor: an alias for `Reg<DMACR_SPEC>`"]
pub type DMACR = crate::Reg<dmacr::DMACR_SPEC>;
#[doc = "DMA Control Register"]
pub mod dmacr;
#[doc = "RXDATA (rw) register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "RXDATA Register. Reading this register returns value in the RX FIFO pointed by the current FIFO read pointer. If the RX FIFO is empty, the last read value is returned. Writing has not effect and is ignored."]
pub mod rxdata;
#[doc = "TXDATA (rw) register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "TXDATA Register. Writing into this register puts the data into the TX FIFO. Reading this register returns the last written value, pointed by the current FIFO write pointer."]
pub mod txdata;
