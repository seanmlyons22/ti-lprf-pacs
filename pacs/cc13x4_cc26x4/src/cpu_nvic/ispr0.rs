#[doc = "Register `ISPR0` reader"]
pub struct R(crate::R<ISPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISPR0` writer"]
pub struct W(crate::W<ISPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISPR0_SPEC>;
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
impl From<crate::W<ISPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SETPEND` reader - 31:0\\]
For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
pub type SETPEND_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SETPEND` writer - 31:0\\]
For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
pub type SETPEND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ISPR0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
    #[inline(always)]
    pub fn setpend(&self) -> SETPEND_R {
        SETPEND_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
For SETPEND\\[m\\]
in NVIC_ISPR*n, indicates whether interrupt 32*n + m is pending"]
    #[inline(always)]
    #[must_use]
    pub fn setpend(&mut self) -> SETPEND_W<0> {
        SETPEND_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enables or reads the pending state of each group of 32 interrupts\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ispr0](index.html) module"]
pub struct ISPR0_SPEC;
impl crate::RegisterSpec for ISPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ispr0::R](R) reader structure"]
impl crate::Readable for ISPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ispr0::W](W) writer structure"]
impl crate::Writable for ISPR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISPR0 to value 0"]
impl crate::Resettable for ISPR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
