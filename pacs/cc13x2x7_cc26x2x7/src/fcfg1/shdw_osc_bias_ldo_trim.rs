#[doc = "Register `SHDW_OSC_BIAS_LDO_TRIM` reader"]
pub type R = crate::R<ShdwOscBiasLdoTrimSpec>;
#[doc = "Register `SHDW_OSC_BIAS_LDO_TRIM` writer"]
pub type W = crate::W<ShdwOscBiasLdoTrimSpec>;
#[doc = "Field `RCOSCHF_CTRIM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RcoschfCtrimR = crate::FieldReader;
#[doc = "Field `VTRIM_COARSE` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type VtrimCoarseR = crate::FieldReader;
#[doc = "Field `VTRIM_DIG` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VtrimDigR = crate::FieldReader;
#[doc = "Field `ITRIM_DIG_LDO` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type ItrimDigLdoR = crate::FieldReader;
#[doc = "Field `TRIMIREF` reader - 22:18\\]
Internal. Only to be used through TI provided API."]
pub type TrimirefR = crate::FieldReader;
#[doc = "Field `TRIMMAG` reader - 26:23\\]
Internal. Only to be used through TI provided API."]
pub type TrimmagR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RcoschfCtrimR {
        RcoschfCtrimR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_coarse(&self) -> VtrimCoarseR {
        VtrimCoarseR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_dig(&self) -> VtrimDigR {
        VtrimDigR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_dig_ldo(&self) -> ItrimDigLdoR {
        ItrimDigLdoR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimiref(&self) -> TrimirefR {
        TrimirefR::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:26 - 26:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimmag(&self) -> TrimmagR {
        TrimmagR::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl W {}
#[doc = "Internal. Only to be used through TI provided API.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shdw_osc_bias_ldo_trim::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shdw_osc_bias_ldo_trim::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShdwOscBiasLdoTrimSpec;
impl crate::RegisterSpec for ShdwOscBiasLdoTrimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shdw_osc_bias_ldo_trim::R`](R) reader structure"]
impl crate::Readable for ShdwOscBiasLdoTrimSpec {}
#[doc = "`write(|w| ..)` method takes [`shdw_osc_bias_ldo_trim::W`](W) writer structure"]
impl crate::Writable for ShdwOscBiasLdoTrimSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHDW_OSC_BIAS_LDO_TRIM to value 0"]
impl crate::Resettable for ShdwOscBiasLdoTrimSpec {
    const RESET_VALUE: u32 = 0;
}
