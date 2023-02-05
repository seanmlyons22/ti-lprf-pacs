#[doc = "Register `OSC_CONF1` reader"]
pub struct R(crate::R<OSC_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CONF1` writer"]
pub struct W(crate::W<OSC_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CONF1_SPEC>;
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
impl From<crate::W<OSC_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCOSC_MF_BIAS_ADJ` reader - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_BIAS_ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_MF_BIAS_ADJ` writer - 3:0\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_BIAS_ADJ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED4` reader - 25:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED4` writer - 25:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF1_SPEC, u32, u32, 22, O>;
#[doc = "Field `RCOSC_MF_SINGLE_TRIM_METHOD` reader - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_SINGLE_TRIM_METHOD_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_MF_SINGLE_TRIM_METHOD` writer - 26:26\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_SINGLE_TRIM_METHOD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OSC_CONF1_SPEC, bool, O>;
#[doc = "Field `RCOSC_MF_TEMP_DEPEND_MODE` reader - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_TEMP_DEPEND_MODE_R = crate::BitReader<bool>;
#[doc = "Field `RCOSC_MF_TEMP_DEPEND_MODE` writer - 27:27\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_TEMP_DEPEND_MODE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, OSC_CONF1_SPEC, bool, O>;
#[doc = "Field `RCOSC_MF_BIAS_HTEMP` reader - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_BIAS_HTEMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCOSC_MF_BIAS_HTEMP` writer - 31:28\\]
Internal. Only to be used through TI provided API."]
pub type RCOSC_MF_BIAS_HTEMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONF1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_mf_bias_adj(&self) -> RCOSC_MF_BIAS_ADJ_R {
        RCOSC_MF_BIAS_ADJ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:25 - 25:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved4(&self) -> RESERVED4_R {
        RESERVED4_R::new((self.bits >> 4) & 0x003f_ffff)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_mf_single_trim_method(&self) -> RCOSC_MF_SINGLE_TRIM_METHOD_R {
        RCOSC_MF_SINGLE_TRIM_METHOD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_mf_temp_depend_mode(&self) -> RCOSC_MF_TEMP_DEPEND_MODE_R {
        RCOSC_MF_TEMP_DEPEND_MODE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn rcosc_mf_bias_htemp(&self) -> RCOSC_MF_BIAS_HTEMP_R {
        RCOSC_MF_BIAS_HTEMP_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_bias_adj(&mut self) -> RCOSC_MF_BIAS_ADJ_W<0> {
        RCOSC_MF_BIAS_ADJ_W::new(self)
    }
    #[doc = "Bits 4:25 - 25:4\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved4(&mut self) -> RESERVED4_W<4> {
        RESERVED4_W::new(self)
    }
    #[doc = "Bit 26 - 26:26\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_single_trim_method(&mut self) -> RCOSC_MF_SINGLE_TRIM_METHOD_W<26> {
        RCOSC_MF_SINGLE_TRIM_METHOD_W::new(self)
    }
    #[doc = "Bit 27 - 27:27\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_temp_depend_mode(&mut self) -> RCOSC_MF_TEMP_DEPEND_MODE_W<27> {
        RCOSC_MF_TEMP_DEPEND_MODE_W::new(self)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn rcosc_mf_bias_htemp(&mut self) -> RCOSC_MF_BIAS_HTEMP_W<28> {
        RCOSC_MF_BIAS_HTEMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_conf1](index.html) module"]
pub struct OSC_CONF1_SPEC;
impl crate::RegisterSpec for OSC_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_conf1::R](R) reader structure"]
impl crate::Readable for OSC_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_conf1::W](W) writer structure"]
impl crate::Writable for OSC_CONF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC_CONF1 to value 0xffff_0000"]
impl crate::Resettable for OSC_CONF1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_0000;
}
