#[doc = "Register `SOFTREQ` reader"]
pub struct R(crate::R<SOFTREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOFTREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOFTREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOFTREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOFTREQ` writer"]
pub struct W(crate::W<SOFTREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFTREQ_SPEC>;
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
impl From<crate::W<SOFTREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFTREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
pub type CHNLS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
pub type CHNLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SOFTREQ_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Set the appropriate bit to generate a software uDMA request on the corresponding uDMA channel Bit \\[Ch\\]
= 0: Does not create a uDMA request for channel Ch Bit \\[Ch\\]
= 1: Creates a uDMA request for channel Ch Writing to a bit where a uDMA channel is not implemented does not create a uDMA request for that channel"]
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
#[doc = "Channel Software Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [softreq](index.html) module"]
pub struct SOFTREQ_SPEC;
impl crate::RegisterSpec for SOFTREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [softreq::R](R) reader structure"]
impl crate::Readable for SOFTREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [softreq::W](W) writer structure"]
impl crate::Writable for SOFTREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFTREQ to value 0"]
impl crate::Resettable for SOFTREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
