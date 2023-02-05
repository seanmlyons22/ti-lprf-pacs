#[doc = "Register `WEPROT_AUX_BY1` reader"]
pub struct R(crate::R<WEPROT_AUX_BY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WEPROT_AUX_BY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WEPROT_AUX_BY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WEPROT_AUX_BY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WEPROT_AUX_BY1` writer"]
pub struct W(crate::W<WEPROT_AUX_BY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WEPROT_AUX_BY1_SPEC>;
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
impl From<crate::W<WEPROT_AUX_BY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WEPROT_AUX_BY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WEPROT_B0_CCFG_BY1` reader - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_CCFG_BY1_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_B0_CCFG_BY1` writer - 0:0\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_CCFG_BY1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WEPROT_AUX_BY1_SPEC, bool, O>;
#[doc = "Field `WEPROT_B1_FCFG_BY1` reader - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B1_FCFG_BY1_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_B1_FCFG_BY1` writer - 1:1\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B1_FCFG_BY1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WEPROT_AUX_BY1_SPEC, bool, O>;
#[doc = "Field `WEPROT_B0_TRIM_BY1` reader - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_TRIM_BY1_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_B0_TRIM_BY1` writer - 2:2\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_TRIM_BY1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WEPROT_AUX_BY1_SPEC, bool, O>;
#[doc = "Field `WEPROT_B1_TRIM_BY1` reader - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B1_TRIM_BY1_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_B1_TRIM_BY1` writer - 3:3\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B1_TRIM_BY1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WEPROT_AUX_BY1_SPEC, bool, O>;
#[doc = "Field `WEPROT_B0_ENGR_BY1` reader - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_ENGR_BY1_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_B0_ENGR_BY1` writer - 4:4\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B0_ENGR_BY1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WEPROT_AUX_BY1_SPEC, bool, O>;
#[doc = "Field `WEPROT_B1_ENGR_BY1` reader - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B1_ENGR_BY1_R = crate::BitReader<bool>;
#[doc = "Field `WEPROT_B1_ENGR_BY1` writer - 5:5\\]
Internal. Only to be used through TI provided API."]
pub type WEPROT_B1_ENGR_BY1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, WEPROT_AUX_BY1_SPEC, bool, O>;
#[doc = "Field `RESERVED6` reader - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED6` writer - 31:6\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, WEPROT_AUX_BY1_SPEC, u32, u32, 26, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_ccfg_by1(&self) -> WEPROT_B0_CCFG_BY1_R {
        WEPROT_B0_CCFG_BY1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b1_fcfg_by1(&self) -> WEPROT_B1_FCFG_BY1_R {
        WEPROT_B1_FCFG_BY1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_trim_by1(&self) -> WEPROT_B0_TRIM_BY1_R {
        WEPROT_B0_TRIM_BY1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b1_trim_by1(&self) -> WEPROT_B1_TRIM_BY1_R {
        WEPROT_B1_TRIM_BY1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b0_engr_by1(&self) -> WEPROT_B0_ENGR_BY1_R {
        WEPROT_B0_ENGR_BY1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn weprot_b1_engr_by1(&self) -> WEPROT_B1_ENGR_BY1_R {
        WEPROT_B1_ENGR_BY1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved6(&self) -> RESERVED6_R {
        RESERVED6_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_ccfg_by1(&mut self) -> WEPROT_B0_CCFG_BY1_W<0> {
        WEPROT_B0_CCFG_BY1_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b1_fcfg_by1(&mut self) -> WEPROT_B1_FCFG_BY1_W<1> {
        WEPROT_B1_FCFG_BY1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_trim_by1(&mut self) -> WEPROT_B0_TRIM_BY1_W<2> {
        WEPROT_B0_TRIM_BY1_W::new(self)
    }
    #[doc = "Bit 3 - 3:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b1_trim_by1(&mut self) -> WEPROT_B1_TRIM_BY1_W<3> {
        WEPROT_B1_TRIM_BY1_W::new(self)
    }
    #[doc = "Bit 4 - 4:4\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b0_engr_by1(&mut self) -> WEPROT_B0_ENGR_BY1_W<4> {
        WEPROT_B0_ENGR_BY1_W::new(self)
    }
    #[doc = "Bit 5 - 5:5\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn weprot_b1_engr_by1(&mut self) -> WEPROT_B1_ENGR_BY1_W<5> {
        WEPROT_B1_ENGR_BY1_W::new(self)
    }
    #[doc = "Bits 6:31 - 31:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved6(&mut self) -> RESERVED6_W<6> {
        RESERVED6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [weprot_aux_by1](index.html) module"]
pub struct WEPROT_AUX_BY1_SPEC;
impl crate::RegisterSpec for WEPROT_AUX_BY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [weprot_aux_by1::R](R) reader structure"]
impl crate::Readable for WEPROT_AUX_BY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [weprot_aux_by1::W](W) writer structure"]
impl crate::Writable for WEPROT_AUX_BY1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WEPROT_AUX_BY1 to value 0x3f"]
impl crate::Resettable for WEPROT_AUX_BY1_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
