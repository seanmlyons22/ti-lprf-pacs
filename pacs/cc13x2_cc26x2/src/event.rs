#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cpuirqsel0: Cpuirqsel0,
    cpuirqsel1: Cpuirqsel1,
    cpuirqsel2: Cpuirqsel2,
    cpuirqsel3: Cpuirqsel3,
    cpuirqsel4: Cpuirqsel4,
    cpuirqsel5: Cpuirqsel5,
    cpuirqsel6: Cpuirqsel6,
    cpuirqsel7: Cpuirqsel7,
    cpuirqsel8: Cpuirqsel8,
    cpuirqsel9: Cpuirqsel9,
    cpuirqsel10: Cpuirqsel10,
    cpuirqsel11: Cpuirqsel11,
    cpuirqsel12: Cpuirqsel12,
    cpuirqsel13: Cpuirqsel13,
    cpuirqsel14: Cpuirqsel14,
    cpuirqsel15: Cpuirqsel15,
    cpuirqsel16: Cpuirqsel16,
    cpuirqsel17: Cpuirqsel17,
    cpuirqsel18: Cpuirqsel18,
    cpuirqsel19: Cpuirqsel19,
    cpuirqsel20: Cpuirqsel20,
    cpuirqsel21: Cpuirqsel21,
    cpuirqsel22: Cpuirqsel22,
    cpuirqsel23: Cpuirqsel23,
    cpuirqsel24: Cpuirqsel24,
    cpuirqsel25: Cpuirqsel25,
    cpuirqsel26: Cpuirqsel26,
    cpuirqsel27: Cpuirqsel27,
    cpuirqsel28: Cpuirqsel28,
    cpuirqsel29: Cpuirqsel29,
    cpuirqsel30: Cpuirqsel30,
    cpuirqsel31: Cpuirqsel31,
    cpuirqsel32: Cpuirqsel32,
    cpuirqsel33: Cpuirqsel33,
    cpuirqsel34: Cpuirqsel34,
    cpuirqsel35: Cpuirqsel35,
    cpuirqsel36: Cpuirqsel36,
    cpuirqsel37: Cpuirqsel37,
    _reserved38: [u8; 0x68],
    rfcsel0: Rfcsel0,
    rfcsel1: Rfcsel1,
    rfcsel2: Rfcsel2,
    rfcsel3: Rfcsel3,
    rfcsel4: Rfcsel4,
    rfcsel5: Rfcsel5,
    rfcsel6: Rfcsel6,
    rfcsel7: Rfcsel7,
    rfcsel8: Rfcsel8,
    rfcsel9: Rfcsel9,
    _reserved48: [u8; 0xd8],
    gpt0acaptsel: Gpt0acaptsel,
    gpt0bcaptsel: Gpt0bcaptsel,
    _reserved50: [u8; 0xf8],
    gpt1acaptsel: Gpt1acaptsel,
    gpt1bcaptsel: Gpt1bcaptsel,
    _reserved52: [u8; 0xf8],
    gpt2acaptsel: Gpt2acaptsel,
    gpt2bcaptsel: Gpt2bcaptsel,
    _reserved54: [u8; 0xf8],
    udmach0ssel: Udmach0ssel,
    udmach0bsel: Udmach0bsel,
    udmach1ssel: Udmach1ssel,
    udmach1bsel: Udmach1bsel,
    udmach2ssel: Udmach2ssel,
    udmach2bsel: Udmach2bsel,
    udmach3ssel: Udmach3ssel,
    udmach3bsel: Udmach3bsel,
    udmach4ssel: Udmach4ssel,
    udmach4bsel: Udmach4bsel,
    udmach5ssel: Udmach5ssel,
    udmach5bsel: Udmach5bsel,
    udmach6ssel: Udmach6ssel,
    udmach6bsel: Udmach6bsel,
    udmach7ssel: Udmach7ssel,
    udmach7bsel: Udmach7bsel,
    udmach8ssel: Udmach8ssel,
    udmach8bsel: Udmach8bsel,
    udmach9ssel: Udmach9ssel,
    udmach9bsel: Udmach9bsel,
    udmach10ssel: Udmach10ssel,
    udmach10bsel: Udmach10bsel,
    udmach11ssel: Udmach11ssel,
    udmach11bsel: Udmach11bsel,
    udmach12ssel: Udmach12ssel,
    udmach12bsel: Udmach12bsel,
    udmach13ssel: Udmach13ssel,
    udmach13bsel: Udmach13bsel,
    udmach14ssel: Udmach14ssel,
    udmach14bsel: Udmach14bsel,
    udmach15ssel: Udmach15ssel,
    udmach15bsel: Udmach15bsel,
    udmach16ssel: Udmach16ssel,
    udmach16bsel: Udmach16bsel,
    udmach17ssel: Udmach17ssel,
    udmach17bsel: Udmach17bsel,
    udmach18ssel: Udmach18ssel,
    udmach18bsel: Udmach18bsel,
    udmach19ssel: Udmach19ssel,
    udmach19bsel: Udmach19bsel,
    udmach20ssel: Udmach20ssel,
    udmach20bsel: Udmach20bsel,
    udmach21ssel: Udmach21ssel,
    udmach21bsel: Udmach21bsel,
    udmach22ssel: Udmach22ssel,
    udmach22bsel: Udmach22bsel,
    udmach23ssel: Udmach23ssel,
    udmach23bsel: Udmach23bsel,
    udmach24ssel: Udmach24ssel,
    udmach24bsel: Udmach24bsel,
    udmach25ssel: Udmach25ssel,
    udmach25bsel: Udmach25bsel,
    udmach26ssel: Udmach26ssel,
    udmach26bsel: Udmach26bsel,
    udmach27ssel: Udmach27ssel,
    udmach27bsel: Udmach27bsel,
    udmach28ssel: Udmach28ssel,
    udmach28bsel: Udmach28bsel,
    udmach29ssel: Udmach29ssel,
    udmach29bsel: Udmach29bsel,
    udmach30ssel: Udmach30ssel,
    udmach30bsel: Udmach30bsel,
    udmach31ssel: Udmach31ssel,
    udmach31bsel: Udmach31bsel,
    gpt3acaptsel: Gpt3acaptsel,
    gpt3bcaptsel: Gpt3bcaptsel,
    _reserved120: [u8; 0xf8],
    auxsel0: Auxsel0,
    _reserved121: [u8; 0xfc],
    cm3nmisel0: Cm3nmisel0,
    _reserved122: [u8; 0xfc],
    i2sstmpsel0: I2sstmpsel0,
    _reserved123: [u8; 0xfc],
    frzsel0: Frzsel0,
    _reserved124: [u8; 0x04fc],
    swev: Swev,
}
impl RegisterBlock {
    #[doc = "0x00 - Output Selection for CPU Interrupt 0"]
    #[inline(always)]
    pub const fn cpuirqsel0(&self) -> &Cpuirqsel0 {
        &self.cpuirqsel0
    }
    #[doc = "0x04 - Output Selection for CPU Interrupt 1"]
    #[inline(always)]
    pub const fn cpuirqsel1(&self) -> &Cpuirqsel1 {
        &self.cpuirqsel1
    }
    #[doc = "0x08 - Output Selection for CPU Interrupt 2"]
    #[inline(always)]
    pub const fn cpuirqsel2(&self) -> &Cpuirqsel2 {
        &self.cpuirqsel2
    }
    #[doc = "0x0c - Output Selection for CPU Interrupt 3"]
    #[inline(always)]
    pub const fn cpuirqsel3(&self) -> &Cpuirqsel3 {
        &self.cpuirqsel3
    }
    #[doc = "0x10 - Output Selection for CPU Interrupt 4"]
    #[inline(always)]
    pub const fn cpuirqsel4(&self) -> &Cpuirqsel4 {
        &self.cpuirqsel4
    }
    #[doc = "0x14 - Output Selection for CPU Interrupt 5"]
    #[inline(always)]
    pub const fn cpuirqsel5(&self) -> &Cpuirqsel5 {
        &self.cpuirqsel5
    }
    #[doc = "0x18 - Output Selection for CPU Interrupt 6"]
    #[inline(always)]
    pub const fn cpuirqsel6(&self) -> &Cpuirqsel6 {
        &self.cpuirqsel6
    }
    #[doc = "0x1c - Output Selection for CPU Interrupt 7"]
    #[inline(always)]
    pub const fn cpuirqsel7(&self) -> &Cpuirqsel7 {
        &self.cpuirqsel7
    }
    #[doc = "0x20 - Output Selection for CPU Interrupt 8"]
    #[inline(always)]
    pub const fn cpuirqsel8(&self) -> &Cpuirqsel8 {
        &self.cpuirqsel8
    }
    #[doc = "0x24 - Output Selection for CPU Interrupt 9"]
    #[inline(always)]
    pub const fn cpuirqsel9(&self) -> &Cpuirqsel9 {
        &self.cpuirqsel9
    }
    #[doc = "0x28 - Output Selection for CPU Interrupt 10"]
    #[inline(always)]
    pub const fn cpuirqsel10(&self) -> &Cpuirqsel10 {
        &self.cpuirqsel10
    }
    #[doc = "0x2c - Output Selection for CPU Interrupt 11"]
    #[inline(always)]
    pub const fn cpuirqsel11(&self) -> &Cpuirqsel11 {
        &self.cpuirqsel11
    }
    #[doc = "0x30 - Output Selection for CPU Interrupt 12"]
    #[inline(always)]
    pub const fn cpuirqsel12(&self) -> &Cpuirqsel12 {
        &self.cpuirqsel12
    }
    #[doc = "0x34 - Output Selection for CPU Interrupt 13"]
    #[inline(always)]
    pub const fn cpuirqsel13(&self) -> &Cpuirqsel13 {
        &self.cpuirqsel13
    }
    #[doc = "0x38 - Output Selection for CPU Interrupt 14"]
    #[inline(always)]
    pub const fn cpuirqsel14(&self) -> &Cpuirqsel14 {
        &self.cpuirqsel14
    }
    #[doc = "0x3c - Output Selection for CPU Interrupt 15"]
    #[inline(always)]
    pub const fn cpuirqsel15(&self) -> &Cpuirqsel15 {
        &self.cpuirqsel15
    }
    #[doc = "0x40 - Output Selection for CPU Interrupt 16"]
    #[inline(always)]
    pub const fn cpuirqsel16(&self) -> &Cpuirqsel16 {
        &self.cpuirqsel16
    }
    #[doc = "0x44 - Output Selection for CPU Interrupt 17"]
    #[inline(always)]
    pub const fn cpuirqsel17(&self) -> &Cpuirqsel17 {
        &self.cpuirqsel17
    }
    #[doc = "0x48 - Output Selection for CPU Interrupt 18"]
    #[inline(always)]
    pub const fn cpuirqsel18(&self) -> &Cpuirqsel18 {
        &self.cpuirqsel18
    }
    #[doc = "0x4c - Output Selection for CPU Interrupt 19"]
    #[inline(always)]
    pub const fn cpuirqsel19(&self) -> &Cpuirqsel19 {
        &self.cpuirqsel19
    }
    #[doc = "0x50 - Output Selection for CPU Interrupt 20"]
    #[inline(always)]
    pub const fn cpuirqsel20(&self) -> &Cpuirqsel20 {
        &self.cpuirqsel20
    }
    #[doc = "0x54 - Output Selection for CPU Interrupt 21"]
    #[inline(always)]
    pub const fn cpuirqsel21(&self) -> &Cpuirqsel21 {
        &self.cpuirqsel21
    }
    #[doc = "0x58 - Output Selection for CPU Interrupt 22"]
    #[inline(always)]
    pub const fn cpuirqsel22(&self) -> &Cpuirqsel22 {
        &self.cpuirqsel22
    }
    #[doc = "0x5c - Output Selection for CPU Interrupt 23"]
    #[inline(always)]
    pub const fn cpuirqsel23(&self) -> &Cpuirqsel23 {
        &self.cpuirqsel23
    }
    #[doc = "0x60 - Output Selection for CPU Interrupt 24"]
    #[inline(always)]
    pub const fn cpuirqsel24(&self) -> &Cpuirqsel24 {
        &self.cpuirqsel24
    }
    #[doc = "0x64 - Output Selection for CPU Interrupt 25"]
    #[inline(always)]
    pub const fn cpuirqsel25(&self) -> &Cpuirqsel25 {
        &self.cpuirqsel25
    }
    #[doc = "0x68 - Output Selection for CPU Interrupt 26"]
    #[inline(always)]
    pub const fn cpuirqsel26(&self) -> &Cpuirqsel26 {
        &self.cpuirqsel26
    }
    #[doc = "0x6c - Output Selection for CPU Interrupt 27"]
    #[inline(always)]
    pub const fn cpuirqsel27(&self) -> &Cpuirqsel27 {
        &self.cpuirqsel27
    }
    #[doc = "0x70 - Output Selection for CPU Interrupt 28"]
    #[inline(always)]
    pub const fn cpuirqsel28(&self) -> &Cpuirqsel28 {
        &self.cpuirqsel28
    }
    #[doc = "0x74 - Output Selection for CPU Interrupt 29"]
    #[inline(always)]
    pub const fn cpuirqsel29(&self) -> &Cpuirqsel29 {
        &self.cpuirqsel29
    }
    #[doc = "0x78 - Output Selection for CPU Interrupt 30"]
    #[inline(always)]
    pub const fn cpuirqsel30(&self) -> &Cpuirqsel30 {
        &self.cpuirqsel30
    }
    #[doc = "0x7c - Output Selection for CPU Interrupt 31"]
    #[inline(always)]
    pub const fn cpuirqsel31(&self) -> &Cpuirqsel31 {
        &self.cpuirqsel31
    }
    #[doc = "0x80 - Output Selection for CPU Interrupt 32"]
    #[inline(always)]
    pub const fn cpuirqsel32(&self) -> &Cpuirqsel32 {
        &self.cpuirqsel32
    }
    #[doc = "0x84 - Output Selection for CPU Interrupt 33"]
    #[inline(always)]
    pub const fn cpuirqsel33(&self) -> &Cpuirqsel33 {
        &self.cpuirqsel33
    }
    #[doc = "0x88 - Output Selection for CPU Interrupt 34"]
    #[inline(always)]
    pub const fn cpuirqsel34(&self) -> &Cpuirqsel34 {
        &self.cpuirqsel34
    }
    #[doc = "0x8c - Output Selection for CPU Interrupt 35"]
    #[inline(always)]
    pub const fn cpuirqsel35(&self) -> &Cpuirqsel35 {
        &self.cpuirqsel35
    }
    #[doc = "0x90 - Output Selection for CPU Interrupt 36"]
    #[inline(always)]
    pub const fn cpuirqsel36(&self) -> &Cpuirqsel36 {
        &self.cpuirqsel36
    }
    #[doc = "0x94 - Output Selection for CPU Interrupt 37"]
    #[inline(always)]
    pub const fn cpuirqsel37(&self) -> &Cpuirqsel37 {
        &self.cpuirqsel37
    }
    #[doc = "0x100 - Output Selection for RFC Event 0"]
    #[inline(always)]
    pub const fn rfcsel0(&self) -> &Rfcsel0 {
        &self.rfcsel0
    }
    #[doc = "0x104 - Output Selection for RFC Event 1"]
    #[inline(always)]
    pub const fn rfcsel1(&self) -> &Rfcsel1 {
        &self.rfcsel1
    }
    #[doc = "0x108 - Output Selection for RFC Event 2"]
    #[inline(always)]
    pub const fn rfcsel2(&self) -> &Rfcsel2 {
        &self.rfcsel2
    }
    #[doc = "0x10c - Output Selection for RFC Event 3"]
    #[inline(always)]
    pub const fn rfcsel3(&self) -> &Rfcsel3 {
        &self.rfcsel3
    }
    #[doc = "0x110 - Output Selection for RFC Event 4"]
    #[inline(always)]
    pub const fn rfcsel4(&self) -> &Rfcsel4 {
        &self.rfcsel4
    }
    #[doc = "0x114 - Output Selection for RFC Event 5"]
    #[inline(always)]
    pub const fn rfcsel5(&self) -> &Rfcsel5 {
        &self.rfcsel5
    }
    #[doc = "0x118 - Output Selection for RFC Event 6"]
    #[inline(always)]
    pub const fn rfcsel6(&self) -> &Rfcsel6 {
        &self.rfcsel6
    }
    #[doc = "0x11c - Output Selection for RFC Event 7"]
    #[inline(always)]
    pub const fn rfcsel7(&self) -> &Rfcsel7 {
        &self.rfcsel7
    }
    #[doc = "0x120 - Output Selection for RFC Event 8"]
    #[inline(always)]
    pub const fn rfcsel8(&self) -> &Rfcsel8 {
        &self.rfcsel8
    }
    #[doc = "0x124 - Output Selection for RFC Event 9"]
    #[inline(always)]
    pub const fn rfcsel9(&self) -> &Rfcsel9 {
        &self.rfcsel9
    }
    #[doc = "0x200 - Output Selection for GPT0 0"]
    #[inline(always)]
    pub const fn gpt0acaptsel(&self) -> &Gpt0acaptsel {
        &self.gpt0acaptsel
    }
    #[doc = "0x204 - Output Selection for GPT0 1"]
    #[inline(always)]
    pub const fn gpt0bcaptsel(&self) -> &Gpt0bcaptsel {
        &self.gpt0bcaptsel
    }
    #[doc = "0x300 - Output Selection for GPT1 0"]
    #[inline(always)]
    pub const fn gpt1acaptsel(&self) -> &Gpt1acaptsel {
        &self.gpt1acaptsel
    }
    #[doc = "0x304 - Output Selection for GPT1 1"]
    #[inline(always)]
    pub const fn gpt1bcaptsel(&self) -> &Gpt1bcaptsel {
        &self.gpt1bcaptsel
    }
    #[doc = "0x400 - Output Selection for GPT2 0"]
    #[inline(always)]
    pub const fn gpt2acaptsel(&self) -> &Gpt2acaptsel {
        &self.gpt2acaptsel
    }
    #[doc = "0x404 - Output Selection for GPT2 1"]
    #[inline(always)]
    pub const fn gpt2bcaptsel(&self) -> &Gpt2bcaptsel {
        &self.gpt2bcaptsel
    }
    #[doc = "0x500 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach0ssel(&self) -> &Udmach0ssel {
        &self.udmach0ssel
    }
    #[doc = "0x504 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach0bsel(&self) -> &Udmach0bsel {
        &self.udmach0bsel
    }
    #[doc = "0x508 - Output Selection for DMA Channel 1 SREQ"]
    #[inline(always)]
    pub const fn udmach1ssel(&self) -> &Udmach1ssel {
        &self.udmach1ssel
    }
    #[doc = "0x50c - Output Selection for DMA Channel 1 REQ"]
    #[inline(always)]
    pub const fn udmach1bsel(&self) -> &Udmach1bsel {
        &self.udmach1bsel
    }
    #[doc = "0x510 - Output Selection for DMA Channel 2 SREQ"]
    #[inline(always)]
    pub const fn udmach2ssel(&self) -> &Udmach2ssel {
        &self.udmach2ssel
    }
    #[doc = "0x514 - Output Selection for DMA Channel 2 REQ"]
    #[inline(always)]
    pub const fn udmach2bsel(&self) -> &Udmach2bsel {
        &self.udmach2bsel
    }
    #[doc = "0x518 - Output Selection for DMA Channel 3 SREQ"]
    #[inline(always)]
    pub const fn udmach3ssel(&self) -> &Udmach3ssel {
        &self.udmach3ssel
    }
    #[doc = "0x51c - Output Selection for DMA Channel 3 REQ"]
    #[inline(always)]
    pub const fn udmach3bsel(&self) -> &Udmach3bsel {
        &self.udmach3bsel
    }
    #[doc = "0x520 - Output Selection for DMA Channel 4 SREQ"]
    #[inline(always)]
    pub const fn udmach4ssel(&self) -> &Udmach4ssel {
        &self.udmach4ssel
    }
    #[doc = "0x524 - Output Selection for DMA Channel 4 REQ"]
    #[inline(always)]
    pub const fn udmach4bsel(&self) -> &Udmach4bsel {
        &self.udmach4bsel
    }
    #[doc = "0x528 - Output Selection for DMA Channel 5 SREQ"]
    #[inline(always)]
    pub const fn udmach5ssel(&self) -> &Udmach5ssel {
        &self.udmach5ssel
    }
    #[doc = "0x52c - Output Selection for DMA Channel 5 REQ"]
    #[inline(always)]
    pub const fn udmach5bsel(&self) -> &Udmach5bsel {
        &self.udmach5bsel
    }
    #[doc = "0x530 - Output Selection for DMA Channel 6 SREQ"]
    #[inline(always)]
    pub const fn udmach6ssel(&self) -> &Udmach6ssel {
        &self.udmach6ssel
    }
    #[doc = "0x534 - Output Selection for DMA Channel 6 REQ"]
    #[inline(always)]
    pub const fn udmach6bsel(&self) -> &Udmach6bsel {
        &self.udmach6bsel
    }
    #[doc = "0x538 - Output Selection for DMA Channel 7 SREQ"]
    #[inline(always)]
    pub const fn udmach7ssel(&self) -> &Udmach7ssel {
        &self.udmach7ssel
    }
    #[doc = "0x53c - Output Selection for DMA Channel 7 REQ"]
    #[inline(always)]
    pub const fn udmach7bsel(&self) -> &Udmach7bsel {
        &self.udmach7bsel
    }
    #[doc = "0x540 - Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
    #[inline(always)]
    pub const fn udmach8ssel(&self) -> &Udmach8ssel {
        &self.udmach8ssel
    }
    #[doc = "0x544 - Output Selection for DMA Channel 8 REQ"]
    #[inline(always)]
    pub const fn udmach8bsel(&self) -> &Udmach8bsel {
        &self.udmach8bsel
    }
    #[doc = "0x548 - Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    #[inline(always)]
    pub const fn udmach9ssel(&self) -> &Udmach9ssel {
        &self.udmach9ssel
    }
    #[doc = "0x54c - Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
    #[inline(always)]
    pub const fn udmach9bsel(&self) -> &Udmach9bsel {
        &self.udmach9bsel
    }
    #[doc = "0x550 - Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    #[inline(always)]
    pub const fn udmach10ssel(&self) -> &Udmach10ssel {
        &self.udmach10ssel
    }
    #[doc = "0x554 - Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
    #[inline(always)]
    pub const fn udmach10bsel(&self) -> &Udmach10bsel {
        &self.udmach10bsel
    }
    #[doc = "0x558 - Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    #[inline(always)]
    pub const fn udmach11ssel(&self) -> &Udmach11ssel {
        &self.udmach11ssel
    }
    #[doc = "0x55c - Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
    #[inline(always)]
    pub const fn udmach11bsel(&self) -> &Udmach11bsel {
        &self.udmach11bsel
    }
    #[doc = "0x560 - Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    #[inline(always)]
    pub const fn udmach12ssel(&self) -> &Udmach12ssel {
        &self.udmach12ssel
    }
    #[doc = "0x564 - Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
    #[inline(always)]
    pub const fn udmach12bsel(&self) -> &Udmach12bsel {
        &self.udmach12bsel
    }
    #[doc = "0x568 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach13ssel(&self) -> &Udmach13ssel {
        &self.udmach13ssel
    }
    #[doc = "0x56c - Output Selection for DMA Channel 13 REQ"]
    #[inline(always)]
    pub const fn udmach13bsel(&self) -> &Udmach13bsel {
        &self.udmach13bsel
    }
    #[doc = "0x570 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach14ssel(&self) -> &Udmach14ssel {
        &self.udmach14ssel
    }
    #[doc = "0x574 - Output Selection for DMA Channel 14 REQ"]
    #[inline(always)]
    pub const fn udmach14bsel(&self) -> &Udmach14bsel {
        &self.udmach14bsel
    }
    #[doc = "0x578 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach15ssel(&self) -> &Udmach15ssel {
        &self.udmach15ssel
    }
    #[doc = "0x57c - Output Selection for DMA Channel 15 REQ"]
    #[inline(always)]
    pub const fn udmach15bsel(&self) -> &Udmach15bsel {
        &self.udmach15bsel
    }
    #[doc = "0x580 - Output Selection for DMA Channel 16 SREQ"]
    #[inline(always)]
    pub const fn udmach16ssel(&self) -> &Udmach16ssel {
        &self.udmach16ssel
    }
    #[doc = "0x584 - Output Selection for DMA Channel 16 REQ"]
    #[inline(always)]
    pub const fn udmach16bsel(&self) -> &Udmach16bsel {
        &self.udmach16bsel
    }
    #[doc = "0x588 - Output Selection for DMA Channel 17 SREQ"]
    #[inline(always)]
    pub const fn udmach17ssel(&self) -> &Udmach17ssel {
        &self.udmach17ssel
    }
    #[doc = "0x58c - Output Selection for DMA Channel 17 REQ"]
    #[inline(always)]
    pub const fn udmach17bsel(&self) -> &Udmach17bsel {
        &self.udmach17bsel
    }
    #[doc = "0x590 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach18ssel(&self) -> &Udmach18ssel {
        &self.udmach18ssel
    }
    #[doc = "0x594 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach18bsel(&self) -> &Udmach18bsel {
        &self.udmach18bsel
    }
    #[doc = "0x598 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach19ssel(&self) -> &Udmach19ssel {
        &self.udmach19ssel
    }
    #[doc = "0x59c - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach19bsel(&self) -> &Udmach19bsel {
        &self.udmach19bsel
    }
    #[doc = "0x5a0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach20ssel(&self) -> &Udmach20ssel {
        &self.udmach20ssel
    }
    #[doc = "0x5a4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach20bsel(&self) -> &Udmach20bsel {
        &self.udmach20bsel
    }
    #[doc = "0x5a8 - Output Selection for DMA Channel 21 SREQ"]
    #[inline(always)]
    pub const fn udmach21ssel(&self) -> &Udmach21ssel {
        &self.udmach21ssel
    }
    #[doc = "0x5ac - Output Selection for DMA Channel 21 REQ"]
    #[inline(always)]
    pub const fn udmach21bsel(&self) -> &Udmach21bsel {
        &self.udmach21bsel
    }
    #[doc = "0x5b0 - Output Selection for DMA Channel 22 SREQ"]
    #[inline(always)]
    pub const fn udmach22ssel(&self) -> &Udmach22ssel {
        &self.udmach22ssel
    }
    #[doc = "0x5b4 - Output Selection for DMA Channel 22 REQ"]
    #[inline(always)]
    pub const fn udmach22bsel(&self) -> &Udmach22bsel {
        &self.udmach22bsel
    }
    #[doc = "0x5b8 - Output Selection for DMA Channel 23 SREQ"]
    #[inline(always)]
    pub const fn udmach23ssel(&self) -> &Udmach23ssel {
        &self.udmach23ssel
    }
    #[doc = "0x5bc - Output Selection for DMA Channel 23 REQ"]
    #[inline(always)]
    pub const fn udmach23bsel(&self) -> &Udmach23bsel {
        &self.udmach23bsel
    }
    #[doc = "0x5c0 - Output Selection for DMA Channel 24 SREQ"]
    #[inline(always)]
    pub const fn udmach24ssel(&self) -> &Udmach24ssel {
        &self.udmach24ssel
    }
    #[doc = "0x5c4 - Output Selection for DMA Channel 24 REQ"]
    #[inline(always)]
    pub const fn udmach24bsel(&self) -> &Udmach24bsel {
        &self.udmach24bsel
    }
    #[doc = "0x5c8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach25ssel(&self) -> &Udmach25ssel {
        &self.udmach25ssel
    }
    #[doc = "0x5cc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach25bsel(&self) -> &Udmach25bsel {
        &self.udmach25bsel
    }
    #[doc = "0x5d0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach26ssel(&self) -> &Udmach26ssel {
        &self.udmach26ssel
    }
    #[doc = "0x5d4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach26bsel(&self) -> &Udmach26bsel {
        &self.udmach26bsel
    }
    #[doc = "0x5d8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach27ssel(&self) -> &Udmach27ssel {
        &self.udmach27ssel
    }
    #[doc = "0x5dc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach27bsel(&self) -> &Udmach27bsel {
        &self.udmach27bsel
    }
    #[doc = "0x5e0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach28ssel(&self) -> &Udmach28ssel {
        &self.udmach28ssel
    }
    #[doc = "0x5e4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach28bsel(&self) -> &Udmach28bsel {
        &self.udmach28bsel
    }
    #[doc = "0x5e8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach29ssel(&self) -> &Udmach29ssel {
        &self.udmach29ssel
    }
    #[doc = "0x5ec - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach29bsel(&self) -> &Udmach29bsel {
        &self.udmach29bsel
    }
    #[doc = "0x5f0 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach30ssel(&self) -> &Udmach30ssel {
        &self.udmach30ssel
    }
    #[doc = "0x5f4 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach30bsel(&self) -> &Udmach30bsel {
        &self.udmach30bsel
    }
    #[doc = "0x5f8 - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach31ssel(&self) -> &Udmach31ssel {
        &self.udmach31ssel
    }
    #[doc = "0x5fc - Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub const fn udmach31bsel(&self) -> &Udmach31bsel {
        &self.udmach31bsel
    }
    #[doc = "0x600 - Output Selection for GPT3 0"]
    #[inline(always)]
    pub const fn gpt3acaptsel(&self) -> &Gpt3acaptsel {
        &self.gpt3acaptsel
    }
    #[doc = "0x604 - Output Selection for GPT3 1"]
    #[inline(always)]
    pub const fn gpt3bcaptsel(&self) -> &Gpt3bcaptsel {
        &self.gpt3bcaptsel
    }
    #[doc = "0x700 - Output Selection for AUX Subscriber 0"]
    #[inline(always)]
    pub const fn auxsel0(&self) -> &Auxsel0 {
        &self.auxsel0
    }
    #[doc = "0x800 - Output Selection for NMI Subscriber 0"]
    #[inline(always)]
    pub const fn cm3nmisel0(&self) -> &Cm3nmisel0 {
        &self.cm3nmisel0
    }
    #[doc = "0x900 - Output Selection for I2S Subscriber 0"]
    #[inline(always)]
    pub const fn i2sstmpsel0(&self) -> &I2sstmpsel0 {
        &self.i2sstmpsel0
    }
    #[doc = "0xa00 - Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
    #[inline(always)]
    pub const fn frzsel0(&self) -> &Frzsel0 {
        &self.frzsel0
    }
    #[doc = "0xf00 - Set or Clear Software Events"]
    #[inline(always)]
    pub const fn swev(&self) -> &Swev {
        &self.swev
    }
}
#[doc = "CPUIRQSEL0 (rw) register accessor: Output Selection for CPU Interrupt 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel0`]
module"]
#[doc(alias = "CPUIRQSEL0")]
pub type Cpuirqsel0 = crate::Reg<cpuirqsel0::Cpuirqsel0Spec>;
#[doc = "Output Selection for CPU Interrupt 0"]
pub mod cpuirqsel0;
#[doc = "CPUIRQSEL1 (rw) register accessor: Output Selection for CPU Interrupt 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel1`]
module"]
#[doc(alias = "CPUIRQSEL1")]
pub type Cpuirqsel1 = crate::Reg<cpuirqsel1::Cpuirqsel1Spec>;
#[doc = "Output Selection for CPU Interrupt 1"]
pub mod cpuirqsel1;
#[doc = "CPUIRQSEL2 (rw) register accessor: Output Selection for CPU Interrupt 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel2`]
module"]
#[doc(alias = "CPUIRQSEL2")]
pub type Cpuirqsel2 = crate::Reg<cpuirqsel2::Cpuirqsel2Spec>;
#[doc = "Output Selection for CPU Interrupt 2"]
pub mod cpuirqsel2;
#[doc = "CPUIRQSEL3 (rw) register accessor: Output Selection for CPU Interrupt 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel3`]
module"]
#[doc(alias = "CPUIRQSEL3")]
pub type Cpuirqsel3 = crate::Reg<cpuirqsel3::Cpuirqsel3Spec>;
#[doc = "Output Selection for CPU Interrupt 3"]
pub mod cpuirqsel3;
#[doc = "CPUIRQSEL4 (rw) register accessor: Output Selection for CPU Interrupt 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel4`]
module"]
#[doc(alias = "CPUIRQSEL4")]
pub type Cpuirqsel4 = crate::Reg<cpuirqsel4::Cpuirqsel4Spec>;
#[doc = "Output Selection for CPU Interrupt 4"]
pub mod cpuirqsel4;
#[doc = "CPUIRQSEL5 (rw) register accessor: Output Selection for CPU Interrupt 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel5`]
module"]
#[doc(alias = "CPUIRQSEL5")]
pub type Cpuirqsel5 = crate::Reg<cpuirqsel5::Cpuirqsel5Spec>;
#[doc = "Output Selection for CPU Interrupt 5"]
pub mod cpuirqsel5;
#[doc = "CPUIRQSEL6 (rw) register accessor: Output Selection for CPU Interrupt 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel6`]
module"]
#[doc(alias = "CPUIRQSEL6")]
pub type Cpuirqsel6 = crate::Reg<cpuirqsel6::Cpuirqsel6Spec>;
#[doc = "Output Selection for CPU Interrupt 6"]
pub mod cpuirqsel6;
#[doc = "CPUIRQSEL7 (rw) register accessor: Output Selection for CPU Interrupt 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel7`]
module"]
#[doc(alias = "CPUIRQSEL7")]
pub type Cpuirqsel7 = crate::Reg<cpuirqsel7::Cpuirqsel7Spec>;
#[doc = "Output Selection for CPU Interrupt 7"]
pub mod cpuirqsel7;
#[doc = "CPUIRQSEL8 (rw) register accessor: Output Selection for CPU Interrupt 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel8`]
module"]
#[doc(alias = "CPUIRQSEL8")]
pub type Cpuirqsel8 = crate::Reg<cpuirqsel8::Cpuirqsel8Spec>;
#[doc = "Output Selection for CPU Interrupt 8"]
pub mod cpuirqsel8;
#[doc = "CPUIRQSEL9 (rw) register accessor: Output Selection for CPU Interrupt 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel9`]
module"]
#[doc(alias = "CPUIRQSEL9")]
pub type Cpuirqsel9 = crate::Reg<cpuirqsel9::Cpuirqsel9Spec>;
#[doc = "Output Selection for CPU Interrupt 9"]
pub mod cpuirqsel9;
#[doc = "CPUIRQSEL10 (rw) register accessor: Output Selection for CPU Interrupt 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel10`]
module"]
#[doc(alias = "CPUIRQSEL10")]
pub type Cpuirqsel10 = crate::Reg<cpuirqsel10::Cpuirqsel10Spec>;
#[doc = "Output Selection for CPU Interrupt 10"]
pub mod cpuirqsel10;
#[doc = "CPUIRQSEL11 (rw) register accessor: Output Selection for CPU Interrupt 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel11`]
module"]
#[doc(alias = "CPUIRQSEL11")]
pub type Cpuirqsel11 = crate::Reg<cpuirqsel11::Cpuirqsel11Spec>;
#[doc = "Output Selection for CPU Interrupt 11"]
pub mod cpuirqsel11;
#[doc = "CPUIRQSEL12 (rw) register accessor: Output Selection for CPU Interrupt 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel12`]
module"]
#[doc(alias = "CPUIRQSEL12")]
pub type Cpuirqsel12 = crate::Reg<cpuirqsel12::Cpuirqsel12Spec>;
#[doc = "Output Selection for CPU Interrupt 12"]
pub mod cpuirqsel12;
#[doc = "CPUIRQSEL13 (rw) register accessor: Output Selection for CPU Interrupt 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel13`]
module"]
#[doc(alias = "CPUIRQSEL13")]
pub type Cpuirqsel13 = crate::Reg<cpuirqsel13::Cpuirqsel13Spec>;
#[doc = "Output Selection for CPU Interrupt 13"]
pub mod cpuirqsel13;
#[doc = "CPUIRQSEL14 (rw) register accessor: Output Selection for CPU Interrupt 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel14`]
module"]
#[doc(alias = "CPUIRQSEL14")]
pub type Cpuirqsel14 = crate::Reg<cpuirqsel14::Cpuirqsel14Spec>;
#[doc = "Output Selection for CPU Interrupt 14"]
pub mod cpuirqsel14;
#[doc = "CPUIRQSEL15 (rw) register accessor: Output Selection for CPU Interrupt 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel15`]
module"]
#[doc(alias = "CPUIRQSEL15")]
pub type Cpuirqsel15 = crate::Reg<cpuirqsel15::Cpuirqsel15Spec>;
#[doc = "Output Selection for CPU Interrupt 15"]
pub mod cpuirqsel15;
#[doc = "CPUIRQSEL16 (rw) register accessor: Output Selection for CPU Interrupt 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel16`]
module"]
#[doc(alias = "CPUIRQSEL16")]
pub type Cpuirqsel16 = crate::Reg<cpuirqsel16::Cpuirqsel16Spec>;
#[doc = "Output Selection for CPU Interrupt 16"]
pub mod cpuirqsel16;
#[doc = "CPUIRQSEL17 (rw) register accessor: Output Selection for CPU Interrupt 17\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel17`]
module"]
#[doc(alias = "CPUIRQSEL17")]
pub type Cpuirqsel17 = crate::Reg<cpuirqsel17::Cpuirqsel17Spec>;
#[doc = "Output Selection for CPU Interrupt 17"]
pub mod cpuirqsel17;
#[doc = "CPUIRQSEL18 (rw) register accessor: Output Selection for CPU Interrupt 18\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel18`]
module"]
#[doc(alias = "CPUIRQSEL18")]
pub type Cpuirqsel18 = crate::Reg<cpuirqsel18::Cpuirqsel18Spec>;
#[doc = "Output Selection for CPU Interrupt 18"]
pub mod cpuirqsel18;
#[doc = "CPUIRQSEL19 (rw) register accessor: Output Selection for CPU Interrupt 19\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel19`]
module"]
#[doc(alias = "CPUIRQSEL19")]
pub type Cpuirqsel19 = crate::Reg<cpuirqsel19::Cpuirqsel19Spec>;
#[doc = "Output Selection for CPU Interrupt 19"]
pub mod cpuirqsel19;
#[doc = "CPUIRQSEL20 (rw) register accessor: Output Selection for CPU Interrupt 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel20`]
module"]
#[doc(alias = "CPUIRQSEL20")]
pub type Cpuirqsel20 = crate::Reg<cpuirqsel20::Cpuirqsel20Spec>;
#[doc = "Output Selection for CPU Interrupt 20"]
pub mod cpuirqsel20;
#[doc = "CPUIRQSEL21 (rw) register accessor: Output Selection for CPU Interrupt 21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel21`]
module"]
#[doc(alias = "CPUIRQSEL21")]
pub type Cpuirqsel21 = crate::Reg<cpuirqsel21::Cpuirqsel21Spec>;
#[doc = "Output Selection for CPU Interrupt 21"]
pub mod cpuirqsel21;
#[doc = "CPUIRQSEL22 (rw) register accessor: Output Selection for CPU Interrupt 22\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel22`]
module"]
#[doc(alias = "CPUIRQSEL22")]
pub type Cpuirqsel22 = crate::Reg<cpuirqsel22::Cpuirqsel22Spec>;
#[doc = "Output Selection for CPU Interrupt 22"]
pub mod cpuirqsel22;
#[doc = "CPUIRQSEL23 (rw) register accessor: Output Selection for CPU Interrupt 23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel23::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel23::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel23`]
module"]
#[doc(alias = "CPUIRQSEL23")]
pub type Cpuirqsel23 = crate::Reg<cpuirqsel23::Cpuirqsel23Spec>;
#[doc = "Output Selection for CPU Interrupt 23"]
pub mod cpuirqsel23;
#[doc = "CPUIRQSEL24 (rw) register accessor: Output Selection for CPU Interrupt 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel24`]
module"]
#[doc(alias = "CPUIRQSEL24")]
pub type Cpuirqsel24 = crate::Reg<cpuirqsel24::Cpuirqsel24Spec>;
#[doc = "Output Selection for CPU Interrupt 24"]
pub mod cpuirqsel24;
#[doc = "CPUIRQSEL25 (rw) register accessor: Output Selection for CPU Interrupt 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel25`]
module"]
#[doc(alias = "CPUIRQSEL25")]
pub type Cpuirqsel25 = crate::Reg<cpuirqsel25::Cpuirqsel25Spec>;
#[doc = "Output Selection for CPU Interrupt 25"]
pub mod cpuirqsel25;
#[doc = "CPUIRQSEL26 (rw) register accessor: Output Selection for CPU Interrupt 26\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel26::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel26::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel26`]
module"]
#[doc(alias = "CPUIRQSEL26")]
pub type Cpuirqsel26 = crate::Reg<cpuirqsel26::Cpuirqsel26Spec>;
#[doc = "Output Selection for CPU Interrupt 26"]
pub mod cpuirqsel26;
#[doc = "CPUIRQSEL27 (rw) register accessor: Output Selection for CPU Interrupt 27\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel27::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel27::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel27`]
module"]
#[doc(alias = "CPUIRQSEL27")]
pub type Cpuirqsel27 = crate::Reg<cpuirqsel27::Cpuirqsel27Spec>;
#[doc = "Output Selection for CPU Interrupt 27"]
pub mod cpuirqsel27;
#[doc = "CPUIRQSEL28 (rw) register accessor: Output Selection for CPU Interrupt 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel28::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel28::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel28`]
module"]
#[doc(alias = "CPUIRQSEL28")]
pub type Cpuirqsel28 = crate::Reg<cpuirqsel28::Cpuirqsel28Spec>;
#[doc = "Output Selection for CPU Interrupt 28"]
pub mod cpuirqsel28;
#[doc = "CPUIRQSEL29 (rw) register accessor: Output Selection for CPU Interrupt 29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel29::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel29::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel29`]
module"]
#[doc(alias = "CPUIRQSEL29")]
pub type Cpuirqsel29 = crate::Reg<cpuirqsel29::Cpuirqsel29Spec>;
#[doc = "Output Selection for CPU Interrupt 29"]
pub mod cpuirqsel29;
#[doc = "CPUIRQSEL30 (rw) register accessor: Output Selection for CPU Interrupt 30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel30::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel30::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel30`]
module"]
#[doc(alias = "CPUIRQSEL30")]
pub type Cpuirqsel30 = crate::Reg<cpuirqsel30::Cpuirqsel30Spec>;
#[doc = "Output Selection for CPU Interrupt 30"]
pub mod cpuirqsel30;
#[doc = "CPUIRQSEL31 (rw) register accessor: Output Selection for CPU Interrupt 31\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel31::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel31::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel31`]
module"]
#[doc(alias = "CPUIRQSEL31")]
pub type Cpuirqsel31 = crate::Reg<cpuirqsel31::Cpuirqsel31Spec>;
#[doc = "Output Selection for CPU Interrupt 31"]
pub mod cpuirqsel31;
#[doc = "CPUIRQSEL32 (rw) register accessor: Output Selection for CPU Interrupt 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel32::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel32::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel32`]
module"]
#[doc(alias = "CPUIRQSEL32")]
pub type Cpuirqsel32 = crate::Reg<cpuirqsel32::Cpuirqsel32Spec>;
#[doc = "Output Selection for CPU Interrupt 32"]
pub mod cpuirqsel32;
#[doc = "CPUIRQSEL33 (rw) register accessor: Output Selection for CPU Interrupt 33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel33`]
module"]
#[doc(alias = "CPUIRQSEL33")]
pub type Cpuirqsel33 = crate::Reg<cpuirqsel33::Cpuirqsel33Spec>;
#[doc = "Output Selection for CPU Interrupt 33"]
pub mod cpuirqsel33;
#[doc = "CPUIRQSEL34 (rw) register accessor: Output Selection for CPU Interrupt 34\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel34`]
module"]
#[doc(alias = "CPUIRQSEL34")]
pub type Cpuirqsel34 = crate::Reg<cpuirqsel34::Cpuirqsel34Spec>;
#[doc = "Output Selection for CPU Interrupt 34"]
pub mod cpuirqsel34;
#[doc = "CPUIRQSEL35 (rw) register accessor: Output Selection for CPU Interrupt 35\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel35`]
module"]
#[doc(alias = "CPUIRQSEL35")]
pub type Cpuirqsel35 = crate::Reg<cpuirqsel35::Cpuirqsel35Spec>;
#[doc = "Output Selection for CPU Interrupt 35"]
pub mod cpuirqsel35;
#[doc = "CPUIRQSEL36 (rw) register accessor: Output Selection for CPU Interrupt 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel36`]
module"]
#[doc(alias = "CPUIRQSEL36")]
pub type Cpuirqsel36 = crate::Reg<cpuirqsel36::Cpuirqsel36Spec>;
#[doc = "Output Selection for CPU Interrupt 36"]
pub mod cpuirqsel36;
#[doc = "CPUIRQSEL37 (rw) register accessor: Output Selection for CPU Interrupt 37\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpuirqsel37::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpuirqsel37::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuirqsel37`]
module"]
#[doc(alias = "CPUIRQSEL37")]
pub type Cpuirqsel37 = crate::Reg<cpuirqsel37::Cpuirqsel37Spec>;
#[doc = "Output Selection for CPU Interrupt 37"]
pub mod cpuirqsel37;
#[doc = "RFCSEL0 (rw) register accessor: Output Selection for RFC Event 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel0`]
module"]
#[doc(alias = "RFCSEL0")]
pub type Rfcsel0 = crate::Reg<rfcsel0::Rfcsel0Spec>;
#[doc = "Output Selection for RFC Event 0"]
pub mod rfcsel0;
#[doc = "RFCSEL1 (rw) register accessor: Output Selection for RFC Event 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel1`]
module"]
#[doc(alias = "RFCSEL1")]
pub type Rfcsel1 = crate::Reg<rfcsel1::Rfcsel1Spec>;
#[doc = "Output Selection for RFC Event 1"]
pub mod rfcsel1;
#[doc = "RFCSEL2 (rw) register accessor: Output Selection for RFC Event 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel2`]
module"]
#[doc(alias = "RFCSEL2")]
pub type Rfcsel2 = crate::Reg<rfcsel2::Rfcsel2Spec>;
#[doc = "Output Selection for RFC Event 2"]
pub mod rfcsel2;
#[doc = "RFCSEL3 (rw) register accessor: Output Selection for RFC Event 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel3`]
module"]
#[doc(alias = "RFCSEL3")]
pub type Rfcsel3 = crate::Reg<rfcsel3::Rfcsel3Spec>;
#[doc = "Output Selection for RFC Event 3"]
pub mod rfcsel3;
#[doc = "RFCSEL4 (rw) register accessor: Output Selection for RFC Event 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel4`]
module"]
#[doc(alias = "RFCSEL4")]
pub type Rfcsel4 = crate::Reg<rfcsel4::Rfcsel4Spec>;
#[doc = "Output Selection for RFC Event 4"]
pub mod rfcsel4;
#[doc = "RFCSEL5 (rw) register accessor: Output Selection for RFC Event 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel5`]
module"]
#[doc(alias = "RFCSEL5")]
pub type Rfcsel5 = crate::Reg<rfcsel5::Rfcsel5Spec>;
#[doc = "Output Selection for RFC Event 5"]
pub mod rfcsel5;
#[doc = "RFCSEL6 (rw) register accessor: Output Selection for RFC Event 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel6`]
module"]
#[doc(alias = "RFCSEL6")]
pub type Rfcsel6 = crate::Reg<rfcsel6::Rfcsel6Spec>;
#[doc = "Output Selection for RFC Event 6"]
pub mod rfcsel6;
#[doc = "RFCSEL7 (rw) register accessor: Output Selection for RFC Event 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel7`]
module"]
#[doc(alias = "RFCSEL7")]
pub type Rfcsel7 = crate::Reg<rfcsel7::Rfcsel7Spec>;
#[doc = "Output Selection for RFC Event 7"]
pub mod rfcsel7;
#[doc = "RFCSEL8 (rw) register accessor: Output Selection for RFC Event 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel8`]
module"]
#[doc(alias = "RFCSEL8")]
pub type Rfcsel8 = crate::Reg<rfcsel8::Rfcsel8Spec>;
#[doc = "Output Selection for RFC Event 8"]
pub mod rfcsel8;
#[doc = "RFCSEL9 (rw) register accessor: Output Selection for RFC Event 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfcsel9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfcsel9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfcsel9`]
module"]
#[doc(alias = "RFCSEL9")]
pub type Rfcsel9 = crate::Reg<rfcsel9::Rfcsel9Spec>;
#[doc = "Output Selection for RFC Event 9"]
pub mod rfcsel9;
#[doc = "GPT0ACAPTSEL (rw) register accessor: Output Selection for GPT0 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt0acaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt0acaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt0acaptsel`]
module"]
#[doc(alias = "GPT0ACAPTSEL")]
pub type Gpt0acaptsel = crate::Reg<gpt0acaptsel::Gpt0acaptselSpec>;
#[doc = "Output Selection for GPT0 0"]
pub mod gpt0acaptsel;
#[doc = "GPT0BCAPTSEL (rw) register accessor: Output Selection for GPT0 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt0bcaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt0bcaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt0bcaptsel`]
module"]
#[doc(alias = "GPT0BCAPTSEL")]
pub type Gpt0bcaptsel = crate::Reg<gpt0bcaptsel::Gpt0bcaptselSpec>;
#[doc = "Output Selection for GPT0 1"]
pub mod gpt0bcaptsel;
#[doc = "GPT1ACAPTSEL (rw) register accessor: Output Selection for GPT1 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt1acaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt1acaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt1acaptsel`]
module"]
#[doc(alias = "GPT1ACAPTSEL")]
pub type Gpt1acaptsel = crate::Reg<gpt1acaptsel::Gpt1acaptselSpec>;
#[doc = "Output Selection for GPT1 0"]
pub mod gpt1acaptsel;
#[doc = "GPT1BCAPTSEL (rw) register accessor: Output Selection for GPT1 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt1bcaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt1bcaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt1bcaptsel`]
module"]
#[doc(alias = "GPT1BCAPTSEL")]
pub type Gpt1bcaptsel = crate::Reg<gpt1bcaptsel::Gpt1bcaptselSpec>;
#[doc = "Output Selection for GPT1 1"]
pub mod gpt1bcaptsel;
#[doc = "GPT2ACAPTSEL (rw) register accessor: Output Selection for GPT2 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt2acaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt2acaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt2acaptsel`]
module"]
#[doc(alias = "GPT2ACAPTSEL")]
pub type Gpt2acaptsel = crate::Reg<gpt2acaptsel::Gpt2acaptselSpec>;
#[doc = "Output Selection for GPT2 0"]
pub mod gpt2acaptsel;
#[doc = "GPT2BCAPTSEL (rw) register accessor: Output Selection for GPT2 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt2bcaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt2bcaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt2bcaptsel`]
module"]
#[doc(alias = "GPT2BCAPTSEL")]
pub type Gpt2bcaptsel = crate::Reg<gpt2bcaptsel::Gpt2bcaptselSpec>;
#[doc = "Output Selection for GPT2 1"]
pub mod gpt2bcaptsel;
#[doc = "UDMACH0SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach0ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach0ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach0ssel`]
module"]
#[doc(alias = "UDMACH0SSEL")]
pub type Udmach0ssel = crate::Reg<udmach0ssel::Udmach0sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach0ssel;
#[doc = "UDMACH0BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach0bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach0bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach0bsel`]
module"]
#[doc(alias = "UDMACH0BSEL")]
pub type Udmach0bsel = crate::Reg<udmach0bsel::Udmach0bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach0bsel;
#[doc = "UDMACH1SSEL (rw) register accessor: Output Selection for DMA Channel 1 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach1ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach1ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach1ssel`]
module"]
#[doc(alias = "UDMACH1SSEL")]
pub type Udmach1ssel = crate::Reg<udmach1ssel::Udmach1sselSpec>;
#[doc = "Output Selection for DMA Channel 1 SREQ"]
pub mod udmach1ssel;
#[doc = "UDMACH1BSEL (rw) register accessor: Output Selection for DMA Channel 1 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach1bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach1bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach1bsel`]
module"]
#[doc(alias = "UDMACH1BSEL")]
pub type Udmach1bsel = crate::Reg<udmach1bsel::Udmach1bselSpec>;
#[doc = "Output Selection for DMA Channel 1 REQ"]
pub mod udmach1bsel;
#[doc = "UDMACH2SSEL (rw) register accessor: Output Selection for DMA Channel 2 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach2ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach2ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach2ssel`]
module"]
#[doc(alias = "UDMACH2SSEL")]
pub type Udmach2ssel = crate::Reg<udmach2ssel::Udmach2sselSpec>;
#[doc = "Output Selection for DMA Channel 2 SREQ"]
pub mod udmach2ssel;
#[doc = "UDMACH2BSEL (rw) register accessor: Output Selection for DMA Channel 2 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach2bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach2bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach2bsel`]
module"]
#[doc(alias = "UDMACH2BSEL")]
pub type Udmach2bsel = crate::Reg<udmach2bsel::Udmach2bselSpec>;
#[doc = "Output Selection for DMA Channel 2 REQ"]
pub mod udmach2bsel;
#[doc = "UDMACH3SSEL (rw) register accessor: Output Selection for DMA Channel 3 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach3ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach3ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach3ssel`]
module"]
#[doc(alias = "UDMACH3SSEL")]
pub type Udmach3ssel = crate::Reg<udmach3ssel::Udmach3sselSpec>;
#[doc = "Output Selection for DMA Channel 3 SREQ"]
pub mod udmach3ssel;
#[doc = "UDMACH3BSEL (rw) register accessor: Output Selection for DMA Channel 3 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach3bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach3bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach3bsel`]
module"]
#[doc(alias = "UDMACH3BSEL")]
pub type Udmach3bsel = crate::Reg<udmach3bsel::Udmach3bselSpec>;
#[doc = "Output Selection for DMA Channel 3 REQ"]
pub mod udmach3bsel;
#[doc = "UDMACH4SSEL (rw) register accessor: Output Selection for DMA Channel 4 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach4ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach4ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach4ssel`]
module"]
#[doc(alias = "UDMACH4SSEL")]
pub type Udmach4ssel = crate::Reg<udmach4ssel::Udmach4sselSpec>;
#[doc = "Output Selection for DMA Channel 4 SREQ"]
pub mod udmach4ssel;
#[doc = "UDMACH4BSEL (rw) register accessor: Output Selection for DMA Channel 4 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach4bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach4bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach4bsel`]
module"]
#[doc(alias = "UDMACH4BSEL")]
pub type Udmach4bsel = crate::Reg<udmach4bsel::Udmach4bselSpec>;
#[doc = "Output Selection for DMA Channel 4 REQ"]
pub mod udmach4bsel;
#[doc = "UDMACH5SSEL (rw) register accessor: Output Selection for DMA Channel 5 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach5ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach5ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach5ssel`]
module"]
#[doc(alias = "UDMACH5SSEL")]
pub type Udmach5ssel = crate::Reg<udmach5ssel::Udmach5sselSpec>;
#[doc = "Output Selection for DMA Channel 5 SREQ"]
pub mod udmach5ssel;
#[doc = "UDMACH5BSEL (rw) register accessor: Output Selection for DMA Channel 5 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach5bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach5bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach5bsel`]
module"]
#[doc(alias = "UDMACH5BSEL")]
pub type Udmach5bsel = crate::Reg<udmach5bsel::Udmach5bselSpec>;
#[doc = "Output Selection for DMA Channel 5 REQ"]
pub mod udmach5bsel;
#[doc = "UDMACH6SSEL (rw) register accessor: Output Selection for DMA Channel 6 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach6ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach6ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach6ssel`]
module"]
#[doc(alias = "UDMACH6SSEL")]
pub type Udmach6ssel = crate::Reg<udmach6ssel::Udmach6sselSpec>;
#[doc = "Output Selection for DMA Channel 6 SREQ"]
pub mod udmach6ssel;
#[doc = "UDMACH6BSEL (rw) register accessor: Output Selection for DMA Channel 6 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach6bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach6bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach6bsel`]
module"]
#[doc(alias = "UDMACH6BSEL")]
pub type Udmach6bsel = crate::Reg<udmach6bsel::Udmach6bselSpec>;
#[doc = "Output Selection for DMA Channel 6 REQ"]
pub mod udmach6bsel;
#[doc = "UDMACH7SSEL (rw) register accessor: Output Selection for DMA Channel 7 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach7ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach7ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach7ssel`]
module"]
#[doc(alias = "UDMACH7SSEL")]
pub type Udmach7ssel = crate::Reg<udmach7ssel::Udmach7sselSpec>;
#[doc = "Output Selection for DMA Channel 7 SREQ"]
pub mod udmach7ssel;
#[doc = "UDMACH7BSEL (rw) register accessor: Output Selection for DMA Channel 7 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach7bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach7bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach7bsel`]
module"]
#[doc(alias = "UDMACH7BSEL")]
pub type Udmach7bsel = crate::Reg<udmach7bsel::Udmach7bselSpec>;
#[doc = "Output Selection for DMA Channel 7 REQ"]
pub mod udmach7bsel;
#[doc = "UDMACH8SSEL (rw) register accessor: Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach8ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach8ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach8ssel`]
module"]
#[doc(alias = "UDMACH8SSEL")]
pub type Udmach8ssel = crate::Reg<udmach8ssel::Udmach8sselSpec>;
#[doc = "Output Selection for DMA Channel 8 SREQ Single request is ignored for this channel"]
pub mod udmach8ssel;
#[doc = "UDMACH8BSEL (rw) register accessor: Output Selection for DMA Channel 8 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach8bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach8bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach8bsel`]
module"]
#[doc(alias = "UDMACH8BSEL")]
pub type Udmach8bsel = crate::Reg<udmach8bsel::Udmach8bselSpec>;
#[doc = "Output Selection for DMA Channel 8 REQ"]
pub mod udmach8bsel;
#[doc = "UDMACH9SSEL (rw) register accessor: Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach9ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach9ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach9ssel`]
module"]
#[doc(alias = "UDMACH9SSEL")]
pub type Udmach9ssel = crate::Reg<udmach9ssel::Udmach9sselSpec>;
#[doc = "Output Selection for DMA Channel 9 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9ssel;
#[doc = "UDMACH9BSEL (rw) register accessor: Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach9bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach9bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach9bsel`]
module"]
#[doc(alias = "UDMACH9BSEL")]
pub type Udmach9bsel = crate::Reg<udmach9bsel::Udmach9bselSpec>;
#[doc = "Output Selection for DMA Channel 9 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMAARIS"]
pub mod udmach9bsel;
#[doc = "UDMACH10SSEL (rw) register accessor: Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach10ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach10ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach10ssel`]
module"]
#[doc(alias = "UDMACH10SSEL")]
pub type Udmach10ssel = crate::Reg<udmach10ssel::Udmach10sselSpec>;
#[doc = "Output Selection for DMA Channel 10 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10ssel;
#[doc = "UDMACH10BSEL (rw) register accessor: Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach10bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach10bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach10bsel`]
module"]
#[doc(alias = "UDMACH10BSEL")]
pub type Udmach10bsel = crate::Reg<udmach10bsel::Udmach10bselSpec>;
#[doc = "Output Selection for DMA Channel 10 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT0 as GPT0:RIS.DMABRIS"]
pub mod udmach10bsel;
#[doc = "UDMACH11SSEL (rw) register accessor: Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach11ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach11ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach11ssel`]
module"]
#[doc(alias = "UDMACH11SSEL")]
pub type Udmach11ssel = crate::Reg<udmach11ssel::Udmach11sselSpec>;
#[doc = "Output Selection for DMA Channel 11 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11ssel;
#[doc = "UDMACH11BSEL (rw) register accessor: Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach11bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach11bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach11bsel`]
module"]
#[doc(alias = "UDMACH11BSEL")]
pub type Udmach11bsel = crate::Reg<udmach11bsel::Udmach11bselSpec>;
#[doc = "Output Selection for DMA Channel 11 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMAARIS"]
pub mod udmach11bsel;
#[doc = "UDMACH12SSEL (rw) register accessor: Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach12ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach12ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach12ssel`]
module"]
#[doc(alias = "UDMACH12SSEL")]
pub type Udmach12ssel = crate::Reg<udmach12ssel::Udmach12sselSpec>;
#[doc = "Output Selection for DMA Channel 12 SREQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12ssel;
#[doc = "UDMACH12BSEL (rw) register accessor: Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach12bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach12bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach12bsel`]
module"]
#[doc(alias = "UDMACH12BSEL")]
pub type Udmach12bsel = crate::Reg<udmach12bsel::Udmach12bselSpec>;
#[doc = "Output Selection for DMA Channel 12 REQ DMA_DONE for the corresponding DMA channel is available as interrupt on GPT1 as GPT1:RIS.DMABRIS"]
pub mod udmach12bsel;
#[doc = "UDMACH13SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach13ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach13ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach13ssel`]
module"]
#[doc(alias = "UDMACH13SSEL")]
pub type Udmach13ssel = crate::Reg<udmach13ssel::Udmach13sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach13ssel;
#[doc = "UDMACH13BSEL (rw) register accessor: Output Selection for DMA Channel 13 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach13bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach13bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach13bsel`]
module"]
#[doc(alias = "UDMACH13BSEL")]
pub type Udmach13bsel = crate::Reg<udmach13bsel::Udmach13bselSpec>;
#[doc = "Output Selection for DMA Channel 13 REQ"]
pub mod udmach13bsel;
#[doc = "UDMACH14SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach14ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach14ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach14ssel`]
module"]
#[doc(alias = "UDMACH14SSEL")]
pub type Udmach14ssel = crate::Reg<udmach14ssel::Udmach14sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach14ssel;
#[doc = "UDMACH14BSEL (rw) register accessor: Output Selection for DMA Channel 14 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach14bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach14bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach14bsel`]
module"]
#[doc(alias = "UDMACH14BSEL")]
pub type Udmach14bsel = crate::Reg<udmach14bsel::Udmach14bselSpec>;
#[doc = "Output Selection for DMA Channel 14 REQ"]
pub mod udmach14bsel;
#[doc = "UDMACH15SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach15ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach15ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach15ssel`]
module"]
#[doc(alias = "UDMACH15SSEL")]
pub type Udmach15ssel = crate::Reg<udmach15ssel::Udmach15sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach15ssel;
#[doc = "UDMACH15BSEL (rw) register accessor: Output Selection for DMA Channel 15 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach15bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach15bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach15bsel`]
module"]
#[doc(alias = "UDMACH15BSEL")]
pub type Udmach15bsel = crate::Reg<udmach15bsel::Udmach15bselSpec>;
#[doc = "Output Selection for DMA Channel 15 REQ"]
pub mod udmach15bsel;
#[doc = "UDMACH16SSEL (rw) register accessor: Output Selection for DMA Channel 16 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach16ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach16ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach16ssel`]
module"]
#[doc(alias = "UDMACH16SSEL")]
pub type Udmach16ssel = crate::Reg<udmach16ssel::Udmach16sselSpec>;
#[doc = "Output Selection for DMA Channel 16 SREQ"]
pub mod udmach16ssel;
#[doc = "UDMACH16BSEL (rw) register accessor: Output Selection for DMA Channel 16 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach16bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach16bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach16bsel`]
module"]
#[doc(alias = "UDMACH16BSEL")]
pub type Udmach16bsel = crate::Reg<udmach16bsel::Udmach16bselSpec>;
#[doc = "Output Selection for DMA Channel 16 REQ"]
pub mod udmach16bsel;
#[doc = "UDMACH17SSEL (rw) register accessor: Output Selection for DMA Channel 17 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach17ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach17ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach17ssel`]
module"]
#[doc(alias = "UDMACH17SSEL")]
pub type Udmach17ssel = crate::Reg<udmach17ssel::Udmach17sselSpec>;
#[doc = "Output Selection for DMA Channel 17 SREQ"]
pub mod udmach17ssel;
#[doc = "UDMACH17BSEL (rw) register accessor: Output Selection for DMA Channel 17 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach17bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach17bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach17bsel`]
module"]
#[doc(alias = "UDMACH17BSEL")]
pub type Udmach17bsel = crate::Reg<udmach17bsel::Udmach17bselSpec>;
#[doc = "Output Selection for DMA Channel 17 REQ"]
pub mod udmach17bsel;
#[doc = "UDMACH18SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach18ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach18ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach18ssel`]
module"]
#[doc(alias = "UDMACH18SSEL")]
pub type Udmach18ssel = crate::Reg<udmach18ssel::Udmach18sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach18ssel;
#[doc = "UDMACH18BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach18bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach18bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach18bsel`]
module"]
#[doc(alias = "UDMACH18BSEL")]
pub type Udmach18bsel = crate::Reg<udmach18bsel::Udmach18bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach18bsel;
#[doc = "UDMACH19SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach19ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach19ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach19ssel`]
module"]
#[doc(alias = "UDMACH19SSEL")]
pub type Udmach19ssel = crate::Reg<udmach19ssel::Udmach19sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach19ssel;
#[doc = "UDMACH19BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach19bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach19bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach19bsel`]
module"]
#[doc(alias = "UDMACH19BSEL")]
pub type Udmach19bsel = crate::Reg<udmach19bsel::Udmach19bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach19bsel;
#[doc = "UDMACH20SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach20ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach20ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach20ssel`]
module"]
#[doc(alias = "UDMACH20SSEL")]
pub type Udmach20ssel = crate::Reg<udmach20ssel::Udmach20sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach20ssel;
#[doc = "UDMACH20BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach20bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach20bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach20bsel`]
module"]
#[doc(alias = "UDMACH20BSEL")]
pub type Udmach20bsel = crate::Reg<udmach20bsel::Udmach20bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach20bsel;
#[doc = "UDMACH21SSEL (rw) register accessor: Output Selection for DMA Channel 21 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach21ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach21ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach21ssel`]
module"]
#[doc(alias = "UDMACH21SSEL")]
pub type Udmach21ssel = crate::Reg<udmach21ssel::Udmach21sselSpec>;
#[doc = "Output Selection for DMA Channel 21 SREQ"]
pub mod udmach21ssel;
#[doc = "UDMACH21BSEL (rw) register accessor: Output Selection for DMA Channel 21 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach21bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach21bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach21bsel`]
module"]
#[doc(alias = "UDMACH21BSEL")]
pub type Udmach21bsel = crate::Reg<udmach21bsel::Udmach21bselSpec>;
#[doc = "Output Selection for DMA Channel 21 REQ"]
pub mod udmach21bsel;
#[doc = "UDMACH22SSEL (rw) register accessor: Output Selection for DMA Channel 22 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach22ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach22ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach22ssel`]
module"]
#[doc(alias = "UDMACH22SSEL")]
pub type Udmach22ssel = crate::Reg<udmach22ssel::Udmach22sselSpec>;
#[doc = "Output Selection for DMA Channel 22 SREQ"]
pub mod udmach22ssel;
#[doc = "UDMACH22BSEL (rw) register accessor: Output Selection for DMA Channel 22 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach22bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach22bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach22bsel`]
module"]
#[doc(alias = "UDMACH22BSEL")]
pub type Udmach22bsel = crate::Reg<udmach22bsel::Udmach22bselSpec>;
#[doc = "Output Selection for DMA Channel 22 REQ"]
pub mod udmach22bsel;
#[doc = "UDMACH23SSEL (rw) register accessor: Output Selection for DMA Channel 23 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach23ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach23ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach23ssel`]
module"]
#[doc(alias = "UDMACH23SSEL")]
pub type Udmach23ssel = crate::Reg<udmach23ssel::Udmach23sselSpec>;
#[doc = "Output Selection for DMA Channel 23 SREQ"]
pub mod udmach23ssel;
#[doc = "UDMACH23BSEL (rw) register accessor: Output Selection for DMA Channel 23 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach23bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach23bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach23bsel`]
module"]
#[doc(alias = "UDMACH23BSEL")]
pub type Udmach23bsel = crate::Reg<udmach23bsel::Udmach23bselSpec>;
#[doc = "Output Selection for DMA Channel 23 REQ"]
pub mod udmach23bsel;
#[doc = "UDMACH24SSEL (rw) register accessor: Output Selection for DMA Channel 24 SREQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach24ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach24ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach24ssel`]
module"]
#[doc(alias = "UDMACH24SSEL")]
pub type Udmach24ssel = crate::Reg<udmach24ssel::Udmach24sselSpec>;
#[doc = "Output Selection for DMA Channel 24 SREQ"]
pub mod udmach24ssel;
#[doc = "UDMACH24BSEL (rw) register accessor: Output Selection for DMA Channel 24 REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach24bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach24bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach24bsel`]
module"]
#[doc(alias = "UDMACH24BSEL")]
pub type Udmach24bsel = crate::Reg<udmach24bsel::Udmach24bselSpec>;
#[doc = "Output Selection for DMA Channel 24 REQ"]
pub mod udmach24bsel;
#[doc = "UDMACH25SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach25ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach25ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach25ssel`]
module"]
#[doc(alias = "UDMACH25SSEL")]
pub type Udmach25ssel = crate::Reg<udmach25ssel::Udmach25sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach25ssel;
#[doc = "UDMACH25BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach25bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach25bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach25bsel`]
module"]
#[doc(alias = "UDMACH25BSEL")]
pub type Udmach25bsel = crate::Reg<udmach25bsel::Udmach25bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach25bsel;
#[doc = "UDMACH26SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach26ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach26ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach26ssel`]
module"]
#[doc(alias = "UDMACH26SSEL")]
pub type Udmach26ssel = crate::Reg<udmach26ssel::Udmach26sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach26ssel;
#[doc = "UDMACH26BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach26bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach26bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach26bsel`]
module"]
#[doc(alias = "UDMACH26BSEL")]
pub type Udmach26bsel = crate::Reg<udmach26bsel::Udmach26bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach26bsel;
#[doc = "UDMACH27SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach27ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach27ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach27ssel`]
module"]
#[doc(alias = "UDMACH27SSEL")]
pub type Udmach27ssel = crate::Reg<udmach27ssel::Udmach27sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach27ssel;
#[doc = "UDMACH27BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach27bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach27bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach27bsel`]
module"]
#[doc(alias = "UDMACH27BSEL")]
pub type Udmach27bsel = crate::Reg<udmach27bsel::Udmach27bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach27bsel;
#[doc = "UDMACH28SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach28ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach28ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach28ssel`]
module"]
#[doc(alias = "UDMACH28SSEL")]
pub type Udmach28ssel = crate::Reg<udmach28ssel::Udmach28sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach28ssel;
#[doc = "UDMACH28BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach28bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach28bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach28bsel`]
module"]
#[doc(alias = "UDMACH28BSEL")]
pub type Udmach28bsel = crate::Reg<udmach28bsel::Udmach28bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach28bsel;
#[doc = "UDMACH29SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach29ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach29ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach29ssel`]
module"]
#[doc(alias = "UDMACH29SSEL")]
pub type Udmach29ssel = crate::Reg<udmach29ssel::Udmach29sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach29ssel;
#[doc = "UDMACH29BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach29bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach29bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach29bsel`]
module"]
#[doc(alias = "UDMACH29BSEL")]
pub type Udmach29bsel = crate::Reg<udmach29bsel::Udmach29bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach29bsel;
#[doc = "UDMACH30SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach30ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach30ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach30ssel`]
module"]
#[doc(alias = "UDMACH30SSEL")]
pub type Udmach30ssel = crate::Reg<udmach30ssel::Udmach30sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach30ssel;
#[doc = "UDMACH30BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach30bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach30bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach30bsel`]
module"]
#[doc(alias = "UDMACH30BSEL")]
pub type Udmach30bsel = crate::Reg<udmach30bsel::Udmach30bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach30bsel;
#[doc = "UDMACH31SSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach31ssel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach31ssel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach31ssel`]
module"]
#[doc(alias = "UDMACH31SSEL")]
pub type Udmach31ssel = crate::Reg<udmach31ssel::Udmach31sselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach31ssel;
#[doc = "UDMACH31BSEL (rw) register accessor: Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`udmach31bsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`udmach31bsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@udmach31bsel`]
module"]
#[doc(alias = "UDMACH31BSEL")]
pub type Udmach31bsel = crate::Reg<udmach31bsel::Udmach31bselSpec>;
#[doc = "Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub mod udmach31bsel;
#[doc = "GPT3ACAPTSEL (rw) register accessor: Output Selection for GPT3 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt3acaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt3acaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt3acaptsel`]
module"]
#[doc(alias = "GPT3ACAPTSEL")]
pub type Gpt3acaptsel = crate::Reg<gpt3acaptsel::Gpt3acaptselSpec>;
#[doc = "Output Selection for GPT3 0"]
pub mod gpt3acaptsel;
#[doc = "GPT3BCAPTSEL (rw) register accessor: Output Selection for GPT3 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpt3bcaptsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpt3bcaptsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpt3bcaptsel`]
module"]
#[doc(alias = "GPT3BCAPTSEL")]
pub type Gpt3bcaptsel = crate::Reg<gpt3bcaptsel::Gpt3bcaptselSpec>;
#[doc = "Output Selection for GPT3 1"]
pub mod gpt3bcaptsel;
#[doc = "AUXSEL0 (rw) register accessor: Output Selection for AUX Subscriber 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`auxsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`auxsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@auxsel0`]
module"]
#[doc(alias = "AUXSEL0")]
pub type Auxsel0 = crate::Reg<auxsel0::Auxsel0Spec>;
#[doc = "Output Selection for AUX Subscriber 0"]
pub mod auxsel0;
#[doc = "CM3NMISEL0 (rw) register accessor: Output Selection for NMI Subscriber 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cm3nmisel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cm3nmisel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cm3nmisel0`]
module"]
#[doc(alias = "CM3NMISEL0")]
pub type Cm3nmisel0 = crate::Reg<cm3nmisel0::Cm3nmisel0Spec>;
#[doc = "Output Selection for NMI Subscriber 0"]
pub mod cm3nmisel0;
#[doc = "I2SSTMPSEL0 (rw) register accessor: Output Selection for I2S Subscriber 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2sstmpsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2sstmpsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sstmpsel0`]
module"]
#[doc(alias = "I2SSTMPSEL0")]
pub type I2sstmpsel0 = crate::Reg<i2sstmpsel0::I2sstmpsel0Spec>;
#[doc = "Output Selection for I2S Subscriber 0"]
pub mod i2sstmpsel0;
#[doc = "FRZSEL0 (rw) register accessor: Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frzsel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frzsel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frzsel0`]
module"]
#[doc(alias = "FRZSEL0")]
pub type Frzsel0 = crate::Reg<frzsel0::Frzsel0Spec>;
#[doc = "Output Selection for FRZ Subscriber The halted debug signal is passed to peripherals such as the General Purpose Timer, Sensor Controller with Digital and Analog Peripherals (AUX), Radio, and RTC. When the system CPU halts, the connected peripherals that have freeze enabled also halt. The programmable output can be set to static values of 0 or 1, and can also be set to pass the halted signal."]
pub mod frzsel0;
#[doc = "SWEV (rw) register accessor: Set or Clear Software Events\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swev::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swev::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swev`]
module"]
#[doc(alias = "SWEV")]
pub type Swev = crate::Reg<swev::SwevSpec>;
#[doc = "Set or Clear Software Events"]
pub mod swev;
