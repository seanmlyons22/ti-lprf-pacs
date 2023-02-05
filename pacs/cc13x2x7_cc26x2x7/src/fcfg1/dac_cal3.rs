#[doc = "Register `DAC_CAL3` reader"]
pub struct R(crate::R<DAC_CAL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_CAL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_CAL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_CAL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_CAL3` writer"]
pub struct W(crate::W<DAC_CAL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_CAL3_SPEC>;
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
impl From<crate::W<DAC_CAL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_CAL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C1` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type SOC_DAC_VOUT_CAL_VDDS_C1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C1` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type SOC_DAC_VOUT_CAL_VDDS_C1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_CAL3_SPEC, u16, u16, 16, O>;
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C2` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SOC_DAC_VOUT_CAL_VDDS_C2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C2` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SOC_DAC_VOUT_CAL_VDDS_C2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DAC_CAL3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_vdds_c1(&self) -> SOC_DAC_VOUT_CAL_VDDS_C1_R {
        SOC_DAC_VOUT_CAL_VDDS_C1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_vdds_c2(&self) -> SOC_DAC_VOUT_CAL_VDDS_C2_R {
        SOC_DAC_VOUT_CAL_VDDS_C2_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_dac_vout_cal_vdds_c1(&mut self) -> SOC_DAC_VOUT_CAL_VDDS_C1_W<0> {
        SOC_DAC_VOUT_CAL_VDDS_C1_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_dac_vout_cal_vdds_c2(&mut self) -> SOC_DAC_VOUT_CAL_VDDS_C2_W<16> {
        SOC_DAC_VOUT_CAL_VDDS_C2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_cal3](index.html) module"]
pub struct DAC_CAL3_SPEC;
impl crate::RegisterSpec for DAC_CAL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_cal3::R](R) reader structure"]
impl crate::Readable for DAC_CAL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_cal3::W](W) writer structure"]
impl crate::Writable for DAC_CAL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAC_CAL3 to value 0"]
impl crate::Resettable for DAC_CAL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
