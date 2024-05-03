#[doc = "Register `DAC_CAL3` reader"]
pub type R = crate::R<DacCal3Spec>;
#[doc = "Register `DAC_CAL3` writer"]
pub type W = crate::W<DacCal3Spec>;
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C1` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalVddsC1R = crate::FieldReader<u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C1` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalVddsC1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C2` reader - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalVddsC2R = crate::FieldReader<u16>;
#[doc = "Field `SOC_DAC_VOUT_CAL_VDDS_C2` writer - 31:16\\]
Internal. Only to be used through TI provided API."]
pub type SocDacVoutCalVddsC2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_vdds_c1(&self) -> SocDacVoutCalVddsC1R {
        SocDacVoutCalVddsC1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn soc_dac_vout_cal_vdds_c2(&self) -> SocDacVoutCalVddsC2R {
        SocDacVoutCalVddsC2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_dac_vout_cal_vdds_c1(&mut self) -> SocDacVoutCalVddsC1W<DacCal3Spec> {
        SocDacVoutCalVddsC1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn soc_dac_vout_cal_vdds_c2(&mut self) -> SocDacVoutCalVddsC2W<DacCal3Spec> {
        SocDacVoutCalVddsC2W::new(self, 16)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dac_cal3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dac_cal3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DacCal3Spec;
impl crate::RegisterSpec for DacCal3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dac_cal3::R`](R) reader structure"]
impl crate::Readable for DacCal3Spec {}
#[doc = "`write(|w| ..)` method takes [`dac_cal3::W`](W) writer structure"]
impl crate::Writable for DacCal3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAC_CAL3 to value 0"]
impl crate::Resettable for DacCal3Spec {
    const RESET_VALUE: u32 = 0;
}
