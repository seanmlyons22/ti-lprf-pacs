#[doc = "Register `AESTAGOUT` reader"]
pub struct R(crate::R<AESTAGOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESTAGOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESTAGOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESTAGOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESTAGOUT` writer"]
pub struct W(crate::W<AESTAGOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESTAGOUT_SPEC>;
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
impl From<crate::W<AESTAGOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESTAGOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AES_TAG` reader - 31:0\\]
AES_TAG\\[31:0\\]
Bits \\[31:0\\]
of this register stores the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the AESCTL.SAVED_CONTEXT_RDY register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
pub type AES_TAG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AES_TAG` writer - 31:0\\]
AES_TAG\\[31:0\\]
Bits \\[31:0\\]
of this register stores the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the AESCTL.SAVED_CONTEXT_RDY register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
pub type AES_TAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AESTAGOUT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
AES_TAG\\[31:0\\]
Bits \\[31:0\\]
of this register stores the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the AESCTL.SAVED_CONTEXT_RDY register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
    #[inline(always)]
    pub fn aes_tag(&self) -> AES_TAG_R {
        AES_TAG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
AES_TAG\\[31:0\\]
Bits \\[31:0\\]
of this register stores the authentication value for the combined and authentication only modes. For a host read operation, these registers contain the last 128-bit TAG output of the EIP-120t; the TAG is available until the next context is written. This register will only contain valid data if the TAG is available and when the AESCTL.SAVED_CONTEXT_RDY register is set. During processing or for operations/modes that do not return a TAG, reads from this register return data from the IV register."]
    #[inline(always)]
    #[must_use]
    pub fn aes_tag(&mut self) -> AES_TAG_W<0> {
        AES_TAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Tag Out 0 The tag registers can be accessed via DMA or directly with host reads. These registers buffer the TAG from the EIP-120t. The registers are shared with the intermediate authentication result registers, but cannot be read until the processing is finished. While processing, a read from these registers returns 0s. If an operation does not return a TAG, reading from these registers returns an IV. If an operation returns a TAG plus an IV and both need to be read by the host, the host must first read the TAG followed by the IV. Reading these in reverse order will return the IV twice.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aestagout](index.html) module"]
pub struct AESTAGOUT_SPEC;
impl crate::RegisterSpec for AESTAGOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aestagout::R](R) reader structure"]
impl crate::Readable for AESTAGOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aestagout::W](W) writer structure"]
impl crate::Writable for AESTAGOUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AESTAGOUT to value 0"]
impl crate::Resettable for AESTAGOUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
