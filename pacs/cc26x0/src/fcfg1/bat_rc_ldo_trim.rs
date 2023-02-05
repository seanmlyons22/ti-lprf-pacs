#[doc = "Register `BAT_RC_LDO_TRIM` reader"]
pub struct R(crate::R<BAT_RC_LDO_TRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BAT_RC_LDO_TRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BAT_RC_LDO_TRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BAT_RC_LDO_TRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BAT_RC_LDO_TRIM` writer"]
pub struct W(crate::W<BAT_RC_LDO_TRIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BAT_RC_LDO_TRIM_SPEC>;
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
impl From<crate::W<BAT_RC_LDO_TRIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BAT_RC_LDO_TRIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEASUREPER` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type MEASUREPER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MEASUREPER` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type MEASUREPER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 2, O>;
#[doc = "Field `RESERVED1` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 6, O>;
#[doc = "Field `RCOSCHF_ITUNE_TRIM` reader - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_ITUNE_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSCHF_ITUNE_TRIM` writer - 11:8\\]
Internal. Only to be used through TI provided API."]
pub type RCOSCHF_ITUNE_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED2` reader - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 15:12\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `VTRIM_UDIG` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_UDIG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTRIM_UDIG` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_UDIG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED3` reader - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 23:20\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `VTRIM_BOD` reader - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_BOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VTRIM_BOD` writer - 27:24\\]
Internal. Only to be used through TI provided API."]
pub type VTRIM_BOD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED4` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BAT_RC_LDO_TRIM_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn measureper(&self) -> MEASUREPER_R {
        MEASUREPER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcoschf_itune_trim(&self) -> RCOSCHF_ITUNE_TRIM_R {
        RCOSCHF_ITUNE_TRIM_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_udig(&self) -> VTRIM_UDIG_R {
        VTRIM_UDIG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn vtrim_bod(&self) -> VTRIM_BOD_R {
        VTRIM_BOD_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn measureper(&mut self) -> MEASUREPER_W<0> {
        MEASUREPER_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<2> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcoschf_itune_trim(&mut self) -> RCOSCHF_ITUNE_TRIM_W<8> {
        RCOSCHF_ITUNE_TRIM_W::new(self)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<12> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtrim_udig(&mut self) -> VTRIM_UDIG_W<16> {
        VTRIM_UDIG_W::new(self)
    }
    #[doc = "Bits 20:23 - 23:20\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<20> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn vtrim_bod(&mut self) -> VTRIM_BOD_W<24> {
        VTRIM_BOD_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<28> {
        RESERVED4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bat_rc_ldo_trim](index.html) module"]
pub struct BAT_RC_LDO_TRIM_SPEC;
impl crate::RegisterSpec for BAT_RC_LDO_TRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bat_rc_ldo_trim::R](R) reader structure"]
impl crate::Readable for BAT_RC_LDO_TRIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bat_rc_ldo_trim::W](W) writer structure"]
impl crate::Writable for BAT_RC_LDO_TRIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BAT_RC_LDO_TRIM to value 0xf0f0_f0fc"]
impl crate::Resettable for BAT_RC_LDO_TRIM_SPEC {
    const RESET_VALUE: Self::Ux = 0xf0f0_f0fc;
}
