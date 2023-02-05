#[doc = "Register `CONFIG_SYNTH_DIV2_CC13_1G` reader"]
pub struct R(crate::R<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONFIG_SYNTH_DIV2_CC13_1G` writer"]
pub struct W(crate::W<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_SYNTH_DIV2_CC13_1G_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R = crate::BitReader<bool>;
#[doc = "Field `RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_SYNTH_DIV2_CC13_1G_SPEC, bool, O>;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` reader - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LDOVCO_TRIM_OUTPUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LDOVCO_TRIM_OUTPUT` writer - 11:6\\]
Internal. Only to be used through TI provided API."]
pub type LDOVCO_TRIM_OUTPUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SYNTH_DIV2_CC13_1G_SPEC, u8, u8, 6, O>;
#[doc = "Field `RFC_MDM_DEMIQMC0` reader - 27:12\\]
Internal. Only to be used through TI provided API."]
pub type RFC_MDM_DEMIQMC0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RFC_MDM_DEMIQMC0` writer - 27:12\\]
Internal. Only to be used through TI provided API."]
pub type RFC_MDM_DEMIQMC0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SYNTH_DIV2_CC13_1G_SPEC, u16, u16, 16, O>;
#[doc = "Field `MIN_ALLOWED_RTRIM` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type MIN_ALLOWED_RTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MIN_ALLOWED_RTRIM` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type MIN_ALLOWED_RTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CONFIG_SYNTH_DIV2_CC13_1G_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&self) -> RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R {
        RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ldovco_trim_output(&self) -> LDOVCO_TRIM_OUTPUT_R {
        LDOVCO_TRIM_OUTPUT_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rfc_mdm_demiqmc0(&self) -> RFC_MDM_DEMIQMC0_R {
        RFC_MDM_DEMIQMC0_R::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn min_allowed_rtrim(&self) -> MIN_ALLOWED_RTRIM_R {
        MIN_ALLOWED_RTRIM_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_mdm_demiqmc0_trimcomplete_n(&mut self) -> RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_W<5> {
        RFC_MDM_DEMIQMC0_TRIMCOMPLETE_N_W::new(self)
    }
    #[doc = "Bits 6:11 - 11:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ldovco_trim_output(&mut self) -> LDOVCO_TRIM_OUTPUT_W<6> {
        LDOVCO_TRIM_OUTPUT_W::new(self)
    }
    #[doc = "Bits 12:27 - 27:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rfc_mdm_demiqmc0(&mut self) -> RFC_MDM_DEMIQMC0_W<12> {
        RFC_MDM_DEMIQMC0_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn min_allowed_rtrim(&mut self) -> MIN_ALLOWED_RTRIM_W<28> {
        MIN_ALLOWED_RTRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_synth_div2_cc13_1g](index.html) module"]
pub struct CONFIG_SYNTH_DIV2_CC13_1G_SPEC;
impl crate::RegisterSpec for CONFIG_SYNTH_DIV2_CC13_1G_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [config_synth_div2_cc13_1g::R](R) reader structure"]
impl crate::Readable for CONFIG_SYNTH_DIV2_CC13_1G_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [config_synth_div2_cc13_1g::W](W) writer structure"]
impl crate::Writable for CONFIG_SYNTH_DIV2_CC13_1G_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_SYNTH_DIV2_CC13_1G to value 0x1f"]
impl crate::Resettable for CONFIG_SYNTH_DIV2_CC13_1G_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
