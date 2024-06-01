#[doc = "Register `SOC_ADC_REL_GAIN` reader"]
pub type R = crate::R<SocAdcRelGainSpec>;
#[doc = "Register `SOC_ADC_REL_GAIN` writer"]
pub type W = crate::W<SocAdcRelGainSpec>;
#[doc = "Field `SOC_ADC_REL_GAIN_TEMP1` reader - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
pub type SocAdcRelGainTemp1R = crate::FieldReader<u16>;
#[doc = "Field `SOC_ADC_REL_GAIN_TEMP1` writer - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
pub type SocAdcRelGainTemp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_gain_temp1(&self) -> SocAdcRelGainTemp1R {
        SocAdcRelGainTemp1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
SOC_ADC gain in relative reference mode at temperature 1 (30C). Calculated in production test.."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_rel_gain_temp1(&mut self) -> SocAdcRelGainTemp1W<SocAdcRelGainSpec> {
        SocAdcRelGainTemp1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> Reserved16W<SocAdcRelGainSpec> {
        Reserved16W::new(self, 16)
    }
}
#[doc = "AUX_ADC Gain in Relative Reference Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_adc_rel_gain::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_adc_rel_gain::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocAdcRelGainSpec;
impl crate::RegisterSpec for SocAdcRelGainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_adc_rel_gain::R`](R) reader structure"]
impl crate::Readable for SocAdcRelGainSpec {}
#[doc = "`write(|w| ..)` method takes [`soc_adc_rel_gain::W`](W) writer structure"]
impl crate::Writable for SocAdcRelGainSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_ADC_REL_GAIN to value 0"]
impl crate::Resettable for SocAdcRelGainSpec {
    const RESET_VALUE: u32 = 0;
}
