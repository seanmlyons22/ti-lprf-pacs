#[doc = "Register `DAC_CAL2` reader"]
pub type R = crate::R<DacCal2Spec>;
#[doc = "Register `DAC_CAL2` writer"]
pub type W = crate::W<DacCal2Spec>;
#[doc = "Field `SOC_DAC_VOUT_CAL_ADCREF_C1` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalAdcrefC1R = crate::FieldReader<u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_ADCREF_C1` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalAdcrefC1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_ADCREF_C2` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalAdcrefC2R = crate::FieldReader<u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_ADCREF_C2` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalAdcrefC2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_adcref_c1(&self) -> SocDacVoutCalAdcrefC1R {
        SocDacVoutCalAdcrefC1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_adcref_c2(&self) -> SocDacVoutCalAdcrefC2R {
        SocDacVoutCalAdcrefC2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_dac_vout_cal_adcref_c1(&mut self) -> SocDacVoutCalAdcrefC1W<DacCal2Spec> {
        SocDacVoutCalAdcrefC1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_dac_vout_cal_adcref_c2(&mut self) -> SocDacVoutCalAdcrefC2W<DacCal2Spec> {
        SocDacVoutCalAdcrefC2W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cal2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cal2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCal2Spec;
impl crate::RegisterSpec for DacCal2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cal2::R`](R) reader structure"]
impl crate::Readable for DacCal2Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_cal2::W`](W) writer structure"]
impl crate::Writable for DacCal2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_CAL2 to value 0"]
impl crate::Resettable for DacCal2Spec {
    const RESET_VALUE: u32 = 0;
}
