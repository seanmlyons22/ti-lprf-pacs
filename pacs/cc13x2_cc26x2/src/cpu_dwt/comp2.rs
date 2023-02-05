#[doc = "Register `COMP2` reader"]
pub struct R(crate::R<COMP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2` writer"]
pub struct W(crate::W<COMP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_SPEC>;
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
impl From<crate::W<COMP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION2."]
pub type COMP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COMP` writer - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION2."]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION2."]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reference value to compare against PC or the data address as given by FUNCTION2."]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<0> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 2 This register is used to write the reference value for comparator 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2](index.html) module"]
pub struct COMP2_SPEC;
impl crate::RegisterSpec for COMP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2::R](R) reader structure"]
impl crate::Readable for COMP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2::W](W) writer structure"]
impl crate::Writable for COMP2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP2 to value 0"]
impl crate::Resettable for COMP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
