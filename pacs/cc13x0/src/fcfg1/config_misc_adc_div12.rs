#[doc = "Register `CONFIG_MISC_ADC_DIV12` reader"]
pub type R = crate::R<ConfigMiscAdcDiv12Spec>;
#[doc = "Register `CONFIG_MISC_ADC_DIV12` writer"]
pub type W = crate::W<ConfigMiscAdcDiv12Spec>;
#[doc = "Field `DACTRIM` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DactrimR = crate::FieldReader;
#[doc = "Field `DACTRIM` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DactrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `QUANTCTLTHRES` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QuantctlthresR = crate::FieldReader;
#[doc = "Field `QUANTCTLTHRES` writer - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QuantctlthresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSSI_OFFSET` reader - 16:9\\]
Internal. Only to be used through TI provided API."]
pub type RssiOffsetR = crate::FieldReader;
#[doc = "Field `RSSI_OFFSET` writer - 16:9\\]
Internal. Only to be used through TI provided API."]
pub type RssiOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MIN_ALLOWED_RTRIM` reader - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type MinAllowedRtrimR = crate::FieldReader;
#[doc = "Field `MIN_ALLOWED_RTRIM` writer - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type MinAllowedRtrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RESERVED1` reader - 31:22\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `RESERVED1` writer - 31:22\\]
Internal. Only to be used through TI provided API."]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn dactrim(&self) -> DactrimR {
        DactrimR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn quantctlthres(&self) -> QuantctlthresR {
        QuantctlthresR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:16 - 16:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RssiOffsetR {
        RssiOffsetR::new(((self.bits >> 9) & 0xff) as u8)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim(&self) -> MinAllowedRtrimR {
        MinAllowedRtrimR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn dactrim(&mut self) -> DactrimW<ConfigMiscAdcDiv12Spec> {
        DactrimW::new(self, 0)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn quantctlthres(&mut self) -> QuantctlthresW<ConfigMiscAdcDiv12Spec> {
        QuantctlthresW::new(self, 6)
    }
    #[doc = "Bits 9:16 - 16:9\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rssi_offset(&mut self) -> RssiOffsetW<ConfigMiscAdcDiv12Spec> {
        RssiOffsetW::new(self, 9)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn min_allowed_rtrim(&mut self) -> MinAllowedRtrimW<ConfigMiscAdcDiv12Spec> {
        MinAllowedRtrimW::new(self, 18)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<ConfigMiscAdcDiv12Spec> {
        Reserved1W::new(self, 22)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_misc_adc_div12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_misc_adc_div12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigMiscAdcDiv12Spec;
impl crate::RegisterSpec for ConfigMiscAdcDiv12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_misc_adc_div12::R`](R) reader structure"]
impl crate::Readable for ConfigMiscAdcDiv12Spec {}
#[doc = "`write(|w| ..)` method takes [`config_misc_adc_div12::W`](W) writer structure"]
impl crate::Writable for ConfigMiscAdcDiv12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_MISC_ADC_DIV12 to value 0xffc2_014d"]
impl crate::Resettable for ConfigMiscAdcDiv12Spec {
    const RESET_VALUE: u32 = 0xffc2_014d;
}
