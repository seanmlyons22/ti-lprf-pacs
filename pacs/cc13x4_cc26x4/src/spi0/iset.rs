#[doc = "Register `ISET` reader"]
pub struct R(crate::R<ISET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISET` writer"]
pub struct W(crate::W<ISET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISET_SPEC>;
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
impl From<crate::W<ISET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXFIFO_OVF` reader - 0:0\\]
Set RXFIFO overflow event."]
pub type RXFIFO_OVF_R = crate::BitReader<RXFIFO_OVF_A>;
#[doc = "0:0\\]
Set RXFIFO overflow event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXFIFO_OVF_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<RXFIFO_OVF_A> for bool {
    #[inline(always)]
    fn from(variant: RXFIFO_OVF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXFIFO_OVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXFIFO_OVF_A {
        match self.bits {
            true => RXFIFO_OVF_A::SET,
            false => RXFIFO_OVF_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RXFIFO_OVF_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RXFIFO_OVF_A::NO_EFFECT
    }
}
#[doc = "Field `RXFIFO_OVF` writer - 0:0\\]
Set RXFIFO overflow event."]
pub type RXFIFO_OVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, RXFIFO_OVF_A, O>;
impl<'a, const O: u8> RXFIFO_OVF_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RXFIFO_OVF_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RXFIFO_OVF_A::NO_EFFECT)
    }
}
#[doc = "Field `PER` reader - 1:1\\]
Set Parity error event."]
pub type PER_R = crate::BitReader<PER_A>;
#[doc = "1:1\\]
Set Parity error event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PER_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<PER_A> for bool {
    #[inline(always)]
    fn from(variant: PER_A) -> Self {
        variant as u8 != 0
    }
}
impl PER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PER_A {
        match self.bits {
            true => PER_A::SET,
            false => PER_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == PER_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == PER_A::NO_EFFECT
    }
}
#[doc = "Field `PER` writer - 1:1\\]
Set Parity error event."]
pub type PER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, PER_A, O>;
impl<'a, const O: u8> PER_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(PER_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(PER_A::NO_EFFECT)
    }
}
#[doc = "Field `RTOUT` reader - 2:2\\]
Set SPI Receive Time-Out event."]
pub type RTOUT_R = crate::BitReader<RTOUT_A>;
#[doc = "2:2\\]
Set SPI Receive Time-Out event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOUT_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<RTOUT_A> for bool {
    #[inline(always)]
    fn from(variant: RTOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTOUT_A {
        match self.bits {
            true => RTOUT_A::SET,
            false => RTOUT_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RTOUT_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RTOUT_A::NO_EFFECT
    }
}
#[doc = "Field `RTOUT` writer - 2:2\\]
Set SPI Receive Time-Out event."]
pub type RTOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, RTOUT_A, O>;
impl<'a, const O: u8> RTOUT_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RTOUT_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RTOUT_A::NO_EFFECT)
    }
}
#[doc = "Field `RX` reader - 3:3\\]
Set Receive FIFO event."]
pub type RX_R = crate::BitReader<RX_A>;
#[doc = "3:3\\]
Set Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RX_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<RX_A> for bool {
    #[inline(always)]
    fn from(variant: RX_A) -> Self {
        variant as u8 != 0
    }
}
impl RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_A {
        match self.bits {
            true => RX_A::SET,
            false => RX_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == RX_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RX_A::NO_EFFECT
    }
}
#[doc = "Field `RX` writer - 3:3\\]
Set Receive FIFO event."]
pub type RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, RX_A, O>;
impl<'a, const O: u8> RX_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(RX_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RX_A::NO_EFFECT)
    }
}
#[doc = "Field `TX` reader - 4:4\\]
Set Transmit FIFO event."]
pub type TX_R = crate::BitReader<TX_A>;
#[doc = "4:4\\]
Set Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            true => TX_A::SET,
            false => TX_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TX_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TX_A::NO_EFFECT
    }
}
#[doc = "Field `TX` writer - 4:4\\]
Set Transmit FIFO event."]
pub type TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, TX_A, O>;
impl<'a, const O: u8> TX_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TX_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TX_A::NO_EFFECT)
    }
}
#[doc = "Field `TXEMPTY` reader - 5:5\\]
Set Transmit FIFO Empty event."]
pub type TXEMPTY_R = crate::BitReader<TXEMPTY_A>;
#[doc = "5:5\\]
Set Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEMPTY_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<TXEMPTY_A> for bool {
    #[inline(always)]
    fn from(variant: TXEMPTY_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEMPTY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEMPTY_A {
        match self.bits {
            true => TXEMPTY_A::SET,
            false => TXEMPTY_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == TXEMPTY_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == TXEMPTY_A::NO_EFFECT
    }
}
#[doc = "Field `TXEMPTY` writer - 5:5\\]
Set Transmit FIFO Empty event."]
pub type TXEMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, TXEMPTY_A, O>;
impl<'a, const O: u8> TXEMPTY_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(TXEMPTY_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TXEMPTY_A::NO_EFFECT)
    }
}
#[doc = "Field `IDLE` reader - 6:6\\]
Set SPI IDLE mode event."]
pub type IDLE_R = crate::BitReader<IDLE_A>;
#[doc = "6:6\\]
Set SPI IDLE mode event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLE_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<IDLE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLE_A {
        match self.bits {
            true => IDLE_A::SET,
            false => IDLE_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == IDLE_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == IDLE_A::NO_EFFECT
    }
}
#[doc = "Field `IDLE` writer - 6:6\\]
Set SPI IDLE mode event."]
pub type IDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, IDLE_A, O>;
impl<'a, const O: u8> IDLE_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(IDLE_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(IDLE_A::NO_EFFECT)
    }
}
#[doc = "Field `DMA_DONE_RX` reader - 7:7\\]
Set DMA Done event for RX."]
pub type DMA_DONE_RX_R = crate::BitReader<DMA_DONE_RX_A>;
#[doc = "7:7\\]
Set DMA Done event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_DONE_RX_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<DMA_DONE_RX_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_DONE_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_DONE_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_DONE_RX_A {
        match self.bits {
            true => DMA_DONE_RX_A::SET,
            false => DMA_DONE_RX_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DMA_DONE_RX_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_DONE_RX_A::NO_EFFECT
    }
}
#[doc = "Field `DMA_DONE_RX` writer - 7:7\\]
Set DMA Done event for RX."]
pub type DMA_DONE_RX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, DMA_DONE_RX_A, O>;
impl<'a, const O: u8> DMA_DONE_RX_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DMA_DONE_RX_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_DONE_RX_A::NO_EFFECT)
    }
}
#[doc = "Field `DMA_DONE_TX` reader - 8:8\\]
Set DMA Done event for TX."]
pub type DMA_DONE_TX_R = crate::BitReader<DMA_DONE_TX_A>;
#[doc = "8:8\\]
Set DMA Done event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_DONE_TX_A {
    #[doc = "1: Set Interrupt"]
    SET = 1,
    #[doc = "0: Writing 0 has no effect"]
    NO_EFFECT = 0,
}
impl From<DMA_DONE_TX_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_DONE_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_DONE_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_DONE_TX_A {
        match self.bits {
            true => DMA_DONE_TX_A::SET,
            false => DMA_DONE_TX_A::NO_EFFECT,
        }
    }
    #[doc = "Checks if the value of the field is `SET`"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == DMA_DONE_TX_A::SET
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == DMA_DONE_TX_A::NO_EFFECT
    }
}
#[doc = "Field `DMA_DONE_TX` writer - 8:8\\]
Set DMA Done event for TX."]
pub type DMA_DONE_TX_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISET_SPEC, DMA_DONE_TX_A, O>;
impl<'a, const O: u8> DMA_DONE_TX_W<'a, O> {
    #[doc = "Set Interrupt"]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(DMA_DONE_TX_A::SET)
    }
    #[doc = "Writing 0 has no effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(DMA_DONE_TX_A::NO_EFFECT)
    }
}
#[doc = "Field `RESERVED9` reader - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED9` writer - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISET_SPEC, u32, u32, 23, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Set RXFIFO overflow event."]
    #[inline(always)]
    pub fn rxfifo_ovf(&self) -> RXFIFO_OVF_R {
        RXFIFO_OVF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Parity error event."]
    #[inline(always)]
    pub fn per(&self) -> PER_R {
        PER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Set SPI Receive Time-Out event."]
    #[inline(always)]
    pub fn rtout(&self) -> RTOUT_R {
        RTOUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Set Receive FIFO event."]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Set Transmit FIFO event."]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Set Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn txempty(&self) -> TXEMPTY_R {
        TXEMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Set SPI IDLE mode event."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Set DMA Done event for RX."]
    #[inline(always)]
    pub fn dma_done_rx(&self) -> DMA_DONE_RX_R {
        DMA_DONE_RX_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Set DMA Done event for TX."]
    #[inline(always)]
    pub fn dma_done_tx(&self) -> DMA_DONE_TX_R {
        DMA_DONE_TX_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved9(&self) -> RESERVED9_R {
        RESERVED9_R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Set RXFIFO overflow event."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_ovf(&mut self) -> RXFIFO_OVF_W<0> {
        RXFIFO_OVF_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Set Parity error event."]
    #[inline(always)]
    #[must_use]
    pub fn per(&mut self) -> PER_W<1> {
        PER_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Set SPI Receive Time-Out event."]
    #[inline(always)]
    #[must_use]
    pub fn rtout(&mut self) -> RTOUT_W<2> {
        RTOUT_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Set Receive FIFO event."]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<3> {
        RX_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Set Transmit FIFO event."]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<4> {
        TX_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Set Transmit FIFO Empty event."]
    #[inline(always)]
    #[must_use]
    pub fn txempty(&mut self) -> TXEMPTY_W<5> {
        TXEMPTY_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Set SPI IDLE mode event."]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IDLE_W<6> {
        IDLE_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Set DMA Done event for RX."]
    #[inline(always)]
    #[must_use]
    pub fn dma_done_rx(&mut self) -> DMA_DONE_RX_W<7> {
        DMA_DONE_RX_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Set DMA Done event for TX."]
    #[inline(always)]
    #[must_use]
    pub fn dma_done_tx(&mut self) -> DMA_DONE_TX_W<8> {
        DMA_DONE_TX_W::new(self)
    }
    #[doc = "Bits 9:31 - 31:9\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved9(&mut self) -> RESERVED9_W<9> {
        RESERVED9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt set. Allows interrupts to be set by software (useful in diagnostics and safety checks). Writing a 1 to a bit in ISET will set the event and therefore the related RIS bit also gets set. If the interrupt is enabled through the mask, then the corresponding MIS bit is also set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iset](index.html) module"]
pub struct ISET_SPEC;
impl crate::RegisterSpec for ISET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iset::R](R) reader structure"]
impl crate::Readable for ISET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iset::W](W) writer structure"]
impl crate::Writable for ISET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for ISET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
