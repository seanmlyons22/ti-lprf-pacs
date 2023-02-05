#[doc = "Register `TBMATCHR` reader"]
pub struct R(crate::R<TBMATCHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBMATCHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBMATCHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBMATCHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBMATCHR` writer"]
pub struct W(crate::W<TBMATCHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBMATCHR_SPEC>;
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
impl From<crate::W<TBMATCHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBMATCHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBMATCHR` reader - 15:0\\]
GPT Timer B Match Register"]
pub type TBMATCHR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TBMATCHR` writer - 15:0\\]
GPT Timer B Match Register"]
pub type TBMATCHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBMATCHR_SPEC, u16, u16, 16, O>;
#[doc = "Field `RESERVED16` reader - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RESERVED16` writer - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED16_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TBMATCHR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
GPT Timer B Match Register"]
    #[inline(always)]
    pub fn tbmatchr(&self) -> TBMATCHR_R {
        TBMATCHR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved16(&self) -> RESERVED16_R {
        RESERVED16_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
GPT Timer B Match Register"]
    #[inline(always)]
    #[must_use]
    pub fn tbmatchr(&mut self) -> TBMATCHR_W<0> {
        TBMATCHR_W::new(self)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved16(&mut self) -> RESERVED16_W<16> {
        RESERVED16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Match Register When a GPT is configured to one of the 32-bit modes, the contents of bits 15:0 in this register are loaded into the upper 16 bits of TAMATCHR. Reads from this register return the current match value of Timer B and writes are ignored. In a 16-bit mode, bits 15:0 are used for the match value. Bits 31:16 are reserved in both cases. Note : This register is updated internally (takes effect) based on TBMR.TBMRSU\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbmatchr](index.html) module"]
pub struct TBMATCHR_SPEC;
impl crate::RegisterSpec for TBMATCHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbmatchr::R](R) reader structure"]
impl crate::Readable for TBMATCHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbmatchr::W](W) writer structure"]
impl crate::Writable for TBMATCHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBMATCHR to value 0xffff"]
impl crate::Resettable for TBMATCHR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
