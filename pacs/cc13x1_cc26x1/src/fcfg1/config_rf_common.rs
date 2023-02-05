#[doc = "Register `CONFIG_RF_COMMON` reader"]
pub struct R(crate::R<CONFIG_RF_COMMON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_RF_COMMON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_RF_COMMON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_RF_COMMON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_RF_COMMON` writer"]
pub struct W(crate::W<CONFIG_RF_COMMON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_RF_COMMON_SPEC>;
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
impl From<crate::W<CONFIG_RF_COMMON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_RF_COMMON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACTRIM` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DACTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACTRIM` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DACTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_COMMON_SPEC, u8, u8, 6, O>;
#[doc = "Field `QUANTCTLTHRES` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QUANTCTLTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `QUANTCTLTHRES` writer - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QUANTCTLTHRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_COMMON_SPEC, u8, u8, 3, O>;
#[doc = "Field `RFLDO_TRIM_OUTPUT` reader - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RFLDO_TRIM_OUTPUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFLDO_TRIM_OUTPUT` writer - 15:9\\]
Internal. Only to be used through TI provided API."]
pub type RFLDO_TRIM_OUTPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_COMMON_SPEC, u8, u8, 7, O>;
#[doc = "Field `CTL_PA_20DBM_TRIM` reader - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type CTL_PA_20DBM_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTL_PA_20DBM_TRIM` writer - 20:16\\]
Internal. Only to be used through TI provided API."]
pub type CTL_PA_20DBM_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_COMMON_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA20DBMTRIMCOMPLETE_N` reader - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type PA20DBMTRIMCOMPLETE_N_R = crate::BitReader<bool>;
#[doc = "Field `PA20DBMTRIMCOMPLETE_N` writer - 21:21\\]
Internal. Only to be used through TI provided API."]
pub type PA20DBMTRIMCOMPLETE_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_RF_COMMON_SPEC, bool, O>;
#[doc = "Field `SLDO_TRIM_OUTPUT` reader - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type SLDO_TRIM_OUTPUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLDO_TRIM_OUTPUT` writer - 30:25\\]
Internal. Only to be used through TI provided API."]
pub type SLDO_TRIM_OUTPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_RF_COMMON_SPEC, u8, u8, 6, O>;
#[doc = "Field `DISABLE_CORNER_CAP` reader - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type DISABLE_CORNER_CAP_R = crate::BitReader<bool>;
#[doc = "Field `DISABLE_CORNER_CAP` writer - 31:31\\]
Internal. Only to be used through TI provided API."]
pub type DISABLE_CORNER_CAP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_RF_COMMON_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&self) -> DACTRIM_R {
        DACTRIM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QUANTCTLTHRES_R {
        QUANTCTLTHRES_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfldo_trim_output(&self) -> RFLDO_TRIM_OUTPUT_R {
        RFLDO_TRIM_OUTPUT_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa_20dbm_trim(&self) -> CTL_PA_20DBM_TRIM_R {
        CTL_PA_20DBM_TRIM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn pa20dbmtrimcomplete_n(&self) -> PA20DBMTRIMCOMPLETE_N_R {
        PA20DBMTRIMCOMPLETE_N_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sldo_trim_output(&self) -> SLDO_TRIM_OUTPUT_R {
        SLDO_TRIM_OUTPUT_R::new(((self.bits >> 25) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable_corner_cap(&self) -> DISABLE_CORNER_CAP_R {
        DISABLE_CORNER_CAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dactrim(&mut self) -> DACTRIM_W<0> {
        DACTRIM_W::new(self)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn quantctlthres(&mut self) -> QUANTCTLTHRES_W<6> {
        QUANTCTLTHRES_W::new(self)
    }
    #[doc = "Bits 9:15 - 15:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rfldo_trim_output(&mut self) -> RFLDO_TRIM_OUTPUT_W<9> {
        RFLDO_TRIM_OUTPUT_W::new(self)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ctl_pa_20dbm_trim(&mut self) -> CTL_PA_20DBM_TRIM_W<16> {
        CTL_PA_20DBM_TRIM_W::new(self)
    }
    #[doc = "Bit 21 - 21:21\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn pa20dbmtrimcomplete_n(&mut self) -> PA20DBMTRIMCOMPLETE_N_W<21> {
        PA20DBMTRIMCOMPLETE_N_W::new(self)
    }
    #[doc = "Bits 25:30 - 30:25\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sldo_trim_output(&mut self) -> SLDO_TRIM_OUTPUT_W<25> {
        SLDO_TRIM_OUTPUT_W::new(self)
    }
    #[doc = "Bit 31 - 31:31\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn disable_corner_cap(&mut self) -> DISABLE_CORNER_CAP_W<31> {
        DISABLE_CORNER_CAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_rf_common](index.html) module"]
pub struct CONFIG_RF_COMMON_SPEC;
impl crate::RegisterSpec for CONFIG_RF_COMMON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_rf_common::R](R) reader structure"]
impl crate::Readable for CONFIG_RF_COMMON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_rf_common::W](W) writer structure"]
impl crate::Writable for CONFIG_RF_COMMON_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_RF_COMMON to value 0x81c0_014d"]
impl crate::Resettable for CONFIG_RF_COMMON_SPEC {
    const RESET_VALUE: Self::Ux = 0x81c0_014d;
}
