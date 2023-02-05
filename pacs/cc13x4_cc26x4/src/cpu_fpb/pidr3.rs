#[doc = "Register `PIDR3` reader"]
pub struct R(crate::R<PIDR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIDR3` writer"]
pub struct W(crate::W<PIDR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIDR3_SPEC>;
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
impl From<crate::W<PIDR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIDR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMOD` reader - 3:0\\]
See CoreSight Architecture Specification"]
pub type CMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMOD` writer - 3:0\\]
See CoreSight Architecture Specification"]
pub type CMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `REVAND` reader - 7:4\\]
See CoreSight Architecture Specification"]
pub type REVAND_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVAND` writer - 7:4\\]
See CoreSight Architecture Specification"]
pub type REVAND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR3_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR3_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn revand(&self) -> REVAND_R {
        REVAND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CMOD_W<0> {
        CMOD_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    #[must_use]
    pub fn revand(&mut self) -> REVAND_W<4> {
        REVAND_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Provides CoreSight discovery information for the FP\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr3](index.html) module"]
pub struct PIDR3_SPEC;
impl crate::RegisterSpec for PIDR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr3::R](R) reader structure"]
impl crate::Readable for PIDR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pidr3::W](W) writer structure"]
impl crate::Writable for PIDR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIDR3 to value 0"]
impl crate::Resettable for PIDR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
