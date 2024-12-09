#[doc = "Register `CONFIG_CC26_FE` reader"]
pub type R = crate::R<ConfigCc26FeSpec>;
#[doc = "Register `CONFIG_CC26_FE` writer"]
pub type W = crate::W<ConfigCc26FeSpec>;
#[doc = "Field `RSSI_OFFSET` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RssiOffsetR = crate::FieldReader;
#[doc = "Field `RSSITRIMCOMPLETE_N` reader - 12:12\\]
Internal. Only to be used through TI provided API."]
pub type RssitrimcompleteNR = crate::BitReader;
#[doc = "Field `PATRIMCOMPLETE_N` reader - 13:13\\]
Internal. Only to be used through TI provided API."]
pub type PatrimcompleteNR = crate::BitReader;
#[doc = "Field `CTL_PA0_TRIM` reader - 18:14\\]
Internal. Only to be used through TI provided API."]
pub type CtlPa0TrimR = crate::FieldReader;
#[doc = "Field `IFAMP_TRIM` reader - 23:19\\]
Internal. Only to be used through TI provided API."]
pub type IfampTrimR = crate::FieldReader;
#[doc = "Field `LNA_IB` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type LnaIbR = crate::FieldReader;
#[doc = "Field `IFAMP_IB` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type IfampIbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssi_offset(&self) -> RssiOffsetR {
        RssiOffsetR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - 12:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rssitrimcomplete_n(&self) -> RssitrimcompleteNR {
        RssitrimcompleteNR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn patrimcomplete_n(&self) -> PatrimcompleteNR {
        PatrimcompleteNR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:18 - 18:14\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ctl_pa0_trim(&self) -> CtlPa0TrimR {
        CtlPa0TrimR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - 23:19\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_trim(&self) -> IfampTrimR {
        IfampTrimR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lna_ib(&self) -> LnaIbR {
        LnaIbR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ifamp_ib(&self) -> IfampIbR {
        IfampIbR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_cc26_fe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_cc26_fe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigCc26FeSpec;
impl crate::RegisterSpec for ConfigCc26FeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_cc26_fe::R`](R) reader structure"]
impl crate::Readable for ConfigCc26FeSpec {}
#[doc = "`write(|w| ..)` method takes [`config_cc26_fe::W`](W) writer structure"]
impl crate::Writable for ConfigCc26FeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_CC26_FE to value 0x7000_0f00"]
impl crate::Resettable for ConfigCc26FeSpec {
    const RESET_VALUE: u32 = 0x7000_0f00;
}
