#[doc = "Register `SOC_ADC_REL_GAIN` reader"]
pub struct R(crate::R<SOC_ADC_REL_GAIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_ADC_REL_GAIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_ADC_REL_GAIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_ADC_REL_GAIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_ADC_REL_GAIN` writer"]
pub struct W(crate::W<SOC_ADC_REL_GAIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_ADC_REL_GAIN_SPEC>;
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
impl From<crate::W<SOC_ADC_REL_GAIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_ADC_REL_GAIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOC_ADC_REL_GAIN_TEMP1` reader - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
pub type SOC_ADC_REL_GAIN_TEMP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SOC_ADC_REL_GAIN_TEMP1` writer - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
pub type SOC_ADC_REL_GAIN_TEMP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_REL_GAIN_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_REL_GAIN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_gain_temp1(&self) -> SOC_ADC_REL_GAIN_TEMP1_R {
        SOC_ADC_REL_GAIN_TEMP1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_rel_gain_temp1(&mut self) -> SOC_ADC_REL_GAIN_TEMP1_W<0> {
        SOC_ADC_REL_GAIN_TEMP1_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX_ADC Gain in Relative Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_rel_gain](index.html) module"]
pub struct SOC_ADC_REL_GAIN_SPEC;
impl crate::RegisterSpec for SOC_ADC_REL_GAIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_adc_rel_gain::R](R) reader structure"]
impl crate::Readable for SOC_ADC_REL_GAIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_adc_rel_gain::W](W) writer structure"]
impl crate::Writable for SOC_ADC_REL_GAIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOC_ADC_REL_GAIN to value 0xffff_0000"]
impl crate::Resettable for SOC_ADC_REL_GAIN_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
