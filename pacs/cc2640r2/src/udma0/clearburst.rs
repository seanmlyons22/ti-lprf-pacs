#[doc = "Register `CLEARBURST` reader"]
pub struct R(crate::R<CLEARBURST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLEARBURST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLEARBURST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLEARBURST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLEARBURST` writer"]
pub struct W(crate::W<CLEARBURST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLEARBURST_SPEC>;
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
impl From<crate::W<CLEARBURST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLEARBURST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a DMA channel is not implemented has no effect."]
pub type CHNLS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a DMA channel is not implemented has no effect."]
pub type CHNLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLEARBURST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to enable single transfer requests. Write as: Bit \\[Ch\\]
= 0: No effect. Use the SETBURST.CHNLS to disable single transfer requests. Bit \\[Ch\\]
= 1: Enables single transfer requests on channel Ch. Writing to a bit where a DMA channel is not implemented has no effect."]
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
#[doc = "Channel Clear UseBurst\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clearburst](index.html) module"]
pub struct CLEARBURST_SPEC;
impl crate::RegisterSpec for CLEARBURST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clearburst::R](R) reader structure"]
impl crate::Readable for CLEARBURST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clearburst::W](W) writer structure"]
impl crate::Writable for CLEARBURST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLEARBURST to value 0"]
impl crate::Resettable for CLEARBURST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
