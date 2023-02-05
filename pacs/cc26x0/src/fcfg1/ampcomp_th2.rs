#[doc = "Register `AMPCOMP_TH2` reader"]
pub struct R(crate::R<AMPCOMP_TH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMP_TH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMP_TH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMP_TH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMP_TH2` writer"]
pub struct W(crate::W<AMPCOMP_TH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMP_TH2_SPEC>;
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
impl From<crate::W<AMPCOMP_TH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMP_TH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESERVED0` reader - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED0` writer - 1:0\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_COMP_AMPTH_HPM` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_HPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_COMP_AMPTH_HPM` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_HPM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED1` reader - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED1` writer - 9:8\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_COMP_AMPTH_LPM` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_LPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_COMP_AMPTH_LPM` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_LPM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED2` reader - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED2` writer - 17:16\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPMUPDATE_HTM` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_HTM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMUPDATE_HTM` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_HTM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 6, O>;
#[doc = "Field `RESERVED3` reader - 25:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESERVED3` writer - 25:24\\]
Internal. Only to be used through TI provided API."]
pub type RESERVED3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPMUPDATE_LTH` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_LTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMUPDATE_LTH` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_LTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMP_TH2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved0(&self) -> RESERVED0_R {
        RESERVED0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPM_R {
        ADC_COMP_AMPTH_HPM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPM_R {
        ADC_COMP_AMPTH_LPM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved2(&self) -> RESERVED2_R {
        RESERVED2_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_htm(&self) -> LPMUPDATE_HTM_R {
        LPMUPDATE_HTM_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn reserved3(&self) -> RESERVED3_R {
        RESERVED3_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_lth(&self) -> LPMUPDATE_LTH_R {
        LPMUPDATE_LTH_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved0(&mut self) -> RESERVED0_W<0> {
        RESERVED0_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_comp_ampth_hpm(&mut self) -> ADC_COMP_AMPTH_HPM_W<2> {
        ADC_COMP_AMPTH_HPM_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<8> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_comp_ampth_lpm(&mut self) -> ADC_COMP_AMPTH_LPM_W<10> {
        ADC_COMP_AMPTH_LPM_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> RESERVED2_W<16> {
        RESERVED2_W::new(self)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpmupdate_htm(&mut self) -> LPMUPDATE_HTM_W<18> {
        LPMUPDATE_HTM_W::new(self)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> RESERVED3_W<24> {
        RESERVED3_W::new(self)
    }
    #[doc = "Bits 26:31 - 31:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpmupdate_lth(&mut self) -> LPMUPDATE_LTH_W<26> {
        LPMUPDATE_LTH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcomp_th2](index.html) module"]
pub struct AMPCOMP_TH2_SPEC;
impl crate::RegisterSpec for AMPCOMP_TH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcomp_th2::R](R) reader structure"]
impl crate::Readable for AMPCOMP_TH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcomp_th2::W](W) writer structure"]
impl crate::Writable for AMPCOMP_TH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPCOMP_TH2 to value 0x6b8b_0303"]
impl crate::Resettable for AMPCOMP_TH2_SPEC {
    const RESET_VALUE: Self::Ux = 0x6b8b_0303;
}
