#[doc = "Register `FLASH_C_E_P_R` reader"]
pub struct R(crate::R<FLASH_C_E_P_R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_C_E_P_R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_C_E_P_R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_C_E_P_R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH_C_E_P_R` writer"]
pub struct W(crate::W<FLASH_C_E_P_R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_C_E_P_R_SPEC>;
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
impl From<crate::W<FLASH_C_E_P_R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_C_E_P_R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CVSU` reader - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type CVSU_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CVSU` writer - 11:0\\]
Internal. Only to be used through TI provided API."]
pub type CVSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_C_E_P_R_SPEC, u16, u16, 12, O>;
#[doc = "Field `A_EXEZ_SETUP` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type A_EXEZ_SETUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `A_EXEZ_SETUP` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type A_EXEZ_SETUP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_C_E_P_R_SPEC, u8, u8, 4, O>;
#[doc = "Field `PV_ACCESS` reader - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type PV_ACCESS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PV_ACCESS` writer - 23:16\\]
Internal. Only to be used through TI provided API."]
pub type PV_ACCESS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FLASH_C_E_P_R_SPEC, u8, u8, 8, O>;
#[doc = "Field `RVSU` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RVSU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RVSU` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RVSU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLASH_C_E_P_R_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn cvsu(&self) -> CVSU_R {
        CVSU_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn a_exez_setup(&self) -> A_EXEZ_SETUP_R {
        A_EXEZ_SETUP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pv_access(&self) -> PV_ACCESS_R {
        PV_ACCESS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rvsu(&self) -> RVSU_R {
        RVSU_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn cvsu(&mut self) -> CVSU_W<0> {
        CVSU_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn a_exez_setup(&mut self) -> A_EXEZ_SETUP_W<12> {
        A_EXEZ_SETUP_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pv_access(&mut self) -> PV_ACCESS_W<16> {
        PV_ACCESS_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rvsu(&mut self) -> RVSU_W<24> {
        RVSU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash_c_e_p_r](index.html) module"]
pub struct FLASH_C_E_P_R_SPEC;
impl crate::RegisterSpec for FLASH_C_E_P_R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash_c_e_p_r::R](R) reader structure"]
impl crate::Readable for FLASH_C_E_P_R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash_c_e_p_r::W](W) writer structure"]
impl crate::Writable for FLASH_C_E_P_R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLASH_C_E_P_R to value 0x0a0a_2000"]
impl crate::Resettable for FLASH_C_E_P_R_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a0a_2000;
}
