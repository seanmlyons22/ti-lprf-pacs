#[doc = "Register `FREQ_OFFSET` reader"]
pub struct R(crate::R<FREQ_OFFSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREQ_OFFSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREQ_OFFSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREQ_OFFSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREQ_OFFSET` writer"]
pub struct W(crate::W<FREQ_OFFSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREQ_OFFSET_SPEC>;
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
impl From<crate::W<FREQ_OFFSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREQ_OFFSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HF_COMP_P2` reader - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_P2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HF_COMP_P2` writer - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_P2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FREQ_OFFSET_SPEC, u8, u8, 8, O>;
#[doc = "Field `HF_COMP_P1` reader - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_P1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HF_COMP_P1` writer - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_P1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FREQ_OFFSET_SPEC, u8, u8, 8, O>;
#[doc = "Field `HF_COMP_P0` reader - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_P0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HF_COMP_P0` writer - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
pub type HF_COMP_P0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FREQ_OFFSET_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p2(&self) -> HF_COMP_P2_R {
        HF_COMP_P2_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p1(&self) -> HF_COMP_P1_R {
        HF_COMP_P1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    pub fn hf_comp_p0(&self) -> HF_COMP_P0_R {
        HF_COMP_P0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn hf_comp_p2(&mut self) -> HF_COMP_P2_W<0> {
        HF_COMP_P2_W::new(self)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn hf_comp_p1(&mut self) -> HF_COMP_P1_W<8> {
        HF_COMP_P1_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved for future use. Software should not rely on the value of a reserved. Writing any other value than the reset/default value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn hf_comp_p0(&mut self) -> HF_COMP_P0_W<16> {
        HF_COMP_P0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frequency Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [freq_offset](index.html) module"]
pub struct FREQ_OFFSET_SPEC;
impl crate::RegisterSpec for FREQ_OFFSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [freq_offset::R](R) reader structure"]
impl crate::Readable for FREQ_OFFSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [freq_offset::W](W) writer structure"]
impl crate::Writable for FREQ_OFFSET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FREQ_OFFSET to value 0xffff_ffff"]
impl crate::Resettable for FREQ_OFFSET_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
