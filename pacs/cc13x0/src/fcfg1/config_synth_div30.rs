#[doc = "Register `CONFIG_SYNTH_DIV30` reader"]
pub type R = crate::R<ConfigSynthDiv30Spec>;
#[doc = "Register `CONFIG_SYNTH_DIV30` writer"]
pub type W = crate::W<ConfigSynthDiv30Spec>;
#[doc = "Field `SLDO_TRIM_OUTPUT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SldoTrimOutputR = crate::FieldReader;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LdovcoTrimOutputR = crate::FieldReader;
#[doc = "Field `RFC_MDM_DEMIQMC0` reader - 27:12\\]
Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
pub type RfcMdmDemiqmc0R = crate::FieldReader<u16>;
#[doc = "Field `DISABLE_CORNER_CAP` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DisableCornerCapR = crate::BitReader;
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
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn disable_corner_cap(&self) -> DisableCornerCapR {
        DisableCornerCapR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSynthDiv30Spec;
impl crate::RegisterSpec for ConfigSynthDiv30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_synth_div30::R`](R) reader structure"]
impl crate::Readable for ConfigSynthDiv30Spec {}
#[doc = "`write(|w| ..)` method takes [`config_synth_div30::W`](W) writer structure"]
impl crate::Writable for ConfigSynthDiv30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_SYNTH_DIV30 to value 0xe000_0000"]
impl crate::Resettable for ConfigSynthDiv30Spec {
    const RESET_VALUE: u32 = 0xe000_0000;
}
