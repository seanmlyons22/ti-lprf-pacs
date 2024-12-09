#[doc = "Register `CONFIG_SYNTH` reader"]
pub type R = crate::R<ConfigSynthSpec>;
#[doc = "Register `CONFIG_SYNTH` writer"]
pub type W = crate::W<ConfigSynthSpec>;
#[doc = "Field `SLDO_TRIM_OUTPUT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SldoTrimOutputR = crate::FieldReader;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LdovcoTrimOutputR = crate::FieldReader;
#[doc = "Field `RFC_MDM_DEMIQMC0` reader - 27:12\\]
Internal. Only to be used through TI provided API."]
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
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&self) -> RfcMdmDemiqmc0R {
        RfcMdmDemiqmc0R::new(((self.bits >> 12) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSynthSpec;
impl crate::RegisterSpec for ConfigSynthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_synth::R`](R) reader structure"]
impl crate::Readable for ConfigSynthSpec {}
#[doc = "`write(|w| ..)` method takes [`config_synth::W`](W) writer structure"]
impl crate::Writable for ConfigSynthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_SYNTH to value 0xffff_f000"]
impl crate::Resettable for ConfigSynthSpec {
    const RESET_VALUE: u32 = 0xffff_f000;
}
