#[doc = "Register `IFLS` reader"]
pub struct R(crate::R<IFLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLS` writer"]
pub struct W(crate::W<IFLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLS_SPEC>;
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
impl From<crate::W<IFLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSEL` reader - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
pub type TXSEL_R = crate::FieldReader<u8, TXSEL_A>;
#[doc = "2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXSEL_A {
    #[doc = "4: Transmit FIFO becomes <= 7/8 full"]
    _7_8 = 4,
    #[doc = "3: Transmit FIFO becomes <= 3/4 full"]
    _6_8 = 3,
    #[doc = "2: Transmit FIFO becomes <= 1/2 full"]
    _4_8 = 2,
    #[doc = "1: Transmit FIFO becomes <= 1/4 full"]
    _2_8 = 1,
    #[doc = "0: Transmit FIFO becomes <= 1/8 full"]
    _1_8 = 0,
}
impl From<TXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXSEL_A) -> Self {
        variant as _
    }
}
impl TXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXSEL_A> {
        match self.bits {
            4 => Some(TXSEL_A::_7_8),
            3 => Some(TXSEL_A::_6_8),
            2 => Some(TXSEL_A::_4_8),
            1 => Some(TXSEL_A::_2_8),
            0 => Some(TXSEL_A::_1_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_7_8`"]
    #[inline(always)]
    pub fn is_7_8(&self) -> bool {
        *self == TXSEL_A::_7_8
    }
    #[doc = "Checks if the value of the field is `_6_8`"]
    #[inline(always)]
    pub fn is_6_8(&self) -> bool {
        *self == TXSEL_A::_6_8
    }
    #[doc = "Checks if the value of the field is `_4_8`"]
    #[inline(always)]
    pub fn is_4_8(&self) -> bool {
        *self == TXSEL_A::_4_8
    }
    #[doc = "Checks if the value of the field is `_2_8`"]
    #[inline(always)]
    pub fn is_2_8(&self) -> bool {
        *self == TXSEL_A::_2_8
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == TXSEL_A::_1_8
    }
}
#[doc = "Field `TXSEL` writer - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
pub type TXSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFLS_SPEC, u8, TXSEL_A, 3, O>;
impl<'a, const O: u8> TXSEL_W<'a, O> {
    #[doc = "Transmit FIFO becomes <= 7/8 full"]
    #[inline(always)]
    pub fn _7_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_7_8)
    }
    #[doc = "Transmit FIFO becomes <= 3/4 full"]
    #[inline(always)]
    pub fn _6_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_6_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/2 full"]
    #[inline(always)]
    pub fn _4_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_4_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/4 full"]
    #[inline(always)]
    pub fn _2_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_2_8)
    }
    #[doc = "Transmit FIFO becomes <= 1/8 full"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(TXSEL_A::_1_8)
    }
}
#[doc = "Field `RXSEL` reader - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
pub type RXSEL_R = crate::FieldReader<u8, RXSEL_A>;
#[doc = "5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXSEL_A {
    #[doc = "4: Receive FIFO becomes >= 7/8 full"]
    _7_8 = 4,
    #[doc = "3: Receive FIFO becomes >= 3/4 full"]
    _6_8 = 3,
    #[doc = "2: Receive FIFO becomes >= 1/2 full"]
    _4_8 = 2,
    #[doc = "1: Receive FIFO becomes >= 1/4 full"]
    _2_8 = 1,
    #[doc = "0: Receive FIFO becomes >= 1/8 full"]
    _1_8 = 0,
}
impl From<RXSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXSEL_A) -> Self {
        variant as _
    }
}
impl RXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXSEL_A> {
        match self.bits {
            4 => Some(RXSEL_A::_7_8),
            3 => Some(RXSEL_A::_6_8),
            2 => Some(RXSEL_A::_4_8),
            1 => Some(RXSEL_A::_2_8),
            0 => Some(RXSEL_A::_1_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_7_8`"]
    #[inline(always)]
    pub fn is_7_8(&self) -> bool {
        *self == RXSEL_A::_7_8
    }
    #[doc = "Checks if the value of the field is `_6_8`"]
    #[inline(always)]
    pub fn is_6_8(&self) -> bool {
        *self == RXSEL_A::_6_8
    }
    #[doc = "Checks if the value of the field is `_4_8`"]
    #[inline(always)]
    pub fn is_4_8(&self) -> bool {
        *self == RXSEL_A::_4_8
    }
    #[doc = "Checks if the value of the field is `_2_8`"]
    #[inline(always)]
    pub fn is_2_8(&self) -> bool {
        *self == RXSEL_A::_2_8
    }
    #[doc = "Checks if the value of the field is `_1_8`"]
    #[inline(always)]
    pub fn is_1_8(&self) -> bool {
        *self == RXSEL_A::_1_8
    }
}
#[doc = "Field `RXSEL` writer - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
pub type RXSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFLS_SPEC, u8, RXSEL_A, 3, O>;
impl<'a, const O: u8> RXSEL_W<'a, O> {
    #[doc = "Receive FIFO becomes >= 7/8 full"]
    #[inline(always)]
    pub fn _7_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_7_8)
    }
    #[doc = "Receive FIFO becomes >= 3/4 full"]
    #[inline(always)]
    pub fn _6_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_6_8)
    }
    #[doc = "Receive FIFO becomes >= 1/2 full"]
    #[inline(always)]
    pub fn _4_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_4_8)
    }
    #[doc = "Receive FIFO becomes >= 1/4 full"]
    #[inline(always)]
    pub fn _2_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_2_8)
    }
    #[doc = "Receive FIFO becomes >= 1/8 full"]
    #[inline(always)]
    pub fn _1_8(self) -> &'a mut W {
        self.variant(RXSEL_A::_1_8)
    }
}
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn txsel(&self) -> TXSEL_R {
        TXSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    pub fn rxsel(&self) -> RXSEL_R {
        RXSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Transmit interrupt FIFO level select: This field sets the trigger points for the transmit interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn txsel(&mut self) -> TXSEL_W<0> {
        TXSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Receive interrupt FIFO level select: This field sets the trigger points for the receive interrupt. Values 0b101-0b111 are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn rxsel(&mut self) -> RXSEL_W<3> {
        RXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt FIFO Level Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](index.html) module"]
pub struct IFLS_SPEC;
impl crate::RegisterSpec for IFLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ifls::R](R) reader structure"]
impl crate::Readable for IFLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ifls::W](W) writer structure"]
impl crate::Writable for IFLS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFLS to value 0x12"]
impl crate::Resettable for IFLS_SPEC {
    const RESET_VALUE: Self::Ux = 0x12;
}
