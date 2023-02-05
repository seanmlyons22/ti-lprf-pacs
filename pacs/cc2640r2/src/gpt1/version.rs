#[doc = "Register `VERSION` reader"]
pub struct R(crate::R<VERSION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERSION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERSION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERSION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VERSION` writer"]
pub struct W(crate::W<VERSION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VERSION_SPEC>;
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
impl From<crate::W<VERSION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VERSION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VERSION` reader - 31:0\\]
Timer Revision."]
pub type VERSION_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VERSION` writer - 31:0\\]
Timer Revision."]
pub type VERSION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VERSION_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Timer Revision."]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Timer Revision."]
    #[inline(always)]
    #[must_use]
    pub fn version(&mut self) -> VERSION_W<0> {
        VERSION_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Version This register provides information regarding the GPT version\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version](index.html) module"]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [version::R](R) reader structure"]
impl crate::Readable for VERSION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [version::W](W) writer structure"]
impl crate::Writable for VERSION_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VERSION to value 0x0400"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0400;
}
