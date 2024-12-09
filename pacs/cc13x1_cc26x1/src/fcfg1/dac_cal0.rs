#[doc = "Register `DAC_CAL0` reader"]
pub type R = crate::R<DacCal0Spec>;
#[doc = "Register `DAC_CAL0` writer"]
pub type W = crate::W<DacCal0Spec>;
#[doc = "Field `SOC_DAC_VOUT_CAL_DECOUPLE_C1` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalDecoupleC1R = crate::FieldReader<u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_DECOUPLE_C2` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalDecoupleC2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_decouple_c1(&self) -> SocDacVoutCalDecoupleC1R {
        SocDacVoutCalDecoupleC1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_decouple_c2(&self) -> SocDacVoutCalDecoupleC2R {
        SocDacVoutCalDecoupleC2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cal0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cal0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCal0Spec;
impl crate::RegisterSpec for DacCal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cal0::R`](R) reader structure"]
impl crate::Readable for DacCal0Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_cal0::W`](W) writer structure"]
impl crate::Writable for DacCal0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_CAL0 to value 0"]
impl crate::Resettable for DacCal0Spec {
    const RESET_VALUE: u32 = 0;
}
