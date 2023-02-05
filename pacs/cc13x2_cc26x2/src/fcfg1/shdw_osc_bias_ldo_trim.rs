#[doc = "Register `SHDW_OSC_BIAS_LDO_TRIM` reader"]
pub struct R(crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHDW_OSC_BIAS_LDO_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHDW_OSC_BIAS_LDO_TRIM` writer"]
pub struct W(crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>;
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
impl From<crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHDW_OSC_BIAS_LDO_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSCHF_CTRIM` reader - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_CTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSCHF_CTRIM` writer - 7:0\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_CTRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_OSC_BIAS_LDO_TRIM_SPEC, u8, u8, 8, O>;
#[doc = "Field `VTRIM_COARSE` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_COARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTRIM_COARSE` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_COARSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_OSC_BIAS_LDO_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `VTRIM_DIG` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_DIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTRIM_DIG` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_DIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_OSC_BIAS_LDO_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `ITRIM_DIG_LDO` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type ITRIM_DIG_LDO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ITRIM_DIG_LDO` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type ITRIM_DIG_LDO_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_OSC_BIAS_LDO_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `TRIMIREF` reader - 22:18\\]
Internal. Only to be used through TI provided API."]
pub type TRIMIREF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMIREF` writer - 22:18\\]
Internal. Only to be used through TI provided API."]
pub type TRIMIREF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_OSC_BIAS_LDO_TRIM_SPEC, u8, u8, 5, O>;
#[doc = "Field `TRIMMAG` reader - 26:23\\]
Internal. Only to be used through TI provided API."]
pub type TRIMMAG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIMMAG` writer - 26:23\\]
Internal. Only to be used through TI provided API."]
pub type TRIMMAG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHDW_OSC_BIAS_LDO_TRIM_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_ctrim(&self) -> RCOSCHF_CTRIM_R {
        RCOSCHF_CTRIM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_coarse(&self) -> VTRIM_COARSE_R {
        VTRIM_COARSE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_dig(&self) -> VTRIM_DIG_R {
        VTRIM_DIG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn itrim_dig_ldo(&self) -> ITRIM_DIG_LDO_R {
        ITRIM_DIG_LDO_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimiref(&self) -> TRIMIREF_R {
        TRIMIREF_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:26 - 26:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn trimmag(&self) -> TRIMMAG_R {
        TRIMMAG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschf_ctrim(&mut self) -> RCOSCHF_CTRIM_W<0> {
        RCOSCHF_CTRIM_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtrim_coarse(&mut self) -> VTRIM_COARSE_W<8> {
        VTRIM_COARSE_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtrim_dig(&mut self) -> VTRIM_DIG_W<12> {
        VTRIM_DIG_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn itrim_dig_ldo(&mut self) -> ITRIM_DIG_LDO_W<16> {
        ITRIM_DIG_LDO_W::new(self)
    }
    #[doc = "Bits 18:22 - 22:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimiref(&mut self) -> TRIMIREF_W<18> {
        TRIMIREF_W::new(self)
    }
    #[doc = "Bits 23:26 - 26:23\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn trimmag(&mut self) -> TRIMMAG_W<23> {
        TRIMMAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shdw_osc_bias_ldo_trim](index.html) module"]
pub struct SHDW_OSC_BIAS_LDO_TRIM_SPEC;
impl crate::RegisterSpec for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shdw_osc_bias_ldo_trim::R](R) reader structure"]
impl crate::Readable for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shdw_osc_bias_ldo_trim::W](W) writer structure"]
impl crate::Writable for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHDW_OSC_BIAS_LDO_TRIM to value 0"]
impl crate::Resettable for SHDW_OSC_BIAS_LDO_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
