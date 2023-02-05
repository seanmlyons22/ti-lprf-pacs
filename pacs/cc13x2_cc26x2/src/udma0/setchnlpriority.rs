#[doc = "Register `SETCHNLPRIORITY` reader"]
pub struct R(crate::R<SETCHNLPRIORITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SETCHNLPRIORITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SETCHNLPRIORITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SETCHNLPRIORITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SETCHNLPRIORITY` writer"]
pub struct W(crate::W<SETCHNLPRIORITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SETCHNLPRIORITY_SPEC>;
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
impl From<crate::W<SETCHNLPRIORITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SETCHNLPRIORITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHNLS` reader - 31:0\\]
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type CHNLS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CHNLS` writer - 31:0\\]
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
pub type CHNLS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SETCHNLPRIORITY_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
    #[inline(always)]
    pub fn chnls(&self) -> CHNLS_R {
        CHNLS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Returns the channel priority mask status, or sets the channel priority to high. Read as: Bit \\[Ch\\]
= 0: uDMA channel Ch is using the default priority level. Bit \\[Ch\\]
= 1: uDMA channel Ch is using a high priority level. Write as: Bit \\[Ch\\]
= 0: No effect. Use the CLEARCHNLPRIORITY.CHNLS to set channel Ch to the default priority level. Bit \\[Ch\\]
= 1: Channel Ch uses the high priority level. Writing to a bit where a uDMA channel is not implemented has no effect"]
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
#[doc = "Set Channel Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setchnlpriority](index.html) module"]
pub struct SETCHNLPRIORITY_SPEC;
impl crate::RegisterSpec for SETCHNLPRIORITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [setchnlpriority::R](R) reader structure"]
impl crate::Readable for SETCHNLPRIORITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [setchnlpriority::W](W) writer structure"]
impl crate::Writable for SETCHNLPRIORITY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SETCHNLPRIORITY to value 0"]
impl crate::Resettable for SETCHNLPRIORITY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
