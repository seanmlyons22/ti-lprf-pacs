#[doc = "Register `CONFIG_SYNTH_DIV10` reader"]
pub type R = crate::R<ConfigSynthDiv10Spec>;
#[doc = "Register `CONFIG_SYNTH_DIV10` writer"]
pub type W = crate::W<ConfigSynthDiv10Spec>;
#[doc = "Field `SLDO_TRIM_OUTPUT` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SldoTrimOutputR = crate::FieldReader;
#[doc = "Field `SLDO_TRIM_OUTPUT` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type SldoTrimOutputW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LdovcoTrimOutputR = crate::FieldReader;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` writer - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LdovcoTrimOutputW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RFC_MDM_DEMIQMC0` reader - 27:12\\]
Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
pub type RfcMdmDemiqmc0R = crate::FieldReader<u16>;
#[doc = "Field `RFC_MDM_DEMIQMC0` writer - 27:12\\]
Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
pub type RfcMdmDemiqmc0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DISABLE_CORNER_CAP` reader - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DisableCornerCapR = crate::BitReader;
#[doc = "Field `DISABLE_CORNER_CAP` writer - 28:28\\]
Internal. Only to be used through TI provided API."]
pub type DisableCornerCapW<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn sldo_trim_output(&mut self) -> SldoTrimOutputW<ConfigSynthDiv10Spec> {
        SldoTrimOutputW::new(self, 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ldovco_trim_output(&mut self) -> LdovcoTrimOutputW<ConfigSynthDiv10Spec> {
        LdovcoTrimOutputW::new(self, 6)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Trim value for RF Core. Value is read by RF Core ROM FW during RF Core initialization."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_mdm_demiqmc0(&mut self) -> RfcMdmDemiqmc0W<ConfigSynthDiv10Spec> {
        RfcMdmDemiqmc0W::new(self, 12)
    }
    #[doc = "Bit 28 - 28:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn disable_corner_cap(&mut self) -> DisableCornerCapW<ConfigSynthDiv10Spec> {
        DisableCornerCapW::new(self, 28)
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config_synth_div10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config_synth_div10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSynthDiv10Spec;
impl crate::RegisterSpec for ConfigSynthDiv10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config_synth_div10::R`](R) reader structure"]
impl crate::Readable for ConfigSynthDiv10Spec {}
#[doc = "`write(|w| ..)` method takes [`config_synth_div10::W`](W) writer structure"]
impl crate::Writable for ConfigSynthDiv10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG_SYNTH_DIV10 to value 0xe000_0000"]
impl crate::Resettable for ConfigSynthDiv10Spec {
    const RESET_VALUE: u32 = 0xe000_0000;
}
