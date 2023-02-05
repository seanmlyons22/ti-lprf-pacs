#[doc = "Register `AIFOUTPTRNEXT` reader"]
pub struct R(crate::R<AIFOUTPTRNEXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIFOUTPTRNEXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIFOUTPTRNEXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIFOUTPTRNEXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIFOUTPTRNEXT` writer"]
pub struct W(crate::W<AIFOUTPTRNEXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIFOUTPTRNEXT_SPEC>;
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
impl From<crate::W<AIFOUTPTRNEXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIFOUTPTRNEXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTR` reader - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
pub type PTR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PTR` writer - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
pub type PTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIFOUTPTRNEXT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
    #[inline(always)]
    pub fn ptr(&self) -> PTR_R {
        PTR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Pointer to the first byte in the next DMA output buffer. The read value equals the last written value until the currently used DMA output buffer is completed, and then becomes null when the last written value is transferred to the DMA controller to start on the next buffer. This event is signalized by IRQFLAGS.AIF_DMA_OUT. At startup, the value must be written once before and once after configuring the DMA buffer size in AIFDMACFG. At this time, the first two samples will be fetched from memory. The next pointer must be written to this register while the DMA function uses the previously written pointer. If not written in time, IRQFLAGS.PTR_ERR will be raised and all output pins will be disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PTR_W<0> {
        PTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Output Buffer Next Pointer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aifoutptrnext](index.html) module"]
pub struct AIFOUTPTRNEXT_SPEC;
impl crate::RegisterSpec for AIFOUTPTRNEXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aifoutptrnext::R](R) reader structure"]
impl crate::Readable for AIFOUTPTRNEXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aifoutptrnext::W](W) writer structure"]
impl crate::Writable for AIFOUTPTRNEXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIFOUTPTRNEXT to value 0"]
impl crate::Resettable for AIFOUTPTRNEXT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
