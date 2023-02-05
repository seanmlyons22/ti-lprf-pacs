#[doc = "Register `LCRH` reader"]
pub struct R(crate::R<LCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCRH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCRH` writer"]
pub struct W(crate::W<LCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRK` reader - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BRK_R = crate::BitReader<bool>;
#[doc = "Field `BRK` writer - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
pub type BRK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
#[doc = "Field `PEN` reader - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
pub type PEN_R = crate::BitReader<PEN_A>;
#[doc = "1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEN_A {
    #[doc = "1: Parity checking and generation is enabled."]
    EN = 1,
    #[doc = "0: Parity is disabled and no parity bit is added to the data frame"]
    DIS = 0,
}
impl From<PEN_A> for bool {
    #[inline(always)]
    fn from(variant: PEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEN_A {
        match self.bits {
            true => PEN_A::EN,
            false => PEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == PEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == PEN_A::DIS
    }
}
#[doc = "Field `PEN` writer - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
pub type PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, PEN_A, O>;
impl<'a, const O: u8> PEN_W<'a, O> {
    #[doc = "Parity checking and generation is enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PEN_A::EN)
    }
    #[doc = "Parity is disabled and no parity bit is added to the data frame"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PEN_A::DIS)
    }
}
#[doc = "Field `EPS` reader - 2:2\\]
UART Even Parity Select"]
pub type EPS_R = crate::BitReader<EPS_A>;
#[doc = "2:2\\]
UART Even Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPS_A {
    #[doc = "1: Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    EVEN = 1,
    #[doc = "0: Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    ODD = 0,
}
impl From<EPS_A> for bool {
    #[inline(always)]
    fn from(variant: EPS_A) -> Self {
        variant as u8 != 0
    }
}
impl EPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPS_A {
        match self.bits {
            true => EPS_A::EVEN,
            false => EPS_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == EPS_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == EPS_A::ODD
    }
}
#[doc = "Field `EPS` writer - 2:2\\]
UART Even Parity Select"]
pub type EPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, EPS_A, O>;
impl<'a, const O: u8> EPS_W<'a, O> {
    #[doc = "Even parity: The UART generates or checks for an even number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(EPS_A::EVEN)
    }
    #[doc = "Odd parity: The UART generates or checks for an odd number of 1s in the data and parity bits."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(EPS_A::ODD)
    }
}
#[doc = "Field `STP2` reader - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type STP2_R = crate::BitReader<bool>;
#[doc = "Field `STP2` writer - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
pub type STP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
#[doc = "Field `FEN` reader - 4:4\\]
UART Enable FIFOs"]
pub type FEN_R = crate::BitReader<FEN_A>;
#[doc = "4:4\\]
UART Enable FIFOs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEN_A {
    #[doc = "1: Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    EN = 1,
    #[doc = "0: FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    DIS = 0,
}
impl From<FEN_A> for bool {
    #[inline(always)]
    fn from(variant: FEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEN_A {
        match self.bits {
            true => FEN_A::EN,
            false => FEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FEN_A::DIS
    }
}
#[doc = "Field `FEN` writer - 4:4\\]
UART Enable FIFOs"]
pub type FEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, FEN_A, O>;
impl<'a, const O: u8> FEN_W<'a, O> {
    #[doc = "Transmit and receive FIFO buffers are enabled (FIFO mode)"]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FEN_A::EN)
    }
    #[doc = "FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FEN_A::DIS)
    }
}
#[doc = "Field `WLEN` reader - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
pub type WLEN_R = crate::FieldReader<u8, WLEN_A>;
#[doc = "6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WLEN_A {
    #[doc = "3: Word Length 8 bits"]
    _8 = 3,
    #[doc = "2: Word Length 7 bits"]
    _7 = 2,
    #[doc = "1: Word Length 6 bits"]
    _6 = 1,
    #[doc = "0: Word Length 5 bits"]
    _5 = 0,
}
impl From<WLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: WLEN_A) -> Self {
        variant as _
    }
}
impl WLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WLEN_A {
        match self.bits {
            3 => WLEN_A::_8,
            2 => WLEN_A::_7,
            1 => WLEN_A::_6,
            0 => WLEN_A::_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == WLEN_A::_8
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == WLEN_A::_7
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == WLEN_A::_6
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == WLEN_A::_5
    }
}
#[doc = "Field `WLEN` writer - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
pub type WLEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, LCRH_SPEC, u8, WLEN_A, 2, O>;
impl<'a, const O: u8> WLEN_W<'a, O> {
    #[doc = "Word Length 8 bits"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(WLEN_A::_8)
    }
    #[doc = "Word Length 7 bits"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(WLEN_A::_7)
    }
    #[doc = "Word Length 6 bits"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(WLEN_A::_6)
    }
    #[doc = "Word Length 5 bits"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(WLEN_A::_5)
    }
}
#[doc = "Field `SPS` reader - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
pub type SPS_R = crate::BitReader<bool>;
#[doc = "Field `SPS` writer - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
pub type SPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCRH_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn brk(&self) -> BRK_R {
        BRK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline(always)]
    pub fn pen(&self) -> PEN_R {
        PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
UART Even Parity Select"]
    #[inline(always)]
    pub fn eps(&self) -> EPS_R {
        EPS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn stp2(&self) -> STP2_R {
        STP2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
UART Enable FIFOs"]
    #[inline(always)]
    pub fn fen(&self) -> FEN_R {
        FEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline(always)]
    pub fn wlen(&self) -> WLEN_R {
        WLEN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline(always)]
    pub fn sps(&self) -> SPS_R {
        SPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
UART Send Break If this bit is set to 1, a low-level is continually output on the UARTTXD output pin, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    #[must_use]
    pub fn brk(&mut self) -> BRK_W<0> {
        BRK_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
UART Parity Enable This bit controls generation and checking of parity bit."]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PEN_W<1> {
        PEN_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
UART Even Parity Select"]
    #[inline(always)]
    #[must_use]
    pub fn eps(&mut self) -> EPS_W<2> {
        EPS_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
UART Two Stop Bits Select: If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    #[must_use]
    pub fn stp2(&mut self) -> STP2_W<3> {
        STP2_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
UART Enable FIFOs"]
    #[inline(always)]
    #[must_use]
    pub fn fen(&mut self) -> FEN_W<4> {
        FEN_W::new(self)
    }
    #[doc = "Bits 5:6 - 6:5\\]
UART Word Length: These bits indicate the number of data bits transmitted or received in a frame."]
    #[inline(always)]
    #[must_use]
    pub fn wlen(&mut self) -> WLEN_W<5> {
        WLEN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
UART Stick Parity Select: 0: Stick parity is disabled 1: The parity bit is transmitted and checked as invert of EPS field (i.e. the parity bit is transmitted and checked as 1 when EPS = 0). This bit has no effect when PEN disables parity checking and generation."]
    #[inline(always)]
    #[must_use]
    pub fn sps(&mut self) -> SPS_W<7> {
        SPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Line Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcrh](index.html) module"]
pub struct LCRH_SPEC;
impl crate::RegisterSpec for LCRH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lcrh::R](R) reader structure"]
impl crate::Readable for LCRH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcrh::W](W) writer structure"]
impl crate::Writable for LCRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCRH to value 0"]
impl crate::Resettable for LCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
