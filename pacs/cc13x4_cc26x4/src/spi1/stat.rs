#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT` writer"]
pub struct W(crate::W<STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_SPEC>;
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
impl From<crate::W<STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFE` reader - 0:0\\]
Transmit FIFO empty."]
pub type TFE_R = crate::BitReader<TFE_A>;
#[doc = "0:0\\]
Transmit FIFO empty.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE_A {
    #[doc = "1: Transmit FIFO is empty."]
    EMPTY = 1,
    #[doc = "0: Transmit FIFO is not empty."]
    NOT_EMPTY = 0,
}
impl From<TFE_A> for bool {
    #[inline(always)]
    fn from(variant: TFE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFE_A {
        match self.bits {
            true => TFE_A::EMPTY,
            false => TFE_A::NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TFE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TFE_A::NOT_EMPTY
    }
}
#[doc = "Field `TFE` writer - 0:0\\]
Transmit FIFO empty."]
pub type TFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, TFE_A, O>;
impl<'a, const O: u8> TFE_W<'a, O> {
    #[doc = "Transmit FIFO is empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TFE_A::EMPTY)
    }
    #[doc = "Transmit FIFO is not empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TFE_A::NOT_EMPTY)
    }
}
#[doc = "Field `TNF` reader - 1:1\\]
Transmit FIFO not full"]
pub type TNF_R = crate::BitReader<TNF_A>;
#[doc = "1:1\\]
Transmit FIFO not full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TNF_A {
    #[doc = "1: Transmit FIFO is not full."]
    NOT_FULL = 1,
    #[doc = "0: Transmit FIFO is full."]
    FULL = 0,
}
impl From<TNF_A> for bool {
    #[inline(always)]
    fn from(variant: TNF_A) -> Self {
        variant as u8 != 0
    }
}
impl TNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNF_A {
        match self.bits {
            true => TNF_A::NOT_FULL,
            false => TNF_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == TNF_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == TNF_A::FULL
    }
}
#[doc = "Field `TNF` writer - 1:1\\]
Transmit FIFO not full"]
pub type TNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, TNF_A, O>;
impl<'a, const O: u8> TNF_W<'a, O> {
    #[doc = "Transmit FIFO is not full."]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut W {
        self.variant(TNF_A::NOT_FULL)
    }
    #[doc = "Transmit FIFO is full."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(TNF_A::FULL)
    }
}
#[doc = "Field `RFE` reader - 2:2\\]
Receive FIFO empty."]
pub type RFE_R = crate::BitReader<RFE_A>;
#[doc = "2:2\\]
Receive FIFO empty.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFE_A {
    #[doc = "1: Receive FIFO is empty."]
    EMPTY = 1,
    #[doc = "0: Receive FIFO is not empty."]
    NOT_EMPTY = 0,
}
impl From<RFE_A> for bool {
    #[inline(always)]
    fn from(variant: RFE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFE_A {
        match self.bits {
            true => RFE_A::EMPTY,
            false => RFE_A::NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RFE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RFE_A::NOT_EMPTY
    }
}
#[doc = "Field `RFE` writer - 2:2\\]
Receive FIFO empty."]
pub type RFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, RFE_A, O>;
impl<'a, const O: u8> RFE_W<'a, O> {
    #[doc = "Receive FIFO is empty."]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(RFE_A::EMPTY)
    }
    #[doc = "Receive FIFO is not empty."]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(RFE_A::NOT_EMPTY)
    }
}
#[doc = "Field `RNF` reader - 3:3\\]
Receive FIFO not full"]
pub type RNF_R = crate::BitReader<RNF_A>;
#[doc = "3:3\\]
Receive FIFO not full\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNF_A {
    #[doc = "1: Receive FIFO is not full."]
    NOT_FULL = 1,
    #[doc = "0: Receive FIFO is full."]
    FULL = 0,
}
impl From<RNF_A> for bool {
    #[inline(always)]
    fn from(variant: RNF_A) -> Self {
        variant as u8 != 0
    }
}
impl RNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNF_A {
        match self.bits {
            true => RNF_A::NOT_FULL,
            false => RNF_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == RNF_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == RNF_A::FULL
    }
}
#[doc = "Field `RNF` writer - 3:3\\]
Receive FIFO not full"]
pub type RNF_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, RNF_A, O>;
impl<'a, const O: u8> RNF_W<'a, O> {
    #[doc = "Receive FIFO is not full."]
    #[inline(always)]
    pub fn not_full(self) -> &'a mut W {
        self.variant(RNF_A::NOT_FULL)
    }
    #[doc = "Receive FIFO is full."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(RNF_A::FULL)
    }
}
#[doc = "Field `BUSY` reader - 4:4\\]
Busy"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "4:4\\]
Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "1: SPI is currently transmitting and/or receiving data, or transmit FIFO is not empty."]
    ACTIVE = 1,
    #[doc = "0: SPI is in idle mode."]
    IDLE = 0,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            true => BUSY_A::ACTIVE,
            false => BUSY_A::IDLE,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == BUSY_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BUSY_A::IDLE
    }
}
#[doc = "Field `BUSY` writer - 4:4\\]
Busy"]
pub type BUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, STAT_SPEC, BUSY_A, O>;
impl<'a, const O: u8> BUSY_W<'a, O> {
    #[doc = "SPI is currently transmitting and/or receiving data, or transmit FIFO is not empty."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(BUSY_A::ACTIVE)
    }
    #[doc = "SPI is in idle mode."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(BUSY_A::IDLE)
    }
}
#[doc = "Field `RESERVED5` reader - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED5` writer - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full"]
    #[inline(always)]
    pub fn tnf(&self) -> TNF_R {
        TNF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO empty."]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO not full"]
    #[inline(always)]
    pub fn rnf(&self) -> RNF_R {
        RNF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved5(&self) -> RESERVED5_R {
        RESERVED5_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmit FIFO empty."]
    #[inline(always)]
    #[must_use]
    pub fn tfe(&mut self) -> TFE_W<0> {
        TFE_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Transmit FIFO not full"]
    #[inline(always)]
    #[must_use]
    pub fn tnf(&mut self) -> TNF_W<1> {
        TNF_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Receive FIFO empty."]
    #[inline(always)]
    #[must_use]
    pub fn rfe(&mut self) -> RFE_W<2> {
        RFE_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Receive FIFO not full"]
    #[inline(always)]
    #[must_use]
    pub fn rnf(&mut self) -> RNF_W<3> {
        RNF_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BUSY_W<4> {
        BUSY_W::new(self)
    }
    #[doc = "Bits 5:31 - 31:5\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved5(&mut self) -> RESERVED5_W<5> {
        RESERVED5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat::W](W) writer structure"]
impl crate::Writable for STAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT to value 0x0f"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
