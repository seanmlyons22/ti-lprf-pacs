#[doc = "Register `AMPCOMPTH2` reader"]
pub struct R(crate::R<AMPCOMPTH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMPCOMPTH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMPCOMPTH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMPCOMPTH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMPCOMPTH2` writer"]
pub struct W(crate::W<AMPCOMPTH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMPCOMPTH2_SPEC>;
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
impl From<crate::W<AMPCOMPTH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMPCOMPTH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPARE0` reader - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE0` writer - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_COMP_AMPTH_HPM` reader - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_HPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_COMP_AMPTH_HPM` writer - 7:2\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_HPM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 6, O>;
#[doc = "Field `SPARE8` reader - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE8` writer - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `ADC_COMP_AMPTH_LPM` reader - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_LPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADC_COMP_AMPTH_LPM` writer - 15:10\\]
Internal. Only to be used through TI provided API."]
pub type ADC_COMP_AMPTH_LPM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 6, O>;
#[doc = "Field `SPARE16` reader - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE16` writer - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPMUPDATE_HTH` reader - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_HTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMUPDATE_HTH` writer - 23:18\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_HTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 6, O>;
#[doc = "Field `SPARE24` reader - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE24_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPARE24` writer - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type SPARE24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LPMUPDATE_LTH` reader - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_LTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPMUPDATE_LTH` writer - 31:26\\]
Internal. Only to be used through TI provided API."]
pub type LPMUPDATE_LTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AMPCOMPTH2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare0(&self) -> SPARE0_R {
        SPARE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_hpm(&self) -> ADC_COMP_AMPTH_HPM_R {
        ADC_COMP_AMPTH_HPM_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare8(&self) -> SPARE8_R {
        SPARE8_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn adc_comp_ampth_lpm(&self) -> ADC_COMP_AMPTH_LPM_R {
        ADC_COMP_AMPTH_LPM_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare16(&self) -> SPARE16_R {
        SPARE16_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpmupdate_hth(&self) -> LPMUPDATE_HTH_R {
        LPMUPDATE_HTH_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn spare24(&self) -> SPARE24_R {
        SPARE24_R::new(((self.bits >> 24) & 3) as u8)
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
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare0(&mut self) -> SPARE0_W<0> {
        SPARE0_W::new(self)
    }
    #[doc = "Bits 2:7 - 7:2\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_comp_ampth_hpm(&mut self) -> ADC_COMP_AMPTH_HPM_W<2> {
        ADC_COMP_AMPTH_HPM_W::new(self)
    }
    #[doc = "Bits 8:9 - 9:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare8(&mut self) -> SPARE8_W<8> {
        SPARE8_W::new(self)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn adc_comp_ampth_lpm(&mut self) -> ADC_COMP_AMPTH_LPM_W<10> {
        ADC_COMP_AMPTH_LPM_W::new(self)
    }
    #[doc = "Bits 16:17 - 17:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare16(&mut self) -> SPARE16_W<16> {
        SPARE16_W::new(self)
    }
    #[doc = "Bits 18:23 - 23:18\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpmupdate_hth(&mut self) -> LPMUPDATE_HTH_W<18> {
        LPMUPDATE_HTH_W::new(self)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn spare24(&mut self) -> SPARE24_W<24> {
        SPARE24_W::new(self)
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
#[doc = "Amplitude Compensation Threshold 2 This register contains threshold values for amplitude compensation algorithm.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ampcompth2](index.html) module"]
pub struct AMPCOMPTH2_SPEC;
impl crate::RegisterSpec for AMPCOMPTH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ampcompth2::R](R) reader structure"]
impl crate::Readable for AMPCOMPTH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ampcompth2::W](W) writer structure"]
impl crate::Writable for AMPCOMPTH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AMPCOMPTH2 to value 0"]
impl crate::Resettable for AMPCOMPTH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
