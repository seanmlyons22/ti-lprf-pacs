#[doc = "Register `SOC_ADC_REF_TRIM_AND_OFFSET_EXT` reader"]
pub type R = crate::R<SocAdcRefTrimAndOffsetExtSpec>;
#[doc = "Register `SOC_ADC_REF_TRIM_AND_OFFSET_EXT` writer"]
pub type W = crate::W<SocAdcRefTrimAndOffsetExtSpec>;
#[doc = "Field `SOC_ADC_REF_VOLTAGE_TRIM_TEMP1` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SocAdcRefVoltageTrimTemp1R = crate::FieldReader;
#[doc = "Field `SOC_ADC_REF_VOLTAGE_TRIM_TEMP1` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SocAdcRefVoltageTrimTemp1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type Reserved6W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_adc_ref_voltage_trim_temp1(&self) -> SocAdcRefVoltageTrimTemp1R {
        SocAdcRefVoltageTrimTemp1R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_adc_ref_voltage_trim_temp1(
        &mut self,
    ) -> SocAdcRefVoltageTrimTemp1W<SocAdcRefTrimAndOffsetExtSpec> {
        SocAdcRefVoltageTrimTemp1W::new(self, 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> Reserved6W<SocAdcRefTrimAndOffsetExtSpec> {
        Reserved6W::new(self, 6)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_adc_ref_trim_and_offset_ext::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_adc_ref_trim_and_offset_ext::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocAdcRefTrimAndOffsetExtSpec;
impl crate::RegisterSpec for SocAdcRefTrimAndOffsetExtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_adc_ref_trim_and_offset_ext::R`](R) reader structure"]
impl crate::Readable for SocAdcRefTrimAndOffsetExtSpec {}
#[doc = "`write(|w| ..)` method takes [`soc_adc_ref_trim_and_offset_ext::W`](W) writer structure"]
impl crate::Writable for SocAdcRefTrimAndOffsetExtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_ADC_REF_TRIM_AND_OFFSET_EXT to value 0xc080"]
impl crate::Resettable for SocAdcRefTrimAndOffsetExtSpec {
    const RESET_VALUE: u32 = 0xc080;
}
