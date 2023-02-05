#[doc = "Register `PWD_CURR_125C` reader"]
pub struct R(crate::R<PWD_CURR_125C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWD_CURR_125C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWD_CURR_125C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWD_CURR_125C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWD_CURR_125C` writer"]
pub struct W(crate::W<PWD_CURR_125C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWD_CURR_125C_SPEC>;
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
impl From<crate::W<PWD_CURR_125C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWD_CURR_125C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASELINE` reader - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
pub type BASELINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BASELINE` writer - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
pub type BASELINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWD_CURR_125C_SPEC, u8, u8, 8, O>;
#[doc = "Field `DELTA_XOSC_LPM` reader - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
pub type DELTA_XOSC_LPM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA_XOSC_LPM` writer - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
pub type DELTA_XOSC_LPM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWD_CURR_125C_SPEC, u8, u8, 8, O>;
#[doc = "Field `DELTA_RFMEM_RET` reader - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
pub type DELTA_RFMEM_RET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA_RFMEM_RET` writer - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
pub type DELTA_RFMEM_RET_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWD_CURR_125C_SPEC, u8, u8, 8, O>;
#[doc = "Field `DELTA_CACHE_REF` reader - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
pub type DELTA_CACHE_REF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DELTA_CACHE_REF` writer - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
pub type DELTA_CACHE_REF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWD_CURR_125C_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
    #[inline(always)]
    pub fn baseline(&self) -> BASELINE_R {
        BASELINE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
    #[inline(always)]
    pub fn delta_xosc_lpm(&self) -> DELTA_XOSC_LPM_R {
        DELTA_XOSC_LPM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
    #[inline(always)]
    pub fn delta_rfmem_ret(&self) -> DELTA_RFMEM_RET_R {
        DELTA_RFMEM_RET_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
    #[inline(always)]
    pub fn delta_cache_ref(&self) -> DELTA_CACHE_REF_R {
        DELTA_CACHE_REF_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Worst-case baseline maximum powerdown current, in units of 0.5uA"]
    #[inline(always)]
    #[must_use]
    pub fn baseline(&mut self) -> BASELINE_W<0> {
        BASELINE_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Additional maximum current, in units of 1uA, with XOSC_HF on in low-power mode"]
    #[inline(always)]
    #[must_use]
    pub fn delta_xosc_lpm(&mut self) -> DELTA_XOSC_LPM_W<8> {
        DELTA_XOSC_LPM_W::new(self)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Additional maximum current, in 1uA units, with RF memory retention"]
    #[inline(always)]
    #[must_use]
    pub fn delta_rfmem_ret(&mut self) -> DELTA_RFMEM_RET_W<16> {
        DELTA_RFMEM_RET_W::new(self)
    }
    #[doc = "Bits 24:31 - 31:24\\]
Additional maximum current, in units of 1uA, with cache retention"]
    #[inline(always)]
    #[must_use]
    pub fn delta_cache_ref(&mut self) -> DELTA_CACHE_REF_W<24> {
        DELTA_CACHE_REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Down Current Control 125C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwd_curr_125c](index.html) module"]
pub struct PWD_CURR_125C_SPEC;
impl crate::RegisterSpec for PWD_CURR_125C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwd_curr_125c::R](R) reader structure"]
impl crate::Readable for PWD_CURR_125C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwd_curr_125c::W](W) writer structure"]
impl crate::Writable for PWD_CURR_125C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWD_CURR_125C to value 0xade1_809a"]
impl crate::Resettable for PWD_CURR_125C_SPEC {
    const RESET_VALUE: Self::Ux = 0xade1_809a;
}
