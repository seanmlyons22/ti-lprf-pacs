#[doc = "Register `AMPCOMP_TH1` reader"]
pub struct R(crate::R<AMPCOMP_TH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMP_TH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMP_TH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMP_TH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMP_TH1` writer"]
pub struct W(crate::W<AMPCOMP_TH1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMP_TH1_SPEC>;
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
impl From<crate::W<AMPCOMP_TH1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMP_TH1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPMRAMP1_TH` reader - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type HPMRAMP1_TH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPMRAMP1_TH` writer - 5:0\\]
Internal. Only to be used through TI provided API."]
pub type HPMRAMP1_TH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH1_SPEC, u8, u8, 6, O>;
#[doc = "Field `IBIASCAP_LPTOHP_OL_CNT` reader - 9:6\\]
Internal. Only to be used through TI provided API."]
pub type IBIASCAP_LPTOHP_OL_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IBIASCAP_LPTOHP_OL_CNT` writer - 9:6\\]
Internal. Only to be used through TI provided API."]
pub type IBIASCAP_LPTOHP_OL_CNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH1_SPEC, u8, u8, 4, O>;
#[doc = "Field `HPMRAMP3_HTH` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type HPMRAMP3_HTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPMRAMP3_HTH` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type HPMRAMP3_HTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED0` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMP_TH1_SPEC, u8, u8, 2, O>;
#[doc = "Field `HPMRAMP3_LTH` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type HPMRAMP3_LTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HPMRAMP3_LTH` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type HPMRAMP3_LTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH1_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED1` reader - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 31:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMP_TH1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp1_th(&self) -> HPMRAMP1_TH_R {
        HPMRAMP1_TH_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn ibiascap_lptohp_ol_cnt(&self) -> IBIASCAP_LPTOHP_OL_CNT_R {
        IBIASCAP_LPTOHP_OL_CNT_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_hth(&self) -> HPMRAMP3_HTH_R {
        HPMRAMP3_HTH_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn hpmramp3_lth(&self) -> HPMRAMP3_LTH_R {
        HPMRAMP3_LTH_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hpmramp1_th(&mut self) -> HPMRAMP1_TH_W<0> {
        HPMRAMP1_TH_W::new(self)
    }
    #[doc = "Bits 6:9 - 9:6\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn ibiascap_lptohp_ol_cnt(&mut self) -> IBIASCAP_LPTOHP_OL_CNT_W<6> {
        IBIASCAP_LPTOHP_OL_CNT_W::new(self)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hpmramp3_hth(&mut self) -> HPMRAMP3_HTH_W<10> {
        HPMRAMP3_HTH_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<16> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn hpmramp3_lth(&mut self) -> HPMRAMP3_LTH_W<18> {
        HPMRAMP3_LTH_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<24> {
        RESERVED1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_th1](index.html) module"]
pub struct AMPCOMP_TH1_SPEC;
impl crate::RegisterSpec for AMPCOMP_TH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcomp_th1::R](R) reader structure"]
impl crate::Readable for AMPCOMP_TH1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcomp_th1::W](W) writer structure"]
impl crate::Writable for AMPCOMP_TH1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPCOMP_TH1 to value 0xff7b_828e"]
impl crate::Resettable for AMPCOMP_TH1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff7b_828e;
}
