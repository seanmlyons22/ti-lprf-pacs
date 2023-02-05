#[doc = "Register `FCFG_B1_START` reader"]
pub struct R(crate::R<FCFG_B1_START_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG_B1_START_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG_B1_START_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG_B1_START_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG_B1_START` writer"]
pub struct W(crate::W<FCFG_B1_START_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG_B1_START_SPEC>;
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
impl From<crate::W<FCFG_B1_START_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG_B1_START_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `B1_START_ADDR` reader - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B1_START_ADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `B1_START_ADDR` writer - 23:0\\]
Internal. Only to be used through TI provided API."]
pub type B1_START_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_B1_START_SPEC, u32, u32, 24, O>;
#[doc = "Field `B1_MUX_FACTOR` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B1_MUX_FACTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B1_MUX_FACTOR` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type B1_MUX_FACTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_B1_START_SPEC, u8, u8, 4, O>;
#[doc = "Field `B1_MAX_SECTOR` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B1_MAX_SECTOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `B1_MAX_SECTOR` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type B1_MAX_SECTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FCFG_B1_START_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_start_addr(&self) -> B1_START_ADDR_R {
        B1_START_ADDR_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_mux_factor(&self) -> B1_MUX_FACTOR_R {
        B1_MUX_FACTOR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn b1_max_sector(&self) -> B1_MAX_SECTOR_R {
        B1_MAX_SECTOR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b1_start_addr(&mut self) -> B1_START_ADDR_W<0> {
        B1_START_ADDR_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b1_mux_factor(&mut self) -> B1_MUX_FACTOR_W<24> {
        B1_MUX_FACTOR_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn b1_max_sector(&mut self) -> B1_MAX_SECTOR_W<28> {
        B1_MAX_SECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg_b1_start](index.html) module"]
pub struct FCFG_B1_START_SPEC;
impl crate::RegisterSpec for FCFG_B1_START_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg_b1_start::R](R) reader structure"]
impl crate::Readable for FCFG_B1_START_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg_b1_start::W](W) writer structure"]
impl crate::Writable for FCFG_B1_START_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFG_B1_START to value 0"]
impl crate::Resettable for FCFG_B1_START_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
