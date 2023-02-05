#[doc = "Register `SOC_ADC_REF_TRIM_AND_OFFSET_EXT` reader"]
pub struct R(crate::R<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_ADC_REF_TRIM_AND_OFFSET_EXT` writer"]
pub struct W(crate::W<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>;
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
impl From<crate::W<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOC_ADC_REF_VOLTAGE_TRIM_TEMP1` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOC_ADC_REF_VOLTAGE_TRIM_TEMP1` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_adc_ref_voltage_trim_temp1(&self) -> SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R {
        SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_ref_voltage_trim_temp1(&mut self) -> SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_W<0> {
        SOC_ADC_REF_VOLTAGE_TRIM_TEMP1_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_ref_trim_and_offset_ext](index.html) module"]
pub struct SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC;
impl crate::RegisterSpec for SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_adc_ref_trim_and_offset_ext::R](R) reader structure"]
impl crate::Readable for SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_adc_ref_trim_and_offset_ext::W](W) writer structure"]
impl crate::Writable for SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOC_ADC_REF_TRIM_AND_OFFSET_EXT to value 0xc080"]
impl crate::Resettable for SOC_ADC_REF_TRIM_AND_OFFSET_EXT_SPEC {
    const RESET_VALUE: Self::Ux = 0xc080;
}
