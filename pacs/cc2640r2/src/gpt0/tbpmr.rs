#[doc = "Register `TBPMR` reader"]
pub struct R(crate::R<TBPMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TBPMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TBPMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TBPMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TBPMR` writer"]
pub struct W(crate::W<TBPMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TBPMR_SPEC>;
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
impl From<crate::W<TBPMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TBPMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TBPSMR` reader - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
pub type TBPSMR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TBPSMR` writer - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
pub type TBPSMR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBPMR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RESERVED8` reader - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RESERVED8` writer - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
pub type RESERVED8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TBPMR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    pub fn tbpsmr(&self) -> TBPSMR_R {
        TBPSMR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    pub fn reserved8(&self) -> RESERVED8_R {
        RESERVED8_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
GPT Timer B Pre-scale Match Register. In 16 bit mode this field holds bits 23 to 16."]
    #[inline(always)]
    #[must_use]
    pub fn tbpsmr(&mut self) -> TBPSMR_W<0> {
        TBPSMR_W::new(self)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Software should not rely on the value of a reserved. Writing any other value than the reset value may result in undefined behavior."]
    #[inline(always)]
    #[must_use]
    pub fn reserved8(&mut self) -> RESERVED8_W<8> {
        RESERVED8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer B Pre-scale Match This register allows software to extend the range of the TBMATCHR when used individually.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tbpmr](index.html) module"]
pub struct TBPMR_SPEC;
impl crate::RegisterSpec for TBPMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tbpmr::R](R) reader structure"]
impl crate::Readable for TBPMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tbpmr::W](W) writer structure"]
impl crate::Writable for TBPMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TBPMR to value 0"]
impl crate::Resettable for TBPMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
