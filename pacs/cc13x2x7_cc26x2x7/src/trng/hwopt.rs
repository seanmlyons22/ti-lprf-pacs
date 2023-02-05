#[doc = "Register `HWOPT` reader"]
pub struct R(crate::R<HWOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HWOPT` writer"]
pub struct W(crate::W<HWOPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HWOPT_SPEC>;
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
impl From<crate::W<HWOPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HWOPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWOPT_SPEC, u8, u8, 6, O>;
#[doc = "Field `NR_OF_FROS` reader - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
pub type NR_OF_FROS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NR_OF_FROS` writer - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
pub type NR_OF_FROS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWOPT_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED12` reader - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED12` writer - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HWOPT_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    pub fn nr_of_fros(&self) -> NR_OF_FROS_R {
        NR_OF_FROS_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved12(&self) -> RESERVED12_R {
        RESERVED12_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Number of FROs implemented in this TRNG, value 24 (decimal)."]
    #[inline(always)]
    #[must_use]
    pub fn nr_of_fros(&mut self) -> NR_OF_FROS_W<6> {
        NR_OF_FROS_W::new(self)
    }
    #[doc = "Bits 12:31 - 31:12\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved12(&mut self) -> RESERVED12_W<12> {
        RESERVED12_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TRNG Engine Options Information\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwopt](index.html) module"]
pub struct HWOPT_SPEC;
impl crate::RegisterSpec for HWOPT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwopt::R](R) reader structure"]
impl crate::Readable for HWOPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hwopt::W](W) writer structure"]
impl crate::Writable for HWOPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HWOPT to value 0x0600"]
impl crate::Resettable for HWOPT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
