#[doc = "Register `CONFIG_SYNTH_DIV12` reader"]
pub type R = crate::R<ConfigSynthDiv12Spec>;
#[doc = "Register `CONFIG_SYNTH_DIV12` writer"]
pub type W = crate::W<ConfigSynthDiv12Spec>;
#[doc = "Field `SLDO_TRIM_OUTPUT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SldoTrimOutputR = crate::FieldReader;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LdovcoTrimOutputR = crate::FieldReader;
#[doc = "Field `RFC_MDM_DEMIQMC0` reader - 27:12\\]
Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
pub type RfcMdmDemiqmc0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn sldo_trim_output(&self) -> SldoTrimOutputR {
        SldoTrimOutputR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ldovco_trim_output(&self) -> LdovcoTrimOutputR {
        LdovcoTrimOutputR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&self) -> RfcMdmDemiqmc0R {
        RfcMdmDemiqmc0R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSynthDiv12Spec;
impl crate::RegisterSpec for ConfigSynthDiv12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_synth_div12::R`](R) reader structure"]
impl crate::Readable for ConfigSynthDiv12Spec {}
#[doc = "`write(|w| ..)` method takes [`config_synth_div12::W`](W) writer structure"]
impl crate::Writable for ConfigSynthDiv12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_SYNTH_DIV12 to value 0xffff_ffff"]
impl crate::Resettable for ConfigSynthDiv12Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
