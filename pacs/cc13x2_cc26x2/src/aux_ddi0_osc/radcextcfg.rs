#[doc = "Register `RADCEXTCFG` reader"]
pub struct R(crate::R<RADCEXTCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RADCEXTCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RADCEXTCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RADCEXTCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RADCEXTCFG` writer"]
pub struct W(crate::W<RADCEXTCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RADCEXTCFG_SPEC>;
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
impl From<crate::W<RADCEXTCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RADCEXTCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RADCEXTCFG_SPEC, u8, u8, 5, O>;
#[doc = "Field `RADC_MODE_IS_SAR` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type RADC_MODE_IS_SAR_R = crate::BitReader<bool>;
#[doc = "Field `RADC_MODE_IS_SAR` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type RADC_MODE_IS_SAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RADCEXTCFG_SPEC, bool, O>;
#[doc = "Field `RADC_DAC_TH` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type RADC_DAC_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RADC_DAC_TH` writer - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type RADC_DAC_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RADCEXTCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `IDAC_STEP` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type IDAC_STEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDAC_STEP` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type IDAC_STEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RADCEXTCFG_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` reader - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_IBIAS_WAIT_CNT` writer - 21:16\\]
Internal. Only to be used through TI provided API."]
pub type LPM_IBIAS_WAIT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RADCEXTCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` reader - 31:22\\]
Internal. Only to be used through TI provided API."]
pub type HPM_IBIAS_WAIT_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPM_IBIAS_WAIT_CNT` writer - 31:22\\]
Internal. Only to be used through TI provided API."]
pub type HPM_IBIAS_WAIT_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, RADCEXTCFG_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_mode_is_sar(&self) -> RADC_MODE_IS_SAR_R {
        RADC_MODE_IS_SAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn radc_dac_th(&self) -> RADC_DAC_TH_R {
        RADC_DAC_TH_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn idac_step(&self) -> IDAC_STEP_R {
        IDAC_STEP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_ibias_wait_cnt(&self) -> LPM_IBIAS_WAIT_CNT_R {
        LPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpm_ibias_wait_cnt(&self) -> HPM_IBIAS_WAIT_CNT_R {
        HPM_IBIAS_WAIT_CNT_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn radc_mode_is_sar(&mut self) -> RADC_MODE_IS_SAR_W<5> {
        RADC_MODE_IS_SAR_W::new(self)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn radc_dac_th(&mut self) -> RADC_DAC_TH_W<6> {
        RADC_DAC_TH_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn idac_step(&mut self) -> IDAC_STEP_W<12> {
        IDAC_STEP_W::new(self)
    }
    #[doc = "Bits 16:21 - 21:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ibias_wait_cnt(&mut self) -> LPM_IBIAS_WAIT_CNT_W<16> {
        LPM_IBIAS_WAIT_CNT_W::new(self)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hpm_ibias_wait_cnt(&mut self) -> HPM_IBIAS_WAIT_CNT_W<22> {
        HPM_IBIAS_WAIT_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RADC External Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [radcextcfg](index.html) module"]
pub struct RADCEXTCFG_SPEC;
impl crate::RegisterSpec for RADCEXTCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [radcextcfg::R](R) reader structure"]
impl crate::Readable for RADCEXTCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [radcextcfg::W](W) writer structure"]
impl crate::Writable for RADCEXTCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RADCEXTCFG to value 0"]
impl crate::Resettable for RADCEXTCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
