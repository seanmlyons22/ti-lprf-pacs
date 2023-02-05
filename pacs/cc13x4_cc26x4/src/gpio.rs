#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Out 0 to 3 Alias register for byte access to each bit in DOUT47_0"]
    pub dout3_0: DOUT3_0,
    #[doc = "0x04 - Data Out 4 to 7 Alias register for byte access to each bit in DOUT47_0"]
    pub dout7_4: DOUT7_4,
    #[doc = "0x08 - Data Out 8 to 11 Alias register for byte access to each bit in DOUT47_0"]
    pub dout11_8: DOUT11_8,
    #[doc = "0x0c - Data Out 12 to 15 Alias register for byte access to each bit in DOUT47_0"]
    pub dout15_12: DOUT15_12,
    #[doc = "0x10 - Data Out 16 to 19 Alias register for byte access to each bit in DOUT47_0"]
    pub dout19_16: DOUT19_16,
    #[doc = "0x14 - Data Out 20 to 23 Alias register for byte access to each bit in DOUT47_0"]
    pub dout23_20: DOUT23_20,
    #[doc = "0x18 - Data Out 24 to 27 Alias register for byte access to each bit in DOUT47_0"]
    pub dout27_24: DOUT27_24,
    #[doc = "0x1c - Data Out 28 to 31 Alias register for byte access to each bit in DOUT47_0"]
    pub dout31_28: DOUT31_28,
    #[doc = "0x20 - Data Out 35 to 32 Alias register for byte access to each bit in DOUT47_0"]
    pub dout35_32: DOUT35_32,
    #[doc = "0x24 - Data Out 39 to 36 Alias register for byte access to each bit in DOUT47_0"]
    pub dout39_36: DOUT39_36,
    #[doc = "0x28 - Data Out 43 to 40 Alias register for byte access to each bit in DOUT47_0"]
    pub dout43_40: DOUT43_40,
    #[doc = "0x2c - Data Out 47 to 44 Alias register for byte access to each bit in DOUT47_0"]
    pub dout47_44: DOUT47_44,
    _reserved12: [u8; 0x50],
    #[doc = "0x80 - Data Output for DIO 0 to 31"]
    pub dout31_0: DOUT31_0,
    #[doc = "0x84 - Data Output for DIO 0 to 31"]
    pub dout47_32: DOUT47_32,
    _reserved14: [u8; 0x08],
    #[doc = "0x90 - Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
    pub doutset31_0: DOUTSET31_0,
    #[doc = "0x94 - Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
    pub doutset47_32: DOUTSET47_32,
    _reserved16: [u8; 0x08],
    #[doc = "0xa0 - Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
    pub doutclr31_0: DOUTCLR31_0,
    #[doc = "0xa4 - Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
    pub doutclr47_32: DOUTCLR47_32,
    _reserved18: [u8; 0x08],
    #[doc = "0xb0 - Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    pub douttgl31_0: DOUTTGL31_0,
    #[doc = "0xb4 - Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
    pub douttgl47_32: DOUTTGL47_32,
    _reserved20: [u8; 0x08],
    #[doc = "0xc0 - Data Input from DIO 0 to 31"]
    pub din31_0: DIN31_0,
    #[doc = "0xc4 - Data Input from DIO 32 to 47"]
    pub din47_32: DIN47_32,
    _reserved22: [u8; 0x08],
    #[doc = "0xd0 - Data Output Enable for DIO 0 to 31"]
    pub doe31_0: DOE31_0,
    #[doc = "0xd4 - Data Output Enable for DIO 32 to 47"]
    pub doe47_32: DOE47_32,
    _reserved24: [u8; 0x08],
    #[doc = "0xe0 - Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    pub evflags31_0: EVFLAGS31_0,
    #[doc = "0xe4 - Event Register for DIO 32 to 47 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
    pub evflags47_32: EVFLAGS47_32,
}
#[doc = "DOUT3_0 (rw) register accessor: an alias for `Reg<DOUT3_0_SPEC>`"]
pub type DOUT3_0 = crate::Reg<dout3_0::DOUT3_0_SPEC>;
#[doc = "Data Out 0 to 3 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout3_0;
#[doc = "DOUT7_4 (rw) register accessor: an alias for `Reg<DOUT7_4_SPEC>`"]
pub type DOUT7_4 = crate::Reg<dout7_4::DOUT7_4_SPEC>;
#[doc = "Data Out 4 to 7 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout7_4;
#[doc = "DOUT11_8 (rw) register accessor: an alias for `Reg<DOUT11_8_SPEC>`"]
pub type DOUT11_8 = crate::Reg<dout11_8::DOUT11_8_SPEC>;
#[doc = "Data Out 8 to 11 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout11_8;
#[doc = "DOUT15_12 (rw) register accessor: an alias for `Reg<DOUT15_12_SPEC>`"]
pub type DOUT15_12 = crate::Reg<dout15_12::DOUT15_12_SPEC>;
#[doc = "Data Out 12 to 15 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout15_12;
#[doc = "DOUT19_16 (rw) register accessor: an alias for `Reg<DOUT19_16_SPEC>`"]
pub type DOUT19_16 = crate::Reg<dout19_16::DOUT19_16_SPEC>;
#[doc = "Data Out 16 to 19 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout19_16;
#[doc = "DOUT23_20 (rw) register accessor: an alias for `Reg<DOUT23_20_SPEC>`"]
pub type DOUT23_20 = crate::Reg<dout23_20::DOUT23_20_SPEC>;
#[doc = "Data Out 20 to 23 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout23_20;
#[doc = "DOUT27_24 (rw) register accessor: an alias for `Reg<DOUT27_24_SPEC>`"]
pub type DOUT27_24 = crate::Reg<dout27_24::DOUT27_24_SPEC>;
#[doc = "Data Out 24 to 27 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout27_24;
#[doc = "DOUT31_28 (rw) register accessor: an alias for `Reg<DOUT31_28_SPEC>`"]
pub type DOUT31_28 = crate::Reg<dout31_28::DOUT31_28_SPEC>;
#[doc = "Data Out 28 to 31 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout31_28;
#[doc = "DOUT35_32 (rw) register accessor: an alias for `Reg<DOUT35_32_SPEC>`"]
pub type DOUT35_32 = crate::Reg<dout35_32::DOUT35_32_SPEC>;
#[doc = "Data Out 35 to 32 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout35_32;
#[doc = "DOUT39_36 (rw) register accessor: an alias for `Reg<DOUT39_36_SPEC>`"]
pub type DOUT39_36 = crate::Reg<dout39_36::DOUT39_36_SPEC>;
#[doc = "Data Out 39 to 36 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout39_36;
#[doc = "DOUT43_40 (rw) register accessor: an alias for `Reg<DOUT43_40_SPEC>`"]
pub type DOUT43_40 = crate::Reg<dout43_40::DOUT43_40_SPEC>;
#[doc = "Data Out 43 to 40 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout43_40;
#[doc = "DOUT47_44 (rw) register accessor: an alias for `Reg<DOUT47_44_SPEC>`"]
pub type DOUT47_44 = crate::Reg<dout47_44::DOUT47_44_SPEC>;
#[doc = "Data Out 47 to 44 Alias register for byte access to each bit in DOUT47_0"]
pub mod dout47_44;
#[doc = "DOUT31_0 (rw) register accessor: an alias for `Reg<DOUT31_0_SPEC>`"]
pub type DOUT31_0 = crate::Reg<dout31_0::DOUT31_0_SPEC>;
#[doc = "Data Output for DIO 0 to 31"]
pub mod dout31_0;
#[doc = "DOUT47_32 (rw) register accessor: an alias for `Reg<DOUT47_32_SPEC>`"]
pub type DOUT47_32 = crate::Reg<dout47_32::DOUT47_32_SPEC>;
#[doc = "Data Output for DIO 0 to 31"]
pub mod dout47_32;
#[doc = "DOUTSET31_0 (rw) register accessor: an alias for `Reg<DOUTSET31_0_SPEC>`"]
pub type DOUTSET31_0 = crate::Reg<doutset31_0::DOUTSET31_0_SPEC>;
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
pub mod doutset31_0;
#[doc = "DOUTSET47_32 (rw) register accessor: an alias for `Reg<DOUTSET47_32_SPEC>`"]
pub type DOUTSET47_32 = crate::Reg<doutset47_32::DOUTSET47_32_SPEC>;
#[doc = "Data Out Set Writing 1 to a bit position sets the corresponding bit in the DOUT47_0 register"]
pub mod doutset47_32;
#[doc = "DOUTCLR31_0 (rw) register accessor: an alias for `Reg<DOUTCLR31_0_SPEC>`"]
pub type DOUTCLR31_0 = crate::Reg<doutclr31_0::DOUTCLR31_0_SPEC>;
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
pub mod doutclr31_0;
#[doc = "DOUTCLR47_32 (rw) register accessor: an alias for `Reg<DOUTCLR47_32_SPEC>`"]
pub type DOUTCLR47_32 = crate::Reg<doutclr47_32::DOUTCLR47_32_SPEC>;
#[doc = "Data Out Clear Writing 1 to a bit position clears the corresponding bit in the DOUT47_0 register"]
pub mod doutclr47_32;
#[doc = "DOUTTGL31_0 (rw) register accessor: an alias for `Reg<DOUTTGL31_0_SPEC>`"]
pub type DOUTTGL31_0 = crate::Reg<douttgl31_0::DOUTTGL31_0_SPEC>;
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub mod douttgl31_0;
#[doc = "DOUTTGL47_32 (rw) register accessor: an alias for `Reg<DOUTTGL47_32_SPEC>`"]
pub type DOUTTGL47_32 = crate::Reg<douttgl47_32::DOUTTGL47_32_SPEC>;
#[doc = "Data Out Toggle Writing 1 to a bit position will invert the corresponding DIO output."]
pub mod douttgl47_32;
#[doc = "DIN31_0 (rw) register accessor: an alias for `Reg<DIN31_0_SPEC>`"]
pub type DIN31_0 = crate::Reg<din31_0::DIN31_0_SPEC>;
#[doc = "Data Input from DIO 0 to 31"]
pub mod din31_0;
#[doc = "DIN47_32 (rw) register accessor: an alias for `Reg<DIN47_32_SPEC>`"]
pub type DIN47_32 = crate::Reg<din47_32::DIN47_32_SPEC>;
#[doc = "Data Input from DIO 32 to 47"]
pub mod din47_32;
#[doc = "DOE31_0 (rw) register accessor: an alias for `Reg<DOE31_0_SPEC>`"]
pub type DOE31_0 = crate::Reg<doe31_0::DOE31_0_SPEC>;
#[doc = "Data Output Enable for DIO 0 to 31"]
pub mod doe31_0;
#[doc = "DOE47_32 (rw) register accessor: an alias for `Reg<DOE47_32_SPEC>`"]
pub type DOE47_32 = crate::Reg<doe47_32::DOE47_32_SPEC>;
#[doc = "Data Output Enable for DIO 32 to 47"]
pub mod doe47_32;
#[doc = "EVFLAGS31_0 (rw) register accessor: an alias for `Reg<EVFLAGS31_0_SPEC>`"]
pub type EVFLAGS31_0 = crate::Reg<evflags31_0::EVFLAGS31_0_SPEC>;
#[doc = "Event Register for DIO 0 to 31 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub mod evflags31_0;
#[doc = "EVFLAGS47_32 (rw) register accessor: an alias for `Reg<EVFLAGS47_32_SPEC>`"]
pub type EVFLAGS47_32 = crate::Reg<evflags47_32::EVFLAGS47_32_SPEC>;
#[doc = "Event Register for DIO 32 to 47 Reading this registers will return 1 for triggered event and 0 for non-triggered events. Writing a 1 to a bit field will clear the event. The configuration of events is done inside MCU IOC, e.g. events for DIO #0 is configured in IOC:IOCFG0.EDGE_DET and IOC:IOCFG0.EDGE_IRQ_EN."]
pub mod evflags47_32;
