#[doc = "Register `CLEARREQMASK` reader"]
pub struct R(crate::R<CLEARREQMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEARREQMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEARREQMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEARREQMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLEARREQMASK` writer"]
pub struct W(crate::W<CLEARREQMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARREQMASK_SPEC>;
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
impl From<crate::W<CLEARREQMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARREQMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
pub type CHNLS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
pub type CHNLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLEARREQMASK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to enable DMA request for the channel. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETREQMASK.CHNLS to disable channel C from generating requests. Bit \\[Ch\\]
= 1: Enables channel \\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn chnls(&mut self) -> CHNLS_W<0> {
        CHNLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Channel Request Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearreqmask](index.html) module"]
pub struct CLEARREQMASK_SPEC;
impl crate::RegisterSpec for CLEARREQMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clearreqmask::R](R) reader structure"]
impl crate::Readable for CLEARREQMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clearreqmask::W](W) writer structure"]
impl crate::Writable for CLEARREQMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLEARREQMASK to value 0"]
impl crate::Resettable for CLEARREQMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
