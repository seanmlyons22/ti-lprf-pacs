#[doc = "Register `FPAC1` reader"]
pub struct R(crate::R<FPAC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPAC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPAC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPAC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPAC1` writer"]
pub struct W(crate::W<FPAC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPAC1_SPEC>;
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
impl From<crate::W<FPAC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPAC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PUMPPWR` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PUMPPWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PUMPPWR` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type PUMPPWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPAC1_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED1` reader - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 3:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPAC1_SPEC, u8, u8, 2, O>;
#[doc = "Field `PUMPRESET_PW` reader - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type PUMPRESET_PW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PUMPRESET_PW` writer - 15:4\\]
Internal. Only to be used through TI provided API."]
pub type PUMPRESET_PW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPAC1_SPEC, u16, u16, 12, O>;
#[doc = "Field `PSLEEPTDIS` reader - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type PSLEEPTDIS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSLEEPTDIS` writer - 27:16\\]
Internal. Only to be used through TI provided API."]
pub type PSLEEPTDIS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPAC1_SPEC, u16, u16, 12, O>;
#[doc = "Field `RESERVED28` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED28` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FPAC1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumppwr(&self) -> PUMPPWR_R {
        PUMPPWR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pumpreset_pw(&self) -> PUMPRESET_PW_R {
        PUMPRESET_PW_R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn psleeptdis(&self) -> PSLEEPTDIS_R {
        PSLEEPTDIS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved28(&self) -> RESERVED28_R {
        RESERVED28_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pumppwr(&mut self) -> PUMPPWR_W<0> {
        PUMPPWR_W::new(self)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<2> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 4:15 - 15:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pumpreset_pw(&mut self) -> PUMPRESET_PW_W<4> {
        PUMPRESET_PW_W::new(self)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn psleeptdis(&mut self) -> PSLEEPTDIS_W<16> {
        PSLEEPTDIS_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved28(&mut self) -> RESERVED28_W<28> {
        RESERVED28_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fpac1](index.html) module"]
pub struct FPAC1_SPEC;
impl crate::RegisterSpec for FPAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fpac1::R](R) reader structure"]
impl crate::Readable for FPAC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fpac1::W](W) writer structure"]
impl crate::Writable for FPAC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPAC1 to value 0x0208_2081"]
impl crate::Resettable for FPAC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0208_2081;
}
