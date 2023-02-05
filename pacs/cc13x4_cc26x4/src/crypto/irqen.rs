#[doc = "Register `IRQEN` reader"]
pub struct R(crate::R<IRQEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQEN` writer"]
pub struct W(crate::W<IRQEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQEN_SPEC>;
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
impl From<crate::W<IRQEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESULT_AVAIL` reader - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
pub type RESULT_AVAIL_R = crate::BitReader<bool>;
#[doc = "Field `RESULT_AVAIL` writer - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
pub type RESULT_AVAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `DMA_IN_DONE` reader - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
pub type DMA_IN_DONE_R = crate::BitReader<bool>;
#[doc = "Field `DMA_IN_DONE` writer - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
pub type DMA_IN_DONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQEN_SPEC, bool, O>;
#[doc = "Field `RESERVED2` reader - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED2` writer - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IRQEN_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
    #[inline(always)]
    pub fn result_avail(&self) -> RESULT_AVAIL_R {
        RESULT_AVAIL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
    #[inline(always)]
    pub fn dma_in_done(&self) -> DMA_IN_DONE_R {
        DMA_IN_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
If this bit is set to 0, the Result Available interrupt is disabled If this bit is set to 1, the Result Available interrupt is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn result_avail(&mut self) -> RESULT_AVAIL_W<0> {
        RESULT_AVAIL_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
If this bit is set to 0, the DMA input done interrupt disabled If this bit is set to 1, the DMA input done interrupt enabled."]
    #[inline(always)]
    #[must_use]
    pub fn dma_in_done(&mut self) -> DMA_IN_DONE_W<1> {
        DMA_IN_DONE_W::new(self)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<2> {
        RESERVED2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqen](index.html) module"]
pub struct IRQEN_SPEC;
impl crate::RegisterSpec for IRQEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqen::R](R) reader structure"]
impl crate::Readable for IRQEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqen::W](W) writer structure"]
impl crate::Writable for IRQEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQEN to value 0"]
impl crate::Resettable for IRQEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
