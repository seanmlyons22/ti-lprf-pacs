#[doc = "Register `FPMTCTL` reader"]
pub struct R(crate::R<FPMTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPMTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPMTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPMTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPMTCTL` writer"]
pub struct W(crate::W<FPMTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPMTCTL_SPEC>;
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
impl From<crate::W<FPMTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPMTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_INCR` reader - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ADDR_INCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADDR_INCR` writer - 31:0\\]
Internal. Only to be used through TI provided API."]
pub type ADDR_INCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPMTCTL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn addr_incr(&self) -> ADDR_INCR_R {
        ADDR_INCR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn addr_incr(&mut self) -> ADDR_INCR_W<0> {
        ADDR_INCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpmtctl](index.html) module"]
pub struct FPMTCTL_SPEC;
impl crate::RegisterSpec for FPMTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpmtctl::R](R) reader structure"]
impl crate::Readable for FPMTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpmtctl::W](W) writer structure"]
impl crate::Writable for FPMTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPMTCTL to value 0"]
impl crate::Resettable for FPMTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
