#[doc = "Register `IRQCLR` reader"]
pub struct R(crate::R<IRQCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQCLR` writer"]
pub struct W(crate::W<IRQCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQCLR_SPEC>;
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
impl From<crate::W<IRQCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT_AVAIL` reader - 0:0\\]
If 1 is written to this bit, the result available interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
pub type RESULT_AVAIL_R = crate::BitReader<bool>;
#[doc = "Field `RESULT_AVAIL` writer - 0:0\\]
If 1 is written to this bit, the result available interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
pub type RESULT_AVAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCLR_SPEC, bool, O>;
#[doc = "Field `DMA_IN_DONE` reader - 1:1\\]
If 1 is written to this bit, the DMA in done interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
pub type DMA_IN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `DMA_IN_DONE` writer - 1:1\\]
If 1 is written to this bit, the DMA in done interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
pub type DMA_IN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCLR_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQCLR_SPEC, u32, u32, 27, O>;
#[doc = "Field `KEY_ST_RD_ERR` reader - 29:29\\]
If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
pub type KEY_ST_RD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `KEY_ST_RD_ERR` writer - 29:29\\]
If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
pub type KEY_ST_RD_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCLR_SPEC, bool, O>;
#[doc = "Field `KEY_ST_WR_ERR` reader - 30:30\\]
If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
pub type KEY_ST_WR_ERR_R = crate::BitReader<bool>;
#[doc = "Field `KEY_ST_WR_ERR` writer - 30:30\\]
If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
pub type KEY_ST_WR_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCLR_SPEC, bool, O>;
#[doc = "Field `DMA_BUS_ERR` reader - 31:31\\]
If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
pub type DMA_BUS_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DMA_BUS_ERR` writer - 31:31\\]
If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
pub type DMA_BUS_ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQCLR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, the result available interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
    #[inline(always)]
    pub fn result_avail(&self) -> RESULT_AVAIL_R {
        RESULT_AVAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, the DMA in done interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x07ff_ffff)
    }
    #[doc = "Bit 29 - 29:29\\]
If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    pub fn key_st_rd_err(&self) -> KEY_ST_RD_ERR_R {
        KEY_ST_RD_ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    pub fn key_st_wr_err(&self) -> KEY_ST_WR_ERR_R {
        KEY_ST_WR_ERR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    pub fn dma_bus_err(&self) -> DMA_BUS_ERR_R {
        DMA_BUS_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If 1 is written to this bit, the result available interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
    #[inline(always)]
    #[must_use]
    pub fn result_avail(&mut self) -> RESULT_AVAIL_W<0> {
        RESULT_AVAIL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
If 1 is written to this bit, the DMA in done interrupt status is cleared. Writing 0 has no effect. Note that clearing an interrupt makes sense only if the interrupt output is programmed as level (refer to IRQTYPE)."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W<1> {
        DMA_IN_DONE_W::new(self)
    }
    #[doc = "Bits 2:28 - 28:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bit 29 - 29:29\\]
If 1 is written to this bit, the key store read error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_rd_err(&mut self) -> KEY_ST_RD_ERR_W<29> {
        KEY_ST_RD_ERR_W::new(self)
    }
    #[doc = "Bit 30 - 30:30\\]
If 1 is written to this bit, the key store write error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn key_st_wr_err(&mut self) -> KEY_ST_WR_ERR_W<30> {
        KEY_ST_WR_ERR_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
If 1 is written to this bit, the DMA bus error status is cleared. Writing 0 has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn dma_bus_err(&mut self) -> DMA_BUS_ERR_W<31> {
        DMA_BUS_ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Interrupt Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqclr](index.html) module"]
pub struct IRQCLR_SPEC;
impl crate::RegisterSpec for IRQCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqclr::R](R) reader structure"]
impl crate::Readable for IRQCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqclr::W](W) writer structure"]
impl crate::Writable for IRQCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQCLR to value 0"]
impl crate::Resettable for IRQCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
