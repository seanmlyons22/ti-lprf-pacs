#[doc = "Register `CONFIG_MISC_ADC` reader"]
pub type R = crate::R<ConfigMiscAdcSpec>;
#[doc = "Register `CONFIG_MISC_ADC` writer"]
pub type W = crate::W<ConfigMiscAdcSpec>;
#[doc = "Field `DACTRIM` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type DactrimR = crate::FieldReader;
#[doc = "Field `QUANTCTLTHRES` reader - 8:6\\]
Internal. Only to be used through TI provided API."]
pub type QuantctlthresR = crate::FieldReader;
#[doc = "Field `RSSI_OFFSET` reader - 16:9\\]
Internal. Only to be used through TI provided API."]
pub type RssiOffsetR = crate::FieldReader;
#[doc = "Field `RSSITRIMCOMPLETE_N` reader - 17:17\\]
Internal. Only to be used through TI provided API."]
pub type RssitrimcompleteNR = crate::BitReader;
#[doc = "Field `MIN_ALLOWED_RTRIM` reader - 21:18\\]
Internal. Only to be used through TI provided API."]
pub type MinAllowedRtrimR = crate::FieldReader;
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
    #[doc = "Bit 17 - 17:17\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssitrimcomplete_n(&self) -> RssitrimcompleteNR {
        RssitrimcompleteNR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - 21:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim(&self) -> MinAllowedRtrimR {
        MinAllowedRtrimR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_misc_adc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_misc_adc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigMiscAdcSpec;
impl crate::RegisterSpec for ConfigMiscAdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_misc_adc::R`](R) reader structure"]
impl crate::Readable for ConfigMiscAdcSpec {}
#[doc = "`write(|w| ..)` method takes [`config_misc_adc::W`](W) writer structure"]
impl crate::Writable for ConfigMiscAdcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_MISC_ADC to value 0xffc0_014d"]
impl crate::Resettable for ConfigMiscAdcSpec {
    const RESET_VALUE: u32 = 0xffc0_014d;
}
