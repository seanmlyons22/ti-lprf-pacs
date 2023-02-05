#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0"]
    pub dout3_0: DOUT3_0,
    #[doc = "0x04 - Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0"]
    pub dout7_4: DOUT7_4,
    #[doc = "0x08 - Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0"]
    pub dout11_8: DOUT11_8,
    #[doc = "0x0c - Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0"]
    pub dout15_12: DOUT15_12,
    #[doc = "0x10 - Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0"]
    pub dout19_16: DOUT19_16,
    #[doc = "0x14 - Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0"]
    pub dout23_20: DOUT23_20,
    #[doc = "0x18 - Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0"]
    pub dout27_24: DOUT27_24,
    #[doc = "0x1c - Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0"]
    pub dout31_28: DOUT31_28,
    _reserved8: [u8; 0x60],
    #[doc = "0x80 - Data Output for DIO 0 to 31"]
    pub dout31_0: DOUT31_0,
    _reserved9: [u8; 0x0c],
    #[doc = "0x90 - Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register"]
    pub doutset31_0: DOUTSET31_0,
    _reserved10: [u8; 0x0c],
    #[doc = "0xa0 - Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register"]
    pub doutclr31_0: DOUTCLR31_0,
    _reserved11: [u8; 0x0c],
    #[doc = "0xb0 - Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    pub douttgl31_0: DOUTTGL31_0,
    _reserved12: [u8; 0x0c],
    #[doc = "0xc0 - Data Input from DIO 0 to 31"]
    pub din31_0: DIN31_0,
    _reserved13: [u8; 0x0c],
    #[doc = "0xd0 - Data Output Enable for DIO 0 to 31"]
    pub doe31_0: DOE31_0,
    _reserved14: [u8; 0x0c],
    #[doc = "0xe0 - Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    pub evflags31_0: EVFLAGS31_0,
}
#[doc = "DOUT3_0 (rw) register accessor: an alias for `Reg<DOUT3_0_SPEC>`"]
pub type DOUT3_0 = crate::Reg<dout3_0::DOUT3_0_SPEC>;
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout3_0;
#[doc = "DOUT7_4 (rw) register accessor: an alias for `Reg<DOUT7_4_SPEC>`"]
pub type DOUT7_4 = crate::Reg<dout7_4::DOUT7_4_SPEC>;
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout7_4;
#[doc = "DOUT11_8 (rw) register accessor: an alias for `Reg<DOUT11_8_SPEC>`"]
pub type DOUT11_8 = crate::Reg<dout11_8::DOUT11_8_SPEC>;
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout11_8;
#[doc = "DOUT15_12 (rw) register accessor: an alias for `Reg<DOUT15_12_SPEC>`"]
pub type DOUT15_12 = crate::Reg<dout15_12::DOUT15_12_SPEC>;
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout15_12;
#[doc = "DOUT19_16 (rw) register accessor: an alias for `Reg<DOUT19_16_SPEC>`"]
pub type DOUT19_16 = crate::Reg<dout19_16::DOUT19_16_SPEC>;
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout19_16;
#[doc = "DOUT23_20 (rw) register accessor: an alias for `Reg<DOUT23_20_SPEC>`"]
pub type DOUT23_20 = crate::Reg<dout23_20::DOUT23_20_SPEC>;
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout23_20;
#[doc = "DOUT27_24 (rw) register accessor: an alias for `Reg<DOUT27_24_SPEC>`"]
pub type DOUT27_24 = crate::Reg<dout27_24::DOUT27_24_SPEC>;
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout27_24;
#[doc = "DOUT31_28 (rw) register accessor: an alias for `Reg<DOUT31_28_SPEC>`"]
pub type DOUT31_28 = crate::Reg<dout31_28::DOUT31_28_SPEC>;
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT31_0"]
pub mod dout31_28;
#[doc = "DOUT31_0 (rw) register accessor: an alias for `Reg<DOUT31_0_SPEC>`"]
pub type DOUT31_0 = crate::Reg<dout31_0::DOUT31_0_SPEC>;
#[doc = "Data Output for DIO 0 to 31"]
pub mod dout31_0;
#[doc = "DOUTSET31_0 (rw) register accessor: an alias for `Reg<DOUTSET31_0_SPEC>`"]
pub type DOUTSET31_0 = crate::Reg<doutset31_0::DOUTSET31_0_SPEC>;
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT31_0 register"]
pub mod doutset31_0;
#[doc = "DOUTCLR31_0 (rw) register accessor: an alias for `Reg<DOUTCLR31_0_SPEC>`"]
pub type DOUTCLR31_0 = crate::Reg<doutclr31_0::DOUTCLR31_0_SPEC>;
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT31_0 register"]
pub mod doutclr31_0;
#[doc = "DOUTTGL31_0 (rw) register accessor: an alias for `Reg<DOUTTGL31_0_SPEC>`"]
pub type DOUTTGL31_0 = crate::Reg<douttgl31_0::DOUTTGL31_0_SPEC>;
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub mod douttgl31_0;
#[doc = "DIN31_0 (rw) register accessor: an alias for `Reg<DIN31_0_SPEC>`"]
pub type DIN31_0 = crate::Reg<din31_0::DIN31_0_SPEC>;
#[doc = "Data Input from DIO 0 to 31"]
pub mod din31_0;
#[doc = "DOE31_0 (rw) register accessor: an alias for `Reg<DOE31_0_SPEC>`"]
pub type DOE31_0 = crate::Reg<doe31_0::DOE31_0_SPEC>;
#[doc = "Data Output Enable for DIO 0 to 31"]
pub mod doe31_0;
#[doc = "EVFLAGS31_0 (rw) register accessor: an alias for `Reg<EVFLAGS31_0_SPEC>`"]
pub type EVFLAGS31_0 = crate::Reg<evflags31_0::EVFLAGS31_0_SPEC>;
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub mod evflags31_0;
