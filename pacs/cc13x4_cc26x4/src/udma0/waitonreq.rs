#[doc = "Register `WAITONREQ` reader"]
pub struct R(crate::R<WAITONREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAITONREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAITONREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAITONREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAITONREQ` writer"]
pub struct W(crate::W<WAITONREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAITONREQ_SPEC>;
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
impl From<crate::W<WAITONREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAITONREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLSTATUS` reader - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
pub type CHNLSTATUS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHNLSTATUS` writer - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
pub type CHNLSTATUS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WAITONREQ_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline(always)]
    pub fn chnlstatus(&self) -> CHNLSTATUS_R {
        CHNLSTATUS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Channel wait on request status: Bit \\[Ch\\]
= 0: Once uDMA receives a single or burst request on channel Ch, this channel may come out of active state even if request is still present. Bit \\[Ch\\]
= 1: Once uDMA receives a single or burst request on channel Ch, it keeps channel Ch in active state until the requests are deasserted. This handshake is necessary for channels where the requester is in an asynchronous domain or can run at slower clock speed than uDMA"]
    #[inline(always)]
    #[must_use]
    pub fn chnlstatus(&mut self) -> CHNLSTATUS_W<0> {
        CHNLSTATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Wait On Request Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [waitonreq](index.html) module"]
pub struct WAITONREQ_SPEC;
impl crate::RegisterSpec for WAITONREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [waitonreq::R](R) reader structure"]
impl crate::Readable for WAITONREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [waitonreq::W](W) writer structure"]
impl crate::Writable for WAITONREQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAITONREQ to value 0xffff_1eff"]
impl crate::Resettable for WAITONREQ_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_1eff;
}
