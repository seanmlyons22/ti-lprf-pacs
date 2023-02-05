#[doc = "Register `PIDR4` reader"]
pub struct R(crate::R<PIDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIDR4` writer"]
pub struct W(crate::W<PIDR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIDR4_SPEC>;
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
impl From<crate::W<PIDR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIDR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_2` reader - 3:0\\]
See CoreSight Architecture Specification"]
pub type DES_2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DES_2` writer - 3:0\\]
See CoreSight Architecture Specification"]
pub type DES_2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR4_SPEC, u8, u8, 4, O>;
#[doc = "Field `SIZE` reader - 7:4\\]
See CoreSight Architecture Specification"]
pub type SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIZE` writer - 7:4\\]
See CoreSight Architecture Specification"]
pub type SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR4_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR4_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_2(&self) -> DES_2_R {
        DES_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
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
    pub fn des_2(&mut self) -> DES_2_W<0> {
        DES_2_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    #[must_use]
    pub fn size(&mut self) -> SIZE_W<4> {
        SIZE_W::new(self)
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
#[doc = "Provides CoreSight discovery information for the DWT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr4](index.html) module"]
pub struct PIDR4_SPEC;
impl crate::RegisterSpec for PIDR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr4::R](R) reader structure"]
impl crate::Readable for PIDR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pidr4::W](W) writer structure"]
impl crate::Writable for PIDR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIDR4 to value 0"]
impl crate::Resettable for PIDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
