#[doc = "Register `SOC_ADC_OFFSET_INT` reader"]
pub struct R(crate::R<SOC_ADC_OFFSET_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOC_ADC_OFFSET_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOC_ADC_OFFSET_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOC_ADC_OFFSET_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOC_ADC_OFFSET_INT` writer"]
pub struct W(crate::W<SOC_ADC_OFFSET_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOC_ADC_OFFSET_INT_SPEC>;
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
impl From<crate::W<SOC_ADC_OFFSET_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOC_ADC_OFFSET_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOC_ADC_ABS_OFFSET_TEMP1` reader - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SOC_ADC_ABS_OFFSET_TEMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOC_ADC_ABS_OFFSET_TEMP1` writer - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SOC_ADC_ABS_OFFSET_TEMP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_OFFSET_INT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_OFFSET_INT_SPEC, u8, u8, 8, O>;
#[doc = "Field `SOC_ADC_REL_OFFSET_TEMP1` reader - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SOC_ADC_REL_OFFSET_TEMP1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOC_ADC_REL_OFFSET_TEMP1` writer - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SOC_ADC_REL_OFFSET_TEMP1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_OFFSET_INT_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED24_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOC_ADC_OFFSET_INT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_offset_temp1(&self) -> SOC_ADC_ABS_OFFSET_TEMP1_R {
        SOC_ADC_ABS_OFFSET_TEMP1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_offset_temp1(&self) -> SOC_ADC_REL_OFFSET_TEMP1_R {
        SOC_ADC_REL_OFFSET_TEMP1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> RESERVED24_R {
        RESERVED24_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_abs_offset_temp1(&mut self) -> SOC_ADC_ABS_OFFSET_TEMP1_W<0> {
        SOC_ADC_ABS_OFFSET_TEMP1_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_rel_offset_temp1(&mut self) -> SOC_ADC_REL_OFFSET_TEMP1_W<16> {
        SOC_ADC_REL_OFFSET_TEMP1_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> RESERVED24_W<24> {
        RESERVED24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soc_adc_offset_int](index.html) module"]
pub struct SOC_ADC_OFFSET_INT_SPEC;
impl crate::RegisterSpec for SOC_ADC_OFFSET_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [soc_adc_offset_int::R](R) reader structure"]
impl crate::Readable for SOC_ADC_OFFSET_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [soc_adc_offset_int::W](W) writer structure"]
impl crate::Writable for SOC_ADC_OFFSET_INT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOC_ADC_OFFSET_INT to value 0"]
impl crate::Resettable for SOC_ADC_OFFSET_INT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
