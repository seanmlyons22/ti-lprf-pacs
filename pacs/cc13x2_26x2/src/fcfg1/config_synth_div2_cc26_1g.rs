#[doc = "Register `CONFIG_SYNTH_DIV2_CC26_1G` reader"]
pub type R = crate::R<ConfigSynthDiv2Cc26_1gSpec>;
#[doc = "Register `CONFIG_SYNTH_DIV2_CC26_1G` writer"]
pub type W = crate::W<ConfigSynthDiv2Cc26_1gSpec>;
#[doc = "Field `RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type RfcMdmDemiqmc0TrimcompleteNR = crate::BitReader;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LdovcoTrimOutputR = crate::FieldReader;
#[doc = "Field `RFC_MDM_DEMIQMC0` reader - 27:12\\]
Internal. Only to be used through TI provided API."]
pub type RfcMdmDemiqmc0R = crate::FieldReader<u16>;
#[doc = "Field `MIN_ALLOWED_RTRIM` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type MinAllowedRtrimR = crate::FieldReader;
impl R {
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&self) -> RfcMdmDemiqmc0TrimcompleteNR {
        RfcMdmDemiqmc0TrimcompleteNR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ldovco_trim_output(&self) -> LdovcoTrimOutputR {
        LdovcoTrimOutputR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&self) -> RfcMdmDemiqmc0R {
        RfcMdmDemiqmc0R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim(&self) -> MinAllowedRtrimR {
        MinAllowedRtrimR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div2_cc26_1g::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div2_cc26_1g::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSynthDiv2Cc26_1gSpec;
impl crate::RegisterSpec for ConfigSynthDiv2Cc26_1gSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_synth_div2_cc26_1g::R`](R) reader structure"]
impl crate::Readable for ConfigSynthDiv2Cc26_1gSpec {}
#[doc = "`write(|w| ..)` method takes [`config_synth_div2_cc26_1g::W`](W) writer structure"]
impl crate::Writable for ConfigSynthDiv2Cc26_1gSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_SYNTH_DIV2_CC26_1G to value 0x1f"]
impl crate::Resettable for ConfigSynthDiv2Cc26_1gSpec {
    const RESET_VALUE: u32 = 0x1f;
}