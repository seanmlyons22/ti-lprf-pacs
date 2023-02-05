#[doc = "Register `IIDX` reader"]
pub struct R(crate::R<IIDX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IIDX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IIDX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IIDX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IIDX` writer"]
pub struct W(crate::W<IIDX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IIDX_SPEC>;
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
impl From<crate::W<IIDX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IIDX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STAT` reader - 7:0\\]
Interrupt index status"]
pub type STAT_R = crate::FieldReader<u8, STAT_A>;
#[doc = "7:0\\]
Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STAT_A {
    #[doc = "9: DMA Done for Transmit Event/interrupt pending"]
    DMA_DONE_TX_EVT = 9,
    #[doc = "8: DMA Done for Receive Event/interrupt pending"]
    DMA_DONE_RX_EVT = 8,
    #[doc = "7: End of Transmit Event/interrupt pending"]
    IDLE_EVT = 7,
    #[doc = "6: Transmit Buffer Empty Event/interrupt pending"]
    TX_EMPTY = 6,
    #[doc = "5: Transmit Event/interrupt pending"]
    TX_EVT = 5,
    #[doc = "4: Receive Event/interrupt pending"]
    RX_EVT = 4,
    #[doc = "3: SPI Receive Time-Out Event/interrupt pending"]
    RTOUT_EVT = 3,
    #[doc = "2: Transmit Parity Event/interrupt pending"]
    PER_EVT = 2,
    #[doc = "1: RX FIFO Overflow Event/interrupt pending"]
    RXFIFO_OFV_EVT = 1,
    #[doc = "0: No interrupt pending"]
    NO_INTR = 0,
}
impl From<STAT_A> for u8 {
    #[inline(always)]
    fn from(variant: STAT_A) -> Self {
        variant as _
    }
}
impl STAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STAT_A> {
        match self.bits {
            9 => Some(STAT_A::DMA_DONE_TX_EVT),
            8 => Some(STAT_A::DMA_DONE_RX_EVT),
            7 => Some(STAT_A::IDLE_EVT),
            6 => Some(STAT_A::TX_EMPTY),
            5 => Some(STAT_A::TX_EVT),
            4 => Some(STAT_A::RX_EVT),
            3 => Some(STAT_A::RTOUT_EVT),
            2 => Some(STAT_A::PER_EVT),
            1 => Some(STAT_A::RXFIFO_OFV_EVT),
            0 => Some(STAT_A::NO_INTR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_DONE_TX_EVT`"]
    #[inline(always)]
    pub fn is_dma_done_tx_evt(&self) -> bool {
        *self == STAT_A::DMA_DONE_TX_EVT
    }
    #[doc = "Checks if the value of the field is `DMA_DONE_RX_EVT`"]
    #[inline(always)]
    pub fn is_dma_done_rx_evt(&self) -> bool {
        *self == STAT_A::DMA_DONE_RX_EVT
    }
    #[doc = "Checks if the value of the field is `IDLE_EVT`"]
    #[inline(always)]
    pub fn is_idle_evt(&self) -> bool {
        *self == STAT_A::IDLE_EVT
    }
    #[doc = "Checks if the value of the field is `TX_EMPTY`"]
    #[inline(always)]
    pub fn is_tx_empty(&self) -> bool {
        *self == STAT_A::TX_EMPTY
    }
    #[doc = "Checks if the value of the field is `TX_EVT`"]
    #[inline(always)]
    pub fn is_tx_evt(&self) -> bool {
        *self == STAT_A::TX_EVT
    }
    #[doc = "Checks if the value of the field is `RX_EVT`"]
    #[inline(always)]
    pub fn is_rx_evt(&self) -> bool {
        *self == STAT_A::RX_EVT
    }
    #[doc = "Checks if the value of the field is `RTOUT_EVT`"]
    #[inline(always)]
    pub fn is_rtout_evt(&self) -> bool {
        *self == STAT_A::RTOUT_EVT
    }
    #[doc = "Checks if the value of the field is `PER_EVT`"]
    #[inline(always)]
    pub fn is_per_evt(&self) -> bool {
        *self == STAT_A::PER_EVT
    }
    #[doc = "Checks if the value of the field is `RXFIFO_OFV_EVT`"]
    #[inline(always)]
    pub fn is_rxfifo_ofv_evt(&self) -> bool {
        *self == STAT_A::RXFIFO_OFV_EVT
    }
    #[doc = "Checks if the value of the field is `NO_INTR`"]
    #[inline(always)]
    pub fn is_no_intr(&self) -> bool {
        *self == STAT_A::NO_INTR
    }
}
#[doc = "Field `STAT` writer - 7:0\\]
Interrupt index status"]
pub type STAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IIDX_SPEC, u8, STAT_A, 8, O>;
impl<'a, const O: u8> STAT_W<'a, O> {
    #[doc = "DMA Done for Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn dma_done_tx_evt(self) -> &'a mut W {
        self.variant(STAT_A::DMA_DONE_TX_EVT)
    }
    #[doc = "DMA Done for Receive Event/interrupt pending"]
    #[inline(always)]
    pub fn dma_done_rx_evt(self) -> &'a mut W {
        self.variant(STAT_A::DMA_DONE_RX_EVT)
    }
    #[doc = "End of Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn idle_evt(self) -> &'a mut W {
        self.variant(STAT_A::IDLE_EVT)
    }
    #[doc = "Transmit Buffer Empty Event/interrupt pending"]
    #[inline(always)]
    pub fn tx_empty(self) -> &'a mut W {
        self.variant(STAT_A::TX_EMPTY)
    }
    #[doc = "Transmit Event/interrupt pending"]
    #[inline(always)]
    pub fn tx_evt(self) -> &'a mut W {
        self.variant(STAT_A::TX_EVT)
    }
    #[doc = "Receive Event/interrupt pending"]
    #[inline(always)]
    pub fn rx_evt(self) -> &'a mut W {
        self.variant(STAT_A::RX_EVT)
    }
    #[doc = "SPI Receive Time-Out Event/interrupt pending"]
    #[inline(always)]
    pub fn rtout_evt(self) -> &'a mut W {
        self.variant(STAT_A::RTOUT_EVT)
    }
    #[doc = "Transmit Parity Event/interrupt pending"]
    #[inline(always)]
    pub fn per_evt(self) -> &'a mut W {
        self.variant(STAT_A::PER_EVT)
    }
    #[doc = "RX FIFO Overflow Event/interrupt pending"]
    #[inline(always)]
    pub fn rxfifo_ofv_evt(self) -> &'a mut W {
        self.variant(STAT_A::RXFIFO_OFV_EVT)
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn no_intr(self) -> &'a mut W {
        self.variant(STAT_A::NO_INTR)
    }
}
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IIDX_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt index status"]
    #[inline(always)]
    pub fn stat(&self) -> STAT_R {
        STAT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Interrupt index status"]
    #[inline(always)]
    #[must_use]
    pub fn stat(&mut self) -> STAT_W<0> {
        STAT_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register provides the highest priority enabled interrupt index. Value 0x00 means no event pending. Interrupt 1 is the highest priority, and 31 is the least priority. That is, the least bit position that is set to 1 denotes the highest priority pending interrupt. The priority order is fixed. However, users can implement their own prioritization schemes using other registers that expose the full set of interrupts that have occurred. On each read, only one interrupt is indicated. On a read, the current interrupt (highest priority) is automatically cleared by the hardware and the corresponding interrupt flag in RIS and MIS are cleared as well. After a read from the CPU (not from the debug interface), the register is updated with the next highest priority interrupt, if none are pending, then it would display 0x0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iidx](index.html) module"]
pub struct IIDX_SPEC;
impl crate::RegisterSpec for IIDX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iidx::R](R) reader structure"]
impl crate::Readable for IIDX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iidx::W](W) writer structure"]
impl crate::Writable for IIDX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IIDX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
