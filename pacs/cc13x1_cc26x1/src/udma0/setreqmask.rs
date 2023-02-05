#[doc = "Register `SETREQMASK` reader"]
pub struct R(crate::R<SETREQMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETREQMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETREQMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETREQMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETREQMASK` writer"]
pub struct W(crate::W<SETREQMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETREQMASK_SPEC>;
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
impl From<crate::W<SETREQMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETREQMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type CHNLS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type CHNLS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SETREQMASK_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the burst and single request mask status, or disables the corresponding channel from generating uDMA requests. Read as: Bit \\[Ch\\]
= 0: External requests are enabled for channel Ch. Bit \\[Ch\\]
= 1: External requests are disabled for channel Ch. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARREQMASK.CHNLS to enable uDMA requests. Bit \\[Ch\\]
= 1: Disables uDMA burst request channel \\[C\\]
and uDMA single request channel \\[C\\]
input from generating uDMA requests. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
#[doc = "Channel Set Request Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setreqmask](index.html) module"]
pub struct SETREQMASK_SPEC;
impl crate::RegisterSpec for SETREQMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setreqmask::R](R) reader structure"]
impl crate::Readable for SETREQMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setreqmask::W](W) writer structure"]
impl crate::Writable for SETREQMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETREQMASK to value 0"]
impl crate::Resettable for SETREQMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
