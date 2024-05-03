#[doc = "Register `SOC_ADC_OFFSET_INT` reader"]
pub type R = crate::R<SocAdcOffsetIntSpec>;
#[doc = "Register `SOC_ADC_OFFSET_INT` writer"]
pub type W = crate::W<SocAdcOffsetIntSpec>;
#[doc = "Field `SOC_ADC_ABS_OFFSET_TEMP1` reader - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SocAdcAbsOffsetTemp1R = crate::FieldReader;
#[doc = "Field `SOC_ADC_ABS_OFFSET_TEMP1` writer - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SocAdcAbsOffsetTemp1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED8` reader - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8R = crate::FieldReader;
#[doc = "Field `RESERVED8` writer - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SOC_ADC_REL_OFFSET_TEMP1` reader - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SocAdcRelOffsetTemp1R = crate::FieldReader;
#[doc = "Field `SOC_ADC_REL_OFFSET_TEMP1` writer - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
pub type SocAdcRelOffsetTemp1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RESERVED24` reader - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24R = crate::FieldReader;
#[doc = "Field `RESERVED24` writer - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_abs_offset_temp1(&self) -> SocAdcAbsOffsetTemp1R {
        SocAdcAbsOffsetTemp1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    pub fn soc_adc_rel_offset_temp1(&self) -> SocAdcRelOffsetTemp1R {
        SocAdcRelOffsetTemp1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
SOC_ADC offset in absolute reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_abs_offset_temp1(&mut self) -> SocAdcAbsOffsetTemp1W<SocAdcOffsetIntSpec> {
        SocAdcAbsOffsetTemp1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> Reserved8W<SocAdcOffsetIntSpec> {
        Reserved8W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
SOC_ADC offset in relative reference mode at temperature 1 (30C). Signed 8-bit number. Calculated in production test.."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_rel_offset_temp1(&mut self) -> SocAdcRelOffsetTemp1W<SocAdcOffsetIntSpec> {
        SocAdcRelOffsetTemp1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved24(&mut self) -> Reserved24W<SocAdcOffsetIntSpec> {
        Reserved24W::new(self, 24)
    }
}
#[doc = "AUX_ADC Temperature Offsets in Absolute Reference Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_adc_offset_int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_adc_offset_int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocAdcOffsetIntSpec;
impl crate::RegisterSpec for SocAdcOffsetIntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_adc_offset_int::R`](R) reader structure"]
impl crate::Readable for SocAdcOffsetIntSpec {}
#[doc = "`write(|w| ..)` method takes [`soc_adc_offset_int::W`](W) writer structure"]
impl crate::Writable for SocAdcOffsetIntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_ADC_OFFSET_INT to value 0"]
impl crate::Resettable for SocAdcOffsetIntSpec {
    const RESET_VALUE: u32 = 0;
}
