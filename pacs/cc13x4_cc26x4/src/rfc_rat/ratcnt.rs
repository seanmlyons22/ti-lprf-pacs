#[doc = "Register `RATCNT` reader"]
pub struct R(crate::R<RATCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RATCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RATCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RATCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RATCNT` writer"]
pub struct W(crate::W<RATCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RATCNT_SPEC>;
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
impl From<crate::W<RATCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RATCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
pub type CNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CNT` writer - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RATCNT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Counter value. This is not writable while radio timer counter is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Radio Timer Counter Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ratcnt](index.html) module"]
pub struct RATCNT_SPEC;
impl crate::RegisterSpec for RATCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ratcnt::R](R) reader structure"]
impl crate::Readable for RATCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ratcnt::W](W) writer structure"]
impl crate::Writable for RATCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RATCNT to value 0"]
impl crate::Resettable for RATCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
