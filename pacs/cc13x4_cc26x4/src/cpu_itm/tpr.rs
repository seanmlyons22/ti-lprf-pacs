#[doc = "Register `TPR` reader"]
pub struct R(crate::R<TPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPR` writer"]
pub struct W(crate::W<TPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPR_SPEC>;
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
impl From<crate::W<TPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVMASK` reader - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
pub type PRIVMASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRIVMASK` writer - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
pub type PRIVMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
    #[inline(always)]
    pub fn privmask(&self) -> PRIVMASK_R {
        PRIVMASK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For PRIVMASK\\[m\\], defines the access permissions of ITM_STIM Stimulus Ports 8m to 8m+7 inclusive"]
    #[inline(always)]
    #[must_use]
    pub fn privmask(&mut self) -> PRIVMASK_W<0> {
        PRIVMASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls which stimulus ports can be accessed by unprivileged code\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr](index.html) module"]
pub struct TPR_SPEC;
impl crate::RegisterSpec for TPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpr::R](R) reader structure"]
impl crate::Readable for TPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpr::W](W) writer structure"]
impl crate::Writable for TPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
