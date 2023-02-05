#[doc = "Register `ANABYPASSVAL1` reader"]
pub struct R(crate::R<ANABYPASSVAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANABYPASSVAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANABYPASSVAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANABYPASSVAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANABYPASSVAL1` writer"]
pub struct W(crate::W<ANABYPASSVAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANABYPASSVAL1_SPEC>;
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
impl From<crate::W<ANABYPASSVAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANABYPASSVAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC_HF_COLUMN_Q12` reader - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_COLUMN_Q12_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XOSC_HF_COLUMN_Q12` writer - 15:0\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_COLUMN_Q12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANABYPASSVAL1_SPEC, u16, u16, 16, O>;
#[doc = "Field `XOSC_HF_ROW_Q12` reader - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_ROW_Q12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XOSC_HF_ROW_Q12` writer - 19:16\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_ROW_Q12_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANABYPASSVAL1_SPEC, u8, u8, 4, O>;
#[doc = "Field `RESERVED20` reader - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED20` writer - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED20_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANABYPASSVAL1_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_column_q12(&self) -> XOSC_HF_COLUMN_Q12_R {
        XOSC_HF_COLUMN_Q12_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_row_q12(&self) -> XOSC_HF_ROW_Q12_R {
        XOSC_HF_ROW_Q12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved20(&self) -> RESERVED20_R {
        RESERVED20_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_column_q12(&mut self) -> XOSC_HF_COLUMN_Q12_W<0> {
        XOSC_HF_COLUMN_Q12_W::new(self)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_row_q12(&mut self) -> XOSC_HF_ROW_Q12_W<16> {
        XOSC_HF_ROW_Q12_W::new(self)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved20(&mut self) -> RESERVED20_W<20> {
        RESERVED20_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Bypass Values 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anabypassval1](index.html) module"]
pub struct ANABYPASSVAL1_SPEC;
impl crate::RegisterSpec for ANABYPASSVAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anabypassval1::R](R) reader structure"]
impl crate::Readable for ANABYPASSVAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anabypassval1::W](W) writer structure"]
impl crate::Writable for ANABYPASSVAL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANABYPASSVAL1 to value 0"]
impl crate::Resettable for ANABYPASSVAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
