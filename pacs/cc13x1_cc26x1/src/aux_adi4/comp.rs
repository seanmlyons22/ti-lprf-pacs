#[doc = "Register `COMP` reader"]
pub struct R(crate::R<COMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP` writer"]
pub struct W(crate::W<COMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP_SPEC>;
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
impl From<crate::W<COMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPA_EN` reader - 0:0\\]
COMPA enable"]
pub type COMPA_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMPA_EN` writer - 0:0\\]
COMPA enable"]
pub type COMPA_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMP_SPEC, bool, O>;
#[doc = "Field `RESERVED1` reader - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_R = crate::BitReader<bool>;
#[doc = "Field `RESERVED1` writer - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED1_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMP_SPEC, bool, O>;
#[doc = "Field `COMPB_EN` reader - 2:2\\]
COMPB enable"]
pub type COMPB_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMPB_EN` writer - 2:2\\]
COMPB enable"]
pub type COMPB_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMP_SPEC, bool, O>;
#[doc = "Field `LPM_BIAS_WIDTH_TRIM` reader - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type LPM_BIAS_WIDTH_TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPM_BIAS_WIDTH_TRIM` writer - 5:3\\]
Internal. Only to be used through TI provided API."]
pub type LPM_BIAS_WIDTH_TRIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, COMP_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMPA_REF_CURR_EN` reader - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
pub type COMPA_REF_CURR_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMPA_REF_CURR_EN` writer - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
pub type COMPA_REF_CURR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMP_SPEC, bool, O>;
#[doc = "Field `COMPA_REF_RES_EN` reader - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
pub type COMPA_REF_RES_EN_R = crate::BitReader<bool>;
#[doc = "Field `COMPA_REF_RES_EN` writer - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
pub type COMPA_REF_RES_EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, COMP_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
COMPA enable"]
    #[inline(always)]
    pub fn compa_en(&self) -> COMPA_EN_R {
        COMPA_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved1(&self) -> RESERVED1_R {
        RESERVED1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
COMPB enable"]
    #[inline(always)]
    pub fn compb_en(&self) -> COMPB_EN_R {
        COMPB_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn lpm_bias_width_trim(&self) -> LPM_BIAS_WIDTH_TRIM_R {
        LPM_BIAS_WIDTH_TRIM_R::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    pub fn compa_ref_curr_en(&self) -> COMPA_REF_CURR_EN_R {
        COMPA_REF_CURR_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    pub fn compa_ref_res_en(&self) -> COMPA_REF_RES_EN_R {
        COMPA_REF_RES_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
COMPA enable"]
    #[inline(always)]
    #[must_use]
    pub fn compa_en(&mut self) -> COMPA_EN_W<0> {
        COMPA_EN_W::new(self)
    }
    #[doc = "Bit 1 - 1:1\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> RESERVED1_W<1> {
        RESERVED1_W::new(self)
    }
    #[doc = "Bit 2 - 2:2\\]
COMPB enable"]
    #[inline(always)]
    #[must_use]
    pub fn compb_en(&mut self) -> COMPB_EN_W<2> {
        COMPB_EN_W::new(self)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn lpm_bias_width_trim(&mut self) -> LPM_BIAS_WIDTH_TRIM_W<3> {
        LPM_BIAS_WIDTH_TRIM_W::new(self)
    }
    #[doc = "Bit 6 - 6:6\\]
Enables 2uA IPTAT current from ISRC to COMPA reference node. Requires ISRC.EN = 1. Used with COMPA_REF_RES_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    #[must_use]
    pub fn compa_ref_curr_en(&mut self) -> COMPA_REF_CURR_EN_W<6> {
        COMPA_REF_CURR_EN_W::new(self)
    }
    #[doc = "Bit 7 - 7:7\\]
Enables 400kohm resistance from COMPA reference node to ground. Used with COMPA_REF_CURR_EN to generate voltage reference for cap-sense."]
    #[inline(always)]
    #[must_use]
    pub fn compa_ref_res_en(&mut self) -> COMPA_REF_RES_EN_W<7> {
        COMPA_REF_RES_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control COMPA and COMPB comparators. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp](index.html) module"]
pub struct COMP_SPEC;
impl crate::RegisterSpec for COMP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [comp::R](R) reader structure"]
impl crate::Readable for COMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp::W](W) writer structure"]
impl crate::Writable for COMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP to value 0"]
impl crate::Resettable for COMP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
