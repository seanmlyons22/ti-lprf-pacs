#[doc = "Register `SHDW_SCAN_MCU3_SEC` reader"]
pub struct R(crate::R<SHDW_SCAN_MCU3_SEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_SCAN_MCU3_SEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_SCAN_MCU3_SEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_SCAN_MCU3_SEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_SCAN_MCU3_SEC` writer"]
pub struct W(crate::W<SHDW_SCAN_MCU3_SEC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_SCAN_MCU3_SEC_SPEC>;
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
impl From<crate::W<SHDW_SCAN_MCU3_SEC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_SCAN_MCU3_SEC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ULL_MCU_RAM_1_REP_1` reader - 10:0\\]
Internal. Only to be used through TI provided API."]
pub type ULL_MCU_RAM_1_REP_1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ULL_MCU_RAM_1_REP_1` writer - 10:0\\]
Internal. Only to be used through TI provided API."]
pub type ULL_MCU_RAM_1_REP_1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_SCAN_MCU3_SEC_SPEC, u16, u16, 11, O>;
#[doc = "Field `ULL_MCU_RAM_0_REP` reader - 22:11\\]
Internal. Only to be used through TI provided API."]
pub type ULL_MCU_RAM_0_REP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ULL_MCU_RAM_0_REP` writer - 22:11\\]
Internal. Only to be used through TI provided API."]
pub type ULL_MCU_RAM_0_REP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_SCAN_MCU3_SEC_SPEC, u16, u16, 12, O>;
#[doc = "Field `SECURITY` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type SECURITY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURITY` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type SECURITY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_SCAN_MCU3_SEC_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ull_mcu_ram_1_rep_1(&self) -> ULL_MCU_RAM_1_REP_1_R {
        ULL_MCU_RAM_1_REP_1_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 11:22 - 22:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ull_mcu_ram_0_rep(&self) -> ULL_MCU_RAM_0_REP_R {
        ULL_MCU_RAM_0_REP_R::new(((self.bits >> 11) & 0x0fff) as u16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn security(&self) -> SECURITY_R {
        SECURITY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - 10:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ull_mcu_ram_1_rep_1(&mut self) -> ULL_MCU_RAM_1_REP_1_W<0> {
        ULL_MCU_RAM_1_REP_1_W::new(self)
    }
    #[doc = "Bits 11:22 - 22:11\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ull_mcu_ram_0_rep(&mut self) -> ULL_MCU_RAM_0_REP_W<11> {
        ULL_MCU_RAM_0_REP_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn security(&mut self) -> SECURITY_W<24> {
        SECURITY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_scan_mcu3_sec](index.html) module"]
pub struct SHDW_SCAN_MCU3_SEC_SPEC;
impl crate::RegisterSpec for SHDW_SCAN_MCU3_SEC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_scan_mcu3_sec::R](R) reader structure"]
impl crate::Readable for SHDW_SCAN_MCU3_SEC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_scan_mcu3_sec::W](W) writer structure"]
impl crate::Writable for SHDW_SCAN_MCU3_SEC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHDW_SCAN_MCU3_SEC to value 0"]
impl crate::Resettable for SHDW_SCAN_MCU3_SEC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
