#[doc = "Register `PIDR2` reader"]
pub struct R(crate::R<PIDR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIDR2` writer"]
pub struct W(crate::W<PIDR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIDR2_SPEC>;
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
impl From<crate::W<PIDR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIDR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DES_1` reader - 2:0\\]
See CoreSight Architecture Specification"]
pub type DES_1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DES_1` writer - 2:0\\]
See CoreSight Architecture Specification"]
pub type DES_1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `JEDEC` reader - 3:3\\]
See CoreSight Architecture Specification"]
pub type JEDEC_R = crate::BitReader<bool>;
#[doc = "Field `JEDEC` writer - 3:3\\]
See CoreSight Architecture Specification"]
pub type JEDEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIDR2_SPEC, bool, O>;
#[doc = "Field `REVISION` reader - 7:4\\]
See CoreSight Architecture Specification"]
pub type REVISION_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVISION` writer - 7:4\\]
See CoreSight Architecture Specification"]
pub type REVISION_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PIDR2_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn des_1(&self) -> DES_1_R {
        DES_1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn jedec(&self) -> JEDEC_R {
        JEDEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn revision(&self) -> REVISION_R {
        REVISION_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    #[must_use]
    pub fn des_1(&mut self) -> DES_1_W<0> {
        DES_1_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    #[must_use]
    pub fn jedec(&mut self) -> JEDEC_W<3> {
        JEDEC_W::new(self)
    }
    #[doc = "Bits 4:7 - 7:4\\]
See CoreSight Architecture Specification"]
    #[inline(always)]
    #[must_use]
    pub fn revision(&mut self) -> REVISION_W<4> {
        REVISION_W::new(self)
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
#[doc = "Provides CoreSight discovery information for the DWT\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr2](index.html) module"]
pub struct PIDR2_SPEC;
impl crate::RegisterSpec for PIDR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr2::R](R) reader structure"]
impl crate::Readable for PIDR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pidr2::W](W) writer structure"]
impl crate::Writable for PIDR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIDR2 to value 0"]
impl crate::Resettable for PIDR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
