#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Output Selection for CPU Interrupt 0"]
    pub cpuirqsel0: CPUIRQSEL0,
    #[doc = "0x04 - Output Selection for CPU Interrupt 1"]
    pub cpuirqsel1: CPUIRQSEL1,
    #[doc = "0x08 - Output Selection for CPU Interrupt 2"]
    pub cpuirqsel2: CPUIRQSEL2,
    #[doc = "0x0c - Output Selection for CPU Interrupt 3"]
    pub cpuirqsel3: CPUIRQSEL3,
    #[doc = "0x10 - Output Selection for CPU Interrupt 4"]
    pub cpuirqsel4: CPUIRQSEL4,
    #[doc = "0x14 - Output Selection for CPU Interrupt 5"]
    pub cpuirqsel5: CPUIRQSEL5,
    #[doc = "0x18 - Output Selection for CPU Interrupt 6"]
    pub cpuirqsel6: CPUIRQSEL6,
    #[doc = "0x1c - Output Selection for CPU Interrupt 7"]
    pub cpuirqsel7: CPUIRQSEL7,
    #[doc = "0x20 - Output Selection for CPU Interrupt 8"]
    pub cpuirqsel8: CPUIRQSEL8,
    #[doc = "0x24 - Output Selection for CPU Interrupt 9"]
    pub cpuirqsel9: CPUIRQSEL9,
    #[doc = "0x28 - Output Selection for CPU Interrupt 10"]
    pub cpuirqsel10: CPUIRQSEL10,
    #[doc = "0x2c - Output Selection for CPU Interrupt 11"]
    pub cpuirqsel11: CPUIRQSEL11,
    #[doc = "0x30 - Output Selection for CPU Interrupt 12"]
    pub cpuirqsel12: CPUIRQSEL12,
    #[doc = "0x34 - Output Selection for CPU Interrupt 13"]
    pub cpuirqsel13: CPUIRQSEL13,
    #[doc = "0x38 - Output Selection for CPU Interrupt 14"]
    pub cpuirqsel14: CPUIRQSEL14,
    #[doc = "0x3c - Output Selection for CPU Interrupt 15"]
    pub cpuirqsel15: CPUIRQSEL15,
    #[doc = "0x40 - Output Selection for CPU Interrupt 16"]
    pub cpuirqsel16: CPUIRQSEL16,
    #[doc = "0x44 - Output Selection for CPU Interrupt 17"]
    pub cpuirqsel17: CPUIRQSEL17,
    #[doc = "0x48 - Output Selection for CPU Interrupt 18"]
    pub cpuirqsel18: CPUIRQSEL18,
    #[doc = "0x4c - Output Selection for CPU Interrupt 19"]
    pub cpuirqsel19: CPUIRQSEL19,
    #[doc = "0x50 - Output Selection for CPU Interrupt 20"]
    pub cpuirqsel20: CPUIRQSEL20,
    #[doc = "0x54 - Output Selection for CPU Interrupt 21"]
    pub cpuirqsel21: CPUIRQSEL21,
    #[doc = "0x58 - Output Selection for CPU Interrupt 22"]
    pub cpuirqsel22: CPUIRQSEL22,
    #[doc = "0x5c - Output Selection for CPU Interrupt 23"]
    pub cpuirqsel23: CPUIRQSEL23,
    #[doc = "0x60 - Output Selection for CPU Interrupt 24"]
    pub cpuirqsel24: CPUIRQSEL24,
    #[doc = "0x64 - Output Selection for CPU Interrupt 25"]
    pub cpuirqsel25: CPUIRQSEL25,
    #[doc = "0x68 - Output Selection for CPU Interrupt 26"]
    pub cpuirqsel26: CPUIRQSEL26,
    #[doc = "0x6c - Output Selection for CPU Interrupt 27"]
    pub cpuirqsel27: CPUIRQSEL27,
    #[doc = "0x70 - Output Selection for CPU Interrupt 28"]
    pub cpuirqsel28: CPUIRQSEL28,
    #[doc = "0x74 - Output Selection for CPU Interrupt 29"]
    pub cpuirqsel29: CPUIRQSEL29,
    #[doc = "0x78 - Output Selection for CPU Interrupt 30"]
    pub cpuirqsel30: CPUIRQSEL30,
    #[doc = "0x7c - Output Selection for CPU Interrupt 31"]
    pub cpuirqsel31: CPUIRQSEL31,
    #[doc = "0x80 - Output Selection for CPU Interrupt 32"]
    pub cpuirqsel32: CPUIRQSEL32,
    #[doc = "0x84 - Output Selection for CPU Interrupt 33"]
    pub cpuirqsel33: CPUIRQSEL33,
    _reserved34: [u8; 0x78],
    #[doc = "0x100 - Output Selection for RFC Event 0"]
    pub rfcsel0: RFCSEL0,
    #[doc = "0x104 - Output Selection for RFC Event 1"]
    pub rfcsel1: RFCSEL1,
    #[doc = "0x108 - Output Selection for RFC Event 2"]
    pub rfcsel2: RFCSEL2,
    #[doc = "0x10c - Output Selection for RFC Event 3"]
    pub rfcsel3: RFCSEL3,
    #[doc = "0x110 - Output Selection for RFC Event 4"]
    pub rfcsel4: RFCSEL4,
    #[doc = "0x114 - Output Selection for RFC Event 5"]
    pub rfcsel5: RFCSEL5,
    #[doc = "0x118 - Output Selection for RFC Event 6"]
    pub rfcsel6: RFCSEL6,
    #[doc = "0x11c - Output Selection for RFC Event 7"]
    pub rfcsel7: RFCSEL7,
    #[doc = "0x120 - Output Selection for RFC Event 8"]
    pub rfcsel8: RFCSEL8,
    #[doc = "0x124 - Output Selection for RFC Event 9"]
    pub rfcsel9: RFCSEL9,
    _reserved44: [u8; 0xd8],
    #[doc = "0x200 - Output Selection for GPT0 0"]
    pub gpt0acaptsel: GPT0ACAPTSEL,
    #[doc = "0x204 - Output Selection for GPT0 1"]
    pub gpt0bcaptsel: GPT0BCAPTSEL,
    _reserved46: [u8; 0xf8],
    #[doc = "0x300 - Output Selection for GPT1 0"]
    pub gpt1acaptsel: GPT1ACAPTSEL,
    #[doc = "0x304 - Output Selection for GPT1 1"]
    pub gpt1bcaptsel: GPT1BCAPTSEL,
    _reserved48: [u8; 0xf8],
    #[doc = "0x400 - Output Selection for GPT2 0"]
    pub gpt2acaptsel: GPT2ACAPTSEL,
    #[doc = "0x404 - Output Selection for GPT2 1"]
    pub gpt2bcaptsel: GPT2BCAPTSEL,
    _reserved50: [u8; 0xf8],
    #[doc = "0x500 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach0ssel: UDMACH0SSEL,
    #[doc = "0x504 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach0bsel: UDMACH0BSEL,
    #[doc = "0x508 - Output Selection for DMA Channel 1 SREQ"]
    pub udmach1ssel: UDMACH1SSEL,
    #[doc = "0x50c - Output Selection for DMA Channel 1 REQ"]
    pub udmach1bsel: UDMACH1BSEL,
    #[doc = "0x510 - Output Selection for DMA Channel 2 SREQ"]
    pub udmach2ssel: UDMACH2SSEL,
    #[doc = "0x514 - Output Selection for DMA Channel 2 REQ"]
    pub udmach2bsel: UDMACH2BSEL,
    #[doc = "0x518 - Output Selection for DMA Channel 3 SREQ"]
    pub udmach3ssel: UDMACH3SSEL,
    #[doc = "0x51c - Output Selection for DMA Channel 3 REQ"]
    pub udmach3bsel: UDMACH3BSEL,
    #[doc = "0x520 - Output Selection for DMA Channel 4 SREQ"]
    pub udmach4ssel: UDMACH4SSEL,
    #[doc = "0x524 - Output Selection for DMA Channel 4 REQ"]
    pub udmach4bsel: UDMACH4BSEL,
    #[doc = "0x528 - Output Selection for DMA Channel 5 SREQ"]
    pub udmach5ssel: UDMACH5SSEL,
    #[doc = "0x52c - Output Selection for DMA Channel 5 REQ"]
    pub udmach5bsel: UDMACH5BSEL,
    #[doc = "0x530 - Output Selection for DMA Channel 6 SREQ"]
    pub udmach6ssel: UDMACH6SSEL,
    #[doc = "0x534 - Output Selection for DMA Channel 6 REQ"]
    pub udmach6bsel: UDMACH6BSEL,
    #[doc = "0x538 - Output Selection for DMA Channel 7 SREQ"]
    pub udmach7ssel: UDMACH7SSEL,
    #[doc = "0x53c - Output Selection for DMA Channel 7 REQ"]
    pub udmach7bsel: UDMACH7BSEL,
    #[doc = "0x540 - Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
    pub udmach8ssel: UDMACH8SSEL,
    #[doc = "0x544 - Output Selection for DMA Channel 8 REQ"]
    pub udmach8bsel: UDMACH8BSEL,
    #[doc = "0x548 - Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    pub udmach9ssel: UDMACH9SSEL,
    #[doc = "0x54c - Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    pub udmach9bsel: UDMACH9BSEL,
    #[doc = "0x550 - Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    pub udmach10ssel: UDMACH10SSEL,
    #[doc = "0x554 - Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    pub udmach10bsel: UDMACH10BSEL,
    #[doc = "0x558 - Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    pub udmach11ssel: UDMACH11SSEL,
    #[doc = "0x55c - Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    pub udmach11bsel: UDMACH11BSEL,
    #[doc = "0x560 - Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    pub udmach12ssel: UDMACH12SSEL,
    #[doc = "0x564 - Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    pub udmach12bsel: UDMACH12BSEL,
    #[doc = "0x568 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach13ssel: UDMACH13SSEL,
    #[doc = "0x56c - Output Selection for DMA Channel 13 REQ"]
    pub udmach13bsel: UDMACH13BSEL,
    #[doc = "0x570 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach14ssel: UDMACH14SSEL,
    #[doc = "0x574 - Output Selection for DMA Channel 14 REQ"]
    pub udmach14bsel: UDMACH14BSEL,
    #[doc = "0x578 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach15ssel: UDMACH15SSEL,
    #[doc = "0x57c - Output Selection for DMA Channel 15 REQ"]
    pub udmach15bsel: UDMACH15BSEL,
    #[doc = "0x580 - Output Selection for DMA Channel 16 SREQ"]
    pub udmach16ssel: UDMACH16SSEL,
    #[doc = "0x584 - Output Selection for DMA Channel 16 REQ"]
    pub udmach16bsel: UDMACH16BSEL,
    #[doc = "0x588 - Output Selection for DMA Channel 17 SREQ"]
    pub udmach17ssel: UDMACH17SSEL,
    #[doc = "0x58c - Output Selection for DMA Channel 17 REQ"]
    pub udmach17bsel: UDMACH17BSEL,
    #[doc = "0x590 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach18ssel: UDMACH18SSEL,
    #[doc = "0x594 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach18bsel: UDMACH18BSEL,
    #[doc = "0x598 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach19ssel: UDMACH19SSEL,
    #[doc = "0x59c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach19bsel: UDMACH19BSEL,
    #[doc = "0x5a0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach20ssel: UDMACH20SSEL,
    #[doc = "0x5a4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach20bsel: UDMACH20BSEL,
    #[doc = "0x5a8 - Output Selection for DMA Channel 21 SREQ"]
    pub udmach21ssel: UDMACH21SSEL,
    #[doc = "0x5ac - Output Selection for DMA Channel 21 REQ"]
    pub udmach21bsel: UDMACH21BSEL,
    #[doc = "0x5b0 - Output Selection for DMA Channel 22 SREQ"]
    pub udmach22ssel: UDMACH22SSEL,
    #[doc = "0x5b4 - Output Selection for DMA Channel 22 REQ"]
    pub udmach22bsel: UDMACH22BSEL,
    #[doc = "0x5b8 - Output Selection for DMA Channel 23 SREQ"]
    pub udmach23ssel: UDMACH23SSEL,
    #[doc = "0x5bc - Output Selection for DMA Channel 23 REQ"]
    pub udmach23bsel: UDMACH23BSEL,
    #[doc = "0x5c0 - Output Selection for DMA Channel 24 SREQ"]
    pub udmach24ssel: UDMACH24SSEL,
    #[doc = "0x5c4 - Output Selection for DMA Channel 24 REQ"]
    pub udmach24bsel: UDMACH24BSEL,
    #[doc = "0x5c8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach25ssel: UDMACH25SSEL,
    #[doc = "0x5cc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach25bsel: UDMACH25BSEL,
    #[doc = "0x5d0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach26ssel: UDMACH26SSEL,
    #[doc = "0x5d4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach26bsel: UDMACH26BSEL,
    #[doc = "0x5d8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach27ssel: UDMACH27SSEL,
    #[doc = "0x5dc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach27bsel: UDMACH27BSEL,
    #[doc = "0x5e0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach28ssel: UDMACH28SSEL,
    #[doc = "0x5e4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach28bsel: UDMACH28BSEL,
    #[doc = "0x5e8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach29ssel: UDMACH29SSEL,
    #[doc = "0x5ec - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach29bsel: UDMACH29BSEL,
    #[doc = "0x5f0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach30ssel: UDMACH30SSEL,
    #[doc = "0x5f4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach30bsel: UDMACH30BSEL,
    #[doc = "0x5f8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach31ssel: UDMACH31SSEL,
    #[doc = "0x5fc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    pub udmach31bsel: UDMACH31BSEL,
    #[doc = "0x600 - Output Selection for GPT3 0"]
    pub gpt3acaptsel: GPT3ACAPTSEL,
    #[doc = "0x604 - Output Selection for GPT3 1"]
    pub gpt3bcaptsel: GPT3BCAPTSEL,
    _reserved116: [u8; 0xf8],
    #[doc = "0x700 - Output Selection for AUX Subscriber 0"]
    pub auxsel0: AUXSEL0,
    _reserved117: [u8; 0xfc],
    #[doc = "0x800 - Output Selection for NMI Subscriber 0"]
    pub cm3nmisel0: CM3NMISEL0,
    _reserved118: [u8; 0xfc],
    #[doc = "0x900 - Output Selection for I2S Subscriber 0"]
    pub i2sstmpsel0: I2SSTMPSEL0,
    _reserved119: [u8; 0xfc],
    #[doc = "0xa00 - Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
    pub frzsel0: FRZSEL0,
    _reserved120: [u8; 0x04fc],
    #[doc = "0xf00 - Set or Clear Software Events"]
    pub swev: SWEV,
}
#[doc = "CPUIRQSEL0 (rw) register accessor: an alias for `Reg<CPUIRQSEL0_SPEC>`"]
pub type CPUIRQSEL0 = crate::Reg<cpuirqsel0::CPUIRQSEL0_SPEC>;
#[doc = "Output Selection for CPU Interrupt 0"]
pub mod cpuirqsel0;
#[doc = "CPUIRQSEL1 (rw) register accessor: an alias for `Reg<CPUIRQSEL1_SPEC>`"]
pub type CPUIRQSEL1 = crate::Reg<cpuirqsel1::CPUIRQSEL1_SPEC>;
#[doc = "Output Selection for CPU Interrupt 1"]
pub mod cpuirqsel1;
#[doc = "CPUIRQSEL2 (rw) register accessor: an alias for `Reg<CPUIRQSEL2_SPEC>`"]
pub type CPUIRQSEL2 = crate::Reg<cpuirqsel2::CPUIRQSEL2_SPEC>;
#[doc = "Output Selection for CPU Interrupt 2"]
pub mod cpuirqsel2;
#[doc = "CPUIRQSEL3 (rw) register accessor: an alias for `Reg<CPUIRQSEL3_SPEC>`"]
pub type CPUIRQSEL3 = crate::Reg<cpuirqsel3::CPUIRQSEL3_SPEC>;
#[doc = "Output Selection for CPU Interrupt 3"]
pub mod cpuirqsel3;
#[doc = "CPUIRQSEL4 (rw) register accessor: an alias for `Reg<CPUIRQSEL4_SPEC>`"]
pub type CPUIRQSEL4 = crate::Reg<cpuirqsel4::CPUIRQSEL4_SPEC>;
#[doc = "Output Selection for CPU Interrupt 4"]
pub mod cpuirqsel4;
#[doc = "CPUIRQSEL5 (rw) register accessor: an alias for `Reg<CPUIRQSEL5_SPEC>`"]
pub type CPUIRQSEL5 = crate::Reg<cpuirqsel5::CPUIRQSEL5_SPEC>;
#[doc = "Output Selection for CPU Interrupt 5"]
pub mod cpuirqsel5;
#[doc = "CPUIRQSEL6 (rw) register accessor: an alias for `Reg<CPUIRQSEL6_SPEC>`"]
pub type CPUIRQSEL6 = crate::Reg<cpuirqsel6::CPUIRQSEL6_SPEC>;
#[doc = "Output Selection for CPU Interrupt 6"]
pub mod cpuirqsel6;
#[doc = "CPUIRQSEL7 (rw) register accessor: an alias for `Reg<CPUIRQSEL7_SPEC>`"]
pub type CPUIRQSEL7 = crate::Reg<cpuirqsel7::CPUIRQSEL7_SPEC>;
#[doc = "Output Selection for CPU Interrupt 7"]
pub mod cpuirqsel7;
#[doc = "CPUIRQSEL8 (rw) register accessor: an alias for `Reg<CPUIRQSEL8_SPEC>`"]
pub type CPUIRQSEL8 = crate::Reg<cpuirqsel8::CPUIRQSEL8_SPEC>;
#[doc = "Output Selection for CPU Interrupt 8"]
pub mod cpuirqsel8;
#[doc = "CPUIRQSEL9 (rw) register accessor: an alias for `Reg<CPUIRQSEL9_SPEC>`"]
pub type CPUIRQSEL9 = crate::Reg<cpuirqsel9::CPUIRQSEL9_SPEC>;
#[doc = "Output Selection for CPU Interrupt 9"]
pub mod cpuirqsel9;
#[doc = "CPUIRQSEL10 (rw) register accessor: an alias for `Reg<CPUIRQSEL10_SPEC>`"]
pub type CPUIRQSEL10 = crate::Reg<cpuirqsel10::CPUIRQSEL10_SPEC>;
#[doc = "Output Selection for CPU Interrupt 10"]
pub mod cpuirqsel10;
#[doc = "CPUIRQSEL11 (rw) register accessor: an alias for `Reg<CPUIRQSEL11_SPEC>`"]
pub type CPUIRQSEL11 = crate::Reg<cpuirqsel11::CPUIRQSEL11_SPEC>;
#[doc = "Output Selection for CPU Interrupt 11"]
pub mod cpuirqsel11;
#[doc = "CPUIRQSEL12 (rw) register accessor: an alias for `Reg<CPUIRQSEL12_SPEC>`"]
pub type CPUIRQSEL12 = crate::Reg<cpuirqsel12::CPUIRQSEL12_SPEC>;
#[doc = "Output Selection for CPU Interrupt 12"]
pub mod cpuirqsel12;
#[doc = "CPUIRQSEL13 (rw) register accessor: an alias for `Reg<CPUIRQSEL13_SPEC>`"]
pub type CPUIRQSEL13 = crate::Reg<cpuirqsel13::CPUIRQSEL13_SPEC>;
#[doc = "Output Selection for CPU Interrupt 13"]
pub mod cpuirqsel13;
#[doc = "CPUIRQSEL14 (rw) register accessor: an alias for `Reg<CPUIRQSEL14_SPEC>`"]
pub type CPUIRQSEL14 = crate::Reg<cpuirqsel14::CPUIRQSEL14_SPEC>;
#[doc = "Output Selection for CPU Interrupt 14"]
pub mod cpuirqsel14;
#[doc = "CPUIRQSEL15 (rw) register accessor: an alias for `Reg<CPUIRQSEL15_SPEC>`"]
pub type CPUIRQSEL15 = crate::Reg<cpuirqsel15::CPUIRQSEL15_SPEC>;
#[doc = "Output Selection for CPU Interrupt 15"]
pub mod cpuirqsel15;
#[doc = "CPUIRQSEL16 (rw) register accessor: an alias for `Reg<CPUIRQSEL16_SPEC>`"]
pub type CPUIRQSEL16 = crate::Reg<cpuirqsel16::CPUIRQSEL16_SPEC>;
#[doc = "Output Selection for CPU Interrupt 16"]
pub mod cpuirqsel16;
#[doc = "CPUIRQSEL17 (rw) register accessor: an alias for `Reg<CPUIRQSEL17_SPEC>`"]
pub type CPUIRQSEL17 = crate::Reg<cpuirqsel17::CPUIRQSEL17_SPEC>;
#[doc = "Output Selection for CPU Interrupt 17"]
pub mod cpuirqsel17;
#[doc = "CPUIRQSEL18 (rw) register accessor: an alias for `Reg<CPUIRQSEL18_SPEC>`"]
pub type CPUIRQSEL18 = crate::Reg<cpuirqsel18::CPUIRQSEL18_SPEC>;
#[doc = "Output Selection for CPU Interrupt 18"]
pub mod cpuirqsel18;
#[doc = "CPUIRQSEL19 (rw) register accessor: an alias for `Reg<CPUIRQSEL19_SPEC>`"]
pub type CPUIRQSEL19 = crate::Reg<cpuirqsel19::CPUIRQSEL19_SPEC>;
#[doc = "Output Selection for CPU Interrupt 19"]
pub mod cpuirqsel19;
#[doc = "CPUIRQSEL20 (rw) register accessor: an alias for `Reg<CPUIRQSEL20_SPEC>`"]
pub type CPUIRQSEL20 = crate::Reg<cpuirqsel20::CPUIRQSEL20_SPEC>;
#[doc = "Output Selection for CPU Interrupt 20"]
pub mod cpuirqsel20;
#[doc = "CPUIRQSEL21 (rw) register accessor: an alias for `Reg<CPUIRQSEL21_SPEC>`"]
pub type CPUIRQSEL21 = crate::Reg<cpuirqsel21::CPUIRQSEL21_SPEC>;
#[doc = "Output Selection for CPU Interrupt 21"]
pub mod cpuirqsel21;
#[doc = "CPUIRQSEL22 (rw) register accessor: an alias for `Reg<CPUIRQSEL22_SPEC>`"]
pub type CPUIRQSEL22 = crate::Reg<cpuirqsel22::CPUIRQSEL22_SPEC>;
#[doc = "Output Selection for CPU Interrupt 22"]
pub mod cpuirqsel22;
#[doc = "CPUIRQSEL23 (rw) register accessor: an alias for `Reg<CPUIRQSEL23_SPEC>`"]
pub type CPUIRQSEL23 = crate::Reg<cpuirqsel23::CPUIRQSEL23_SPEC>;
#[doc = "Output Selection for CPU Interrupt 23"]
pub mod cpuirqsel23;
#[doc = "CPUIRQSEL24 (rw) register accessor: an alias for `Reg<CPUIRQSEL24_SPEC>`"]
pub type CPUIRQSEL24 = crate::Reg<cpuirqsel24::CPUIRQSEL24_SPEC>;
#[doc = "Output Selection for CPU Interrupt 24"]
pub mod cpuirqsel24;
#[doc = "CPUIRQSEL25 (rw) register accessor: an alias for `Reg<CPUIRQSEL25_SPEC>`"]
pub type CPUIRQSEL25 = crate::Reg<cpuirqsel25::CPUIRQSEL25_SPEC>;
#[doc = "Output Selection for CPU Interrupt 25"]
pub mod cpuirqsel25;
#[doc = "CPUIRQSEL26 (rw) register accessor: an alias for `Reg<CPUIRQSEL26_SPEC>`"]
pub type CPUIRQSEL26 = crate::Reg<cpuirqsel26::CPUIRQSEL26_SPEC>;
#[doc = "Output Selection for CPU Interrupt 26"]
pub mod cpuirqsel26;
#[doc = "CPUIRQSEL27 (rw) register accessor: an alias for `Reg<CPUIRQSEL27_SPEC>`"]
pub type CPUIRQSEL27 = crate::Reg<cpuirqsel27::CPUIRQSEL27_SPEC>;
#[doc = "Output Selection for CPU Interrupt 27"]
pub mod cpuirqsel27;
#[doc = "CPUIRQSEL28 (rw) register accessor: an alias for `Reg<CPUIRQSEL28_SPEC>`"]
pub type CPUIRQSEL28 = crate::Reg<cpuirqsel28::CPUIRQSEL28_SPEC>;
#[doc = "Output Selection for CPU Interrupt 28"]
pub mod cpuirqsel28;
#[doc = "CPUIRQSEL29 (rw) register accessor: an alias for `Reg<CPUIRQSEL29_SPEC>`"]
pub type CPUIRQSEL29 = crate::Reg<cpuirqsel29::CPUIRQSEL29_SPEC>;
#[doc = "Output Selection for CPU Interrupt 29"]
pub mod cpuirqsel29;
#[doc = "CPUIRQSEL30 (rw) register accessor: an alias for `Reg<CPUIRQSEL30_SPEC>`"]
pub type CPUIRQSEL30 = crate::Reg<cpuirqsel30::CPUIRQSEL30_SPEC>;
#[doc = "Output Selection for CPU Interrupt 30"]
pub mod cpuirqsel30;
#[doc = "CPUIRQSEL31 (rw) register accessor: an alias for `Reg<CPUIRQSEL31_SPEC>`"]
pub type CPUIRQSEL31 = crate::Reg<cpuirqsel31::CPUIRQSEL31_SPEC>;
#[doc = "Output Selection for CPU Interrupt 31"]
pub mod cpuirqsel31;
#[doc = "CPUIRQSEL32 (rw) register accessor: an alias for `Reg<CPUIRQSEL32_SPEC>`"]
pub type CPUIRQSEL32 = crate::Reg<cpuirqsel32::CPUIRQSEL32_SPEC>;
#[doc = "Output Selection for CPU Interrupt 32"]
pub mod cpuirqsel32;
#[doc = "CPUIRQSEL33 (rw) register accessor: an alias for `Reg<CPUIRQSEL33_SPEC>`"]
pub type CPUIRQSEL33 = crate::Reg<cpuirqsel33::CPUIRQSEL33_SPEC>;
#[doc = "Output Selection for CPU Interrupt 33"]
pub mod cpuirqsel33;
#[doc = "RFCSEL0 (rw) register accessor: an alias for `Reg<RFCSEL0_SPEC>`"]
pub type RFCSEL0 = crate::Reg<rfcsel0::RFCSEL0_SPEC>;
#[doc = "Output Selection for RFC Event 0"]
pub mod rfcsel0;
#[doc = "RFCSEL1 (rw) register accessor: an alias for `Reg<RFCSEL1_SPEC>`"]
pub type RFCSEL1 = crate::Reg<rfcsel1::RFCSEL1_SPEC>;
#[doc = "Output Selection for RFC Event 1"]
pub mod rfcsel1;
#[doc = "RFCSEL2 (rw) register accessor: an alias for `Reg<RFCSEL2_SPEC>`"]
pub type RFCSEL2 = crate::Reg<rfcsel2::RFCSEL2_SPEC>;
#[doc = "Output Selection for RFC Event 2"]
pub mod rfcsel2;
#[doc = "RFCSEL3 (rw) register accessor: an alias for `Reg<RFCSEL3_SPEC>`"]
pub type RFCSEL3 = crate::Reg<rfcsel3::RFCSEL3_SPEC>;
#[doc = "Output Selection for RFC Event 3"]
pub mod rfcsel3;
#[doc = "RFCSEL4 (rw) register accessor: an alias for `Reg<RFCSEL4_SPEC>`"]
pub type RFCSEL4 = crate::Reg<rfcsel4::RFCSEL4_SPEC>;
#[doc = "Output Selection for RFC Event 4"]
pub mod rfcsel4;
#[doc = "RFCSEL5 (rw) register accessor: an alias for `Reg<RFCSEL5_SPEC>`"]
pub type RFCSEL5 = crate::Reg<rfcsel5::RFCSEL5_SPEC>;
#[doc = "Output Selection for RFC Event 5"]
pub mod rfcsel5;
#[doc = "RFCSEL6 (rw) register accessor: an alias for `Reg<RFCSEL6_SPEC>`"]
pub type RFCSEL6 = crate::Reg<rfcsel6::RFCSEL6_SPEC>;
#[doc = "Output Selection for RFC Event 6"]
pub mod rfcsel6;
#[doc = "RFCSEL7 (rw) register accessor: an alias for `Reg<RFCSEL7_SPEC>`"]
pub type RFCSEL7 = crate::Reg<rfcsel7::RFCSEL7_SPEC>;
#[doc = "Output Selection for RFC Event 7"]
pub mod rfcsel7;
#[doc = "RFCSEL8 (rw) register accessor: an alias for `Reg<RFCSEL8_SPEC>`"]
pub type RFCSEL8 = crate::Reg<rfcsel8::RFCSEL8_SPEC>;
#[doc = "Output Selection for RFC Event 8"]
pub mod rfcsel8;
#[doc = "RFCSEL9 (rw) register accessor: an alias for `Reg<RFCSEL9_SPEC>`"]
pub type RFCSEL9 = crate::Reg<rfcsel9::RFCSEL9_SPEC>;
#[doc = "Output Selection for RFC Event 9"]
pub mod rfcsel9;
#[doc = "GPT0ACAPTSEL (rw) register accessor: an alias for `Reg<GPT0ACAPTSEL_SPEC>`"]
pub type GPT0ACAPTSEL = crate::Reg<gpt0acaptsel::GPT0ACAPTSEL_SPEC>;
#[doc = "Output Selection for GPT0 0"]
pub mod gpt0acaptsel;
#[doc = "GPT0BCAPTSEL (rw) register accessor: an alias for `Reg<GPT0BCAPTSEL_SPEC>`"]
pub type GPT0BCAPTSEL = crate::Reg<gpt0bcaptsel::GPT0BCAPTSEL_SPEC>;
#[doc = "Output Selection for GPT0 1"]
pub mod gpt0bcaptsel;
#[doc = "GPT1ACAPTSEL (rw) register accessor: an alias for `Reg<GPT1ACAPTSEL_SPEC>`"]
pub type GPT1ACAPTSEL = crate::Reg<gpt1acaptsel::GPT1ACAPTSEL_SPEC>;
#[doc = "Output Selection for GPT1 0"]
pub mod gpt1acaptsel;
#[doc = "GPT1BCAPTSEL (rw) register accessor: an alias for `Reg<GPT1BCAPTSEL_SPEC>`"]
pub type GPT1BCAPTSEL = crate::Reg<gpt1bcaptsel::GPT1BCAPTSEL_SPEC>;
#[doc = "Output Selection for GPT1 1"]
pub mod gpt1bcaptsel;
#[doc = "GPT2ACAPTSEL (rw) register accessor: an alias for `Reg<GPT2ACAPTSEL_SPEC>`"]
pub type GPT2ACAPTSEL = crate::Reg<gpt2acaptsel::GPT2ACAPTSEL_SPEC>;
#[doc = "Output Selection for GPT2 0"]
pub mod gpt2acaptsel;
#[doc = "GPT2BCAPTSEL (rw) register accessor: an alias for `Reg<GPT2BCAPTSEL_SPEC>`"]
pub type GPT2BCAPTSEL = crate::Reg<gpt2bcaptsel::GPT2BCAPTSEL_SPEC>;
#[doc = "Output Selection for GPT2 1"]
pub mod gpt2bcaptsel;
#[doc = "UDMACH0SSEL (rw) register accessor: an alias for `Reg<UDMACH0SSEL_SPEC>`"]
pub type UDMACH0SSEL = crate::Reg<udmach0ssel::UDMACH0SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach0ssel;
#[doc = "UDMACH0BSEL (rw) register accessor: an alias for `Reg<UDMACH0BSEL_SPEC>`"]
pub type UDMACH0BSEL = crate::Reg<udmach0bsel::UDMACH0BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach0bsel;
#[doc = "UDMACH1SSEL (rw) register accessor: an alias for `Reg<UDMACH1SSEL_SPEC>`"]
pub type UDMACH1SSEL = crate::Reg<udmach1ssel::UDMACH1SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 1 SREQ"]
pub mod udmach1ssel;
#[doc = "UDMACH1BSEL (rw) register accessor: an alias for `Reg<UDMACH1BSEL_SPEC>`"]
pub type UDMACH1BSEL = crate::Reg<udmach1bsel::UDMACH1BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 1 REQ"]
pub mod udmach1bsel;
#[doc = "UDMACH2SSEL (rw) register accessor: an alias for `Reg<UDMACH2SSEL_SPEC>`"]
pub type UDMACH2SSEL = crate::Reg<udmach2ssel::UDMACH2SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 2 SREQ"]
pub mod udmach2ssel;
#[doc = "UDMACH2BSEL (rw) register accessor: an alias for `Reg<UDMACH2BSEL_SPEC>`"]
pub type UDMACH2BSEL = crate::Reg<udmach2bsel::UDMACH2BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 2 REQ"]
pub mod udmach2bsel;
#[doc = "UDMACH3SSEL (rw) register accessor: an alias for `Reg<UDMACH3SSEL_SPEC>`"]
pub type UDMACH3SSEL = crate::Reg<udmach3ssel::UDMACH3SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 3 SREQ"]
pub mod udmach3ssel;
#[doc = "UDMACH3BSEL (rw) register accessor: an alias for `Reg<UDMACH3BSEL_SPEC>`"]
pub type UDMACH3BSEL = crate::Reg<udmach3bsel::UDMACH3BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 3 REQ"]
pub mod udmach3bsel;
#[doc = "UDMACH4SSEL (rw) register accessor: an alias for `Reg<UDMACH4SSEL_SPEC>`"]
pub type UDMACH4SSEL = crate::Reg<udmach4ssel::UDMACH4SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 4 SREQ"]
pub mod udmach4ssel;
#[doc = "UDMACH4BSEL (rw) register accessor: an alias for `Reg<UDMACH4BSEL_SPEC>`"]
pub type UDMACH4BSEL = crate::Reg<udmach4bsel::UDMACH4BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 4 REQ"]
pub mod udmach4bsel;
#[doc = "UDMACH5SSEL (rw) register accessor: an alias for `Reg<UDMACH5SSEL_SPEC>`"]
pub type UDMACH5SSEL = crate::Reg<udmach5ssel::UDMACH5SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 5 SREQ"]
pub mod udmach5ssel;
#[doc = "UDMACH5BSEL (rw) register accessor: an alias for `Reg<UDMACH5BSEL_SPEC>`"]
pub type UDMACH5BSEL = crate::Reg<udmach5bsel::UDMACH5BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 5 REQ"]
pub mod udmach5bsel;
#[doc = "UDMACH6SSEL (rw) register accessor: an alias for `Reg<UDMACH6SSEL_SPEC>`"]
pub type UDMACH6SSEL = crate::Reg<udmach6ssel::UDMACH6SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 6 SREQ"]
pub mod udmach6ssel;
#[doc = "UDMACH6BSEL (rw) register accessor: an alias for `Reg<UDMACH6BSEL_SPEC>`"]
pub type UDMACH6BSEL = crate::Reg<udmach6bsel::UDMACH6BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 6 REQ"]
pub mod udmach6bsel;
#[doc = "UDMACH7SSEL (rw) register accessor: an alias for `Reg<UDMACH7SSEL_SPEC>`"]
pub type UDMACH7SSEL = crate::Reg<udmach7ssel::UDMACH7SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 7 SREQ"]
pub mod udmach7ssel;
#[doc = "UDMACH7BSEL (rw) register accessor: an alias for `Reg<UDMACH7BSEL_SPEC>`"]
pub type UDMACH7BSEL = crate::Reg<udmach7bsel::UDMACH7BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 7 REQ"]
pub mod udmach7bsel;
#[doc = "UDMACH8SSEL (rw) register accessor: an alias for `Reg<UDMACH8SSEL_SPEC>`"]
pub type UDMACH8SSEL = crate::Reg<udmach8ssel::UDMACH8SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
pub mod udmach8ssel;
#[doc = "UDMACH8BSEL (rw) register accessor: an alias for `Reg<UDMACH8BSEL_SPEC>`"]
pub type UDMACH8BSEL = crate::Reg<udmach8bsel::UDMACH8BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 8 REQ"]
pub mod udmach8bsel;
#[doc = "UDMACH9SSEL (rw) register accessor: an alias for `Reg<UDMACH9SSEL_SPEC>`"]
pub type UDMACH9SSEL = crate::Reg<udmach9ssel::UDMACH9SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9ssel;
#[doc = "UDMACH9BSEL (rw) register accessor: an alias for `Reg<UDMACH9BSEL_SPEC>`"]
pub type UDMACH9BSEL = crate::Reg<udmach9bsel::UDMACH9BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9bsel;
#[doc = "UDMACH10SSEL (rw) register accessor: an alias for `Reg<UDMACH10SSEL_SPEC>`"]
pub type UDMACH10SSEL = crate::Reg<udmach10ssel::UDMACH10SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10ssel;
#[doc = "UDMACH10BSEL (rw) register accessor: an alias for `Reg<UDMACH10BSEL_SPEC>`"]
pub type UDMACH10BSEL = crate::Reg<udmach10bsel::UDMACH10BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10bsel;
#[doc = "UDMACH11SSEL (rw) register accessor: an alias for `Reg<UDMACH11SSEL_SPEC>`"]
pub type UDMACH11SSEL = crate::Reg<udmach11ssel::UDMACH11SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11ssel;
#[doc = "UDMACH11BSEL (rw) register accessor: an alias for `Reg<UDMACH11BSEL_SPEC>`"]
pub type UDMACH11BSEL = crate::Reg<udmach11bsel::UDMACH11BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11bsel;
#[doc = "UDMACH12SSEL (rw) register accessor: an alias for `Reg<UDMACH12SSEL_SPEC>`"]
pub type UDMACH12SSEL = crate::Reg<udmach12ssel::UDMACH12SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12ssel;
#[doc = "UDMACH12BSEL (rw) register accessor: an alias for `Reg<UDMACH12BSEL_SPEC>`"]
pub type UDMACH12BSEL = crate::Reg<udmach12bsel::UDMACH12BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12bsel;
#[doc = "UDMACH13SSEL (rw) register accessor: an alias for `Reg<UDMACH13SSEL_SPEC>`"]
pub type UDMACH13SSEL = crate::Reg<udmach13ssel::UDMACH13SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach13ssel;
#[doc = "UDMACH13BSEL (rw) register accessor: an alias for `Reg<UDMACH13BSEL_SPEC>`"]
pub type UDMACH13BSEL = crate::Reg<udmach13bsel::UDMACH13BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 13 REQ"]
pub mod udmach13bsel;
#[doc = "UDMACH14SSEL (rw) register accessor: an alias for `Reg<UDMACH14SSEL_SPEC>`"]
pub type UDMACH14SSEL = crate::Reg<udmach14ssel::UDMACH14SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach14ssel;
#[doc = "UDMACH14BSEL (rw) register accessor: an alias for `Reg<UDMACH14BSEL_SPEC>`"]
pub type UDMACH14BSEL = crate::Reg<udmach14bsel::UDMACH14BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 14 REQ"]
pub mod udmach14bsel;
#[doc = "UDMACH15SSEL (rw) register accessor: an alias for `Reg<UDMACH15SSEL_SPEC>`"]
pub type UDMACH15SSEL = crate::Reg<udmach15ssel::UDMACH15SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach15ssel;
#[doc = "UDMACH15BSEL (rw) register accessor: an alias for `Reg<UDMACH15BSEL_SPEC>`"]
pub type UDMACH15BSEL = crate::Reg<udmach15bsel::UDMACH15BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 15 REQ"]
pub mod udmach15bsel;
#[doc = "UDMACH16SSEL (rw) register accessor: an alias for `Reg<UDMACH16SSEL_SPEC>`"]
pub type UDMACH16SSEL = crate::Reg<udmach16ssel::UDMACH16SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 16 SREQ"]
pub mod udmach16ssel;
#[doc = "UDMACH16BSEL (rw) register accessor: an alias for `Reg<UDMACH16BSEL_SPEC>`"]
pub type UDMACH16BSEL = crate::Reg<udmach16bsel::UDMACH16BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 16 REQ"]
pub mod udmach16bsel;
#[doc = "UDMACH17SSEL (rw) register accessor: an alias for `Reg<UDMACH17SSEL_SPEC>`"]
pub type UDMACH17SSEL = crate::Reg<udmach17ssel::UDMACH17SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 17 SREQ"]
pub mod udmach17ssel;
#[doc = "UDMACH17BSEL (rw) register accessor: an alias for `Reg<UDMACH17BSEL_SPEC>`"]
pub type UDMACH17BSEL = crate::Reg<udmach17bsel::UDMACH17BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 17 REQ"]
pub mod udmach17bsel;
#[doc = "UDMACH18SSEL (rw) register accessor: an alias for `Reg<UDMACH18SSEL_SPEC>`"]
pub type UDMACH18SSEL = crate::Reg<udmach18ssel::UDMACH18SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach18ssel;
#[doc = "UDMACH18BSEL (rw) register accessor: an alias for `Reg<UDMACH18BSEL_SPEC>`"]
pub type UDMACH18BSEL = crate::Reg<udmach18bsel::UDMACH18BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach18bsel;
#[doc = "UDMACH19SSEL (rw) register accessor: an alias for `Reg<UDMACH19SSEL_SPEC>`"]
pub type UDMACH19SSEL = crate::Reg<udmach19ssel::UDMACH19SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach19ssel;
#[doc = "UDMACH19BSEL (rw) register accessor: an alias for `Reg<UDMACH19BSEL_SPEC>`"]
pub type UDMACH19BSEL = crate::Reg<udmach19bsel::UDMACH19BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach19bsel;
#[doc = "UDMACH20SSEL (rw) register accessor: an alias for `Reg<UDMACH20SSEL_SPEC>`"]
pub type UDMACH20SSEL = crate::Reg<udmach20ssel::UDMACH20SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach20ssel;
#[doc = "UDMACH20BSEL (rw) register accessor: an alias for `Reg<UDMACH20BSEL_SPEC>`"]
pub type UDMACH20BSEL = crate::Reg<udmach20bsel::UDMACH20BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach20bsel;
#[doc = "UDMACH21SSEL (rw) register accessor: an alias for `Reg<UDMACH21SSEL_SPEC>`"]
pub type UDMACH21SSEL = crate::Reg<udmach21ssel::UDMACH21SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 21 SREQ"]
pub mod udmach21ssel;
#[doc = "UDMACH21BSEL (rw) register accessor: an alias for `Reg<UDMACH21BSEL_SPEC>`"]
pub type UDMACH21BSEL = crate::Reg<udmach21bsel::UDMACH21BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 21 REQ"]
pub mod udmach21bsel;
#[doc = "UDMACH22SSEL (rw) register accessor: an alias for `Reg<UDMACH22SSEL_SPEC>`"]
pub type UDMACH22SSEL = crate::Reg<udmach22ssel::UDMACH22SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 22 SREQ"]
pub mod udmach22ssel;
#[doc = "UDMACH22BSEL (rw) register accessor: an alias for `Reg<UDMACH22BSEL_SPEC>`"]
pub type UDMACH22BSEL = crate::Reg<udmach22bsel::UDMACH22BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 22 REQ"]
pub mod udmach22bsel;
#[doc = "UDMACH23SSEL (rw) register accessor: an alias for `Reg<UDMACH23SSEL_SPEC>`"]
pub type UDMACH23SSEL = crate::Reg<udmach23ssel::UDMACH23SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 23 SREQ"]
pub mod udmach23ssel;
#[doc = "UDMACH23BSEL (rw) register accessor: an alias for `Reg<UDMACH23BSEL_SPEC>`"]
pub type UDMACH23BSEL = crate::Reg<udmach23bsel::UDMACH23BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 23 REQ"]
pub mod udmach23bsel;
#[doc = "UDMACH24SSEL (rw) register accessor: an alias for `Reg<UDMACH24SSEL_SPEC>`"]
pub type UDMACH24SSEL = crate::Reg<udmach24ssel::UDMACH24SSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 24 SREQ"]
pub mod udmach24ssel;
#[doc = "UDMACH24BSEL (rw) register accessor: an alias for `Reg<UDMACH24BSEL_SPEC>`"]
pub type UDMACH24BSEL = crate::Reg<udmach24bsel::UDMACH24BSEL_SPEC>;
#[doc = "Output Selection for DMA Channel 24 REQ"]
pub mod udmach24bsel;
#[doc = "UDMACH25SSEL (rw) register accessor: an alias for `Reg<UDMACH25SSEL_SPEC>`"]
pub type UDMACH25SSEL = crate::Reg<udmach25ssel::UDMACH25SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach25ssel;
#[doc = "UDMACH25BSEL (rw) register accessor: an alias for `Reg<UDMACH25BSEL_SPEC>`"]
pub type UDMACH25BSEL = crate::Reg<udmach25bsel::UDMACH25BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach25bsel;
#[doc = "UDMACH26SSEL (rw) register accessor: an alias for `Reg<UDMACH26SSEL_SPEC>`"]
pub type UDMACH26SSEL = crate::Reg<udmach26ssel::UDMACH26SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach26ssel;
#[doc = "UDMACH26BSEL (rw) register accessor: an alias for `Reg<UDMACH26BSEL_SPEC>`"]
pub type UDMACH26BSEL = crate::Reg<udmach26bsel::UDMACH26BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach26bsel;
#[doc = "UDMACH27SSEL (rw) register accessor: an alias for `Reg<UDMACH27SSEL_SPEC>`"]
pub type UDMACH27SSEL = crate::Reg<udmach27ssel::UDMACH27SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach27ssel;
#[doc = "UDMACH27BSEL (rw) register accessor: an alias for `Reg<UDMACH27BSEL_SPEC>`"]
pub type UDMACH27BSEL = crate::Reg<udmach27bsel::UDMACH27BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach27bsel;
#[doc = "UDMACH28SSEL (rw) register accessor: an alias for `Reg<UDMACH28SSEL_SPEC>`"]
pub type UDMACH28SSEL = crate::Reg<udmach28ssel::UDMACH28SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach28ssel;
#[doc = "UDMACH28BSEL (rw) register accessor: an alias for `Reg<UDMACH28BSEL_SPEC>`"]
pub type UDMACH28BSEL = crate::Reg<udmach28bsel::UDMACH28BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach28bsel;
#[doc = "UDMACH29SSEL (rw) register accessor: an alias for `Reg<UDMACH29SSEL_SPEC>`"]
pub type UDMACH29SSEL = crate::Reg<udmach29ssel::UDMACH29SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach29ssel;
#[doc = "UDMACH29BSEL (rw) register accessor: an alias for `Reg<UDMACH29BSEL_SPEC>`"]
pub type UDMACH29BSEL = crate::Reg<udmach29bsel::UDMACH29BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach29bsel;
#[doc = "UDMACH30SSEL (rw) register accessor: an alias for `Reg<UDMACH30SSEL_SPEC>`"]
pub type UDMACH30SSEL = crate::Reg<udmach30ssel::UDMACH30SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach30ssel;
#[doc = "UDMACH30BSEL (rw) register accessor: an alias for `Reg<UDMACH30BSEL_SPEC>`"]
pub type UDMACH30BSEL = crate::Reg<udmach30bsel::UDMACH30BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach30bsel;
#[doc = "UDMACH31SSEL (rw) register accessor: an alias for `Reg<UDMACH31SSEL_SPEC>`"]
pub type UDMACH31SSEL = crate::Reg<udmach31ssel::UDMACH31SSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach31ssel;
#[doc = "UDMACH31BSEL (rw) register accessor: an alias for `Reg<UDMACH31BSEL_SPEC>`"]
pub type UDMACH31BSEL = crate::Reg<udmach31bsel::UDMACH31BSEL_SPEC>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach31bsel;
#[doc = "GPT3ACAPTSEL (rw) register accessor: an alias for `Reg<GPT3ACAPTSEL_SPEC>`"]
pub type GPT3ACAPTSEL = crate::Reg<gpt3acaptsel::GPT3ACAPTSEL_SPEC>;
#[doc = "Output Selection for GPT3 0"]
pub mod gpt3acaptsel;
#[doc = "GPT3BCAPTSEL (rw) register accessor: an alias for `Reg<GPT3BCAPTSEL_SPEC>`"]
pub type GPT3BCAPTSEL = crate::Reg<gpt3bcaptsel::GPT3BCAPTSEL_SPEC>;
#[doc = "Output Selection for GPT3 1"]
pub mod gpt3bcaptsel;
#[doc = "AUXSEL0 (rw) register accessor: an alias for `Reg<AUXSEL0_SPEC>`"]
pub type AUXSEL0 = crate::Reg<auxsel0::AUXSEL0_SPEC>;
#[doc = "Output Selection for AUX Subscriber 0"]
pub mod auxsel0;
#[doc = "CM3NMISEL0 (rw) register accessor: an alias for `Reg<CM3NMISEL0_SPEC>`"]
pub type CM3NMISEL0 = crate::Reg<cm3nmisel0::CM3NMISEL0_SPEC>;
#[doc = "Output Selection for NMI Subscriber 0"]
pub mod cm3nmisel0;
#[doc = "I2SSTMPSEL0 (rw) register accessor: an alias for `Reg<I2SSTMPSEL0_SPEC>`"]
pub type I2SSTMPSEL0 = crate::Reg<i2sstmpsel0::I2SSTMPSEL0_SPEC>;
#[doc = "Output Selection for I2S Subscriber 0"]
pub mod i2sstmpsel0;
#[doc = "FRZSEL0 (rw) register accessor: an alias for `Reg<FRZSEL0_SPEC>`"]
pub type FRZSEL0 = crate::Reg<frzsel0::FRZSEL0_SPEC>;
#[doc = "Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
pub mod frzsel0;
#[doc = "SWEV (rw) register accessor: an alias for `Reg<SWEV_SPEC>`"]
pub type SWEV = crate::Reg<swev::SWEV_SPEC>;
#[doc = "Set or Clear Software Events"]
pub mod swev;
