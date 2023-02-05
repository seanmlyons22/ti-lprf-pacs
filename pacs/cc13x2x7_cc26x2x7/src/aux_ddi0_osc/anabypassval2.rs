#[doc = "Register `ANABYPASSVAL2` reader"]
pub struct R(crate::R<ANABYPASSVAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANABYPASSVAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANABYPASSVAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANABYPASSVAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANABYPASSVAL2` writer"]
pub struct W(crate::W<ANABYPASSVAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANABYPASSVAL2_SPEC>;
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
impl From<crate::W<ANABYPASSVAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANABYPASSVAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XOSC_HF_IBIASTHERM` reader - 13:0\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_IBIASTHERM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XOSC_HF_IBIASTHERM` writer - 13:0\\]
Internal. Only to be used through TI provided API."]
pub type XOSC_HF_IBIASTHERM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANABYPASSVAL2_SPEC, u16, u16, 14, O>;
#[doc = "Field `RESERVED14` reader - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED14` writer - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED14_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ANABYPASSVAL2_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    pub fn xosc_hf_ibiastherm(&self) -> XOSC_HF_IBIASTHERM_R {
        XOSC_HF_IBIASTHERM_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved14(&self) -> RESERVED14_R {
        RESERVED14_R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
Internal. Only to be used through TI provided API."]
    #[inline(always)]
    #[must_use]
    pub fn xosc_hf_ibiastherm(&mut self) -> XOSC_HF_IBIASTHERM_W<0> {
        XOSC_HF_IBIASTHERM_W::new(self)
    }
    #[doc = "Bits 14:31 - 31:14\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved14(&mut self) -> RESERVED14_W<14> {
        RESERVED14_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal. Only to be used through TI provided API.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anabypassval2](index.html) module"]
pub struct ANABYPASSVAL2_SPEC;
impl crate::RegisterSpec for ANABYPASSVAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anabypassval2::R](R) reader structure"]
impl crate::Readable for ANABYPASSVAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anabypassval2::W](W) writer structure"]
impl crate::Writable for ANABYPASSVAL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ANABYPASSVAL2 to value 0"]
impl crate::Resettable for ANABYPASSVAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
