#[doc = "Register `LCRH` reader"]
pub type R = crate::R<LcrhSpec>;
#[doc = "Register `LCRH` writer"]
pub type W = crate::W<LcrhSpec>;
#[doc = "Field `BRK` reader - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BrkR = crate::BitReader;
#[doc = "Field `BRK` writer - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BrkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pen {
    #[doc = "1: Parity checking and generation is enabled."]
    En = 1,
    #[doc = "0: Parity is disabled and no parity bit is added to the data frame"]
    Dis = 0,
}
impl From<Pen> for bool {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEN` reader - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
pub type PenR = crate::BitReader<Pen>;
impl PenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pen {
        match self.bits {
            true => Pen::En,
            false => Pen::Dis,
        }
    }
    #[doc = "Parity checking and generation is enabled."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Pen::En
    }
    #[doc = "Parity is disabled and no parity bit is added to the data frame"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Pen::Dis
    }
}
#[doc = "Field `PEN` writer - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
pub type PenW<'a, REG> = crate::BitWriter<'a, REG, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Parity checking and generation is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::En)
    }
    #[doc = "Parity is disabled and no parity bit is added to the data frame"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::Dis)
    }
}
#[doc = "2:2\\]
UART Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eps {
    #[doc = "1: Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    Even = 1,
    #[doc = "0: Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    Odd = 0,
}
impl From<Eps> for bool {
    #[inline(always)]
    fn from(variant: Eps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPS` reader - 2:2\\]
UART Even Parity Select"]
pub type EpsR = crate::BitReader<Eps>;
impl EpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eps {
        match self.bits {
            true => Eps::Even,
            false => Eps::Odd,
        }
    }
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Eps::Even
    }
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Eps::Odd
    }
}
#[doc = "Field `EPS` writer - 2:2\\]
UART Even Parity Select"]
pub type EpsW<'a, REG> = crate::BitWriter<'a, REG, Eps>;
impl<'a, REG> EpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Even)
    }
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Eps::Odd)
    }
}
#[doc = "Field `STP2` reader - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type Stp2R = crate::BitReader;
#[doc = "Field `STP2` writer - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type Stp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "4:4\\]
UART Enable FIFOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fen {
    #[doc = "1: Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    En = 1,
    #[doc = "0: FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    Dis = 0,
}
impl From<Fen> for bool {
    #[inline(always)]
    fn from(variant: Fen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEN` reader - 4:4\\]
UART Enable FIFOs"]
pub type FenR = crate::BitReader<Fen>;
impl FenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fen {
        match self.bits {
            true => Fen::En,
            false => Fen::Dis,
        }
    }
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Fen::En
    }
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Fen::Dis
    }
}
#[doc = "Field `FEN` writer - 4:4\\]
UART Enable FIFOs"]
pub type FenW<'a, REG> = crate::BitWriter<'a, REG, Fen>;
impl<'a, REG> FenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Fen::En)
    }
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Fen::Dis)
    }
}
#[doc = "6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wlen {
    #[doc = "3: Word Length 8 bits"]
    Bitl8 = 3,
    #[doc = "2: Word Length 7 bits"]
    Bitl7 = 2,
    #[doc = "1: Word Length 6 bits"]
    Bitl6 = 1,
    #[doc = "0: Word Length 5 bits"]
    Bitl5 = 0,
}
impl From<Wlen> for u8 {
    #[inline(always)]
    fn from(variant: Wlen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wlen {
    type Ux = u8;
}
impl crate::IsEnum for Wlen {}
#[doc = "Field `WLEN` reader - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
pub type WlenR = crate::FieldReader<Wlen>;
impl WlenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wlen {
        match self.bits {
            3 => Wlen::Bitl8,
            2 => Wlen::Bitl7,
            1 => Wlen::Bitl6,
            0 => Wlen::Bitl5,
            _ => unreachable!(),
        }
    }
    #[doc = "Word Length 8 bits"]
    #[inline(always)]
    pub fn is_bitl8(&self) -> bool {
        *self == Wlen::Bitl8
    }
    #[doc = "Word Length 7 bits"]
    #[inline(always)]
    pub fn is_bitl7(&self) -> bool {
        *self == Wlen::Bitl7
    }
    #[doc = "Word Length 6 bits"]
    #[inline(always)]
    pub fn is_bitl6(&self) -> bool {
        *self == Wlen::Bitl6
    }
    #[doc = "Word Length 5 bits"]
    #[inline(always)]
    pub fn is_bitl5(&self) -> bool {
        *self == Wlen::Bitl5
    }
}
#[doc = "Field `WLEN` writer - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
pub type WlenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wlen, crate::Safe>;
impl<'a, REG> WlenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Word Length 8 bits"]
    #[inline(always)]
    pub fn bitl8(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Bitl8)
    }
    #[doc = "Word Length 7 bits"]
    #[inline(always)]
    pub fn bitl7(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Bitl7)
    }
    #[doc = "Word Length 6 bits"]
    #[inline(always)]
    pub fn bitl6(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Bitl6)
    }
    #[doc = "Word Length 5 bits"]
    #[inline(always)]
    pub fn bitl5(self) -> &'a mut crate::W<REG> {
        self.variant(Wlen::Bitl5)
    }
}
#[doc = "Field `SPS` reader - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
pub type SpsR = crate::BitReader;
#[doc = "Field `SPS` writer - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
pub type SpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved8R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&self) -> BrkR {
        BrkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
UART Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EpsR {
        EpsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&self) -> Stp2R {
        Stp2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
UART Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FenR {
        FenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline(always)]
    pub fn wlen(&self) -> WlenR {
        WlenR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&self) -> SpsR {
        SpsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BrkW<LcrhSpec> {
        BrkW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<LcrhSpec> {
        PenW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
UART Even Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EpsW<LcrhSpec> {
        EpsW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    #[must_use]
    pub fn stp2(&mut self) -> Stp2W<LcrhSpec> {
        Stp2W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
UART Enable FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FenW<LcrhSpec> {
        FenW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WlenW<LcrhSpec> {
        WlenW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SpsW<LcrhSpec> {
        SpsW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reads to this field return zero, writes to this field are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<LcrhSpec> {
        Reserved8W::new(self, 8)
    }
}
#[doc = "Line Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrhSpec;
impl crate::RegisterSpec for LcrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcrh::R`](R) reader structure"]
impl crate::Readable for LcrhSpec {}
#[doc = "`write(|w| ..)` method takes [`lcrh::W`](W) writer structure"]
impl crate::Writable for LcrhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LcrhSpec {
    const RESET_VALUE: u32 = 0;
}
