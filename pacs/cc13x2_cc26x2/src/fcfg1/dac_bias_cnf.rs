#[doc = "Register `DAC_BIAS_CNF` reader"]
pub struct R(crate::R<DAC_BIAS_CNF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_BIAS_CNF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_BIAS_CNF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_BIAS_CNF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_BIAS_CNF` writer"]
pub struct W(crate::W<DAC_BIAS_CNF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_BIAS_CNF_SPEC>;
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
impl From<crate::W<DAC_BIAS_CNF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_BIAS_CNF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED1` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_BIAS_CNF_SPEC, u8, u8, 8, O>;
#[doc = "Field `LPM_BIAS_BACKUP_EN` reader - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type LPM_BIAS_BACKUP_EN_R = crate::BitReader<bool>;
#[doc = "Field `LPM_BIAS_BACKUP_EN` writer - 8:8\\]
Internal. Only to be used through TI provided API."]
pub type LPM_BIAS_BACKUP_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DAC_BIAS_CNF_SPEC, bool, O>;
#[doc = "Field `LPM_BIAS_WIDTH_TRIM` reader - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type LPM_BIAS_WIDTH_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_BIAS_WIDTH_TRIM` writer - 11:9\\]
Internal. Only to be used through TI provided API."]
pub type LPM_BIAS_WIDTH_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_BIAS_CNF_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPM_TRIM_IOUT` reader - 17:12\\]
Internal. Only to be used through TI provided API."]
pub type LPM_TRIM_IOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_TRIM_IOUT` writer - 17:12\\]
Internal. Only to be used through TI provided API."]
pub type LPM_TRIM_IOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_BIAS_CNF_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_backup_en(&self) -> LPM_BIAS_BACKUP_EN_R {
        LPM_BIAS_BACKUP_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_width_trim(&self) -> LPM_BIAS_WIDTH_TRIM_R {
        LPM_BIAS_WIDTH_TRIM_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_trim_iout(&self) -> LPM_TRIM_IOUT_R {
        LPM_TRIM_IOUT_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<0> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 8 - 8:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_bias_backup_en(&mut self) -> LPM_BIAS_BACKUP_EN_W<8> {
        LPM_BIAS_BACKUP_EN_W::new(self)
    }
    #[doc = "Bits 9:11 - 11:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_bias_width_trim(&mut self) -> LPM_BIAS_WIDTH_TRIM_W<9> {
        LPM_BIAS_WIDTH_TRIM_W::new(self)
    }
    #[doc = "Bits 12:17 - 17:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_trim_iout(&mut self) -> LPM_TRIM_IOUT_W<12> {
        LPM_TRIM_IOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_bias_cnf](index.html) module"]
pub struct DAC_BIAS_CNF_SPEC;
impl crate::RegisterSpec for DAC_BIAS_CNF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_bias_cnf::R](R) reader structure"]
impl crate::Readable for DAC_BIAS_CNF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_bias_cnf::W](W) writer structure"]
impl crate::Writable for DAC_BIAS_CNF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_BIAS_CNF to value 0xfffc_00ff"]
impl crate::Resettable for DAC_BIAS_CNF_SPEC {
    const RESET_VALUE: Self::Ux = 0xfffc_00ff;
}
