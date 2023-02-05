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
#[doc = "Field `TXIFLSEL` reader - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type TXIFLSEL_R = crate::FieldReader<u8, TXIFLSEL_A>;
#[doc = "2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TXIFLSEL_A {
    #[doc = "7: Trigger when TX FIFO has >= 1 byte free"]
    LEVEL_1 = 7,
    #[doc = "6: Reserved"]
    LVL_RES6 = 6,
    #[doc = "5: TX FIFO is empty"]
    LVL_EMPTY = 5,
    #[doc = "4: Reserved"]
    LVL_RES4 = 4,
    #[doc = "3: TX FIFO <= 1/4 empty"]
    LVL_1_4 = 3,
    #[doc = "2: TX FIFO <= 1/2 empty (default)"]
    LVL_1_2 = 2,
    #[doc = "1: TX FIFO <= 3/4 empty"]
    LVL_3_4 = 1,
    #[doc = "0: Reserved"]
    LVL_OFF = 0,
}
impl From<TXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TXIFLSEL_A) -> Self {
        variant as _
    }
}
impl TXIFLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIFLSEL_A {
        match self.bits {
            7 => TXIFLSEL_A::LEVEL_1,
            6 => TXIFLSEL_A::LVL_RES6,
            5 => TXIFLSEL_A::LVL_EMPTY,
            4 => TXIFLSEL_A::LVL_RES4,
            3 => TXIFLSEL_A::LVL_1_4,
            2 => TXIFLSEL_A::LVL_1_2,
            1 => TXIFLSEL_A::LVL_3_4,
            0 => TXIFLSEL_A::LVL_OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == TXIFLSEL_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LVL_RES6`"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == TXIFLSEL_A::LVL_RES6
    }
    #[doc = "Checks if the value of the field is `LVL_EMPTY`"]
    #[inline(always)]
    pub fn is_lvl_empty(&self) -> bool {
        *self == TXIFLSEL_A::LVL_EMPTY
    }
    #[doc = "Checks if the value of the field is `LVL_RES4`"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == TXIFLSEL_A::LVL_RES4
    }
    #[doc = "Checks if the value of the field is `LVL_1_4`"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == TXIFLSEL_A::LVL_1_4
    }
    #[doc = "Checks if the value of the field is `LVL_1_2`"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == TXIFLSEL_A::LVL_1_2
    }
    #[doc = "Checks if the value of the field is `LVL_3_4`"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == TXIFLSEL_A::LVL_3_4
    }
    #[doc = "Checks if the value of the field is `LVL_OFF`"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == TXIFLSEL_A::LVL_OFF
    }
}
#[doc = "Field `TXIFLSEL` writer - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
pub type TXIFLSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IFLS_SPEC, u8, TXIFLSEL_A, 3, O>;
impl<'a, const O: u8> TXIFLSEL_W<'a, O> {
    #[doc = "Trigger when TX FIFO has >= 1 byte free"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LEVEL_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_RES6)
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn lvl_empty(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_EMPTY)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_RES4)
    }
    #[doc = "TX FIFO <= 1/4 empty"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_1_4)
    }
    #[doc = "TX FIFO <= 1/2 empty (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_1_2)
    }
    #[doc = "TX FIFO <= 3/4 empty"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_3_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut W {
        self.variant(TXIFLSEL_A::LVL_OFF)
    }
}
#[doc = "Field `RXIFLSEL` reader - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type RXIFLSEL_R = crate::FieldReader<u8, RXIFLSEL_A>;
#[doc = "5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RXIFLSEL_A {
    #[doc = "7: Trigger when RX FIFO contains >= 1 byte"]
    LEVEL_1 = 7,
    #[doc = "6: Reserved"]
    LVL_RES6 = 6,
    #[doc = "5: RX FIFO is full"]
    LVL_FULL = 5,
    #[doc = "4: Reserved"]
    LVL_RES4 = 4,
    #[doc = "3: RX FIFO >= 3/4 full"]
    LVL_3_4 = 3,
    #[doc = "2: RX FIFO >= 1/2 full (default)"]
    LVL_1_2 = 2,
    #[doc = "1: RX FIFO >= 1/4 full"]
    LVL_1_4 = 1,
    #[doc = "0: Reserved"]
    LVL_OFF = 0,
}
impl From<RXIFLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXIFLSEL_A) -> Self {
        variant as _
    }
}
impl RXIFLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIFLSEL_A {
        match self.bits {
            7 => RXIFLSEL_A::LEVEL_1,
            6 => RXIFLSEL_A::LVL_RES6,
            5 => RXIFLSEL_A::LVL_FULL,
            4 => RXIFLSEL_A::LVL_RES4,
            3 => RXIFLSEL_A::LVL_3_4,
            2 => RXIFLSEL_A::LVL_1_2,
            1 => RXIFLSEL_A::LVL_1_4,
            0 => RXIFLSEL_A::LVL_OFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LEVEL_1`"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == RXIFLSEL_A::LEVEL_1
    }
    #[doc = "Checks if the value of the field is `LVL_RES6`"]
    #[inline(always)]
    pub fn is_lvl_res6(&self) -> bool {
        *self == RXIFLSEL_A::LVL_RES6
    }
    #[doc = "Checks if the value of the field is `LVL_FULL`"]
    #[inline(always)]
    pub fn is_lvl_full(&self) -> bool {
        *self == RXIFLSEL_A::LVL_FULL
    }
    #[doc = "Checks if the value of the field is `LVL_RES4`"]
    #[inline(always)]
    pub fn is_lvl_res4(&self) -> bool {
        *self == RXIFLSEL_A::LVL_RES4
    }
    #[doc = "Checks if the value of the field is `LVL_3_4`"]
    #[inline(always)]
    pub fn is_lvl_3_4(&self) -> bool {
        *self == RXIFLSEL_A::LVL_3_4
    }
    #[doc = "Checks if the value of the field is `LVL_1_2`"]
    #[inline(always)]
    pub fn is_lvl_1_2(&self) -> bool {
        *self == RXIFLSEL_A::LVL_1_2
    }
    #[doc = "Checks if the value of the field is `LVL_1_4`"]
    #[inline(always)]
    pub fn is_lvl_1_4(&self) -> bool {
        *self == RXIFLSEL_A::LVL_1_4
    }
    #[doc = "Checks if the value of the field is `LVL_OFF`"]
    #[inline(always)]
    pub fn is_lvl_off(&self) -> bool {
        *self == RXIFLSEL_A::LVL_OFF
    }
}
#[doc = "Field `RXIFLSEL` writer - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
pub type RXIFLSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, IFLS_SPEC, u8, RXIFLSEL_A, 3, O>;
impl<'a, const O: u8> RXIFLSEL_W<'a, O> {
    #[doc = "Trigger when RX FIFO contains >= 1 byte"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LEVEL_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res6(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_RES6)
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn lvl_full(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_FULL)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_res4(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_RES4)
    }
    #[doc = "RX FIFO >= 3/4 full"]
    #[inline(always)]
    pub fn lvl_3_4(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_3_4)
    }
    #[doc = "RX FIFO >= 1/2 full (default)"]
    #[inline(always)]
    pub fn lvl_1_2(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_1_2)
    }
    #[doc = "RX FIFO >= 1/4 full"]
    #[inline(always)]
    pub fn lvl_1_4(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_1_4)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn lvl_off(self) -> &'a mut W {
        self.variant(RXIFLSEL_A::LVL_OFF)
    }
}
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFLS_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    pub fn txiflsel(&self) -> TXIFLSEL_R {
        TXIFLSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    pub fn rxiflsel(&self) -> RXIFLSEL_R {
        RXIFLSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
SPI Transmit Interrupt FIFO Level Select The trigger points for the transmit interrupt are as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn txiflsel(&mut self) -> TXIFLSEL_W<0> {
        TXIFLSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - 5:3\\]
SPI Receive Interrupt FIFO Level Select The trigger points for the receive interrupt are as follows:"]
    #[inline(always)]
    #[must_use]
    pub fn rxiflsel(&mut self) -> RXIFLSEL_W<3> {
        RXIFLSEL_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The IFLS register is the interrupt FIFO level select register. This register can be used to define the levels at which the TX, RX FIFO interrupt flags are triggered. Out of reset, the TXIFLSEL and RXIFLSEL bits are configured so that the FIFOs trigger an interrupt at the half-way mark.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifls](index.html) module"]
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
