#[doc = "Register `CONFIG_OSC_TOP` reader"]
pub type R = crate::R<ConfigOscTopSpec>;
#[doc = "Register `CONFIG_OSC_TOP` writer"]
pub type W = crate::W<ConfigOscTopSpec>;
#[doc = "Field `RCOSCLF_RTUNE_TRIM` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type RcosclfRtuneTrimR = crate::FieldReader;
#[doc = "Field `RCOSCLF_CTUNE_TRIM` reader - 9:2\\]
Internal. Only to be used through TI provided API."]
pub type RcosclfCtuneTrimR = crate::FieldReader;
#[doc = "Field `XOSC_HF_COLUMN_Q12` reader - 25:10\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfColumnQ12R = crate::FieldReader<u16>;
#[doc = "Field `XOSC_HF_ROW_Q12` reader - 29:26\\]
Internal. Only to be used through TI provided API."]
pub type XoscHfRowQ12R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_rtune_trim(&self) -> RcosclfRtuneTrimR {
        RcosclfRtuneTrimR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - 9:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosclf_ctune_trim(&self) -> RcosclfCtuneTrimR {
        RcosclfCtuneTrimR::new(((self.bits >> 2) & 0xff) as u8)
    }
    #[doc = "Bits 10:25 - 25:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XoscHfColumnQ12R {
        XoscHfColumnQ12R::new(((self.bits >> 10) & 0xffff) as u16)
    }
    #[doc = "Bits 26:29 - 29:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XoscHfRowQ12R {
        XoscHfRowQ12R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_osc_top::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_osc_top::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigOscTopSpec;
impl crate::RegisterSpec for ConfigOscTopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_osc_top::R`](R) reader structure"]
impl crate::Readable for ConfigOscTopSpec {}
#[doc = "`write(|w| ..)` method takes [`config_osc_top::W`](W) writer structure"]
impl crate::Writable for ConfigOscTopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_OSC_TOP to value 0xdc07_fc00"]
impl crate::Resettable for ConfigOscTopSpec {
    const RESET_VALUE: u32 = 0xdc07_fc00;
}
