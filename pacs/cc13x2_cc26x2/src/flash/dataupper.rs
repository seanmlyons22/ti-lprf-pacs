#[doc = "Register `DATAUPPER` reader"]
pub struct R(crate::R<DATAUPPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAUPPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAUPPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAUPPER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAUPPER` writer"]
pub struct W(crate::W<DATAUPPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAUPPER_SPEC>;
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
impl From<crate::W<DATAUPPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAUPPER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEN` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type EEN_R = crate::BitReader<bool>;
#[doc = "Field `EEN` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type EEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATAUPPER_SPEC, bool, O>;
#[doc = "Field `R` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type R_R = crate::BitReader<bool>;
#[doc = "Field `R` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type R_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATAUPPER_SPEC, bool, O>;
#[doc = "Field `P` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type P_R = crate::BitReader<bool>;
#[doc = "Field `P` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type P_W<'a, const O: u8> = crate::BitWriter<'a, u32, DATAUPPER_SPEC, bool, O>;
#[doc = "Field `SPARE` reader - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type SPARE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE` writer - 7:3\\]
Internal. Only to be used through TI provided API."]
pub type SPARE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATAUPPER_SPEC, u8, u8, 5, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DATAUPPER_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn een(&self) -> EEN_R {
        EEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn p(&self) -> P_R {
        P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn spare(&self) -> SPARE_R {
        SPARE_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn een(&mut self) -> EEN_W<0> {
        EEN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<1> {
        R_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> P_W<2> {
        P_W::new(self)
    }
    #[doc = "Bits 3:7 - 7:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SPARE_W<3> {
        SPARE_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Internal. Only to be used through TI provided API."]
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
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataupper](index.html) module"]
pub struct DATAUPPER_SPEC;
impl crate::RegisterSpec for DATAUPPER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dataupper::R](R) reader structure"]
impl crate::Readable for DATAUPPER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dataupper::W](W) writer structure"]
impl crate::Writable for DATAUPPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAUPPER to value 0"]
impl crate::Resettable for DATAUPPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
